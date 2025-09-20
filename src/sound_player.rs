use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use thiserror::Error;

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

pub type SoundPlayerResult<T> = Result<T, SoundPlayerError>;

pub struct SoundPlayer {
    current_song: String,
    stream_handle: OutputStream,
    sink: Option<Sink>,
}

impl SoundPlayer {
    pub fn new() -> SoundPlayerResult<Self> {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .map_err(SoundPlayerError::StreamError)?;

        Ok(Self {
            current_song: String::new(),
            stream_handle,
            sink: None,
        })
    }

    fn get_sink(&self) -> SoundPlayerResult<&Sink> {
        self.sink.as_ref().ok_or(SoundPlayerError::NoSongLoaded)
    }

    pub fn play(&mut self, sound_file: &str) -> SoundPlayerResult<()> {
        if let Some(ref sink) = self.sink {
            sink.stop();
            self.sink = None;
        }

        let file = File::open(sound_file).map_err(|e| SoundPlayerError::FileOpenError {
            file: sound_file.to_string(),
            source: e,
        })?;

        let buf_reader = BufReader::new(file);

        let sink = rodio::play(&self.stream_handle.mixer(), buf_reader).map_err(|e| {
            SoundPlayerError::PlayError {
                file: sound_file.to_string(),
                source: e,
            }
        })?;

        self.sink = Some(sink);
        self.current_song = sound_file.to_string();

        Ok(())
    }

    pub fn pause(&self) -> SoundPlayerResult<()> {
        let sink = self.get_sink()?;
        if !sink.is_paused() {
            sink.pause();
        }
        Ok(())
    }

    pub fn resume(&self) -> SoundPlayerResult<()> {
        let sink = self.get_sink()?;
        if sink.is_paused() {
            sink.play();
        }
        Ok(())
    }

    pub fn stop(&mut self) -> SoundPlayerResult<()> {
        let sink = self.get_sink()?;
        sink.stop();
        self.sink = None;
        self.current_song.clear();
        Ok(())
    }

    pub fn seek(&self, position: u64) -> SoundPlayerResult<()> {
        let sink = self.get_sink()?;
        sink.try_seek(Duration::from_secs(position))
            .map_err(|e| SoundPlayerError::SeekError {
                position,
                source: e,
            })?;
        Ok(())
    }

    pub fn volume(&self, volume: f32) -> SoundPlayerResult<()> {
        if !(0.0..=1.0).contains(&volume) {
            return Err(SoundPlayerError::InvalidVolume { volume });
        }
        let sink = self.get_sink()?;
        sink.set_volume(volume);
        Ok(())
    }

    pub fn speed(&self, speed: f32) -> SoundPlayerResult<()> {
        if speed <= 0.0 {
            return Err(SoundPlayerError::InvalidSpeed { speed });
        }
        let sink = self.get_sink()?;
        sink.set_speed(speed);
        Ok(())
    }

    pub fn current_song(&self) -> &str {
        &self.current_song
    }

    pub fn is_paused(&self) -> SoundPlayerResult<bool> {
        let sink = self.get_sink()?;
        Ok(sink.is_paused())
    }

    pub fn is_playing(&self) -> SoundPlayerResult<bool> {
        let sink = self.get_sink()?;
        Ok(!sink.empty() && !sink.is_paused())
    }

    pub fn is_empty(&self) -> SoundPlayerResult<bool> {
        let sink = self.get_sink()?;
        Ok(sink.empty())
    }

    pub fn get_volume(&self) -> SoundPlayerResult<f32> {
        let sink = self.get_sink()?;
        Ok(sink.volume())
    }
}
