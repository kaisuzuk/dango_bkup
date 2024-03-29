use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <h1 class="title">
                { "Page not found" }
            </h1>
            <Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>
            </>
        }
    }
}
