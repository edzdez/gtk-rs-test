extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("A Gtk Window");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let box1 = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let button_box1 = gtk::ButtonBox::new(gtk::Orientation::Horizontal);

    let label1 = gtk::Label::new(Some("Application Test"));
    label1.set_markup("<big>Application Test</big>");

    let button1 = gtk::Button::with_label("Button 1");
    let button2 = gtk::Button::with_label("Button 2");

    button1.set_border_width(40);
    button2.set_border_width(40);

    box1.pack_start(&label1, true, true, 0);
    box1.pack_end(&button_box1, true, true, 10);

    button_box1.pack_start(&button1, true, true, 0);
    button_box1.pack_end(&button2, true, true, 0);

    window.add(&box1);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.edzdez.gtk-test"),
        Default::default()
    ).expect("Failed to initialize GTK");

    application.connect_activate(build_ui);

    application.run(&[]);
}