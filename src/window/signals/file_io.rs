use gtk::gio::{ActionEntry, Cancellable};
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::{ActionMapExtManual, ButtonExt, FileExt, NativeDialogExtManual, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use libadwaita::prelude::AdwDialogExt;

use crate::utils;
use crate::{TEMPLATES_DIR};
use crate::skin_dialog::SkinDialog;
use crate::window::Window;

pub(super) fn connect(win: &Window) {
    connect_open(win);
    connect_save(win);
}

fn connect_open(win: &Window) {
    win.imp().open_button.connect_clicked(clone!(#[weak(rename_to = win)] win, move |_| {
        let mut file_dialog = gtk::FileDialog::builder().title("Open a skin").build();
        file_dialog.open(Some(&win), Cancellable::NONE, clone!(#[weak] win, move |file| {
            let file = match file {
                Ok(file) => file,
                Err(_) => return,
            };
            let texture_path = file.path().unwrap();
            let skin_dialog = SkinDialog::new(texture_path, win.clone());
            skin_dialog.present(Some(&win));
        }));
    }));
}

fn connect_save(win: &Window) {
    let action = ActionEntry::builder("action")
        .activate(clone!(#[weak(rename_to = win)] win, move |_, _, _| {
            let img = win.export_texture();
            let random_filename = utils::generate_random_filename();
            let path = TEMPLATES_DIR.as_path().join(random_filename);
            match img.save(&path) {
                Ok(_) => {
                    println!("Saved as template at {:?}", path.as_path());
                    win.refresh_template_list();
                }
                Err(error) => println!("{}", error.to_string()),
            }
        }))
        .build();
    win.add_action_entries([action]);

    win.imp().save_button.connect_clicked(clone!(#[weak(rename_to = win)] win, move |_| {
        let mut file_dialog = gtk::FileDialog::builder().title("Save a skin").build();
        file_dialog.set_initial_name(Some("untitled.png"));
        file_dialog.save(Some(&win), Cancellable::NONE, clone!(#[weak] win, move |file| {
            let file = match file {
                Ok(file) => file,
                Err(_) => return,
            };

            let path = match file.path() {
                Some(path) => path,
                None => {
                    println!("Selected file has no path");
                    return;
                }
            };
            let path = match path.to_str() {
                Some(path) => path,
                None => {
                    println!("Path of the selected file cannot be converted to string");
                    return;
                }
            };

            let imgbuf = win.export_texture();
            match imgbuf.save(path) {
                Ok(_) => println!("Saved at {}", path),
                Err(error) => println!("{}", error.to_string()),
            }
        }));
    }));
}
