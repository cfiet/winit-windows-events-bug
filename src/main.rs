extern crate winit;

use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder,
    platform::desktop::EventLoopExtDesktop
};

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let event_loop_proxy: EventLoopProxy<()> = event_loop.create_proxy();
    
    let close = Arc::new(AtomicBool::new(false));
    let c1 = close.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        {
            // Notify to close
            close.store(true, Ordering::Relaxed);
        }
    });

    thread::spawn(move || {
        // Event spammer
        loop {
            event_loop_proxy.send_event(()).unwrap();
        }
    });

    event_loop.run_return(move |event, _, control_flow| {
        // Artificailly slowing down so spammer always overproduces events
        thread::sleep(Duration::from_millis(1));

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => *control_flow = ControlFlow::Wait,
        }

        let should_close = c1.load(Ordering::Relaxed);
        if should_close {
            *control_flow = ControlFlow::Exit
        }
    });
}
