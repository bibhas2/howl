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

	println!("wnd_proc called for HWND: {}", window as i32);

    if let Some(handler) = window.get_event_handler() {
        if handler.on_event(message, w_param, l_param) {
		    return 0;
	    }
    }

    return user32::DefWindowProcW(window, message, w_param, l_param);
}

struct Application;

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
        loop {
            unsafe {
                let status = user32::GetMessageW(&mut message, ptr::null_mut(), 0, 0);
                if status == 0 {
                    break;
                }

                user32::TranslateMessage(&message);
                user32::DispatchMessageW(&message);
            }
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
}

impl Window for winapi::HWND {
	fn get_hwnd(&self) -> winapi::HWND {
		return *self;
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

    fn on_close(&mut self) {
		println!("Window closed.");
	}

	fn on_event(&mut self, message : winapi::UINT,  w_param : winapi::WPARAM,  l_param : winapi::LPARAM) -> bool {
		match message {
			winapi::WM_SIZE => {
				self.on_size(winapi::LOWORD(l_param as winapi::DWORD), winapi::HIWORD(l_param as winapi::DWORD));
			},
			winapi::WM_COMMAND => {
				self.on_command(winapi::LOWORD(w_param as winapi::DWORD), winapi::HIWORD(w_param as winapi::DWORD));
			},
            winapi::WM_MOVE => {
				self.on_move(winapi::GET_X_LPARAM(l_param), winapi::GET_Y_LPARAM(l_param));
			},
            winapi::WM_CLOSE => {
				self.on_close();
			},
			_ => {
				return false;
			}
		}

        return true;
	}
}

pub struct MyMainWindow {
	main_window: winapi::HWND,
	button: winapi::HWND,
	is_shown: bool
}

impl WindowEventHandler for MyMainWindow {
	fn on_command(&mut self, source_id: u16, command_type: u16) {
		println!("MyMainWindow got command from: {}.", source_id);
		if source_id == 10 {
			if self.is_shown {
				self.button.hide();
			} else {
				self.button.show();
			}
			self.is_shown = !self.is_shown;
		}
	}
}

impl MyMainWindow {
	pub fn new() -> MyMainWindow {
		let mut wnd = WindowBuilder::new()
			.frame("My Main Window")
			.size(200, 200)
            .create();
		let btn = WindowBuilder::new()
			.checkbox("Show")
			.position(10, 10)
			.size(95, 20)
			.parent(wnd)
			.id(10)
			.create();

		let btn = WindowBuilder::new()
			.button("Press me")
			.position(10, 40)
			.size(95, 50)
			.parent(wnd)
			.id(20)
			.create();

		let mut w = MyMainWindow {
			main_window: wnd,
			button: btn,
			is_shown: true
		};

		return w;
	}
}

fn main() {
	unsafe {
		Application::init();

		let mut wnd = MyMainWindow::new();
        wnd.main_window.attach_event_handler(&wnd);

		wnd.main_window.show();

        Application::main_loop();
    }
}
