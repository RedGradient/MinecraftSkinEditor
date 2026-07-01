use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::{ComboBoxExt, ToggleButtonExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::glium_area::body_part::BodyPart;
use crate::glium_area::body_part::BodyPart::*;
use crate::glium_area::skin_parser::ModelType;
use crate::model_switcher::ModelSwitcher;
use crate::window::Window;

pub(super) fn connect(win: &Window) {
    let model_switcher = win.imp().model_switcher.get();

    model_switcher.inner_layer_toggle().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |_| sync_inner_layer(&win, &model_switcher)
    ));

    model_switcher.outer_layer_toggle().connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |_| sync_outer_layer(&win, &model_switcher)
    ));

    connect_part_toggle(&model_switcher, win.clone(), Head, HeadOuter, |ms| ms.head());
    connect_part_toggle(&model_switcher, win.clone(), Torso, TorsoOuter, |ms| ms.torso());
    connect_part_toggle(&model_switcher, win.clone(), LeftArm, LeftArmOuter, |ms| ms.left_arm());
    connect_part_toggle(&model_switcher, win.clone(), RightArm, RightArmOuter, |ms| ms.right_arm());
    connect_part_toggle(&model_switcher, win.clone(), LeftLeg, LeftLegOuter, |ms| ms.left_leg());
    connect_part_toggle(&model_switcher, win.clone(), RightLeg, RightLegOuter, |ms| ms.right_leg());

    model_switcher.imp().model_type_selector.connect_selected_notify(clone!(
        #[weak(rename_to = win)]
        win,
        move |dropdown| {
            if win.consume_skin_import_model_change() {
                return;
            }
            let model_type = match dropdown.selected() {
                0 => ModelType::Slim,
                1 => ModelType::Classic,
                _ => panic!("Unknown model type"),
            };
            win.change_model_type(model_type);
        }
    ));
}

fn sync_inner_layer(win: &Window, model_switcher: &ModelSwitcher) {
    let layer_active = model_switcher.inner_layer_toggle().is_active();
    win.set_body_parts_visible(&[
        (&Head, layer_active && model_switcher.head().is_active()),
        (&Torso, layer_active && model_switcher.torso().is_active()),
        (&RightArm, layer_active && model_switcher.right_arm().is_active()),
        (&LeftArm, layer_active && model_switcher.left_arm().is_active()),
        (&RightLeg, layer_active && model_switcher.right_leg().is_active()),
        (&LeftLeg, layer_active && model_switcher.left_leg().is_active()),
    ]);
}

fn sync_outer_layer(win: &Window, model_switcher: &ModelSwitcher) {
    let layer_active = model_switcher.outer_layer_toggle().is_active();
    win.set_body_parts_visible(&[
        (&HeadOuter, layer_active && model_switcher.head().is_active()),
        (&TorsoOuter, layer_active && model_switcher.torso().is_active()),
        (&RightArmOuter, layer_active && model_switcher.right_arm().is_active()),
        (&LeftArmOuter, layer_active && model_switcher.left_arm().is_active()),
        (&RightLegOuter, layer_active && model_switcher.right_leg().is_active()),
        (&LeftLegOuter, layer_active && model_switcher.left_leg().is_active()),
    ]);
}

fn connect_part_toggle(
    model_switcher: &ModelSwitcher,
    win: Window,
    inner: BodyPart,
    outer: BodyPart,
    part_toggle: impl Fn(&ModelSwitcher) -> gtk::ToggleButton + 'static,
) {
    part_toggle(model_switcher).connect_toggled(clone!(
        #[weak]
        model_switcher,
        #[weak(rename_to = win)]
        win,
        move |toggle| {
            let visible = toggle.is_active();
            let mut updates = Vec::new();
            if model_switcher.inner_layer_toggle().is_active() {
                updates.push((&inner, visible));
            }
            if model_switcher.outer_layer_toggle().is_active() {
                updates.push((&outer, visible));
            }
            if !updates.is_empty() {
                win.set_body_parts_visible(&updates);
            }
        }
    ));
}
