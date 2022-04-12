use pollster;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use wgpu;

use super::context::*;
use super::layout::Layout;
use super::panel::Panel;
use crate::prim::{Pnt2, Rect};

pub struct Window {
    layout: Layout,
    panels: Vec<Box<dyn Panel>>,
}

impl Window {
    pub fn new() -> Window {
        Window {
            layout: Layout::new(Rect::from_corner(
                Pnt2::new(-1.0, -1.0),
                Pnt2::new(1.0, 1.0),
            )),
            panels: vec![],
        }
    }

    pub fn append(&mut self, panel: Box<dyn Panel>) {
        self.panels.push(panel);
        self.layout.grow();
    }

    pub fn size(&self) -> usize {
        self.panels.len()
    }

    pub fn run(mut self) {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let context = pollster::block_on(ContextGlobal::init_wgpu(&window));
        self.panels.iter_mut().for_each(|x| x.init(&context));

        event_loop.run(move |event, _, contrl_flow| {
            *contrl_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *contrl_flow = ControlFlow::Exit,
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    self.render(&context);
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }

    pub fn render(&self, context_global: &ContextGlobal) {
        let output = context_global.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let context = Context {
            global: &context_global,
            local: ContextFrame { view: view },
        };

        context_global.queue.submit(
            self.panels
                .iter()
                .zip(self.layout.leaves())
                .map(|(panel, rect)| panel.render(rect, &context)),
        );
        output.present();
    }
}
