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
            Self::View { id } => {
                let fallback = yew::html! {<div>{format!("Loading thesis {id}...")}</div>};
                yew::html! {
                    <yew::Suspense {fallback}>
                        <get::Get cfg={cfg.clone()} {id} />
                    </yew::Suspense>
                    }
                }
            _ => yew::html!()
        }
    }
}