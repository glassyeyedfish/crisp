mod chip8;

use std::{
    num::NonZeroU32,
    time::{Duration, SystemTime},
};

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use softbuffer::*;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_resizable(false)
        .build(&event_loop)
        .expect("[error] Failed to create a window!");

    let context =
        unsafe { Context::new(&window) }.expect("[error] Failed to create a graphics context!");

    let mut surface = unsafe { Surface::new(&context, &window) }
        .expect("[error] Failed to create a drawing surface!");

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                if window_id == window.id() {
                    *control_flow = ControlFlow::Exit
                }
            }
            Event::MainEventsCleared => {
                // Screen stuff
                let width = window.inner_size().width;
                let height = window.inner_size().height;
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for i in 0..(width * height) {
                    buffer[i as usize] = 0x00444444;
                }
                for x in 80..(width - 80) {
                    for y in 140..(height - 140) {
                        buffer[(x + y * width) as usize] = 0x00eeeeee;
                    }
                }
                buffer.present().unwrap();
            }
            _ => (),
        }
    });
}
