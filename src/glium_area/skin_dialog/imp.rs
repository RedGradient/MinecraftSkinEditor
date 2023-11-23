use std::cell::{Cell, RefCell};

use gtk::{CompositeTemplate, glib, TemplateChild};
use gtk::prelude::GtkWindowExt;
use gtk::subclass::prelude::{CompositeTemplate, CompositeTemplateInitializingExt, ObjectImpl, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassExt};
use gtk::subclass::widget::WidgetImpl;
use gtk::subclass::window::WindowImpl;
use libadwaita as adw;
use libadwaita::subclass::prelude::{AdwWindowImpl, ObjectSubclass};

#[derive(Debug)]
pub enum SkinDialogResponse {
    Slim,
    Classic,
    Discard,
}

#[derive(CompositeTemplate, Default)]
#[template(file = "../../../resources/ui/skin_dialog.ui")]
pub struct SkinDialog {
    #[template_child]
    pub slim: TemplateChild<gtk::Button>,
    #[template_child]
    pub classic: TemplateChild<gtk::Button>,
    #[template_child]
    pub discard: TemplateChild<gtk::Button>,

    pub response: Cell<Option<SkinDialogResponse>>,
    pub texture_path: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SkinDialog {
    const NAME: &'static str = "SkinDialog";
    type Type = super::SkinDialog;
    type ParentType = adw::Window;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);

        klass.install_action(
            "skin_dialog.discard",
            None,
            move |dialog, _, _| {
                // dialog.imp().response.replace(Some(SkinDialogResponse::Discard));
                dialog.destroy();
            }
        );

        klass.install_action(
            "skin_dialog.slim",
            None,
            move |dialog, _, _| {
                // dialog.imp().response.replace(Some(SkinDialogResponse::Slim));
                dialog.close();
            }
        );

        klass.install_action(
            "skin_dialog.classic",
            None,
            move |dialog, _, _| {
                // dialog.imp().response.replace(Some(SkinDialogResponse::Classic));
                dialog.close();
            }
        );
    }
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SkinDialog {}
impl WidgetImpl for SkinDialog {}
impl WindowImpl for SkinDialog {}
impl AdwWindowImpl for SkinDialog {}