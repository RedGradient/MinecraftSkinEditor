use std::{fs, io};

use gtk::{CssProvider, gio, glib};
use gtk::gdk::Display;
use gtk::glib::Object;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use libadwaita as adw;

use crate::{APP_ID, ROOT_DIR, TEMPLATES_DIR};
use crate::window::Window;

mod imp {
    use std::cell::OnceCell;
    use gtk::{gio, glib};
    use gtk::glib::WeakRef;
    use gtk::prelude::{GtkApplicationExt, GtkWindowExt, ObjectExt};
    use libadwaita as adw;
    use libadwaita::subclass::prelude::*;
    use crate::window::Window;

    #[derive(Default)]
    pub struct Application {
        pub window: OnceCell<WeakRef<Window>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "MCSkinEditorApplication";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Application {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_accels_for_action("win.undo", &["<Primary>Z", "<Meta>Z"]);
            obj.set_accels_for_action("win.redo", &["<Primary><Shift>Z", "<Meta><Shift>Z"]);
        }
    }
    impl ApplicationImpl for Application {
        fn activate(&self) {
            self.parent_activate();
            let app = self.obj();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = Window::new(&app);
            self.window.set(window.downgrade())
                .expect("Window already set");

            app.main_window().present();

        }

        fn startup(&self) {
            self.parent_startup();
            let app = self.obj();
            
            app.setup_css();
        }
    }
    impl GtkApplicationImpl for Application {}
    impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Application {
    pub fn new() -> Self {
        let app: Application = Object::builder()
            .property("application-id", APP_ID)
            .build();

        app.create_application_dir();
        app.create_templates_dir();

        app
    }

    fn setup_css(&self) {
        // Load the CSS file and add it to the provider
        let provider = CssProvider::new();
        provider.load_from_path("resources/css/style.css");

        // Add the provider to the default screen
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );
    }

    fn create_application_dir(&self) {
        if ROOT_DIR.exists() {
            println!("Application folder: {:?}", ROOT_DIR.as_path());
            return;
        }
        fs::create_dir_all(ROOT_DIR.as_path()).unwrap();
        println!("Application folder created: {:?}", ROOT_DIR.as_path());
    }
    fn create_templates_dir(&self) {
        if TEMPLATES_DIR.exists() {
            println!("Templates folder:   {:?}", TEMPLATES_DIR.as_path());
            return;
        }
        fs::create_dir_all(TEMPLATES_DIR.as_path()).unwrap();
        println!("Templates folder created:   {:?}", TEMPLATES_DIR.as_path());
    }

    fn main_window(&self) -> Window {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

}