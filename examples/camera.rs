#[macro_use]
extern crate mini_gl_fb;

use std::{
    convert::TryInto,
    time::{Duration, Instant},
};

use glutin::event::VirtualKeyCode;
use mini_gl_fb::glutin::dpi::LogicalSize;
use mini_gl_fb::glutin::event_loop::EventLoop;
use mini_gl_fb::BufferFormat;

use opencv::{
    core::{Mat, Vec3b},
    prelude::*,
    videoio, Result,
};

fn main() -> Result<()> {
    let width: usize = 640;
    let height: usize = 480;
    let size: usize = width * height * 3;

    #[cfg(ocvrs_opencv_branch_32)]
    let mut cam = videoio::VideoCapture::new_default(0)?; // 0 is the default camera
    #[cfg(not(ocvrs_opencv_branch_32))]
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut event_loop = EventLoop::new();
    let mut fb = mini_gl_fb::get_fancy(
        config! {
            window_title: String::from("Hello world!"),
            window_size: LogicalSize::new(800.0, 600.0),
            buffer_size: Some(LogicalSize::new(width.try_into().unwrap(), height.try_into().unwrap())),
            invert_y: false,
            resizable: false
        },
        &event_loop,
    );

    fb.change_buffer_format::<u8>(BufferFormat::BGR);
    let mut buffer: Vec<u8> = vec![128; size];

    let mut update_id: Option<u32> = None;

    fb.glutin_handle_basic_input(&mut event_loop, |fb, input| {
        input.wait = true;

        if update_id.is_none() {
            update_id = Some(input.schedule_wakeup(Instant::now() + Duration::from_millis(10)))
        } else if let Some(mut wakeup) = input.wakeup {
            if Some(wakeup.id) == update_id {
                let mut frame = Mat::default();
                if let Ok(true) = cam.read(&mut frame) {
                    let b = &mut *buffer;
                    unsafe {
                        match Mat::data_typed_unchecked::<u8>(&frame.reshape(1, 1).unwrap()) {
                            Ok(data) => {
                                b[0..size].copy_from_slice(data)
                            }
                            Err(why) => panic!("{}", why),
                        }
                    }
                    fb.update_buffer(&buffer);
                }

                wakeup.when = Instant::now() + Duration::from_millis(5);
                input.reschedule_wakeup(wakeup);
            }

            // We will get called again after all wakeups are handled
            return true;
        }

        if input.key_is_down(VirtualKeyCode::Escape) {
            panic!("Bye")
        }

        true
    });

    fb.persist(&mut event_loop);
    Ok(())
}
