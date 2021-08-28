use simple_logger::SimpleLogger;
use std::fmt::Debug;
use winit::{
    event::{ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub enum KeymojiInputEvent {
    KeyPressed(VirtualKeyCode),
}

pub trait ViewStateModel {
    fn update(&self, input: KeymojiInputEvent) -> Self;
}

pub fn run_window<T>(init: T)
where
    T: ViewStateModel + Debug + 'static,
{
    SimpleLogger::new().env().init().unwrap();
    let mut model = init;
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .with_title("Keymoji Selector")
        .build(&event_loop)
        .expect("failed to initialize window");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                *control_flow = ControlFlow::Wait;
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Released,
                            ..
                        },
                    ..
                } => match virtual_code {
                    k => {
                        model = model.update(KeymojiInputEvent::KeyPressed(virtual_code));
                        println!("updated model: {:?}", model);
                    }
                },
                _ => (),
            },
            _ => (),
        }
    });
}
