extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub fn to_wchar(str : &str) -> Vec<u16> {
    OsStr::new(str).encode_wide(). chain(Some(0).into_iter()).collect()
}

unsafe fn get_instance() -> winapi::HINSTANCE {
    let instance = kernel32::GetModuleHandleW(ptr::null());
    if instance.is_null() {
        panic!("GetModuleHandleW error: {}", kernel32::GetLastError());
    }

    instance
}

unsafe fn register_class(class_name : &str, wnd_proc: winapi::WNDPROC) {
    let class = winapi::WNDCLASSW {
        style: winapi::CS_HREDRAW | winapi::CS_VREDRAW | winapi::CS_DBLCLKS,
        lpfnWndProc: wnd_proc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: get_instance(),
        hIcon: ptr::null_mut(),
        hCursor: ptr::null_mut(),
        hbrBackground: winapi::COLOR_WINDOW as winapi::HBRUSH,
        lpszMenuName: ptr::null_mut(),
        lpszClassName: to_wchar(class_name).as_ptr()
    };
    let atom = user32::RegisterClassW(&class);
    if atom == 0 {
        panic!("RegisterClassW error: {}", kernel32::GetLastError());
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
	parent: winapi::HWND
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
			parent: 0 as winapi::HWND
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
		        ptr::null_mut(),
		        get_instance(),
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
}

impl Window for winapi::HWND {
	fn get_hwnd(&self) -> winapi::HWND {
		return *self;
	}
}

fn main() {
	unsafe {
		let class_name = "HOWL";
		register_class(class_name, Some(user32::DefWindowProcW));
		
		let wnd = WindowBuilder::new()
			.class_name("HOWL")
			.size(500, 500)
			.title("Cool World")
			.style(winapi::WS_THICKFRAME | winapi::WS_MINIMIZEBOX | winapi::WS_MAXIMIZEBOX | winapi::WS_SYSMENU)
			.extra_style(winapi::WS_EX_CLIENTEDGE)
			.create();
		
		let btn = WindowBuilder::new()
			.checkbox("Press me")
			.position(10, 10)
			.size(95, 50)
			.parent(wnd)
			.create();
		
		//user32::ShowWindow(wnd, 5);
		wnd.show();
		wnd.show();
		
		btn.set_text("I am changed");
		
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
	        let status = user32::GetMessageW(&mut message, ptr::null_mut(), 0, 0);
	        if status == 0 {
	            break;
	        }
	
	        user32::TranslateMessage(&message);
	        user32::DispatchMessageW(&message);
	    }
	}
}