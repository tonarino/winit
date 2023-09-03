#![allow(clippy::single_match)]

use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    platform::unix::WindowBuilderExtUnix,
    window::WindowBuilder,
};

fn main() {
    let screen_number = std::env::args()
        .nth(1)
        .expect("please specify screen number to run on as first commandline argument")
        .parse()
        .expect("could not parse first commandline argument as i32");

    SimpleLogger::new().env().init().unwrap();
    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(500.0, 500.0))
        .with_x11_screen(screen_number);
    let window = window_builder.build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
