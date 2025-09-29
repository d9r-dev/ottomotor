use tracing::{event, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::{Window, WindowId};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};

#[derive(Default)]
struct App {
    window: Option<Window>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event{
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => (),
        }
    }
}

fn main()  -> Result<(), Box<dyn std::error::Error>> {

    initialize_logging();

    let span = tracing::span!(Level::INFO, "main");
    let _enter = span.enter();
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event!(Level::INFO, "Starting event loop");
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn initialize_logging() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let file_appender = tracing_appender::rolling::hourly("./logs", "engine.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
    let stdout_layer = tracing_subscriber::fmt::layer().with_writer(non_blocking);
    let file_layer = tracing_subscriber::fmt::layer().with_writer(non_blocking_file);
    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();
}
