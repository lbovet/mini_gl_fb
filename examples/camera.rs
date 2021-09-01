#[macro_use]
extern crate mini_gl_fb;

use std::time::{Duration, Instant};

use glutin::event::VirtualKeyCode;
use mini_gl_fb::BufferFormat;
use mini_gl_fb::glutin::event_loop::EventLoop;
use mini_gl_fb::glutin::dpi::LogicalSize;

use nokhwa::{Camera, CameraFormat, FrameFormat, CaptureAPIBackend};


fn main() {

    let mut camera = Camera::new(
        0, // index
        Some(CameraFormat::new_from(640, 480, FrameFormat::YUYV, 30)),
        CaptureAPIBackend::Video4Linux,
    )
    .unwrap();
    camera.open_stream().unwrap();

    let mut event_loop = EventLoop::new();
    let mut fb = mini_gl_fb::get_fancy(config! {
        window_title: String::from("Hello world!"),
        window_size: LogicalSize::new(800.0, 600.0),
        buffer_size: Some(LogicalSize::new(640, 480)),
        invert_y: false,
        resizable: false
    }, &event_loop);

    fb.change_buffer_format::<u8>(BufferFormat::RGB);
    let mut buffer = vec![0; 640*480*3];

    let mut update_id: Option<u32> = None;

    fb.glutin_handle_basic_input(&mut event_loop, |fb, input| {
        input.wait = true;

        if update_id.is_none() {
            update_id = Some(input.schedule_wakeup(Instant::now() + Duration::from_millis(5)))
        } else if let Some(mut wakeup) = input.wakeup {
            if Some(wakeup.id) == update_id {
                camera.frame_to_buffer(&mut buffer, false);
                fb.update_buffer(&buffer);

                wakeup.when = Instant::now() + Duration::from_millis(5);

                input.reschedule_wakeup(wakeup);
            }

            // We will get called again after all wakeups are handled
            return true;
        }

        if input.key_is_down(VirtualKeyCode::Escape) {
            return false;
        }

        true
    });

    fb.persist(&mut event_loop);
}
