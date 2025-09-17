pub enum Command {
    Play { song_name: String },
    Stop,
    Pause,
    Resume,
    Seek { position: u64 },
    Volume { level: f32 },
    Speed { factor: f32 },
}

pub enum CommandParseError {
    InvalidParameters,
    UnknownCommand,
}

impl TryFrom<&str> for Command {
    type Error = CommandParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();

        let command_name = match parts.get(0) {
            Some(name) => name.to_lowercase(),
            None => return Err(CommandParseError::UnknownCommand),
        };

        match command_name.as_str() {
            "play" => {
                if parts.len() >= 2 {
                    Ok(Command::Play {
                        song_name: parts[1].to_owned(),
                    })
                } else {
                    Err(CommandParseError::InvalidParameters)
                }
            }
            "stop" => Ok(Command::Stop),
            "pause" => Ok(Command::Pause),
            "resume" => Ok(Command::Resume),
            "seek" => {
                if let Some(pos_str) = parts.get(1) {
                    if let Ok(position) = pos_str.parse::<u64>() {
                        Ok(Command::Seek { position })
                    } else {
                        Err(CommandParseError::InvalidParameters)
                    }
                } else {
                    Err(CommandParseError::InvalidParameters)
                }
            }
            "volume" => {
                if let Some(vol_str) = parts.get(1) {
                    if let Ok(level) = vol_str.parse::<f32>() {
                        Ok(Command::Volume { level })
                    } else {
                        Err(CommandParseError::InvalidParameters)
                    }
                } else {
                    Err(CommandParseError::InvalidParameters)
                }
            }
            "speed" => {
                if let Some(factor_str) = parts.get(1) {
                    if let Ok(factor) = factor_str.parse::<f32>() {
                        Ok(Command::Speed { factor })
                    } else {
                        Err(CommandParseError::InvalidParameters)
                    }
                } else {
                    Err(CommandParseError::InvalidParameters)
                }
            }
            _ => Err(CommandParseError::UnknownCommand),
        }
    }
}
