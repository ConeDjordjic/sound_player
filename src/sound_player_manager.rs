use log::{debug, error, info, warn};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use thiserror::Error;

// Main error type
#[derive(Error, Debug)]
pub enum SoundPlayerError {
    #[error("No song currently loaded")]
    NoSongLoaded,

    #[error("Failed to open audio file: {file}")]
    FileOpenError {
        file: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to decode audio file: {file}")]
    DecodingError {
        file: String,
        #[source]
        source: rodio::decoder::DecoderError,
    },

    #[error("Audio stream error")]
    StreamError(#[from] rodio::StreamError),

    #[error("Seek operation failed: seeking to {position}s")]
    SeekError {
        position: u64,
        #[source]
        source: rodio::source::SeekError,
    },

    #[error("Failed to play audio file: {file}")]
    PlayError {
        file: String,
        #[source]
        source: rodio::PlayError,
    },

    #[error("Invalid volume level: {volume} (must be between 0.0 and 1.0)")]
    InvalidVolume { volume: f32 },

    #[error("Invalid speed: {speed} (must be greater than 0.0)")]
    InvalidSpeed { speed: f32 },

    #[error("Stream handle is no longer valid")]
    InvalidStreamHandle,
}

// Custom result type for convenience
pub type SoundPlayerResult<T> = Result<T, SoundPlayerError>;

pub struct SoundPlayerManager {
    current_song: String,
    stream_handle: rodio::OutputStream,
    sink: Option<rodio::Sink>,
}

impl SoundPlayerManager {
    pub fn new() -> SoundPlayerResult<Self> {
        info!("Initializing sound player...");

        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .map_err(SoundPlayerError::StreamError)?;

        info!("Sound player initialized successfully");

        Ok(Self {
            current_song: String::new(),
            stream_handle,
            sink: None,
        })
    }

    fn get_sink(&self) -> SoundPlayerResult<&rodio::Sink> {
        match self.sink.as_ref() {
            Some(sink) => {
                debug!(
                    "Retrieved audio sink for current song: {}",
                    self.current_song
                );
                Ok(sink)
            }
            None => {
                warn!("No audio sink available - no song is currently loaded");
                Err(SoundPlayerError::NoSongLoaded)
            }
        }
    }

    pub fn play(&mut self, sound_file: &str) -> SoundPlayerResult<()> {
        info!("Attempting to play: {}", sound_file);

        // Stop current song if playing
        if let Some(ref sink) = self.sink {
            info!("Stopping current song: {}", self.current_song);
            sink.stop();
            self.sink = None;
        }

        // Validate file exists and is readable
        debug!("Opening audio file: {}", sound_file);
        let file = File::open(sound_file).map_err(|e| {
            error!("Failed to open file: {} - {}", sound_file, e);
            SoundPlayerError::FileOpenError {
                file: sound_file.to_string(),
                source: e,
            }
        })?;

        debug!("File opened successfully, creating buffer reader");
        let buf_reader = BufReader::new(file);

        // Create sink and start playback
        debug!("Creating audio sink and starting playback");
        let sink = rodio::play(&self.stream_handle.mixer(), buf_reader).map_err(|e| {
            error!("Failed to create sink for: {} - {}", sound_file, e);
            SoundPlayerError::PlayError {
                file: sound_file.to_string(),
                source: e,
            }
        })?;

        self.sink = Some(sink);
        self.current_song = sound_file.to_string();

        info!("Now playing: {}", sound_file);
        Ok(())
    }

    pub fn is_paused(&self) -> SoundPlayerResult<bool> {
        let sink = self.get_sink()?;
        let paused = sink.is_paused();
        debug!(
            "Pause status check: {}",
            if paused { "PAUSED" } else { "PLAYING" }
        );
        Ok(paused)
    }

    pub fn pause(&self) -> SoundPlayerResult<()> {
        info!("Attempting to pause playback");
        let sink = self.get_sink()?;

        if sink.is_paused() {
            warn!("Audio is already paused");
        } else {
            sink.pause();
            info!("Audio paused");
        }
        Ok(())
    }

    pub fn resume(&self) -> SoundPlayerResult<()> {
        info!("Attempting to resume playback");
        let sink = self.get_sink()?;

        if !sink.is_paused() {
            warn!("Audio is already playing");
        } else {
            sink.play();
            info!("Audio resumed");
        }
        Ok(())
    }

    pub fn stop(&mut self) -> SoundPlayerResult<()> {
        info!("Attempting to stop playback");
        let sink = self.get_sink()?;

        sink.stop();
        let stopped_song = self.current_song.clone();
        self.sink = None;
        self.current_song.clear();

        info!("Stopped playback of: {}", stopped_song);
        Ok(())
    }

    pub fn seek(&self, where_to_seek: u64) -> SoundPlayerResult<()> {
        info!(
            "Attempting to seek to {}s in: {}",
            where_to_seek, self.current_song
        );

        let sink = self.get_sink()?;

        sink.try_seek(Duration::from_secs(where_to_seek))
            .map_err(|e| {
                error!("Seek failed to {}s - {}", where_to_seek, e);
                SoundPlayerError::SeekError {
                    position: where_to_seek,
                    source: e,
                }
            })?;

        info!("Seeked to {}s", where_to_seek);
        Ok(())
    }

    pub fn current_song(&self) -> &str {
        if self.current_song.is_empty() {
            debug!("No song currently loaded");
        } else {
            debug!("Current song: {}", self.current_song);
        }
        &self.current_song
    }

    pub fn volume(&self, volume: f32) -> SoundPlayerResult<()> {
        info!("Attempting to set volume to: {}", volume);

        // Validate volume range
        if !(0.0..=1.0).contains(&volume) {
            error!("Invalid volume level: {} (must be 0.0-1.0)", volume);
            return Err(SoundPlayerError::InvalidVolume { volume });
        }

        let sink = self.get_sink()?;
        sink.set_volume(volume);

        info!("Volume set to: {}", volume);
        Ok(())
    }

    pub fn get_volume(&self) -> SoundPlayerResult<f32> {
        let sink = self.get_sink()?;
        let volume = sink.volume();
        debug!("Current volume: {}", volume);
        Ok(volume)
    }

    pub fn speed(&self, speed: f32) -> SoundPlayerResult<()> {
        info!("Attempting to set speed to: {}x", speed);

        // Validate speed
        if speed <= 0.0 {
            error!("Invalid speed: {} (must be greater than 0.0)", speed);
            return Err(SoundPlayerError::InvalidSpeed { speed });
        }

        let sink = self.get_sink()?;
        sink.set_speed(speed);

        info!("Speed set to: {}x", speed);
        Ok(())
    }

    pub fn is_empty(&self) -> SoundPlayerResult<bool> {
        let sink = self.get_sink()?;
        let empty = sink.empty();
        debug!(
            "Queue status: {}",
            if empty { "EMPTY" } else { "HAS AUDIO" }
        );
        Ok(empty)
    }

    pub fn is_playing(&self) -> SoundPlayerResult<bool> {
        let sink = self.get_sink()?;
        let playing = !sink.empty() && !sink.is_paused();
        debug!(
            "Playback status: {}",
            if playing { "PLAYING" } else { "NOT PLAYING" }
        );
        Ok(playing)
    }
}
