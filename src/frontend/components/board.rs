use yew::prelude::*;
use crate::game::{Mancala, PLAYER1_STORE, PLAYER2_STORE, PLAYER1_PITS, PLAYER2_PITS};
use crate::frontend::audio::{use_audio, DrumSound};
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game: Mancala,
    pub on_pit_click: Callback<usize>,
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let board = &props.game.board;
    let audio = use_audio();

    html! {
        <div class="board">
            <div class="player-2-store">{board[PLAYER2_STORE]}</div>
            <div class="pits">
                <div class="player-2-pits">
                    {PLAYER2_PITS.rev().map(|i| {
                        let audio = Rc::clone(&audio);
                        let on_click = props.on_pit_click.reform(move |_| i);
                        html!{
                            <div class="pit" onclick={Callback::from(move |_| {
                                audio.play_drum_sound(DrumSound::Click);
                                on_click.emit(());
                            })}>{board[i]}</div>
                        }
                    }).collect::<Html>()}
                </div>
                <div class="player-1-pits">
                    {PLAYER1_PITS.map(|i| {
                        let audio = Rc::clone(&audio);
                        let on_click = props.on_pit_click.reform(move |_| i);
                        html!{
                            <div class="pit" onclick={Callback::from(move |_| {
                                audio.play_drum_sound(DrumSound::Click);
                                on_click.emit(());
                            })}>{board[i]}</div>
                        }
                    }).collect::<Html>()}
                </div>
            </div>
            <div class="player-1-store">{board[PLAYER1_STORE]}</div>
        </div>
    }
}
