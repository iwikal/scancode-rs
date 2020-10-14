extern crate glutin;
extern crate scancode;

use std::collections::BTreeMap;
use scancode::Scancode;

fn main() {
    let mut errors = BTreeMap::new();
    let events_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .build_windowed(window_builder, &events_loop).unwrap();
    let context = unsafe { context.make_current().unwrap() };
    // println!("Pixel format of the window: {:?}", window.get_pixel_format());
    // let context = support::load(&window);

    events_loop.run(move |event, _, control_flow| {
        let _ = context.swap_buffers();

        let event = match event {
            glutin::event::Event::WindowEvent { event, .. } => event,
            _ => return,
        };

        match event {
            glutin::event::WindowEvent::KeyboardInput {
                input: glutin::event::KeyboardInput {
                    state,
                    scancode,
                    virtual_keycode: vk,
                    ..
                },
                ..
            }=> {
                if state == glutin::event::ElementState::Pressed {
                    print!("pressed")
                } else {
                    print!("released")
                }

                print!(" {} -> ", scancode);
                if let Some(code) = Scancode::new(scancode as u8) {
                    print!("{:?}", code);
                } else {
                    print!("*** ERROR: UNKNOWN SCANCODE ***");
                    errors.insert(scancode, format!("{:?}", vk));
                }

                if let Some(vk) = vk {
                    println!(" (virtual keycode {:?})", vk);
                } else {
                    println!(" (virtual keycode UNKNOWN)");
                }
            }
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;

                if !errors.is_empty() {
                    println!("Unhandled scancodes:");
                    for (k, v) in errors.iter() {
                        println!("{}: {}", k, v);
                    }
                }
            }
            _ => (),
        }
    });
}
