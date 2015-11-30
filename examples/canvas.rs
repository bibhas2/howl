extern crate howl;
extern crate winapi;
extern crate user32;
extern crate gdi32;

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
        let c = Canvas::new(&frame, 10, 10, 400, 400);

        frame.show();

        SimpleDrawingApp {
            frame: frame,
            canvas: c
        }
    }

    pub fn draw_canvas(&self, dc : winapi::HDC) {

        let mut width = 0i32;
        let mut height = 0i32;

        self.canvas.size(&mut width, &mut height);

        let rect = [
            winapi::POINT {
                x: 0,
                y: 0
            },
            winapi::POINT {
                x: width - 1,
                y: 0
            },
            winapi::POINT {
                x: width - 1,
                y: height - 1
            },
            winapi::POINT {
                x: 0,
                y: height - 1
            }
        ];

        unsafe {
            //let pen = gdi32::CreatePen(winapi::PS_SOLID, 5, 0);
            gdi32::SelectObject(dc, gdi32::GetStockObject(winapi::BLACK_PEN));
            gdi32::Polygon(dc, rect.as_ptr(), rect.len() as i32);
            //gdi32::DeleteObject(pen);
        }

    }
}

impl WindowEventHandler for SimpleDrawingApp {
    fn on_close(&mut self) {
        Application::exit_loop();
    }
    fn on_size(&mut self, width: i32, height: i32) {

		let margin = 10i32;

        self.canvas.resize(margin, margin, width - margin * 2, height - margin * 2);

        println!("Frame size: {} {}", width, height);
	}
}

struct DrawingInputHandler<'a> {
    app: &'a SimpleDrawingApp
}

impl <'a> DrawingInputHandler<'a> {
    pub fn new(app: &SimpleDrawingApp) -> DrawingInputHandler {
        DrawingInputHandler{
            app: app
        }
    }
}

impl <'a> WindowEventHandler for DrawingInputHandler<'a> {
    fn on_left_mouse_down(&mut self, x: i32, y: i32) {
        println!("Drawing mouse input: {}, {}", x, y);
    }
    fn on_event(&mut self, window: winapi::HWND, message : winapi::UINT,  w_param : winapi::WPARAM, l_param : winapi::LPARAM) -> bool {
        //println!("on_event called for HWND: {} message: {:?}", window as i32, message);

        match message {
            winapi::WM_PAINT => {
                let mut ps = winapi::PAINTSTRUCT {
                    hdc: 0 as winapi::HDC,
                    fErase: 0,
                    rcPaint: winapi::RECT {
                        top: 0,
                        left: 0,
                        right: 0,
                        bottom: 0
                    },
                    fRestore: 0,
                    fIncUpdate: 0,
                    rgbReserved: [0; 32]
                };
                unsafe {
                    user32::BeginPaint(self.app.canvas.get_hwnd(), &mut ps);
    			    self.app.draw_canvas(ps.hdc);
    			    user32::EndPaint(self.app.canvas.get_hwnd(), &mut ps);
                }
			},
            _ => {
                return self.dispatch_event(window, message,  w_param, l_param);
            }
        }

        return true;
    }
}

fn main() {
	Application::init();

	let my_app = SimpleDrawingApp::new();

    my_app.frame.attach_event_handler(&my_app);

    let input_handler = DrawingInputHandler::new(&my_app);

    my_app.canvas.attach_event_handler(&input_handler);
    my_app.frame.resize(10, 10, 400, 400);

	//my_app.main_window.show();

    Application::main_loop();
}
