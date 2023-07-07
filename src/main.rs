#![feature(result_flattening)]
#![feature(anonymous_lifetime_in_impl_trait)]
mod app;
mod models;
mod views;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
