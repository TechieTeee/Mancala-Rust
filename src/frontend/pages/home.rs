use yew::prelude::*;
use crate::game::Mancala;
use crate::frontend::components::board::Board;
use crate::frontend::audio::{use_audio, DrumSound};
use std::rc::Rc;

#[function_component(Home)]
pub fn home() -> Html {
    let game = use_state(Mancala::new);
    let audio = use_audio();
    let show_instructions = use_state(|| true); // Show on first load
    let music_playing = use_state(|| false);

    let on_pit_click = {
        let game = game.clone();
        Callback::from(move |pit_index: usize| {
            let mut new_game = (*game).clone();
            // Only allow moves for current player's pits
            let valid_move = if new_game.current_player == 0 && (0..6).contains(&pit_index) {
                true
            } else if new_game.current_player == 1 && (7..13).contains(&pit_index) {
                true
            } else {
                false
            };
            
            if valid_move {
                new_game.make_move(pit_index);
                game.set(new_game);
            }
        })
    };

    let reset_game = {
        let game = game.clone();
        let audio_clone = Rc::clone(&audio);
        Callback::from(move |_| {
            audio_clone.play_drum_sound(DrumSound::NewGame);
            game.set(Mancala::new());
        })
    };

    let hide_instructions = {
        let show_instructions = show_instructions.clone();
        Callback::from(move |_| {
            show_instructions.set(false);
        })
    };

    let show_instructions_again = {
        let show_instructions = show_instructions.clone();
        Callback::from(move |_| {
            show_instructions.set(true);
        })
    };

    let toggle_music = {
        let audio_clone = Rc::clone(&audio);
        let music_playing = music_playing.clone();
        Callback::from(move |_| {
            if *music_playing {
                audio_clone.pause_background_music();
                music_playing.set(false);
            } else {
                audio_clone.play_background_music();
                music_playing.set(true);
            }
        })
    };

    // Determine game state messages
    let (turn_message, game_status) = if game.game_over {
        let winner_msg = match game.winner {
            Some(0) => "Player 1 Wins! 🎉",
            Some(1) => "Player 2 Wins! 🎉", 
            None => "It's a Draw! 🤝",
            Some(_) => "Game Over!", // Handle any other values
        };
        ("Game Over".to_string(), winner_msg.to_string())
    } else {
        let current_player = if game.current_player == 0 { "Player 1" } else { "Player 2" };
        (format!("{}'s Turn", current_player), "Click your pits to move stones".to_string())
    };

    html! {
        <div class="container">
            <h1>{ "Mancala" }</h1>
            
            // Show confetti when game is over and there's a winner
            if game.game_over && game.winner.is_some() {
                <div class="confetti">
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                    <div class="confetti-piece"></div>
                </div>
            }
            
            if *show_instructions {
                <div class="instructions-modal">
                    <div class="instructions-content">
                        <h2>{"Welcome to Mancala"}</h2>
                        <div class="instructions-text">
                            <div class="cultural-background">
                                <p class="intro"><em>{"Mancala is one of the world's oldest and most beloved board games, with roots stretching back over 1,300 years to ancient Africa. This strategic game has traveled across continents and cultures, carrying with it generations of wisdom."}</em></p>
                                
                                <h4>{"The Name 'Mancala' and African Origins"}</h4>
                                <p>{"The word "}<strong>{"\"Mancala\""}</strong>{" comes from the Arabic "}<em>{"naqala"}</em>{", meaning \"to move\" or \"to transfer,\" which perfectly captures what the game is all about. You're moving stones around the board. Over time, this became the umbrella term for an entire family of African count and capture games that share similar strategic principles."}</p>
                                
                                <h4>{"A Game of Many Names"}</h4>
                                <p>{"Throughout Africa, this beloved game goes by different names that reflect local languages and traditions: "}<strong>{"Awale"}</strong>{" in West Africa, "}<strong>{"Bao"}</strong>{" in East Africa, "}<strong>{"Oware"}</strong>{" in Ghana, "}<strong>{"Wari"}</strong>{" in Nigeria, "}<strong>{"Giuthi"}</strong>{" in Kenya, and "}<strong>{"Omweso"}</strong>{" in Uganda."}</p>
                                
                                <p>{"When the African diaspora carried these traditions across the Atlantic, the names traveled too. "}<strong>{"Warri"}</strong>{" took root in Trinidad and the Caribbean, "}<strong>{"Adi"}</strong>{" found a home in Cape Verde, and many variations spread throughout Black communities across the Americas. Each name preserves a piece of linguistic heritage and cultural identity."}</p>
                                
                                <h4>{"More Than a Game, A Training Ground for Warriors"}</h4>
                                <p>{"Mancala wasn't just entertainment. It was a sophisticated training tool for military strategy. African generals and warriors used the game to sharpen crucial battlefield skills like resource management, territorial control, and strategic timing. Think about it. The game's mechanics mirror real warfare perfectly. You're managing limited resources, trying to predict your opponent's moves, and executing multi-step plans."}</p>
                                
                                <p>{"Historical accounts describe military leaders spending hours perfecting their Mancala game, knowing that the strategic thinking it developed would give them an advantage in actual combat. The ability to think several moves ahead and adapt to changing conditions? That's what separated good leaders from great ones."}</p>
                                
                                <h4>{"Building Brilliant Minds"}</h4>
                                <p>{"In traditional African societies, Mancala was how children learned to think. Through play, they absorbed complex mathematical concepts like counting, addition, probability, pattern recognition, and spatial reasoning. Every move teaches you that actions have consequences, sometimes immediate, sometimes several turns down the line."}</p>
                                
                                <p>{"Communities recognized that strong Mancala players had sharp, analytical minds. Game mastery was seen as a sign of intelligence and wisdom, and skilled players often became trusted advisors. The game built essential life skills like patience, foresight, adaptability, and the ability to find patterns in seemingly chaotic situations."}</p>
                                
                                <h4>{"Connecting Communities Across Time and Space"}</h4>
                                <p>{"Today, Mancala continues to bring people together around the world. Whether it's in the bustling markets of Accra, Brooklyn community centers, Caribbean family gatherings, or international tournaments in Europe, the game serves as a cultural bridge. It keeps alive the intellectual heritage of Africa, passing down centuries of strategic knowledge to new generations."}</p>
                                
                                <p>{"In our increasingly digital world, Mancala offers something special. It's genuine face to face strategic engagement rooted in ancestral knowledge. Every game you play today honors the many generations who perfected this beautiful blend of mathematics, strategy, and cultural expression."}</p>
                                
                                <p class="closing"><em><strong>{"Your move. Let's play."}</strong></em></p>
                            </div>
                            
                            <h3>{"How to Play"}</h3>
                            <p><strong>{"🎯 Goal:"}</strong> {" Collect more stones than your opponent in your store!"}</p>
                            
                            <p><strong>{"⚙️ Setup:"}</strong> {" Each player controls 6 pits with 4 stones each. Player 1 controls the bottom row, Player 2 controls the top row. The large pits on each side are your 'stores' where you collect captured stones."}</p>
                            
                            <p><strong>{"🎮 Your Turn:"}</strong> {" Click on one of your pits to pick up all stones. Drop them one by one into the following pits, moving counterclockwise around the board (including your store, but skipping your opponent's store)."}</p>
                            
                            <p><strong>{"🎉 Extra Turn:"}</strong> {" If your last stone lands in your own store, you get another turn immediately!"}</p>
                            
                            <p><strong>{"💎 Capture Stones:"}</strong> {" If your last stone lands in an empty pit on your side, and the opposite pit has stones, you capture all those stones plus your own stone!"}</p>
                            
                            <p><strong>{"🏆 Winning:"}</strong> {" The game ends when all pits on one side are empty. The player with the most stones in their store wins!"}</p>
                            
                            <p class="tip"><strong>{"💡 Strategy Tip:"}</strong> {" Plan your moves carefully to get extra turns and set up captures. The key is controlling the flow of stones around the board!"}</p>
                        </div>
                        <button class="close-instructions" onclick={hide_instructions}>{"Start Playing!"}</button>
                    </div>
                </div>
            }
            
            <div class="game-status">
                <div class="turn-indicator">
                    <span class="turn-message">{turn_message}</span>
                    <span class="status-message">{game_status}</span>
                </div>
                <div class="controls">
                    <button class="music-button" onclick={toggle_music}>
                        {if *music_playing { "🔊" } else { "🔇" }}
                    </button>
                    <button class="help-button" onclick={show_instructions_again}>{"?"}</button>
                </div>
            </div>
            
            <Board game={(*game).clone()} on_pit_click={on_pit_click} />
            
            <div class="game-controls">
                <button onclick={reset_game}>{ "New Game" }</button>
            </div>
        </div>
    }
}

