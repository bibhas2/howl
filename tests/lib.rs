extern crate howl;
extern crate winapi;

use howl::*;

#[test]
fn checkbox_test() {
    Application::init();

    let wnd = Frame::new("My Main Window", 200, 400);
    let cb = Checkbox::new(&wnd, 10, "Check me", 10, 10, 100, 10);

    assert_eq!(false, cb.is_checked());
    assert_eq!("Check me", cb.get_text());

    cb.set_checked(true);
    assert_eq!(true, cb.is_checked());

    cb.set_checked(false);
    assert_eq!(false, cb.is_checked());
}

#[test]
fn set_text_test() {
    Application::init();

    let wnd = Frame::new("My Main Window", 200, 400);

    assert_eq!(wnd.get_text(), "My Main Window");

    wnd.set_text("Hello World");
    assert_eq!(wnd.get_text(), "Hello World");
}

#[test]
fn append_test() {
    Application::init();

    let wnd = Frame::new("My Main Window", 200, 400);

    let edt = Edit::new(&wnd, 0, 0, 100, 100, true);
    edt.set_text("Hello World. ");
    edt.append_text("Hello Moon.");

    assert_eq!(edt.get_text(), "Hello World. Hello Moon.");
}

#[test]
fn list_box_test() {
    Application::init();

    let wnd = Frame::new("My Main Window", 200, 400);

    let lb = ListBox::new(&wnd, 0, 0, 0, 100, 100);
    assert_eq!(0, lb.get_item_count());

    lb.add_item("Item 1");
    lb.add_item("Item 2");
    lb.add_item("Item 3");
    assert_eq!(3, lb.get_item_count());

    lb.set_sel(2);
    assert_eq!(2, lb.get_sel());

    lb.delete_item(1);
    assert_eq!(2, lb.get_item_count());

    lb.clear();
    assert_eq!(0, lb.get_item_count());
}

#[test]
fn size_test() {
    Application::init();

    let mut x = 0i32;
    let mut y = 0i32;
    let mut width = 0i32;
    let mut height = 0i32;

    let wnd = Frame::new("My Main Window", 200, 400);

    wnd.get_size(&mut x, &mut y, &mut width, &mut height);
    assert_eq!(200, width);
    assert_eq!(400, height);

    let lb = ListBox::new(&wnd, 0, 10, 50, 100, 100);

    lb.get_size(&mut x, &mut y, &mut width, &mut height);
    assert_eq!(10, x);
    assert_eq!(50, y);
    assert_eq!(100, width);
    assert_eq!(100, height);
}
