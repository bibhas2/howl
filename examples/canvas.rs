extern crate howl;

use howl::Frame;
use howl::Canvas;
use howl::Application;
use howl::Window;
use howl::WindowEventHandler;

struct SimpleDrawingApp {
    frame: Frame,
    canvas: Canvas
}

impl SimpleDrawingApp {
    pub fn new() -> SimpleDrawingApp {
        let frame = Frame::new("Drawing Area", 400, 400);
        let c = Canvas::new(&frame, 0, 0, 400, 400);

        frame.show();

        SimpleDrawingApp {
            frame: frame,
            canvas: c
        }
    }
}

impl WindowEventHandler for SimpleDrawingApp {
}

fn main() {
	Application::init();

	let my_app = SimpleDrawingApp::new();

    my_app.frame.attach_event_handler(&my_app);
    my_app.canvas.attach_event_handler(&my_app);

	//my_app.main_window.show();

    Application::main_loop();
}
