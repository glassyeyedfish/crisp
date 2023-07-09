mod chip8;
mod log;

use std::num::NonZeroU32;

use log::Log;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use softbuffer::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WIDTH as u32, HEIGHT as u32))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let context = unsafe { Context::new(&window) }.unwrap();
    let mut surface = unsafe { Surface::new(&context, &window) }.unwrap();
    surface
        .resize(
            NonZeroU32::new(window.inner_size().width).unwrap(),
            NonZeroU32::new(window.inner_size().height).unwrap(),
        )
        .unwrap();

    let mut log = Log::new();
    log.log(b"Hello, world!");
    log.log(b"Goodbye!");

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
                let mut buffer = surface.buffer_mut().unwrap();
                for i in 0..(WIDTH * HEIGHT) {
                    buffer[i] = 0x00444444;
                }
                for x in 80..(WIDTH - 80) {
                    for y in 140..(HEIGHT - 140) {
                        buffer[x + y * WIDTH] = 0x00eeeeee;
                    }
                }
                buffer.present().unwrap();
            }
            _ => (),
        }
    });
}
