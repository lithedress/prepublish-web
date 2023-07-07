use std::rc::Rc;

use yew::{Component, Context, Html, html, Properties};

use crate::models::{thesis::Thesis, profile::PublicProfile, common::AppConfig};

#[derive(Default)]
pub struct Page {
    err: Option<crate::models::common::FetchError>,
}

pub enum PageMsg {
}

#[derive(PartialEq, Properties)]
pub struct PageProps {
    pub val: Rc<Thesis>,
    pub cfg: AppConfig,
}

impl Component for Page {
    type Message = PageMsg;
    type Properties = PageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref err) = self.err {
            return err.view();
        }
        let cfg = ctx.props().cfg.to_owned();
        let val = ctx.props().val.clone();
        html! {
            <div>
                <h1>
                    { val.title.clone() }
                </h1>

                <h3>
                    <ul>
                        { for val.author_ids.iter().map(|id| PublicProfile::tiny_from_id(id.to_owned(), cfg.clone())) }
                    </ul>
                </h3>

                <p>
                    { val.abstraction.clone() }
                </p>

                <p><b>
                    <ul>
                        { for val.keywords.iter() } 
                    </ul>
                </b></p>
            </div>
        }
    }
}