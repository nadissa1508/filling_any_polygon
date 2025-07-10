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

    // Polígono 2
    let poly2 = vec![(321, 335), (288, 286), (339, 251), (374, 302)]
        .into_iter()
        .map(|(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect::<Vec<_>>();
    framebuffer.set_current_color(Color::BLUE);
    fill_polygon(&mut framebuffer, &poly2);
    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &poly2);

    // Polígono 3
    let poly3 = vec![(377, 249), (411, 197), (436, 249)]
        .into_iter()
        .map(|(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect::<Vec<_>>();
    framebuffer.set_current_color(Color::RED);
    fill_polygon(&mut framebuffer, &poly3);
    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &poly3);

    // Polígono 4
    let poly4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36),
        (676, 37), (660, 52), (750, 145), (761, 179), (672, 192),
        (659, 214), (615, 214), (632, 230), (580, 230), (597, 215),
        (552, 214), (517, 144), (466, 180)
        ].into_iter()
        .map(|(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect::<Vec<_>>();
    framebuffer.set_current_color(Color::GREEN);
    fill_polygon(&mut framebuffer, &poly4);
    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &poly4);

    // El agujero 
    let poly5 = vec![(682, 175), (708, 120), (735, 148), (739, 170)]
        .into_iter()
        .map(|(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect::<Vec<_>>();
    framebuffer.set_current_color(Color::WHITE); 
    fill_polygon(&mut framebuffer, &poly5);

    framebuffer.render_to_file("out.png");
}
