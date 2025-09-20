mod app;
mod models;
mod view_models;
mod views;

fn main() {
    sycamore::render(app::App);
}
