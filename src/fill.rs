use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

/// Llena un polígono utilizando el algoritmo de scanline.
pub fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vector2]) {
    let height = framebuffer.height as i32;

    // Convierte puntos a enteros
    let points: Vec<(i32, i32)> = vertices.iter()
        .map(|v| (v.x.round() as i32, v.y.round() as i32))
        .collect();

    // Encuentra el ymin y ymax del polígono
    let min_y = points.iter().map(|&(_, y)| y).min().unwrap_or(0).max(0);
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap_or(0).min(height - 1);

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            // Ordena los extremos verticalmente
            let (x0, y0, x1, y1) = if y0 < y1 { (x0, y0, x1, y1) } else { (x1, y1, x0, y0) };

            if y0 <= y && y < y1 {
                // Calcula intersección en X usando interpolación lineal
                let t = (y - y0) as f32 / (y1 - y0) as f32;
                let x = x0 as f32 + t * (x1 - x0) as f32;
                intersections.push(x.round() as i32);
            }
        }

        intersections.sort();

        // Rellenar entre pares de intersecciones
        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                let x_start = pair[0].max(0);
                let x_end = pair[1].min(framebuffer.width as i32 - 1);
                for x in x_start..=x_end {
                    framebuffer.set_pixel(x as u32, y as u32);
                }
            }
        }
    }
}
