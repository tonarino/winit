#![allow(clippy::single_match)]

use raw_window_handle::{HasRawDisplayHandle, RawDisplayHandle, XlibWindowHandle};
use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    platform::x11::{WindowBuilderExtX11, WindowExtX11},
    window::WindowBuilder,
};
use x11_dl::xlib;

#[path = "util/fill.rs"]
mod fill;

fn main() {
    let screen_number = std::env::args()
        .nth(1)
        .expect("please specify screen number to run on as first commandline argument")
        .parse()
        .expect("could not parse first commandline argument as i32");

    SimpleLogger::new().env().init().unwrap();
    let event_loop = EventLoop::new();

    // Extract connection fro X from winit't event loop. 🤷
    let display_handle = event_loop.raw_display_handle();
    let RawDisplayHandle::Xlib(xlib_display_handle) = display_handle else {
        eprintln!("{display_handle:?} was not XlibDisplayHandle");
        return;
    };
    let display: *mut xlib::_XDisplay = xlib_display_handle.display.cast();

    // Load Xlib library.
    let xlib = xlib::Xlib::open().unwrap();
    log::debug!(
        "X server we connected to has {} screens (using `XScreenCount(display)`).",
        unsafe { (xlib.XScreenCount)(display) }
    );

    let root_window_id = unsafe { (xlib.XRootWindow)(display, screen_number) };
    log::debug!(
        "id of root window of screen {} is {}.",
        screen_number,
        root_window_id
    );

    let mut root_window_handle = XlibWindowHandle::empty();
    root_window_handle.window = root_window_id;
    dbg!(root_window_handle);

    let window_builder = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(500.0, 500.0))
        .with_x11_screen(screen_number);
    // We need to specify matching root window (every X screen has its own one):
    let window_builder =
        unsafe { window_builder.with_parent_window(Some(root_window_handle.into())) };
    let window = window_builder.build(&event_loop).unwrap();

    dbg!(window.xlib_screen_id());

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
            Event::RedrawRequested(_) => {
                fill::fill_window(&window);
            }
            _ => (),
        }
    });
}
