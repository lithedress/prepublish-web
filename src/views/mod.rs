use yew::{Html, html};

mod versions;
pub mod route;
mod alerts;
mod errs;
mod theses;
mod tiny_profiles;
mod signup;
mod login;

pub(crate) use route::Route;

impl Route {
    pub(super) fn switch(cfg: std::rc::Rc<crate::models::common::AppConfig>) -> impl Fn(Self) -> Html {
        move |routes| match routes {
            Self::ThesesRoot | Self::Theses => html! {
                <yew_router::Switch<theses::Route> render={ theses::Route::switch(cfg.clone()) }/>
            },
            Self::Home => html! {
                <div>
                    { "WIP" }
                </div>
            },
            Self::Signup => html! {
                <signup::Signup cfg={(*cfg).clone()} />
            },
            Self::Login => html! {
                <login::Login cfg={(*cfg).clone()} />
            },
            Self::Versions { id } => html! {
                <versions::Get cfg={(*cfg).clone()} {id} />
            },
        }
    }
}
