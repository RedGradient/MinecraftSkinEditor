use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::{ComboBoxExt, ToggleButtonExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::glium_area::body_part::BodyPart::*;
use crate::glium_area::skin_parser::ModelType;
use crate::window::Window;

pub(super) fn connect(win: &Window) {
    let model_switcher = win.imp().model_switcher.get();

    model_switcher.inner_layer_toggle().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |_| {
            let renderer = win.gl_area().renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            let inner_is_active = model_switcher.inner_layer_toggle().is_active();
            renderer.set_body_part_active(&Head, inner_is_active && model_switcher.head().is_active());
            renderer.set_body_part_active(&Torso, inner_is_active && model_switcher.torso().is_active());
            renderer.set_body_part_active(&RightArm, inner_is_active && model_switcher.right_arm().is_active());
            renderer.set_body_part_active(&LeftArm, inner_is_active && model_switcher.left_arm().is_active());
            renderer.set_body_part_active(&RightLeg, inner_is_active && model_switcher.right_leg().is_active());
            renderer.set_body_part_active(&LeftLeg, inner_is_active && model_switcher.left_leg().is_active());
            win.request_viewport_redraw();
        }
    ));

    model_switcher.outer_layer_toggle().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |_| {
            let renderer = win.gl_area().renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            let outer_is_active = model_switcher.outer_layer_toggle().is_active();
            renderer.set_body_part_active(&HeadOuter, outer_is_active && model_switcher.head().is_active());
            renderer.set_body_part_active(&TorsoOuter, outer_is_active && model_switcher.torso().is_active());
            renderer.set_body_part_active(&RightArmOuter, outer_is_active && model_switcher.right_arm().is_active());
            renderer.set_body_part_active(&LeftArmOuter, outer_is_active && model_switcher.left_arm().is_active());
            renderer.set_body_part_active(&RightLegOuter, outer_is_active && model_switcher.right_leg().is_active());
            renderer.set_body_part_active(&LeftLegOuter, outer_is_active && model_switcher.left_leg().is_active());
            win.request_viewport_redraw();
        }
    ));

    model_switcher.head().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |cb| {
            let renderer = win.gl_area().renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            if model_switcher.inner_layer_toggle().is_active() {
                renderer.set_body_part_active(&Head, cb.is_active());
            }
            if model_switcher.outer_layer_toggle().is_active() {
                renderer.set_body_part_active(&HeadOuter, cb.is_active());
            }
            win.request_viewport_redraw();
        }
    ));

    model_switcher.torso().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |cb| {
            let renderer = win.gl_area().renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            if model_switcher.inner_layer_toggle().is_active() {
                renderer.set_body_part_active(&Torso, cb.is_active());
            }
            if model_switcher.outer_layer_toggle().is_active() {
                renderer.set_body_part_active(&TorsoOuter, cb.is_active());
            }
            win.request_viewport_redraw();
        }
    ));

    model_switcher.left_arm().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |cb| {
            let renderer = win.gl_area().renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            if model_switcher.inner_layer_toggle().is_active() {
                renderer.set_body_part_active(&LeftArm, cb.is_active());
            }
            if model_switcher.outer_layer_toggle().is_active() {
                renderer.set_body_part_active(&LeftArmOuter, cb.is_active());
            }
            win.request_viewport_redraw();
        }
    ));

    model_switcher.right_arm().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |cb| {
            let renderer = win.gl_area().renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            if model_switcher.inner_layer_toggle().is_active() {
                renderer.set_body_part_active(&RightArm, cb.is_active());
            }
            if model_switcher.outer_layer_toggle().is_active() {
                renderer.set_body_part_active(&RightArmOuter, cb.is_active());
            }
            win.request_viewport_redraw();
        }
    ));

    model_switcher.left_leg().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |cb| {
            let renderer = win.gl_area().renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            if model_switcher.inner_layer_toggle().is_active() {
                renderer.set_body_part_active(&LeftLeg, cb.is_active());
            }
            if model_switcher.outer_layer_toggle().is_active() {
                renderer.set_body_part_active(&LeftLegOuter, cb.is_active());
            }
            win.request_viewport_redraw();
        }
    ));

    model_switcher.right_leg().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |cb| {
            let renderer = win.gl_area().renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            if model_switcher.inner_layer_toggle().is_active() {
                renderer.set_body_part_active(&RightLeg, cb.is_active());
            }
            if model_switcher.outer_layer_toggle().is_active() {
                renderer.set_body_part_active(&RightLegOuter, cb.is_active());
            }
            win.request_viewport_redraw();
        }
    ));

    model_switcher.imp().model_type_selector.connect_selected_notify(clone!(
        #[weak(rename_to = win)]
        win,
        move |dropdown| {
            if win.imp().opening_new_skin.take() {
                win.imp().opening_new_skin.replace(false);
                return;
            }
            let renderer = win.gl_area().renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            let model_type = match dropdown.selected() {
                0 => ModelType::Slim,
                1 => ModelType::Classic,
                _ => panic!("Unknown model type"),
            };
            renderer.reset_model_type(&model_type);
            win.request_viewport_redraw();
        }
    ));
}
