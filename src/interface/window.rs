use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster;

use super::context::Context;
use super::panel::Panel;

pub struct Window {
    panel: Panel,
}

impl Window {
    pub fn new() -> Window {
        Window {
            panel: Panel::new(),
        }
    }

    pub fn run(self) {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let context = pollster::block_on(Context::init_wgpu(&window));

        event_loop.run(move |event, _, contrl_flow| {
            *contrl_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *contrl_flow = ControlFlow::Exit,
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    self.panel.render(&context);
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
