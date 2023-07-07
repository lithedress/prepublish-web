pub mod versions;
pub(super) mod route;
mod alerts;
mod errs;
mod theses;
mod tiny_profiles;

impl route::Route {
    pub(super) fn switch(cfg: std::rc::Rc<crate::models::common::AppConfig>) -> impl Fn(Self) -> yew::Html {
        move |routes| match routes {
            Self::ThesesRoot | Self::Theses => yew::html! {
                <yew_router::Switch<theses::Route> render={ theses::Route::switch(cfg.clone()) }/>
            },
            _ => todo!()
        }
    }
}