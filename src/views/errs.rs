impl crate::models::common::FetchError {
    pub fn view(&self) -> yew::Html {
        yew::html! {
            <div style="color:red">
                <p>{ self.to_string() }</p>
                <p>{ "Please report this error to our administrator!" }</p>
            </div>
        }
    }
}