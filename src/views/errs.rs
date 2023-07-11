impl crate::models::common::AppError {
    pub fn view(&self) -> yew::Html {
        yew::html! {
            <div>
                <p>{ self.to_string() }</p>
                <p>{ "Please report this error to our administrator!" }</p>
            </div>
        }
    }
}