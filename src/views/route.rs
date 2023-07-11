#[derive(Clone, yew_router::Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[at("/theses")]
    ThesesRoot,
    #[at("/theses/*")]
    Theses,
    #[at("/versions/:id")]
    Versions { id: bson::oid::ObjectId },
}