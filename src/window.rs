use std::cell::{Ref, RefCell, RefMut};
use std::hash::{Hash, Hasher};

use gtk::gio;
use gtk::gio::{ActionEntry, Cancellable};
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::{ActionExt, ActionMapExt, ActionMapExtManual, BoxExt, ButtonExt, Cast, DialogExtManual, FileChooserExt, FileExt, GestureExt, GLAreaExt, GtkWindowExt, IsA, ListModelExt, NativeDialogExtManual, ObjectExt, StaticVariantType, ToggleButtonExt, ToVariant};
use gtk::prelude::ColorChooserExt;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use libadwaita as adw;
use libadwaita::prelude::AdwDialogExt;
use rand::Rng;

use crate::{TEMPLATES_DIR, utils};
use crate::application::Application;
use crate::command::{Action, Tool};
use crate::command::DrawingHistory;
use crate::command::Tool::*;
use crate::glium_area::body_part::BodyPart::*;
use crate::glium_area::renderer::{ModelCell, Renderer};
use crate::glium_area::skin_parser::ModelType;
use crate::skin_dialog::SkinDialog;
use crate::skin_loader_popover::SkinLoaderPopover;

mod imp {
    use std::cell::{Cell, RefCell};
    use std::hash::Hash;

    use gtk::{CompositeTemplate, glib, TemplateChild};
    use gtk::prelude::GtkWindowExt;
    use gtk::subclass::application_window::ApplicationWindowImpl;
    use gtk::subclass::prelude::{CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetImpl, WindowImpl};
    use gtk::subclass::widget::WidgetClassExt;
    use libadwaita as adw;
    use libadwaita::subclass::application_window::AdwApplicationWindowImpl;

    use crate::APP_ID;
    use crate::command::DrawingHistory;
    use crate::command::Tool;
    use crate::glium_area::GliumArea;
    use crate::model_switcher::ModelSwitcher;
    use crate::template_list::TemplateList;

    #[derive(CompositeTemplate, Default)]
    #[template(file = "../resources/ui/window.ui")]
    pub struct Window {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub open_button: TemplateChild<adw::SplitButton>,
        #[template_child]
        pub save_button: TemplateChild<adw::SplitButton>,
        #[template_child]
        pub undo_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub redo_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub grid_toggle: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub color_button: TemplateChild<gtk::ColorDialogButton>,
        #[template_child]
        pub content_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub left_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub right_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub pencil: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub rubber: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub color_picker: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub random_color: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub fill: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub replace_color: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub gl_area: TemplateChild<GliumArea>,
        #[template_child]
        pub model_switcher: TemplateChild<ModelSwitcher>,
        #[template_child]
        pub reset_skin_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub wardrobe: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub template_list: TemplateChild<TemplateList>,
        #[template_child]
        pub save_as_template_button: TemplateChild<gtk::Button>,

        pub current_tool: Cell<Tool>,
        pub is_tool_active: Cell<bool>,
        pub opening_new_skin: Cell<bool>,
        pub drawing_history: RefCell<Option<RefCell<DrawingHistory>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "MCSkinEditorWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);

            klass.install_action("win.undo", None, move |win, _, _| {
                win.imp().drawing_history.borrow().as_ref().unwrap().borrow_mut().undo();
            });

            klass.install_action("win.redo", None, move |win, _, _| {
                win.imp().drawing_history.borrow().as_ref().unwrap().borrow_mut().redo();
            });

            klass.install_action("win.about", None, move |win, _, _| {
                win.imp().show_about();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {}
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}

    impl Window {
        fn show_about(&self) {
            let about_window = adw::AboutWindow::builder()
                .application_name("Minecraft Skin Editor")
                .application_icon(APP_ID)
                .version("0.1.0")
                .website("https://github.com/RedGradient/MinecraftSkinEditor")
                .issue_url("https://github.com/RedGradient/MinecraftSkinEditor/issues")
                .copyright("© 2023 RedGradient")
                .developers(vec!["RedGradient"])
                .license_type(gtk::License::Gpl30)
                .build();

            about_window.present();
        }
    }
}


glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        let win = glib::Object::builder::<Window>()
            .property("application", app)
            .build();

        win.setup();
        win.set_icons();
        win.connect_signals();

        win
    }

    fn setup(&self) {
        // set devel class
        self.add_css_class("devel");
        
        // header bar title
        self.imp().header_bar.set_show_title(false);

        // set popover for Open button
        let popover = SkinLoaderPopover::new(self);
        self.imp().open_button.set_popover(Some(&popover));

        // set dialog for color button
        let color_dialog = gtk::ColorDialog::builder().with_alpha(false).build();
        self.imp().color_button.set_dialog(&color_dialog);
        
        let gl_area = self.imp().gl_area.get();
        gl_area.setup(self);
        // initialize drawing history
        self.imp().drawing_history.replace(Some(RefCell::new(DrawingHistory::new(gl_area))));

        self.set_tool_active(true);
    }

    fn set_icons(&self) {
        let pencil_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/pencil.png");
        let rubber_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/eraser.png");
        let color_picker_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/color_picker.png");
        let grid_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/grid.png");
        let fill_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/fill.png");
        let replace_ico = gtk::Image::from_resource("/io/redgradient/MCSkinEditor/media/replace.png");
        self.imp().pencil.set_child(Some(&pencil_ico));
        self.imp().rubber.set_child(Some(&rubber_ico));
        self.imp().color_picker.set_child(Some(&color_picker_ico));
        self.imp().grid_toggle.set_child(Some(&grid_ico));
        self.imp().fill.set_child(Some(&fill_ico));
        self.imp().replace_color.set_child(Some(&replace_ico));
    }

    fn connect_signals(&self) {
        self.connect_wardrobe();
        self.connect_reset_skin_button();
        self.connect_tools();
        self.connect_grid_button();
        self.connect_open_button();
        self.connect_save_button();
        self.connect_model_switcher();
    }

    fn connect_grid_button(&self) {
        self.imp().grid_toggle.connect_toggled(
            clone!(@weak self as win => move |btn| {
                let gl_area = win.imp().gl_area.get();
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                renderer.set_grid_show(btn.is_active());
                gl_area.queue_draw();
            })
        );
    }

    fn connect_tools(&self) {
        self.imp().pencil.connect_toggled(clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::Pencil); }));
        self.imp().rubber.connect_toggled(clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::Rubber); }));
        self.imp().color_picker.connect_toggled(clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::ColorPicker); }));
        self.imp().fill.connect_toggled(clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::Fill); }));
        self.imp().random_color.connect_toggled(clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::Random); }));
        self.imp().replace_color.connect_toggled(clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::Replace); }));
    }

    fn connect_reset_skin_button(&self) {
        self.imp().reset_skin_button.connect_clicked(
            clone!(@weak self as win => move |btn| {
                let renderer = win.imp().gl_area.renderer().unwrap();
                let mut renderer: RefMut<Renderer> = renderer.borrow_mut();
                renderer.reset_skin();
                drop(renderer);
                win.imp().grid_toggle.set_active(true);
                win.imp().gl_area.queue_draw();
            })
        );
    }

    fn connect_wardrobe(&self) {
        self.imp().wardrobe.connect_toggled(clone!(@weak self as win => move |btn| {
            // --- toggle left_box ---
            win.imp().left_box.set_visible(!btn.is_active());

            // --- toggle right_box ---
            win.imp().right_box.set_visible(!btn.is_active());

            // --- toggle template_list ---
            win.imp().template_list.set_visible(btn.is_active());
            // --- update list of templates---
            if btn.is_active() {
                win.imp().template_list.load_list(&win.clone());
            }

            win.set_tool_active(!btn.is_active());
        }));
    }

    fn activate_pencil_toggle(&self) {
        self.imp().pencil.set_active(true);
    }

    fn connect_model_switcher(&self) {
        let model_switcher = self.imp().model_switcher.get();
        let gl_area = self.imp().gl_area.get();

        model_switcher.inner_layer_toggle().connect_toggled(clone!(
            @weak model_switcher, @weak gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                let ms = model_switcher.clone();
                let inner_is_active = ms.inner_layer_toggle().is_active();
                renderer.set_body_part_active(&Head,     inner_is_active && ms.head().is_active());
                renderer.set_body_part_active(&Torso,    inner_is_active && ms.torso().is_active());
                renderer.set_body_part_active(&RightArm, inner_is_active && ms.right_arm().is_active());
                renderer.set_body_part_active(&LeftArm,  inner_is_active && ms.left_arm().is_active());
                renderer.set_body_part_active(&RightLeg, inner_is_active && ms.right_leg().is_active());
                renderer.set_body_part_active(&LeftLeg,  inner_is_active && ms.left_leg().is_active());
                gl_area.queue_draw();
            }
        ));

        model_switcher.outer_layer_toggle().connect_toggled(clone!(
            @weak model_switcher, @weak gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                let ms = model_switcher.clone();

                let outer_is_active = model_switcher.outer_layer_toggle().is_active();
                renderer.set_body_part_active(&HeadOuter, outer_is_active && ms.head().is_active());
                renderer.set_body_part_active(&TorsoOuter, outer_is_active && ms.torso().is_active());
                renderer.set_body_part_active(&RightArmOuter, outer_is_active && ms.right_arm().is_active());
                renderer.set_body_part_active(&LeftArmOuter, outer_is_active && ms.left_arm().is_active());
                renderer.set_body_part_active(&RightLegOuter, outer_is_active && ms.right_leg().is_active());
                renderer.set_body_part_active(&LeftLegOuter, outer_is_active && ms.left_leg().is_active());

                gl_area.queue_draw();
            }
        ));

        model_switcher.head().connect_toggled(clone!(
            @weak model_switcher, @weak gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&Head, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&HeadOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.torso().connect_toggled(clone!(
            @weak model_switcher, @weak gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&Torso, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&TorsoOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.left_arm().connect_toggled(clone!(
            @weak model_switcher, @weak gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&LeftArm, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&LeftArmOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.right_arm().connect_toggled(clone!(
            @weak model_switcher, @weak gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&RightArm, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&RightArmOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.left_leg().connect_toggled(
            clone!(@weak model_switcher, @weak gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&LeftLeg, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&LeftLegOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.right_leg().connect_toggled(
            clone!(@weak model_switcher, @weak gl_area => move |cb| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                if model_switcher.inner_layer_toggle().is_active() {
                    renderer.set_body_part_active(&RightLeg, cb.is_active());
                }
                if model_switcher.outer_layer_toggle().is_active() {
                    renderer.set_body_part_active(&RightLegOuter, cb.is_active());
                }
                gl_area.queue_draw();
            }
        ));
        model_switcher.imp().model_type_selector.connect_selected_notify(
            clone!(@weak self as win => move |dropdown| {
                if win.imp().opening_new_skin.take() {
                    win.imp().opening_new_skin.replace(false);
                    return
                }
                let renderer = win.imp().gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                let model_type = match dropdown.selected() {
                    0 => ModelType::Slim,
                    1 => ModelType::Classic,
                    _ => panic!("Unknown model type"),
                };
                renderer.reset_model_type(&model_type);
                win.imp().gl_area.queue_draw();
        }));
    }

    fn connect_open_button(&self) {
        self.imp().open_button.connect_clicked(clone!(@weak self as win => move |_| {
            let mut file_dialog = gtk::FileDialog::builder().title("Open a skin").build();
            file_dialog.open(Some(&win), Cancellable::NONE, clone!(@weak win => move |file| {
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
    
    fn connect_save_button(&self) {
        let action = ActionEntry::builder("action")
            .activate(clone!(@weak self as win => move |_, _, _| {
                let renderer = win.imp().gl_area.renderer().unwrap();
                let renderer: Ref<Renderer> = renderer.borrow();
                let img = renderer.export_texture();
                let random_filename = utils::generate_random_filename();
                let path = TEMPLATES_DIR.as_path().join(random_filename);
                match img.save(&path) {
                    Ok(_) => {
                        println!("Saved as template at {:?}", path.as_path());
                        win.imp().template_list.load_list(&win.clone());
                    },
                    Err(error) => println!("{}", error.to_string()),
                }
            }))
            .build();
        self.add_action_entries([action]);

        self.imp().save_button.connect_clicked(clone!(@weak self as win => move |btn| {
            let mut file_dialog = gtk::FileDialog::builder().title("Save a skin").build();
            file_dialog.set_initial_name(Some("untitled.png"));
            file_dialog.save(Some(&win), Cancellable::NONE, clone!(@weak win => move |file| {
                let file = match file {
                    Ok(file) => file,
                    Err(_) => return,
                };

                let renderer = win.imp().gl_area.renderer().unwrap();
                let renderer = renderer.borrow();

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

                let imgbuf = renderer.export_texture();
                match imgbuf.save(path) {
                    Ok(_) => println!("Saved at {}", path),
                    Err(error) => println!("{}", error.to_string()),
                }
            }));
        }));
    }
    
    pub fn get_last_modified_cell(&self) -> Option<ModelCell> {
        self.imp().drawing_history.borrow()
            .as_ref()
            .expect("Drawing history is not initialized")
            .borrow()
            .get_last_modified()
    }

    pub fn set_last_modified(&self, cell: ModelCell) {
        self.imp().drawing_history.borrow()
            .as_ref()
            .expect("Drawing history is not initialized")
            .borrow_mut()
            .set_last_modified(cell);
    }

    pub fn add_command_to_history(&self, command: Box<dyn Action>) {
        self.imp().drawing_history.borrow()
            .as_ref()
            .expect("Drawing history is not initialized")
            .borrow_mut()
            .add_command(command);
    }

    fn set_tool_active(&self, active: bool) {
        self.imp().is_tool_active.replace(active);
    }

    pub fn is_tool_active(&self) -> bool {
        self.imp().is_tool_active.get()
    }
}
