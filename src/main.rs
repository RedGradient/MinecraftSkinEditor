#![allow(warnings)]

use std::io::{Read, Write};
use std::path::PathBuf;
use std::ptr;

use glium::backend::Backend;
use gtk::{gio, glib};
use gtk::gdk::prelude::*;
use gtk::prelude::*;
use lazy_static::lazy_static;
use libadwaita::prelude::AdwApplicationWindowExt;

mod glium_area;
mod model_switcher;
mod window;
mod template_list;
mod template_widget_item;
mod skin_loader_popover;
mod skin_dialog;
mod application;
mod command;
mod utils;
pub const APP_ID: &str = "io.redgradient.MCSkinEditor";

lazy_static! {
    static ref ROOT_DIR: PathBuf = dirs::home_dir().expect("Home directory not found").join("MinecraftSkinEditor");
    static ref TEMPLATES_DIR: PathBuf = ROOT_DIR.join("templates");
}

fn load_gl_function() {
    // Load GL pointers from epoxy (GL context management library used by GTK).

    #[cfg(target_os = "macos")]
        let library = unsafe { libloading::os::unix::Library::new("/usr/local/lib/libepoxy.0.dylib") }.unwrap();
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

fn register_resources() {
    let resource = {
        let resource_bytes = glib::Bytes::from_static(include_bytes!("../resources/mcskineditor.gresource"));
        gio::Resource::from_data(&resource_bytes).expect("Could not load gresource file")
    };
    gio::resources_register(&resource);
}

fn main() {
    load_gl_function();
    register_resources();
    gtk::init().expect("Failed to initialize GTK");
    
    let app = application::Application::new();
    app.run();
}