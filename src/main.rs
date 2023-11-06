#![allow(warnings)]

use std::io::Write;
use std::ptr;

use glium::backend::Backend;
use gtk::CssProvider;
use gtk::gdk::Display;
use gtk::gdk::prelude::*;
use gtk::glib::PropertyGet;
use gtk::prelude::*;
use libadwaita::prelude::AdwApplicationWindowExt;
use log::LevelFilter;

use crate::window::Window;

mod glium_area;
mod model_switcher;
mod application;
mod window;

const APP_ID: &str = "org.redgradient.mc-skin-editor";
const RESOURCES_PATH: &str = "mcskineditor.gresource.xml";

fn load_gl_function() {
    // Load GL pointers from epoxy (GL context management library used by GTK).

    #[cfg(target_os = "macos")]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.0.dylib") }.unwrap();
    #[cfg(all(unix, not(target_os = "macos")))]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.so.0") }.unwrap();
    #[cfg(windows)]
        let library = libloading::os::windows::Library::open_already_loaded("libepoxy-0.dll")
        .or_else(|_| libloading::os::windows::Library::open_already_loaded("epoxy-0.dll"))
        .unwrap();

    epoxy::load_with(|name| {
        unsafe { library.get::<_>(name.as_bytes()) }
            .map(|symbol| *symbol)
            .unwrap_or(ptr::null())
    });
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../resources/css/style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}

fn main() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(buf, "[{}] {}", record.level(), record.args())
        })
        .filter(None, LevelFilter::Off)
        .init();

    load_gl_function();

    gtk::init().expect("Failed to initialize GTK");

    let app = application::Application::new();
    app.connect_startup(|_| load_css());
    app.connect_activate(|app| Window::new(app).present());

    app.run();
}