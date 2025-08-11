use yew::prelude::*;
use crate::game::Mancala;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game: Mancala,
    pub on_pit_click: Callback<usize>,
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let board = &props.game.board;

    html! {
        <div class="board">
            <div class="player-2-store">{board[13]}</div>
            <div class="pits">
                <div class="player-2-pits">
                    {(7..13).rev().map(|i| html!{
                        <div class="pit" onclick={props.on_pit_click.reform(move |_| i)}>{board[i]}</div>
                    }).collect::<Html>()}
                </div>
                <div class="player-1-pits">
                    {(0..6).map(|i| html!{
                        <div class="pit" onclick={props.on_pit_click.reform(move |_| i)}>{board[i]}</div>
                    }).collect::<Html>()}
                </div>
            </div>
            <div class="player-1-store">{board[6]}</div>
        </div>
    }
}
