use crate::{
    command::*,
    order::{self, Order},
    sound_player::*,
};
use log::{error, info, warn};

pub struct SoundPlayerManager {
    sound_player: SoundPlayer,
}

#[derive(Debug)]
pub enum SoundPlayerManagerError {
    InitFail,
}

impl SoundPlayerManager {
    pub fn new() -> Result<Self, SoundPlayerManagerError> {
        let sound_player = match SoundPlayer::new() {
            Ok(sp) => sp,
            Err(e) => {
                error!("Failed to initialize SoundPlayer: {}", e);
                return Err(SoundPlayerManagerError::InitFail);
            }
        };
        Ok(Self { sound_player })
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

    pub fn process_order(&mut self, order: Order) -> String {
        let cmd = match Command::try_from(&order) {
            Ok(c) => c,
            Err(e) => match e {
                CommandParseError::InvalidParameters => {
                    error!(
                        "Invalid parameters in command: '{}'",
                        order.parameters.join(" ")
                    );
                    return format!(
                        "Invalid parameters in command: '{}'",
                        order.parameters.join(" ")
                    );
                }
                CommandParseError::UnknownCommand => {
                    error!("Unknown command: '{}'", order.command_name);
                    return format!("Unknown command: '{}'", order.command_name);
                }
            },
        };

        if let Err(e) = self.execute_command(cmd) {
            match e {
                SoundPlayerError::PlayError { file, source } => {
                    error!("Failed to play '{}': {}", file, source);
                    return format!("Failed to play '{}': {}", file, source);
                }
                SoundPlayerError::SeekError { position, source } => {
                    error!("Failed to seek to {}: {}", position, source);
                    return format!("Failed to seek to {}: {}", position, source);
                }
                SoundPlayerError::InvalidVolume { volume } => {
                    warn!("Invalid volume: {}", volume);
                    return format!("Invalid volume: {}", volume);
                }
                SoundPlayerError::InvalidSpeed { speed } => {
                    warn!("Invalid speed: {}", speed);
                    return format!("Invalid speed: {}", speed);
                }
                SoundPlayerError::NoSongLoaded => {
                    warn!("No song is currently loaded.");
                    return format!("No song is currently loaded.");
                }
                SoundPlayerError::InvalidStreamHandle => {
                    error!("Stream handle is no longer valid.");
                    return format!("Stream handle is no longer valid.");
                }
                SoundPlayerError::StreamError(source) => {
                    return format!("Audio stream error: {}", source);
                }
                SoundPlayerError::FileOpenError { file, source } => {
                    error!("Failed to open file '{}': {}", file, source);
                    return format!("Failed to open file '{}': {}", file, source);
                }
                SoundPlayerError::DecodingError { file, source } => {
                    error!("Failed to decode file '{}': {}", file, source);
                    return format!("Failed to decode file '{}': {}", file, source);
                }
            }
        } else {
            info!(
                "Command '{}' with params '{}' executed successfully",
                order.command_name,
                order.parameters.join(" ")
            );
            return format!(
                "Command '{}' with params '{}' executed successfully",
                order.command_name,
                order.parameters.join(" ")
            );
        }
    }
}
