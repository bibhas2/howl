#![feature(raw)]

extern crate kernel32;
extern crate user32;
extern crate winapi;
extern crate libc;


use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem;
use std::raw;

pub fn to_wchar(str : &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect()
}

unsafe extern "system" fn wnd_proc(
    window: winapi::HWND,
    message: winapi::UINT,
    w_param: winapi::WPARAM,
    l_param: winapi::LPARAM) -> winapi::LRESULT {

	//println!("wnd_proc called for HWND: {}", window as i32);

    if let Some(handler) = window.get_event_handler() {
        if handler.on_event(window, message, w_param, l_param) {
		    return 0;
	    }
    }

    return user32::DefWindowProcW(window, message, w_param, l_param);
}

unsafe extern "system" fn timer_proc(
    window: winapi::HWND,
    message: winapi::UINT,
    id: winapi::UINT_PTR,
    not_used: winapi::DWORD) {

	//println!("timer_proc called for HWND: {}", window as i32);

    if let Some(handler) = window.get_event_handler() {
        handler.on_timer(window, id as usize);
    }
}

struct Application;
static mut continue_loop: bool = false;

impl Application {
    fn init() {
        let class_name = "HOWL";

        Application::register_class(class_name, Some(wnd_proc));
    }

    fn get_instance() -> winapi::HINSTANCE {
        unsafe {
            let instance = kernel32::GetModuleHandleW(ptr::null());
            if !instance.is_null() {
                return instance;
            }

            panic!("GetModuleHandleW error: {}", kernel32::GetLastError());
        }
    }

    fn register_class(class_name : &str, wnd_proc: winapi::WNDPROC) {
        let class = winapi::WNDCLASSW {
            style: winapi::CS_HREDRAW | winapi::CS_VREDRAW | winapi::CS_DBLCLKS,
            lpfnWndProc: wnd_proc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: Application::get_instance(),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: winapi::COLOR_WINDOW as winapi::HBRUSH,
            lpszMenuName: ptr::null_mut(),
            lpszClassName: to_wchar(class_name).as_ptr()
        };
        unsafe {
            let atom = user32::RegisterClassW(&class);
            if atom == 0 {
                panic!("RegisterClassW error: {}", kernel32::GetLastError());
            }
        }
    }

    fn main_loop() {
        let mut message = winapi::MSG {
            hwnd: ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: winapi::POINT {
                x: 0,
                y: 0
            }
        };

        unsafe {
            continue_loop = true;

            while continue_loop {
                    let status = user32::GetMessageW(&mut message, ptr::null_mut(), 0, 0);
                    if status == 0 {
                        break;
                    }

                    user32::TranslateMessage(&message);
                    user32::DispatchMessageW(&message);
            }
        }
    }

    fn exit_loop() {
        unsafe {
            continue_loop = false;
        }
    }
}

pub struct WindowBuilder<'a> {
	x: i32,
	y: i32,
	width: i32,
	height: i32,
	style: winapi::DWORD,
	extra_style: winapi::DWORD,
	class_name: &'a str,
	title: &'a str,
	parent: winapi::HWND,
	id: u16
}

impl <'a> WindowBuilder<'a> {
	pub fn new() -> WindowBuilder<'a> {
		WindowBuilder {
			x: winapi::CW_USEDEFAULT,
			y: winapi::CW_USEDEFAULT,
			width: winapi::CW_USEDEFAULT,
			height: winapi::CW_USEDEFAULT,
			style: 0,
			extra_style: 0,
			class_name: "",
			title: "",
			parent: 0 as winapi::HWND,
			id: 0
		}
	}
	pub fn style(&'a mut self, style: winapi::DWORD) -> &mut WindowBuilder {
		self.style = style;

		self
	}
	pub fn extra_style(&'a mut self, extra_style: winapi::DWORD) -> &mut WindowBuilder {
		self.extra_style = extra_style;

		self
	}
	pub fn parent(&'a mut self, parent: winapi::HWND) -> &mut WindowBuilder {
		self.parent = parent;

		self
	}
	pub fn position(&'a mut self, x: i32, y: i32) -> &mut WindowBuilder {
		self.x = x;
		self.y = y;

		self
	}
	pub fn size(&'a mut self, width: i32, height: i32) -> &mut WindowBuilder {
		self.width = width;
		self.height = height;

		self
	}

	pub fn id(&'a mut self, id: u16) -> &mut WindowBuilder {
		self.id = id;

		self
	}

	pub fn class_name(&'a mut self, class_name: &'a str) -> &mut WindowBuilder {
		self.class_name = class_name;

		self
	}
	pub fn title(&'a mut self, title: &'a str) -> &mut WindowBuilder {
		self.title = title;

		self
	}

	pub fn button(&'a mut self, title: &'a str) -> &mut WindowBuilder {
		self.title(title)
		.class_name("BUTTON")
		.style(winapi::WS_VISIBLE | winapi::WS_TABSTOP | winapi::WS_CHILD | winapi::BS_PUSHBUTTON)
	}

	pub fn checkbox(&'a mut self, title: &'a str) -> &mut WindowBuilder {
		self.title(title)
		.class_name("BUTTON")
		.style(winapi::WS_VISIBLE | winapi::WS_TABSTOP | winapi::WS_CHILD | winapi::BS_CHECKBOX)
	}

	pub fn frame(&'a mut self, title: &'a str) -> &mut WindowBuilder {
		self.title(title)
		.class_name("HOWL")
		.style(winapi::WS_THICKFRAME | winapi::WS_MINIMIZEBOX | winapi::WS_MAXIMIZEBOX | winapi::WS_SYSMENU)
		.extra_style(winapi::WS_EX_CLIENTEDGE)
	}

	pub fn create(&self) -> winapi::HWND {
		unsafe {
		    let window =  user32::CreateWindowExW(
		        self.extra_style,
		        to_wchar(self.class_name).as_ptr(),
		        to_wchar(self.title).as_ptr(),
		        self.style,
		        self.x,
		        self.y,
		        self.width,
		        self.height,
		        self.parent,
		        self.id as winapi::HMENU,
		        Application::get_instance(),
		        ptr::null_mut());

		    if window.is_null() {
		        panic!("CreateWindowExW error: {}", kernel32::GetLastError());
		    }

		    window
		}
	}
}

pub trait Window {
	fn get_hwnd(&self) -> winapi::HWND;

	fn show(&self) {
		unsafe {
			user32::ShowWindow(self.get_hwnd(), 5);
		}
	}

	fn hide(&self) {
		unsafe {
			user32::ShowWindow(self.get_hwnd(), 0);
		}
	}

	fn set_text(&self, txt : &str) {
		unsafe {
			user32::SendMessageW(self.get_hwnd(), winapi::WM_SETTEXT, 0, to_wchar(txt).as_ptr() as winapi::LPARAM);
		}
	}

    fn attach_event_handler(&self, handler: &WindowEventHandler) {
        unsafe {
            let raw_obj: raw::TraitObject = mem::transmute(handler);

            let prop = to_wchar("cwnd.data");
            user32::SetPropW(self.get_hwnd(), prop.as_ptr(), raw_obj.data as *mut libc::c_void);
            let prop = to_wchar("cwnd.vtable");
            user32::SetPropW(self.get_hwnd(), prop.as_ptr(), raw_obj.vtable as *mut libc::c_void);
        }
    }

    fn detach_event_handler(&mut self) {
        unsafe {
            let prop = to_wchar("cwnd.data");
            user32::RemovePropW(self.get_hwnd(), prop.as_ptr());
            let prop = to_wchar("cwnd.vtable");
            user32::RemovePropW(self.get_hwnd(), prop.as_ptr());
        }
    }

    fn get_event_handler(&self) -> Option<&mut WindowEventHandler> {
        unsafe {
            let prop = to_wchar("cwnd.data");
            let data = user32::GetPropW(self.get_hwnd(), prop.as_ptr());
            let prop = to_wchar("cwnd.vtable");
            let vtable = user32::GetPropW(self.get_hwnd(), prop.as_ptr());

            if data == ptr::null_mut() || vtable == ptr::null_mut() {
                println!("wnd_proc Can not find attached data for HWND: {}.", self.get_hwnd() as i32);
                return None;
            }

            let synthesized: &mut WindowEventHandler =
                mem::transmute(raw::TraitObject {
                 data: data as *const _ as *mut (),
                 vtable: vtable as *const _ as *mut ()
                });

            return Some(synthesized);
        }
    }

    fn set_timer(&mut self, id : usize, interval : usize) {
        unsafe {
            user32::SetTimer(self.get_hwnd(), id as winapi::UINT_PTR, interval as winapi::UINT, Some(timer_proc));
        }
    }
    fn kill_timer(&mut self, id : usize) {
        unsafe {
            user32::KillTimer(self.get_hwnd(), id as winapi::UINT_PTR);
        }
    }
    fn message_box(&self, msg: &str) {
        let msg = to_wchar(msg);
        let title = to_wchar("Message");

        unsafe {
    	       user32::MessageBoxW(self.get_hwnd(), msg.as_ptr(), title.as_ptr(), winapi::MB_OK);
        }
    }
    fn error_box(&self, msg: &str) {
        let msg = to_wchar(msg);
        let title = to_wchar("Error");

        unsafe {
    	       user32::MessageBoxW(self.get_hwnd(), msg.as_ptr(), title.as_ptr(), winapi::MB_OK | winapi::MB_ICONERROR);
        }
    }

    fn question_box(&self, msg: &str) -> bool {
        let msg = to_wchar(msg);
        let title = to_wchar("Question");

        unsafe {
    	       return user32::MessageBoxW(self.get_hwnd(), msg.as_ptr(), title.as_ptr(),
                winapi::MB_YESNO | winapi::MB_ICONQUESTION) == 6; //winapi::IDYES
        }
    }

    fn confirm_box(&self, msg: &str) -> bool {
        let msg = to_wchar(msg);
        let title = to_wchar("Confirm");

        unsafe {
    	       return user32::MessageBoxW(self.get_hwnd(), msg.as_ptr(), title.as_ptr(),
                winapi::MB_OKCANCEL | winapi::MB_ICONINFORMATION) == 1; //winapi::IDOK
        }
    }
}

impl Window for winapi::HWND {
	fn get_hwnd(&self) -> winapi::HWND {
		return *self;
	}
}

struct Button {
    window : winapi::HWND
}

impl Window for Button {
    fn get_hwnd(&self) -> winapi::HWND {
		return self.window;
	}
}

impl Button {
    fn new(parent: &Window, id: u16, title: &str, x: i32, y: i32, width: i32, height: i32) -> Button {
        let wnd = WindowBuilder::new()
            .title(title)
            .class_name("BUTTON")
            .style(winapi::WS_VISIBLE | winapi::WS_TABSTOP | winapi::WS_CHILD | winapi::BS_PUSHBUTTON)
            .position(x, y)
            .size(width, height)
            .parent(parent.get_hwnd())
            .id(id)
            .create();
        Button {
            window: wnd
        }
    }
}

struct Checkbox {
    window : winapi::HWND
}

impl Window for Checkbox {
    fn get_hwnd(&self) -> winapi::HWND {
		return self.window;
	}
}

impl Checkbox {
    fn new(parent: &Window, id: u16, title: &str, x: i32, y: i32, width: i32, height: i32) -> Checkbox {
        let wnd = WindowBuilder::new()
            .title(title)
            .class_name("BUTTON")
            .style(winapi::WS_VISIBLE | winapi::WS_TABSTOP | winapi::WS_CHILD | winapi::BS_CHECKBOX)            .position(x, y)
            .size(width, height)
            .parent(parent.get_hwnd())
            .id(id)
            .create();
        Checkbox {
            window: wnd
        }
    }
}

struct Frame {
    window : winapi::HWND
}

impl Window for Frame {
    fn get_hwnd(&self) -> winapi::HWND {
		return self.window;
	}
}

impl Frame {
    fn new(title: &str, width: i32, height: i32) -> Frame {
        let wnd = WindowBuilder::new()
            .title(title)
            .class_name("HOWL")
            .style(winapi::WS_THICKFRAME | winapi::WS_MINIMIZEBOX | winapi::WS_MAXIMIZEBOX | winapi::WS_SYSMENU)
    		.extra_style(winapi::WS_EX_CLIENTEDGE)
            .size(width, height)
            .create();
        Frame {
            window: wnd
        }
    }
}

pub trait WindowEventHandler {
	fn on_command(&mut self, source_id: u16, command_type: u16) {
		println!("Window got command from: {}.", source_id);
	}

	fn on_size(&mut self, width: u16, height: u16) {
		println!("Window resized. {} {}.", width, height);
	}

    fn on_move(&mut self, x: i32, y: i32) {
		println!("Window moved. {} {}.", x, y);
	}

    fn on_mouse_move(&mut self, x: i32, y: i32) {
		println!("Mouse moved. {} {}.", x, y);
	}

    fn on_right_mouse_down(&mut self, x: i32, y: i32) {
		println!("Right mouse down. {} {}.", x, y);
	}

    fn on_right_mouse_up(&mut self, x: i32, y: i32) {
		println!("Right mouse up. {} {}.", x, y);
	}

    fn on_left_mouse_down(&mut self, x: i32, y: i32) {
		println!("Left mouse down. {} {}.", x, y);
	}

    fn on_left_mouse_up(&mut self, x: i32, y: i32) {
		println!("Left mouse up. {} {}.", x, y);
	}

    fn on_close(&mut self) {
		println!("Window closed.");
	}

    fn on_destroy(&mut self) {
		println!("Window destroyed.");
	}

    fn on_timer(&mut self, window: winapi::HWND, id : usize) {
        println!("Timer fired.");
    }

	fn on_event(&mut self, window: winapi::HWND, message : winapi::UINT,  w_param : winapi::WPARAM, l_param : winapi::LPARAM) -> bool {
		match message {
			winapi::WM_SIZE => {
				self.on_size(winapi::LOWORD(l_param as winapi::DWORD), winapi::HIWORD(l_param as winapi::DWORD));
			},
			winapi::WM_COMMAND => {
				self.on_command(winapi::LOWORD(w_param as winapi::DWORD), winapi::HIWORD(w_param as winapi::DWORD));
			},
            winapi::WM_MOUSEMOVE => {
				self.on_mouse_move(winapi::GET_X_LPARAM(l_param), winapi::GET_Y_LPARAM(l_param));
			},
            winapi::WM_RBUTTONDOWN => {
				self.on_right_mouse_down(winapi::GET_X_LPARAM(l_param), winapi::GET_Y_LPARAM(l_param));
			},
            winapi::WM_RBUTTONUP => {
				self.on_right_mouse_up(winapi::GET_X_LPARAM(l_param), winapi::GET_Y_LPARAM(l_param));
			},
            winapi::WM_LBUTTONDOWN => {
				self.on_left_mouse_down(winapi::GET_X_LPARAM(l_param), winapi::GET_Y_LPARAM(l_param));
			},
            winapi::WM_LBUTTONUP => {
				self.on_left_mouse_up(winapi::GET_X_LPARAM(l_param), winapi::GET_Y_LPARAM(l_param));
			},
            winapi::WM_MOVE => {
				self.on_move(winapi::GET_X_LPARAM(l_param), winapi::GET_Y_LPARAM(l_param));
			},
            winapi::WM_CLOSE => {
				self.on_close();
			},
            winapi::WM_DESTROY => {
				self.on_destroy();
			},
			_ => {
				return false;
			}
		}

        return true;
	}
}

pub struct MyApplication {
	main_window: Frame,
	button: Button,
	is_shown: bool
}

impl WindowEventHandler for MyApplication {
	fn on_command(&mut self, source_id: u16, command_type: u16) {
		println!("MyApplication got command from: {}.", source_id);
		if source_id == 10 {
			if self.is_shown {
				self.button.hide();
			} else {
				self.button.show();
			}
			self.is_shown = !self.is_shown;
		} else {
            if self.main_window.question_box("Do you want to exit?") {
                Application::exit_loop();
            }
        }
	}

    fn on_close(&mut self) {
        if self.main_window.confirm_box("Do you want to exit?") {
            Application::exit_loop();
        }
    }
}

impl MyApplication {
	pub fn new() -> MyApplication {
		let wnd = Frame::new("My Main Window", 200, 200);

        Checkbox::new(&wnd, 10, "Show", 10, 10, 95, 20);

		let btn = Button::new(&wnd, 20, "Press me", 10, 40, 95, 50);

		MyApplication {
			main_window: wnd,
			button: btn,
			is_shown: true
		}
	}
}

fn main() {
	Application::init();

	let my_app = MyApplication::new();

    my_app.main_window.attach_event_handler(&my_app);
	my_app.main_window.show();

    Application::main_loop();
}
