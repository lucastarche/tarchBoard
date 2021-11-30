use std::{fs, path::Path};

use eframe::{egui, epi};

pub fn load_image<P: AsRef<Path>>(
    frame: &mut epi::Frame<'_>,
    path: P,
) -> anyhow::Result<(egui::Vec2, egui::TextureId)> {
    // Load the image:
    let image_data = fs::read(path)?;
    use image::GenericImageView;
    let image = image::load_from_memory(&image_data).expect("Failed to load image");
    let image_buffer = image.to_rgba8();
    let size = (image.width() as usize, image.height() as usize);
    let pixels = image_buffer.into_vec();
    assert_eq!(size.0 * size.1 * 4, pixels.len());
    let pixels: Vec<_> = pixels
        .chunks_exact(4)
        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    // Allocate a texture:
    let texture = frame
        .tex_allocator()
        .alloc_srgba_premultiplied(size, &pixels);
    let size = egui::Vec2::new(size.0 as f32, size.1 as f32);
    Ok((size, texture))
}
