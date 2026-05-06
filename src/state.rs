use std::time::{Duration, Instant};

pub struct AppState {
    pub last_play_time: Option<Instant>,
    pub was_playing: bool,
    pub idle_timeout: Duration,
}

impl AppState {
    pub fn new(idle_timeout_secs: u64) -> Self {
        Self {
            last_play_time: None,
            was_playing: false,
            idle_timeout: Duration::from_secs(idle_timeout_secs),
        }
    }

    pub fn update_playing(&mut self, is_playing: bool) {
        if is_playing {
            self.last_play_time = Some(Instant::now());
            self.was_playing = true;
        } else if self.was_playing && self.last_play_time.is_some() {
            self.was_playing = false;
        }
    }

    pub fn should_clear(&self) -> bool {
        if let Some(last_play) = self.last_play_time {
            last_play.elapsed() >= self.idle_timeout
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.last_play_time = None;
        self.was_playing = false;
    }
}