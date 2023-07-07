use yew::{Component, Context, Html, html, Properties};

#[derive(Default)]
pub struct AlertBox {
    content: Option<std::rc::Rc<crate::models::common::FetchOther>>,
}

#[derive(PartialEq, Properties)]
pub struct AlertBoxProps {
    pub refresh: Option<std::rc::Rc<crate::models::common::FetchOther>>,
}

impl Component for AlertBox {
    type Message = ();
    type Properties = AlertBoxProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            content: ctx.props().refresh.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.content = ctx.props().refresh.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref res) = self.content {
            html! {
                <b>
                    <p>{ res.status }</p>
                    <p>{ "Please report this error to our administrator!" }</p>
                    <p>{ &res.msg }</p>
                </b>
            }
        } else {
            Html::default()
        }
        
    }
}