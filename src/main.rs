mod framebuffer;
mod line;
mod fill;

use fill::fill_polygon;
use framebuffer::Framebuffer;
use line::{line, draw_polygon};
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::WHITE);
    framebuffer.clear();

    // Polígono 1
    let poly1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)]
        .into_iter()
        .map(|(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect::<Vec<_>>();
    framebuffer.set_current_color(Color::YELLOW);
    fill_polygon(&mut framebuffer, &poly1);
    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &poly1);

    framebuffer.render_to_file("out.png");
}
