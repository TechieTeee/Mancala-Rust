use web_sys::HtmlAudioElement;
use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct AudioManager {
    background_music: Option<HtmlAudioElement>,
    stone_drop: Option<HtmlAudioElement>,
    capture: Option<HtmlAudioElement>,
    invalid_move: Option<HtmlAudioElement>,
    extra_turn: Option<HtmlAudioElement>,
    game_over: Option<HtmlAudioElement>,
}

fn make_audio(src: &str, volume: f64) -> Option<HtmlAudioElement> {
    HtmlAudioElement::new().ok().map(|audio| {
        audio.set_src(src);
        audio.set_volume(volume);
        audio
    })
}

fn play(audio: &Option<HtmlAudioElement>) {
    if let Some(a) = audio {
        a.set_current_time(0.0);
        let _ = a.play();
    }
}

// Short base64-encoded WAV sounds for different game events
const STONE_DROP_SRC: &str = "data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFbF1fdJivrJBhNjVgodDbq2EcBj+a2/LDciUFLIHO8tiJNwgZaLvt559NEAxQp+PwtmMcBjiR1/LMeSwFJHfH8N2QQAoUXrTp66hVFApGn+DyvmMcBz2Y2e3A";
const CAPTURE_SRC: &str = "data:audio/wav;base64,UklGRl9nAABXQVZFZm10IBIAAAABAAEARKwAAIhYAQACABAAAABkYXRhO2cAAP//AgAFAAkADAAOAA8ADwANAAoABgACAP7/+v/3//X/9P/0//X/9//5//z//v8AAAIAAwAEAAQABAADAAIAAQD///7//P/7//r/+f/5//n/+v/7//z//f/+/wAAAQACAAMAAwADAAMAAgABAP///v/9//z/+//7//v/+//7//z//f/+////AAABAAIABAAEAAQABAARABEAEQARABEAEQA=";
const INVALID_SRC: &str = "data:audio/wav;base64,UklGRjIAAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQ4AAACAgoSGh4aFgoCA";
const EXTRA_TURN_SRC: &str = "data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACAgYOFh4iJiYiHhYOBgH+AgYOFh4iJiYiHhYOBgH+AgYOFh4iJiYiHhYOBgA==";
const GAME_OVER_SRC: &str = "data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFbF1fdJivrJBhNjVgodDbq2EcBj+a2/LDciUFLIHO8tiJNwgZaLvt559NEAxQp+Pwt2McBjiR1/LMeSwFJHfH8N2QQAoUXrTp66hVFApGn+DyvmwhBz2Y2e3AcSMEK3/L7taGMwoWZLjn45pLEg5Sp+Tys2AaBTiR1/LMeSwFJHfH8N2QQAo=";

impl AudioManager {
    pub fn new() -> Self {
        let background_music = HtmlAudioElement::new().ok().map(|audio| {
            audio.set_src("static/afro-beat-pop-african-afrobeat-music-357196.mp3");
            audio.set_loop(true);
            audio.set_volume(0.15);
            audio
        });

        Self {
            background_music,
            stone_drop: make_audio(STONE_DROP_SRC, 0.3),
            capture: make_audio(CAPTURE_SRC, 0.5),
            invalid_move: make_audio(INVALID_SRC, 0.3),
            extra_turn: make_audio(EXTRA_TURN_SRC, 0.4),
            game_over: make_audio(GAME_OVER_SRC, 0.5),
        }
    }

    pub fn play_background_music(&self) {
        if let Some(audio) = &self.background_music {
            let _ = audio.play();
        }
    }

    pub fn pause_background_music(&self) {
        if let Some(audio) = &self.background_music {
            audio.pause().ok();
        }
    }

    pub fn play_sound(&self, event: SoundEvent) {
        match event {
            SoundEvent::StoneDrop => play(&self.stone_drop),
            SoundEvent::Capture => play(&self.capture),
            SoundEvent::InvalidMove => play(&self.invalid_move),
            SoundEvent::ExtraTurn => play(&self.extra_turn),
            SoundEvent::GameOver => play(&self.game_over),
            SoundEvent::NewGame => {
                self.pause_background_music();
                self.play_background_music();
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SoundEvent {
    StoneDrop,
    Capture,
    InvalidMove,
    ExtraTurn,
    GameOver,
    NewGame,
}

#[hook]
pub fn use_audio() -> Rc<AudioManager> {
    use_context::<Rc<AudioManager>>().expect("AudioManager context not found. Wrap your app in <AudioProvider>.")
}

#[derive(Properties, PartialEq)]
pub struct AudioProviderProps {
    pub children: Html,
}

#[function_component(AudioProvider)]
pub fn audio_provider(props: &AudioProviderProps) -> Html {
    let audio = use_memo((), |_| Rc::new(AudioManager::new()));

    html! {
        <ContextProvider<Rc<AudioManager>> context={(*audio).clone()}>
            { props.children.clone() }
        </ContextProvider<Rc<AudioManager>>>
    }
}
