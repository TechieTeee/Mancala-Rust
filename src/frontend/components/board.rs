use yew::prelude::*;
use crate::game::{Mancala, PLAYER1_STORE, PLAYER2_STORE, PLAYER1_PITS, PLAYER2_PITS};
use crate::frontend::audio::{use_audio, SoundEvent};
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game: Mancala,
    pub on_pit_click: Callback<usize>,
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let board = &props.game.board;
    let current_player = props.game.current_player;
    let game_over = props.game.game_over;
    let audio = use_audio();
    let shaking_pit = use_state(|| Option::<usize>::None);

    let render_pit = |i: usize, is_own_side: bool| {
        let audio = Rc::clone(&audio);
        let on_pit_click = props.on_pit_click.clone();
        let shaking_pit = shaking_pit.clone();
        let stones = board[i];
        let is_valid = is_own_side && !game_over && stones > 0;
        let is_shaking = *shaking_pit == Some(i);

        let mut classes = vec!["pit"];
        if !is_valid {
            classes.push("pit-disabled");
        }
        if is_shaking {
            classes.push("pit-shake");
        }
        if stones == 0 {
            classes.push("pit-empty");
        }

        let onclick = Callback::from(move |_: MouseEvent| {
            if is_valid {
                audio.play_sound(SoundEvent::StoneDrop);
                on_pit_click.emit(i);
            } else if !game_over {
                audio.play_sound(SoundEvent::InvalidMove);
                shaking_pit.set(Some(i));
                let shaking_pit = shaking_pit.clone();
                gloo_timers::callback::Timeout::new(400, move || {
                    shaking_pit.set(None);
                }).forget();
            }
        });

        html! {
            <div class={classes.join(" ")} {onclick}>{stones}</div>
        }
    };

    html! {
        <div class="board">
            <div class="store-wrapper">
                <span class="store-label">{"Player 2"}</span>
                <div class="player-2-store">{board[PLAYER2_STORE]}</div>
            </div>
            <div class="pits">
                <div class="player-2-pits">
                    {PLAYER2_PITS.rev().map(|i| render_pit(i, current_player == 1)).collect::<Html>()}
                </div>
                <div class="player-1-pits">
                    {PLAYER1_PITS.map(|i| render_pit(i, current_player == 0)).collect::<Html>()}
                </div>
            </div>
            <div class="store-wrapper">
                <span class="store-label">{"Player 1"}</span>
                <div class="player-1-store">{board[PLAYER1_STORE]}</div>
            </div>
        </div>
    }
}
