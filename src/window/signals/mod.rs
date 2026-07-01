mod file_io;
mod grid;
mod model_switcher;
mod reset_skin;
mod tools;
mod wardrobe;

use super::Window;

pub(super) fn connect(win: &Window) {
    wardrobe::connect(win);
    reset_skin::connect(win);
    tools::connect(win);
    grid::connect(win);
    file_io::connect(win);
    model_switcher::connect(win);
}
