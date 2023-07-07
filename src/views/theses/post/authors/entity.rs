use yew::{Callback, Html, html};
use crate::models::{common::AppConfig, profile::PublicProfile};

pub(super) fn view_entity(val: &PublicProfile, index: usize, delete: &Callback<usize>, cfg: &AppConfig) -> Html {
    let onclick = {
        let delete = delete.clone();
        move |_| delete.emit(index)
    };
    html! {
        <div>
            { val.clone().view_tiny(cfg) }
            <button {onclick} >{ " ❎" }</button>
        </div>
    }
}
