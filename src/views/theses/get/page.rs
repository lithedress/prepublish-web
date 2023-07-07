use yew::{AttrValue, Callback, Component, Context, Html, html, Properties, TargetCast, html_nested};
use web_sys::{HtmlInputElement, InputEvent};

#[derive(Default)]
pub struct Get {
    err: Option<crate::models::common::FetchError>,

    thesis: crate::models::thesis::Thesis,
    //versions
}

pub enum GetMsg {
}

#[derive(PartialEq, Properties)]
pub struct GetProps {
    pub id: bson::oid::ObjectId,
}

impl Component for Get {
    type Message = GetMsg;
    type Properties = GetProps;

    fn create(ctx: &Context<Self>) -> Self {

        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref err) = self.err {
            return err.view();
        }
        html! {
            
        }
    }
}