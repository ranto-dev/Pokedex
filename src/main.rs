mod models;
mod storage;
mod ui;
mod banner;

use banner::show_banner;
use ui::start;

fn main() {
    show_banner();
    start();
}