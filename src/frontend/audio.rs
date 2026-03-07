use web_sys::HtmlAudioElement;
use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct AudioManager {
    background_music: Option<HtmlAudioElement>,
    click_sound: Option<HtmlAudioElement>,
}

impl AudioManager {
    pub fn new() -> Self {
        let background_music = HtmlAudioElement::new().ok().map(|audio| {
            audio.set_src("static/afro-beat-pop-african-afrobeat-music-357196.mp3");
            audio.set_loop(true);
            audio.set_volume(0.15);
            audio
        });

        let click_sound = HtmlAudioElement::new().ok().map(|audio| {
            audio.set_src("data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFbF1fdJivrJBhNjVgodDbq2EcBj+a2/LDciUFLIHO8tiJNwgZaLvt559NEAxQp+PwtmMcBjiR1/LMeSwFJHfH8N2QQAoUXrTp66hVFApGn+DyvmMcBz2Y2e3A");
            audio.set_volume(0.4);
            audio
        });

        Self { background_music, click_sound }
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

    pub fn play_drum_sound(&self, sound_type: DrumSound) {
        match sound_type {
            DrumSound::Click => {
                if let Some(audio) = &self.click_sound {
                    audio.set_current_time(0.0);
                    let _ = audio.play();
                }
            }
            DrumSound::NewGame => {
                self.pause_background_music();
                self.play_background_music();
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum DrumSound {
    Click,
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
