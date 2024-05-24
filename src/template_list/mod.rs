use std::fs;
use std::path::Path;

use gtk::glib;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::template_widget_item::TemplateWidgetItem;
use crate::TEMPLATES_DIR;
use crate::window::Window;

mod imp;


glib::wrapper! {
    pub struct TemplateList(ObjectSubclass<imp::TemplateList>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for TemplateList {
    fn default() -> Self {
        TemplateList::new()
    }
}

impl TemplateList {
    pub fn new() -> Self {
        glib::Object::new()
    }
    
    pub fn load_list(&self, win: &Window) {
        let png_files = TemplateList::find_png_files(TEMPLATES_DIR.as_path());
        self.imp().list.remove_all();
        for file in png_files {
            let title = Path::new(file.as_str())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();

            let w = win.clone();
            let list_item = TemplateWidgetItem::new(file.as_str(), title);
            list_item.connect_clicked(move |item| {
                let renderer = w.imp().gl_area.renderer();
                let mut renderer = renderer.as_ref().unwrap().borrow_mut();
                let model_type = renderer.get_model_type();
                let result = renderer.load_texture(file.clone().as_str(), &model_type, true);
                match result {
                    Ok(_) => {
                        println!("Template successfully loaded");
                        w.imp().gl_area.queue_draw();
                    },
                    Err(_) => println!("Error loading template")
                }
            });
            self.imp().list.append(&list_item);
        }
    }
    
    fn find_png_files<P: AsRef<Path>>(path: P) -> Vec<String> {
        let mut png_files = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "png" {
                            if let Some(path) = path.to_str() {
                                png_files.push(path.to_string());
                            }
                            
                            // if let Some(file_name) = path.file_name() {
                            //     if let Some(file_name_str) = file_name.to_str() {
                            //         png_files.push(file_name_str.to_string());
                            //     }
                            // }
                        }
                    }
                }
            }
        } else {
            println!("No such dir");
        }
        
        png_files
    }
}