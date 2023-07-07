use yew::{AttrValue, Callback, Html};

fn view_entity(val: &AttrValue, index: usize, delete: &Callback<usize>) -> Html {
    let onclick = {
        let delete = delete.clone();
        move |_| delete.emit(index)
    };
    html! {
        <div>
            { val.clone() }
            <button {onclick} >{ " ❎" }</button>
        </div>
    }
}