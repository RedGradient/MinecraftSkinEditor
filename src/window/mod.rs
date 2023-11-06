use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use gtk::{EventControllerMotion, EventControllerScroll, EventControllerScrollFlags, FileChooserAction, FileChooserNative, GestureClick, gio};
use gtk::gdk::RGBA;
use gtk::glib;
use gtk::glib::{clone, ObjectExt, Propagation};
use gtk::prelude::{ButtonExt, FileChooserExt, FileExt, GLAreaExt, GtkWindowExt, NativeDialogExtManual, ToggleButtonExt};
use gtk::prelude::ColorChooserExt;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use libadwaita as adw;
use log::info;

use Tool::*;

use crate::application::Application;
use crate::glium_area::body_part::BodyPart::*;
use crate::glium_area::hover_state::HoverState;

mod imp;


#[derive(Copy, Clone)]
pub enum Tool {
    Pencil,
    Rubber,
    ColorPicker,
    Fill,
    Dither,
}

impl Default for Tool {
    fn default() -> Self {
        Self::Pencil
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

        win.connect_signals();

        win
    }

    fn connect_signals(&self) {
        self.connect_open_button();
        self.connect_model_switcher();
        self.connect_gl_area();


        self.imp().save_button.connect_clicked(
            clone!(@weak self as win => move |btn| {
                let renderer = win.imp().gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                let texture = renderer.export_texture();
                match texture.save("resources/textures/texture.png") {
                    Ok(()) => println!("Saved"),
                    Err(error) => println!("{}", error.to_string()),
                }
            })
        );

        // self.imp().color_button.connect_color_set();

        let pencil_ico = gtk::Image::from_file("resources/pencil.png");
        self.imp().pencil.set_child(Some(&pencil_ico));

        self.imp().pencil.connect_toggled(
            clone!(@weak self as win => move |btn| {
                win.action_set_enabled("win.test", true);
                win.imp().current_tool.replace(Tool::Pencil);
            })
        );

        let rubber_ico = gtk::Image::from_file("resources/eraser.png");
        self.imp().rubber.set_child(Some(&rubber_ico));
        self.imp().rubber.connect_toggled(
            clone!(@weak self as win => move |btn| {
                win.imp().current_tool.replace(Tool::Rubber);
            })
        );

        let color_picker_ico = gtk::Image::from_file("resources/color_picker.png");
        self.imp().color_picker.set_child(Some(&color_picker_ico));
        self.imp().color_picker.connect_toggled(
            clone!(@weak self as win => move |btn| { win.imp().current_tool.replace(Tool::ColorPicker); })
        );
    }

    fn activate_pencil_toggle(&self) {
        self.imp().pencil.set_active(true);
    }

    fn connect_model_switcher(&self) {
        let model_switcher = self.imp().model_switcher.get();
        let gl_area = self.imp().gl_area.get();

        model_switcher.inner_layer_toggle().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
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
            @strong model_switcher, @strong gl_area => move |cb| {
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
            @strong model_switcher, @strong gl_area => move |cb| {
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
            @strong model_switcher, @strong gl_area => move |cb| {
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
            @strong model_switcher, @strong gl_area => move |cb| {
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
            @strong model_switcher, @strong gl_area => move |cb| {
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
        model_switcher.left_leg().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
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
        model_switcher.right_leg().connect_toggled(clone!(
            @strong model_switcher, @strong gl_area => move |cb| {
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
    }

    fn connect_open_button(&self) {
        let dialog = FileChooserNative::new(
            Some("Open a skin"),
            Some(self),
            FileChooserAction::Open,
            Some("Open"),
            Some("Cancel")
        );

        let win = self.clone();
        self.imp().open_button.connect_clicked(move |btn: &gtk::Button| {
            let gl_area = win.imp().gl_area.get();
            dialog.run_async(move |this, _response| {
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                let file = match this.file() {
                    Some(file) => file,
                    None => return,
                };

                let path = file.path().unwrap();
                let path = path.to_str().unwrap();

                renderer.load_texture(path);
                gl_area.queue_draw();
            });
        });
    }

    fn connect_gl_area(&self) {
        // --- CALCULATE FPS ---
        let frame_count = Rc::new(Cell::new(0));
        let current_time = Rc::new(Cell::new(Instant::now()));
        let previous_time = Rc::new(Cell::new(Instant::now()));
        self.imp().gl_area.connect_render(
            clone!(@strong frame_count, @strong current_time, @strong previous_time => move |_, _| {
                frame_count.set(frame_count.get() + 1);
                current_time.set(Instant::now());
                let elapsed_time = current_time.get().duration_since(previous_time.get());
                if elapsed_time.as_secs_f32() > 1.0 {
                    let fps = (frame_count.get() as f32) / elapsed_time.as_secs_f32();
                    println!("FPS: {:.2}", fps);
                    frame_count.set(0);
                    previous_time.set(current_time.get());
                }
                Propagation::Proceed
            })
        );

        // --- CLICK EVENTS ---
        let click = GestureClick::new();
        // --- PRESSED ---
        click.connect_pressed(
            clone!(@weak self as win => move |_, _, x, y| {
                let gl_area = &win.imp().gl_area;
                let renderer = gl_area.renderer().unwrap();
                let mut renderer = renderer.borrow_mut();
                let (x, y) = (x as f32, y as f32);

                let is_clicked_on_model = renderer.is_model_clicked(x, y);
                if is_clicked_on_model {
                    match win.imp().current_tool.get() {
                        Pencil => {
                            let rgba = win.imp().color_button.rgba();
                            let color = [rgba.red(), rgba.green(), rgba.blue(), rgba.alpha()];
                            renderer.paint(x, y, color);
                        },
                        Rubber => {
                            let color = [0.0, 0.0, 0.0, 0.0];
                            renderer.paint(x, y, color);
                        },
                        ColorPicker => {
                            let clicked_cell_color = renderer.get_color(x, y);
                            if let Some(color) = clicked_cell_color {
                                let rgba = RGBA::new(color[0], color[1], color[2], color[3]);
                                win.imp().color_button.set_rgba(&rgba);
                                win.imp().pencil.set_active(true);
                            }
                        }
                        _ => unimplemented!("This tool is unimplemented yet"),
                    }
                    renderer.set_mouse_hover(Some(HoverState::OnModel));
                } else {
                    renderer.set_mouse_hover(Some(HoverState::OnEmptyArea));
                }
                renderer.start_motion(x, y);
                gl_area.queue_draw();
            })
        );
        // --- RELEASED ---
        let g = self.imp().gl_area.clone();
        click.connect_released(move |_, _, _, _| {
            let renderer = g.renderer().unwrap();
            let mut renderer = renderer.borrow_mut();
            renderer.stop_motion();
            renderer.set_mouse_hover(None);
            info!("released");
        });

        // --- MOUSE MOTION ---
        let mv = EventControllerMotion::new();
        let click_event = click.clone();
        let g = self.imp().gl_area.clone();
        mv.connect_motion(move |_, x, y| {
            let renderer = g.renderer().unwrap();
            let mouse_hover_opt = renderer.borrow().get_mouse_hover();
            if let Some(mouse_hover) = mouse_hover_opt {
                match mouse_hover {
                    HoverState::OnModel => {
                        click_event.emit_by_name_with_values("pressed", &[0.into(), x.into(), y.into()]);
                    },
                    HoverState::OnEmptyArea => {
                        renderer.borrow_mut().mouse_move(x as f32, y as f32);
                        renderer.borrow_mut().update_camera();
                    },
                }
                g.queue_draw();
            }
        });

        // --- SCROLL ---
        let scroll = EventControllerScroll::new(EventControllerScrollFlags::VERTICAL);
        let g = self.imp().gl_area.clone();
        scroll.connect_scroll(move |_, _, y| {
            let renderer = g.renderer().unwrap();
            let mut renderer = renderer.borrow_mut();

            let distance = (y as f32) * 0.025;
            renderer.update_scale(distance);

            g.queue_draw();
            Propagation::Proceed
        });

        self.imp().gl_area.add_controller(scroll);
        self.imp().gl_area.add_controller(click);
        self.imp().gl_area.add_controller(mv);
    }
}