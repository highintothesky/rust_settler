extern crate piston;
extern crate piston_window;
extern crate graphics;
// extern crate glutin_window;
// extern crate opengl_graphics;
extern crate find_folder;
extern crate tiled;

// use piston::window::WindowSettings;
use piston_window::*;
use piston::event_loop::*;
// use piston::input::*;
// use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::fs::File;
use tiled::parse;

pub mod units;
use units::map::Map as Map;
// use units;
// use units::mod::map;

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    image: Image,
    layer: &'a tiled::Layer,
    tile_width: u32,
    tile_height: u32,
    width: u32,
    tilesheet: Texture<piston::Key>,
    window: PistonWindow
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0,
                      args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // draw tiles
            for (yy, row) in self.layer.tiles.iter().enumerate().clone() {
               for (xx, &tile) in row.iter().enumerate() {
                   if tile == 0 {
                       continue;
                   }

                   let tile = tile - 1; // tiled counts from 1

                   // rect of the particular tile in the tilesheet
                   let src_rect = [
                       (tile % (self.width / self.tile_width) * self.tile_width) as f64,
                       (tile / (self.width / self.tile_height) * self.tile_height) as f64,
                       self.tile_width as f64,
                       self.tile_height as f64,
                   ];

                   let trans = c.transform.trans(
                       xx as f64 * self.tile_width as f64,
                       yy as f64 * self.tile_height as f64,
                   );

                   self.image.src_rect(src_rect).draw(
                       &self.tilesheet,
                       &DrawState::default(),
                       trans,
                       gl,
                   );
               }
            }

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });

        // self.gl.draw_2d(&e, |c, g| {
        //             clear([0.5; 4], g);
        // });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // make a map
    let mut mymap: Map = units::map::Map::new(
        20.0,
        40.0
    );
    mymap.getmap();

    // Load map tiles etc
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let file = File::open(assets.join("tiled_base64_zlib.tmx")).unwrap();
    let tilemap = parse(file).unwrap();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new(
            "spinning-square",
            [2000, 2000]
        )
        .graphics_api(opengl)
        // .opengl(opengl)
        .exit_on_esc(true)
        .fullscreen(true)
        .build()
        .unwrap();
        // .fullscreeen(true);

    let tileset = tilemap.get_tileset_by_gid(1).unwrap();
    let tile_width = tileset.tile_width;
    let tile_height = tileset.tile_height;

    let tilesheet = assets.join(&tileset.images[0].source);
    let tilesheet = Texture::from_path(
        &mut window.create_texture_context(),
        &tilesheet,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let (width, _) = tilesheet.get_size();
    let layer: &tiled::Layer = &tilemap.layers[0];
    let image = Image::new();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        image: image,
        layer: layer,
        tile_width: tile_width,
        tile_height: tile_height,
        width: width,
        tilesheet: tilesheet,
        window: window
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
