use mpris::{Metadata, PlaybackStatus, Player, PlayerFinder};
use std::time::Duration;

pub struct MprisPlayer {
    player: Player,
    pub artist: String,
    pub duration: u64,
    pub progress: u64,
    pub title: String,
    pub status: String,
}

impl MprisPlayer {
    pub fn new() -> Self {
        let player = PlayerFinder::new().unwrap().find_active().unwrap();

        Self {
            player,
            artist: String::new(),
            duration: 0,
            progress: 0,
            title: String::new(),
            status: String::new(),
        }
    }

    fn get_time(duration: Option<Duration>) -> u64 {
        match duration {
            Some(duration) => duration.as_secs(),
            None => 0,
        }
    }

    fn get_artist(metadata: &Metadata) -> String {
        match metadata.artists() {
            Some(artists) if !artists.is_empty() => artists.join(" + "),
            _ => "Unknown artist".to_string(),
        }
    }

    fn get_title(metadata: &Metadata) -> String {
        metadata.title().unwrap_or("Unknown title").to_string()
    }

    fn get_playback_status(status: PlaybackStatus) -> String {
        match status {
            PlaybackStatus::Playing => "▶".to_string(),
            PlaybackStatus::Paused => "▮▮".to_string(),
            PlaybackStatus::Stopped => "◼".to_string(),
        }
    }

    pub fn update(&mut self) {
        let metadata = match self.player.get_metadata() {
            Ok(m) => m,
            Err(_) => return, // player fechou/desapareceu, não atualiza
        };

        let position = self.player.get_position().ok();
        let status = self
            .player
            .get_playback_status()
            .unwrap_or(PlaybackStatus::Stopped);

        self.artist = Self::get_artist(&metadata);
        self.title = Self::get_title(&metadata);
        self.duration = Self::get_time(metadata.length());
        self.progress = Self::get_time(position);
        self.status = Self::get_playback_status(status);
    }
}
