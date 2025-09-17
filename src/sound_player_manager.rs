use crate::{command::*, sound_player::*};
use log::{error, info, warn};

pub struct WebSocket {}

pub struct SoundPlayerManager {
    sound_player: SoundPlayer,
    web_socket: WebSocket,
}

#[derive(Debug)]
pub enum SoundPlayerManagerError {
    InitFail,
}

// TODO: Add websockets and don't close the program until given the order to exit
impl SoundPlayerManager {
    pub fn new() -> Result<Self, SoundPlayerManagerError> {
        let sound_player = match SoundPlayer::new() {
            Ok(sp) => sp,
            Err(e) => {
                eprintln!("Failed to initialize SoundPlayer: {}", e);
                return Err(SoundPlayerManagerError::InitFail);
            }
        };
        let web_socket = WebSocket {};
        Ok(Self {
            sound_player,
            web_socket,
        })
    }

    pub fn execute_command(&mut self, command: Command) -> SoundPlayerResult<()> {
        match command {
            Command::Play { song_name } => self.sound_player.play(&song_name)?,
            Command::Stop => self.sound_player.stop()?,
            Command::Pause => self.sound_player.pause()?,
            Command::Resume => self.sound_player.resume()?,
            Command::Seek { position } => self.sound_player.seek(position)?,
            Command::Volume { level } => self.sound_player.volume(level)?,
            Command::Speed { factor } => self.sound_player.speed(factor)?,
        }
        Ok(())
    }

    pub fn process_command(&mut self, command_str: &str) {
        let cmd = match Command::try_from(command_str) {
            Ok(c) => c,
            Err(e) => {
                match e {
                    CommandParseError::InvalidParameters => {
                        error!("Invalid parameters in command: '{}'", command_str);
                    }
                    CommandParseError::UnknownCommand => {
                        error!("Unknown command: '{}'", command_str);
                    }
                }
                return;
            }
        };

        if let Err(e) = self.execute_command(cmd) {
            match e {
                SoundPlayerError::PlayError { file, source } => {
                    error!("Failed to play '{}': {}", file, source);
                }
                SoundPlayerError::SeekError { position, source } => {
                    error!("Failed to seek to {}: {}", position, source);
                }
                SoundPlayerError::InvalidVolume { volume } => {
                    warn!("Invalid volume: {}", volume);
                }
                SoundPlayerError::InvalidSpeed { speed } => {
                    warn!("Invalid speed: {}", speed);
                }
                SoundPlayerError::NoSongLoaded => {
                    warn!("No song is currently loaded.");
                }
                SoundPlayerError::InvalidStreamHandle => {
                    error!("Stream handle is no longer valid.");
                }
                SoundPlayerError::StreamError(source) => {
                    error!("Audio stream error: {}", source);
                }
                SoundPlayerError::FileOpenError { file, source } => {
                    error!("Failed to open file '{}': {}", file, source);
                }
                SoundPlayerError::DecodingError { file, source } => {
                    error!("Failed to decode file '{}': {}", file, source);
                }
            }
        } else {
            info!("Command '{}' executed successfully", command_str);
        }
    }
}
