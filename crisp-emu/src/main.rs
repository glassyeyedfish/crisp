use std::num::NonZeroU32;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use softbuffer::*;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let context = unsafe { Context::new(&window) }.unwrap();
    let mut surface = unsafe { Surface::new(&context, &window) }.unwrap();

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
                    buffer[i as usize] = 0x00FFFFFF;
                }
                buffer.present().unwrap();
            }
            _ => (),
        }
    });
}
