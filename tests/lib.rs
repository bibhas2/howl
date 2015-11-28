extern crate howl;

use howl::*;

#[test]
fn set_text_test() {
    Application::init();

    let wnd = Frame::new("My Main Window", 200, 400);

    assert_eq!(wnd.get_text(), "My Main Window");
}

#[test]
fn append_test() {
    Application::init();

    let wnd = Frame::new("My Main Window", 200, 400);

    let edt = Edit::new(&wnd, 0, 0, 100, 100, true);
    edt.set_text("Hello World. ");
    edt.append_text("Hello Moon.");
}
