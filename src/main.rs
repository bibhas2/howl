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

    fn resize(&self, x : i32, y : i32, width : i32, height : i32) {
        unsafe {
            user32::MoveWindow(self.get_hwnd(), x, y, width, height, 1);
        }
    }

    fn get_text_length(&self) -> i32 {
		unsafe {
			return user32::SendMessageW(self.get_hwnd(), winapi::WM_GETTEXTLENGTH, 0, 0);
		}
	}

	fn set_text(&self, txt : &str) {
		unsafe {
			user32::SendMessageW(self.get_hwnd(), winapi::WM_SETTEXT, 0, to_wchar(txt).as_ptr() as winapi::LPARAM);
		}
	}
    
    fn get_text(&self) -> String {
        let size = self.get_text_length() + 1; //Win32 adds the NULL. So we need one extra space
        let mut v : Vec<u16> = Vec::with_capacity(size as usize);

		unsafe {
            v.set_len((size - 1) as usize);
			user32::SendMessageW(self.get_hwnd(), winapi::WM_GETTEXT, size as winapi::WPARAM, v.as_ptr() as winapi::LPARAM);
		}

        return String::from_utf16(&v[..]).unwrap();
	}

    fn attach_event_handler(&self, handler: &WindowEventHandler) {
        unsafe {
            let raw_obj: raw::TraitObject = mem::transmute(handler);

            let prop = to_wchar("cwnd.data");
            user32::SetPropW(self.get_hwnd(), prop.as_ptr(), raw_obj.data as winapi::HANDLE);
            let prop = to_wchar("cwnd.vtable");
            user32::SetPropW(self.get_hwnd(), prop.as_ptr(), raw_obj.vtable as winapi::HANDLE);
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

struct Edit {
    window : winapi::HWND
}

impl Window for Edit {
    fn get_hwnd(&self) -> winapi::HWND {
		return self.window;
	}
}

impl Edit {
    fn new(parent: &Window, x: i32, y: i32, width: i32, height: i32, multi_line:bool) -> Edit {
        let ES_LEFT             = 0x0000;
        let ES_CENTER           = 0x0001;
        let ES_RIGHT            = 0x0002;
        let ES_MULTILINE        = 0x0004;
        let ES_UPPERCASE        = 0x0008;
        let ES_LOWERCASE        = 0x0010;
        let ES_PASSWORD         = 0x0020;
        let ES_AUTOVSCROLL      = 0x0040;
        let ES_AUTOHSCROLL      = 0x0080;
        let ES_NOHIDESEL        = 0x0100;
        let ES_OEMCONVERT       = 0x0400;
        let ES_READONLY         = 0x0800;
        let ES_WANTRETURN       = 0x1000;

        let mut style : winapi::DWORD  = winapi::WS_CHILD | winapi::WS_VISIBLE;

    	if multi_line {
    		style = style | winapi::WS_VSCROLL |
                ES_WANTRETURN | ES_LEFT | ES_MULTILINE | ES_AUTOVSCROLL;
        }


        let wnd = WindowBuilder::new()
            .class_name("EDIT")
            .style(style)
            .position(x, y)
            .size(width, height)
            .parent(parent.get_hwnd())
            .create();
        Edit {
            window: wnd
        }
    }
}

impl Edit {
    pub fn set_read_only(&self, read_only : bool) {
        let EM_SETREADONLY = 0x00CF;

        unsafe {
            user32::SendMessageW(self.window, EM_SETREADONLY, if read_only {1} else {0}, 0);
        }
    }
}

pub struct ListBox {
    window : winapi::HWND
}

impl Window for ListBox {
    fn get_hwnd(&self) -> winapi::HWND {
		return self.window;
	}
}

impl ListBox {
    pub fn new(parent: &Window, id: u16, x: i32, y: i32, width: i32, height: i32) -> ListBox {
        let LBS_NOTIFY = 1;
        let LBS_HASSTRINGS = 64;

        let wnd = WindowBuilder::new()
            .class_name("LISTBOX")
            .style(winapi::WS_VISIBLE | winapi::WS_VSCROLL | winapi::WS_CHILD | LBS_NOTIFY | LBS_HASSTRINGS)
            .position(x, y)
            .size(width, height)
            .parent(parent.get_hwnd())
            .id(id)
            .create();
        ListBox {
            window: wnd
        }
    }

    pub fn add_item(&self, val : &str) {
        let val = to_wchar(val);
        let LB_ADDSTRING = 384;

        unsafe {
            user32::SendMessageW(self.window, LB_ADDSTRING, 0, val.as_ptr() as winapi::LPARAM);
        }
    }

    pub fn delete_item(&self, idx : u32) {
        let LB_DELETESTRING = 386;

        unsafe {
            user32::SendMessageW(self.window, LB_DELETESTRING, idx, 0);
        }
    }
    pub fn get_item_count(&self) -> i32 {
        let LB_GETCOUNT = 395;

        unsafe {
    	       return user32::SendMessageW(self.window, LB_GETCOUNT, 0, 0);
        }
    }
    pub fn clear(&self) {
        let LB_RESETCONTENT = 388;

        unsafe {
            user32::SendMessageW(self.window, LB_RESETCONTENT, 0, 0);
        }
    }
    pub fn get_sel(&self) -> i32 {
        let LB_GETCURSEL = 392;

        unsafe {
               return user32::SendMessageW(self.window, LB_GETCURSEL, 0, 0);
        }
    }
    pub fn set_sel(&self, idx : u32) {
        let LB_SETCURSEL = 390;

        unsafe {
            user32::SendMessageW(self.window, LB_SETCURSEL, idx, 0);
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
    pub fn new(parent: &Window, id: u16, title: &str, x: i32, y: i32, width: i32, height: i32) -> Checkbox {
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

    pub fn is_checked(&self) -> bool {
        unsafe {
            let BST_CHECKED = 1;
            let BM_GETCHECK = 240;

            return user32::SendMessageW(self.window, BM_GETCHECK, 0, 0) == BST_CHECKED;
        }
    }

    pub fn set_checked(&self, checked : bool) {
        unsafe {
            let BST_CHECKED = 1;
            let BST_UNCHECKED = 0;
            let BM_SETCHECK = 241;

            user32::SendMessageW(self.window, BM_SETCHECK,
                if checked {BST_CHECKED} else {BST_UNCHECKED}, 0);
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
	list_box: ListBox,
    edit : Edit
}

impl WindowEventHandler for MyApplication {
	fn on_command(&mut self, source_id: u16, command_type: u16) {
		println!("MyApplication got command from: {}.", source_id);
        println!("Len: {} Text: {}", self.edit.get_text_length(), self.edit.get_text())
	}

    fn on_close(&mut self) {
        if self.main_window.confirm_box("Do you want to exit?") {
            Application::exit_loop();
        }
    }

    fn on_size(&mut self, width: u16, height: u16) {
        //self.list_box.resize(10, 10, width as i32 - 20i32, height as i32 - 20i32);
    }
}

impl MyApplication {
	pub fn new() -> MyApplication {
        let margin = 10;
		let wnd = Frame::new("My Main Window", 200, 400);

        let lb = ListBox::new(&wnd, 10, margin, margin, 200 - margin * 2, 200);

        lb.add_item("Item 1");
        lb.add_item("Item 2");
        lb.add_item("Item 3");

        lb.set_sel(1);

        let edt = Edit::new(&wnd, margin, 200 + margin * 2, 200 - margin * 2, 50, false);
        edt.set_text("Hello World");
        //edt.set_read_only(true);

		MyApplication {
			main_window: wnd,
			list_box: lb,
            edit: edt
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
