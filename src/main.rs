use std::cell::Cell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use glib::{clone, Continue, MainContext, PRIORITY_DEFAULT};
use gdk::Display;
use gtk::{prelude::*, CssProvider, StyleContext};
use gtk::{self, gdk, glib, Application, ApplicationWindow, Box, Label, Button, Orientation};

const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement4";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."), 
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

fn build_ui(app: &Application) {

    let clicks = Rc::new(Cell::new(0));
    let started = Rc::new(Cell::new(1));

    let clicks_name = Label::builder()
        .label("Clicks")
        .margin_start(12)
        .margin_end(12)
        .width_request(160)
        .height_request(48)
        .build();

    clicks_name.add_css_class("label");
    
    let time_name = Label::builder()
        .label("Time")
        .margin_start(12)
        .margin_end(12)
        .width_request(160)
        .height_request(48)
        .build();

    time_name.add_css_class("label");

    let clicks_label = Label::builder()
        .label(&clicks.get().to_string())
        .margin_start(12)
        .margin_end(12)
        .width_request(160)
        .height_request(48)
        .build();

    clicks_label.add_css_class("counter");
    
    let time_label = Label::builder()
        .label("0")
        .margin_start(12)
        .margin_end(12)
        .width_request(160)
        .height_request(48)
        .build();

    time_label.add_css_class("counter");

    let button_add = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .width_request(160)
        .height_request(48)
        .build();

    button_add.add_css_class("button-on");

    let button_reset = Button::builder()
        .label("Reset")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .width_request(160)
        .height_request(32)
        .build();

    button_reset.add_css_class("button-on");

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
    button_add.connect_clicked(clone!(@weak clicks, @weak clicks_label, @weak button_reset =>
        move |button_add| {
            clicks.set(clicks.get() + 1);
            clicks_label.set_label(&clicks.get().to_string());
            if clicks == started {
                button_add.set_label("Click");
                button_reset.set_sensitive(false);
                button_reset.remove_css_class("button-on");
                button_reset.add_css_class("button-off");
                let sender = sender.clone(); 
                thread::spawn(move || {
                    for i in 1..11 {
                        sender.send(i).expect("Could not send through channel");
                        thread::sleep(Duration::from_secs(1));
                    }
                });
            }
    }));

    receiver.attach(
        None,
        clone!(@weak time_label, @weak button_add, @weak button_reset => @default-return Continue(false),
                    move |time| {
                        time_label.set_label(&time.to_string());
                        if time == 10 {
                            button_add.set_sensitive(false);
                            button_add.remove_css_class("button-on");
                            button_add.add_css_class("button-off");
                            button_reset.set_sensitive(true);
                            button_reset.remove_css_class("button-off");
                            button_reset.add_css_class("button-on");
                        }
                        Continue(true)
                    }
        ),
    );

    button_reset.connect_clicked(clone!(@weak clicks_label, @weak time_label, @weak button_add =>
        move |_| {
            clicks.set(0);
            time_label.set_label("0");
            clicks_label.set_label(&clicks.get().to_string());
            button_add.set_sensitive(true);
            button_add.add_css_class("button-on");
            button_add.remove_css_class("button-off");
    }));

    let button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    button_box.add_css_class("button_box");

    button_box.append(&button_add);
    button_box.append(&button_reset);

    let label_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    label_box.add_css_class("label_box");

    label_box.append(&clicks_name);
    label_box.append(&time_name);

    let counter_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    counter_box.add_css_class("counter_box");

    counter_box.append(&clicks_label);
    counter_box.append(&time_label);

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    label_box.add_css_class("main_box");
    
    main_box.append(&label_box);
    main_box.append(&counter_box);
    main_box.append(&button_box);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("CPS Test")
        .child(&main_box)
        .resizable(false)
        .build();

    window.add_css_class("body");

    window.present();
}
