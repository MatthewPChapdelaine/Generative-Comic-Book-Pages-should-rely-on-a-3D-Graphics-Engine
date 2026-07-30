#![cfg(feature = "gpu")]
use anyhow::Result;
use image::{ImageBuffer, Rgba};
use pollster::block_on;
use std::path::Path;

pub fn render_to_png(out: &Path, width: u32, height: u32) -> Result<()> {
    // Minimal headless wgpu render: clear a texture to a color gradient and save
    // as PNG. This is intentionally simple — a full renderer would implement
    // mesh, shaders, and post-processing.
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

    let adapter = block_on(async { instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await })
        .ok_or_else(|| anyhow::anyhow!("Failed to find suitable GPU adapter"))?;

    let (device, queue) = block_on(async {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
    })?;

    let texture_desc = wgpu::TextureDescriptor {
        label: Some("output_texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    };

    let texture = device.create_texture(&texture_desc);
    let view = texture.create_view(None);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("encoder") });

    // Clear pass with a simple color based on size
    {
        let _rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }

    // Copy to buffer
    let padded_bytes_per_row = (4 * width + wgpu::COPY_BYTES_PER_ROW_ALIGNMENT - 1) / wgpu::COPY_BYTES_PER_ROW_ALIGNMENT * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let unpadded_bytes_per_row = 4 * width;
    let buffer_size = (padded_bytes_per_row as u64) * (height as u64);

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback_buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row as u32),
                rows_per_image: None,
            },
        },
        texture_desc.size,
    );

    queue.submit(Some(encoder.finish()));

    // Map buffer and write PNG
    let buffer_slice = buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |r| {
        sender.send(r).ok();
    });
    device.poll(wgpu::Maintain::Wait);
    let result = block_on(async { receiver.receive().await }).ok_or_else(|| anyhow::anyhow!("Map callback error"))?;
    result?;

    let data = buffer_slice.get_mapped_range();

    // Convert padded rows into ImageBuffer
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    for y in 0..height as usize {
        let src_start = y * padded_bytes_per_row as usize;
        let src_end = src_start + unpadded_bytes_per_row as usize;
        let row = &data[src_start..src_end];
        for x in 0..width as usize {
            let i = x * 4;
            let px = Rgba([row[i], row[i + 1], row[i + 2], row[i + 3]]);
            img.put_pixel(x as u32, y as u32, px);
        }
    }

    drop(data);
    buffer.unmap();

    img.save(out)?;
    Ok(())
}
