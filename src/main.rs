mod app;
mod models;
mod views;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
