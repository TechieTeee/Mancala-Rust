use yew::prelude::*;
use crate::game::Mancala;
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
            <div class="player-2-store">{board[13]}</div>
            <div class="pits">
                <div class="player-2-pits">
                    {(7..13).rev().map(|i| {
                        let audio_clone = Rc::clone(&audio);
                        let on_click = props.on_pit_click.reform(move |_| i);
                        html!{
                            <div class="pit" onclick={Callback::from(move |_| {
                                audio_clone.play_drum_sound(DrumSound::Click);
                                on_click.emit(());
                            })}>{board[i]}</div>
                        }
                    }).collect::<Html>()}
                </div>
                <div class="player-1-pits">
                    {(0..6).map(|i| {
                        let audio_clone = Rc::clone(&audio);
                        let on_click = props.on_pit_click.reform(move |_| i);
                        html!{
                            <div class="pit" onclick={Callback::from(move |_| {
                                audio_clone.play_drum_sound(DrumSound::Click);
                                on_click.emit(());
                            })}>{board[i]}</div>
                        }
                    }).collect::<Html>()}
                </div>
            </div>
            <div class="player-1-store">{board[6]}</div>
        </div>
    }
}
