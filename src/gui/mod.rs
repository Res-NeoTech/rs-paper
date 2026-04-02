use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::{
    event::*,
    event_loop::EventLoop,
};
use glium::Surface;

mod window;

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("RsPaper")
        .build(&event_loop);

    let mut imgui = imgui::Context::create();
    let mut platform = WinitPlatform::new(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        HiDpiMode::Default,
    );
    let mut renderer = imgui_glium_renderer::Renderer::new(&mut imgui, &display).unwrap();

    event_loop.run(move |event, elwt| {
        platform.handle_event(imgui.io_mut(), &window, &event);
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        platform.prepare_frame(imgui.io_mut(), &window).expect("Failed to prepare frame");
                        let ui = imgui.frame();
                        window::Window::build(&ui);
                        
                        let mut target = display.draw();
                        target.clear_color(0.0, 0.0, 0.0, 1.0);
                        renderer
                            .render(&mut target, imgui.render())
                            .expect("Rendering failed");
                        target.finish().unwrap();
                    }
                    _ => (),
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        }
    }).unwrap();
}
