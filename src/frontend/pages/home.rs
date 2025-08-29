use yew::prelude::*;
use crate::game::Mancala;
use crate::frontend::components::board::Board;

#[function_component(Home)]
pub fn home() -> Html {
    let game = use_state(Mancala::new);

    let on_pit_click = {
        let game = game.clone();
        Callback::from(move |pit_index: usize| {
            game.set(game.deref().make_move(pit_index));
        })
    };

    let reset_game = {
        let game = game.clone();
        Callback::from(move |_| game.set(Mancala::new()))
    };

    html! {
        <div class="container">
            <h1>{ "Mancala" }</h1>
            <Board game={(*game).clone()} on_pit_click={on_pit_click} />
            <button {reset_game}>{ "New Game" }</button>
        </div>
    }
}

