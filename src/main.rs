extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Minecraft Map Viewer", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    //Parsing

    //View
    const TILE_SIZE: f64 = 16.0;

    let mut scale: f64 = 1.0;

    #[derive(Clone)]
    struct Tile {
        block_id: i32,
    }

    //Textures    
    let resource_pack = find_folder::Search::Parents(3)
        .for_folder("mc-resource-pack")
        .unwrap();

    let ref mut texture_context = window.create_texture_context();
    let tile_texture = resource_pack.join("assets/minecraft/textures/block").join("acacia_log_top.png");
    let tile_texture = Texture::from_path(
        texture_context,
        &tile_texture,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let (width, _) = tile_texture.get_size();
    let image = Image::new();

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.5; 4], g);
                for i in 0..10 {
                    let x = i as f64 * TILE_SIZE * scale;
                    let y = TILE_SIZE * scale;
                    image.rect(rectangle::square(x, y, TILE_SIZE * scale)).draw(
                        &tile_texture,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }

            });
        }
        if let Some(m) = e.mouse_scroll_args() {
            if m[1] > 0.0 {
                scale += 0.1;
            }
            else {
                if scale > 0.0 {
                    scale -= 0.1;
                }
            }
        }
    }
}