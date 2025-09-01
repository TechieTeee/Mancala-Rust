use web_sys::HtmlAudioElement;
use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct AudioManager {
    background_music: Option<HtmlAudioElement>,
}

impl AudioManager {
    pub fn new() -> Self {
        let background_music = HtmlAudioElement::new().ok().map(|audio| {
            audio.set_src("static/afro-beat-pop-african-afrobeat-music-357196.mp3");
            audio.set_loop(true);
            audio.set_volume(0.15); // Low volume for background
            audio
        });
        
        Self { background_music }
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
        let audio = HtmlAudioElement::new().ok();
        if let Some(audio) = audio {
            match sound_type {
                DrumSound::Click => {
                    // Simple drum click sound - quick tap
                    audio.set_src("data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFbF1fdJivrJBhNjVgodDbq2EcBj+a2/LDciUFLIHO8tiJNwgZaLvt559NEAxQp+PwtmMcBjiR1/LMeSwFJHfH8N2QQAoUXrTp66hVFApGn+DyvmMcBz2Y2e3A");
                    audio.set_volume(0.4);
                },
                DrumSound::NewGame => {
                    // New game celebration - restart background music
                    self.pause_background_music();
                    self.play_background_music();
                    return;
                }
            }
            let _ = audio.play();
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
    let audio_state = use_state(|| Rc::new(AudioManager::new()));
    (*audio_state).clone()
}