use yew::prelude::*;

use crate::frontend::audio::AudioProvider;
use crate::frontend::pages::home::Home;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AudioProvider>
            <Home />
        </AudioProvider>
    }
}
