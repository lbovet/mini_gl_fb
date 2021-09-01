#[macro_use]
extern crate mini_gl_fb;

use std::time::{Duration, Instant};

use glutin::event::VirtualKeyCode;
use mini_gl_fb::BufferFormat;
use mini_gl_fb::glutin::event_loop::EventLoop;
use mini_gl_fb::glutin::dpi::LogicalSize;

use opencv::{
    core::{Mat, Vec3b},
    prelude::*,
    Result,
    videoio,
};

fn main() -> Result<()> {

    #[cfg(ocvrs_opencv_branch_32)]
    let mut cam = videoio::VideoCapture::new_default(0)?; // 0 is the default camera
    #[cfg(not(ocvrs_opencv_branch_32))]
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
            panic!("Unable to open default camera!");
    }

    let mut event_loop = EventLoop::new();
    let mut fb = mini_gl_fb::get_fancy(config! {
        window_title: String::from("Hello world!"),
        window_size: LogicalSize::new(800.0, 600.0),
        buffer_size: Some(LogicalSize::new(640, 480)),
        invert_y: false,
        resizable: false
    }, &event_loop);

    fb.change_buffer_format::<u8>(BufferFormat::RGBA);
    let mut buffer = vec![0; 640*480*4];

    let mut update_id: Option<u32> = None;

    fb.glutin_handle_basic_input(&mut event_loop, |fb, input| {
        input.wait = true;

        if update_id.is_none() {
            update_id = Some(input.schedule_wakeup(Instant::now() + Duration::from_millis(5)))
        } else if let Some(mut wakeup) = input.wakeup {
            if Some(wakeup.id) == update_id {
                let mut frame = Mat::default();
                cam.read(&mut frame);
                let frame_data_vec = Mat::data_typed::<Vec3b>(&frame);
                let mut offset = 0;

                if let Ok(data) = frame_data_vec {
                    for pixel in data {
                        let pixel_slice = &**pixel;
                            buffer[offset] = pixel_slice[2];
                            buffer[offset+1] = pixel_slice[1];
                            buffer[offset+2] = pixel_slice[0];
                            buffer[offset+3] = 128;
                            offset += 4;
                    }
                }

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
    Ok(())
}
