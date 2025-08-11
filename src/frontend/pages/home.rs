use yew::prelude::*;
use crate::game::Mancala;
use crate::frontend::components::board::Board;

#[function_component(Home)]
pub fn home() -> Html {
    let game = use_state(Mancala::new);

    let on_pit_click = {
        let game = game.clone();
        Callback::from(move |pit_index: usize| {
            let mut new_game = (*game).clone();
            new_game.make_move(pit_index);
            game.set(new_game);
        })
    };

    html! {
        <div class="container">
            <h1>{"Mancala"}</h1>
            <Board game={(*game).clone()} on_pit_click={on_pit_click} />
            <button onclick={let game = game.clone(); move |_| game.set(Mancala::new())}>{"New Game"}</button>
        </div>
    }
}
