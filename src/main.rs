#![feature(result_flattening)]
mod app;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
