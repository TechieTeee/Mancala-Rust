use yew::prelude::*;
use yew_router::prelude::*;

use crate::frontend::audio::AudioProvider;
use crate::frontend::pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AudioProvider>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </AudioProvider>
    }
}
