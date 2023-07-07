mod route;
mod get;
mod post;

pub(super) use route::Route;

impl Route {
    pub(super) fn switch(cfg: std::rc::Rc<crate::models::common::AppConfig>) -> impl Fn(Self) -> yew::Html {
        move |routes| match routes {
            Self::New => yew::html! {
                <post::Post cfg={cfg.clone()} />
            },
            _ => yew::html!()
        }
    }
}