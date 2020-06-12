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
    let mut drag_active: bool = false;
    //todo make 2dvector
    let mut drag_offset: [f64; 2] = [0.0, 0.0];

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

    let image = Image::new();

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.5; 4], g);
                for i in 0..10 {
                    let x = i as f64 * TILE_SIZE * scale + drag_offset[0];
                    let y = TILE_SIZE * scale + drag_offset[1];
                    image.rect(rectangle::square(x, y, TILE_SIZE * scale)).draw(
                        &tile_texture,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }

            });
        }
        if let Some(ms) = e.mouse_scroll_args() {
            if ms[1] > 0.0 {
                scale += 0.1;
            }
            else {
                if scale > 0.0 {
                    scale -= 0.1;
                }
            }
        }
        if let Some(m) = e.mouse_relative_args() {
            println!("{:?}", m);
            if drag_active {
                drag_offset[0] += m[0];
                drag_offset[1] += m[1];
            }
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            drag_active = true;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            drag_active = false;
        };
    }
}