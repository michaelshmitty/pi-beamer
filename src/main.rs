extern crate sdl2;

use std::path::Path;
use sdl2::image::{LoadTexture, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const SLIDES: &'static [&'static str] = &["slide1.jpg", "slide2.jpg", "slide3.jpg"];
const SLIDE_INTERVAL: u32 = 600;

fn main()
{
    let mut current_slide = 0;
    let mut ticks = 0;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_JPG).unwrap();
    let window = video_subsystem.window("SDL2", WIDTH, HEIGHT).position_centered()
        .fullscreen().build().unwrap();

    // Hide the cursor
    sdl_context.mouse().show_cursor(false);

    let mut canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.load_texture(Path::new("assets").join(SLIDES[current_slide])).unwrap();

    let mut query = texture.query();
    let mut image_with = query.width;
    let mut image_height = query.height;

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::MouseButtonDown{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }

        if ticks >= SLIDE_INTERVAL
        {
            ticks = 0;
            if current_slide >= 2
            {
                current_slide = 0;
            }
            else
            {
                current_slide = current_slide + 1;
            }
            // Switch slides
            texture = texture_creator.load_texture(
                Path::new("assets").join(SLIDES[current_slide])
            ).unwrap();
            query = texture.query();
            image_with = query.width;
            image_height = query.height;
        }
        else
        {
            ticks = ticks + 1;
        }

        canvas.clear();
        let mut dest_rect = Rect::new(0, 0, image_with, image_height);
        dest_rect.center_on(Point::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32));
        canvas.copy(&texture, None, Some(dest_rect)).unwrap();
        canvas.present();
    }
}
