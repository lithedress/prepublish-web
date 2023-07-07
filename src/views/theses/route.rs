#[derive(Clone, yew_router::Routable, PartialEq)]
pub enum Route {
    #[at("/theses")]
    List,
    #[at("/theses/new")]
    New,
    #[at("/theses/:id")]
    View { id: bson::oid::ObjectId },
}