#[macro_use]
extern crate mini_gl_fb;

use mini_gl_fb::BufferFormat;
use mini_gl_fb::glutin::event_loop::EventLoop;
use mini_gl_fb::glutin::dpi::LogicalSize;

fn main() {
    let mut event_loop = EventLoop::new();
    let mut fb = mini_gl_fb::get_fancy(config! {
        window_title: String::from("Hello world!"),
        window_size: LogicalSize::new(800.0, 600.0),
        buffer_size: Some(LogicalSize::new(2, 2)),
        resizable: false
    }, &event_loop);

    fb.change_buffer_format::<u8>(BufferFormat::RGBA);

    let mut buffer = vec![[128u8, 0, 0, 34]; 4];
    buffer[3] = [0, 0, 0, 0];
    fb.update_buffer(&buffer);

    // This can also be configured at creation
    // fb.set_resizable(true);

    fb.persist(&mut event_loop);
}
