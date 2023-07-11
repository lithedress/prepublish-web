use yew::{AttrValue, Callback, Html, html};

pub(super) fn view_entity(val: &AttrValue, index: usize, delete: &Callback<usize>) -> Html {
    let onclick = {
        let delete = delete.clone();
        move |_| delete.emit(index)
    };
    html! {
        <div>
            { val.clone() }
            <label {onclick} >{ "❎" }</label>
        </div>
    }
}
