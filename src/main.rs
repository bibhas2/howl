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

pub unsafe fn create_window(exStyle : winapi::DWORD, style : winapi::DWORD, class_name : &str, title: &str) -> winapi::HWND {
    let window = user32::CreateWindowExW(
        exStyle,
        to_wchar(class_name).as_ptr(),
        to_wchar(title).as_ptr(),
        style,
        0,
        0,
        100,
        100,
        ptr::null_mut(),
        ptr::null_mut(),
        get_instance(),
        ptr::null_mut());
		
    if window.is_null() {
        panic!("CreateWindowExW error: {}", kernel32::GetLastError());
    }

    window
}

fn main() {
	unsafe {
		let class_name = "HOWL";
		register_class(class_name, Some(user32::DefWindowProcW));
		
	    let wnd = create_window(winapi::WS_EX_CLIENTEDGE, 
			winapi::WS_THICKFRAME | winapi::WS_MINIMIZEBOX | winapi::WS_MAXIMIZEBOX | winapi::WS_SYSMENU, 
			class_name,
			"Hello World");

		user32::ShowWindow(wnd, 5);
		
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