use crate::order::Order;

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

impl TryFrom<&Order> for Command {
    type Error = CommandParseError;

    fn try_from(order: &Order) -> Result<Self, Self::Error> {
        match order.command_name.to_lowercase().as_str() {
            "play" => {
                if let Some(song_name) = order.parameters.get(0) {
                    Ok(Command::Play {
                        song_name: song_name.clone(),
                    })
                } else {
                    Err(CommandParseError::InvalidParameters)
                }
            }
            "stop" => Ok(Command::Stop),
            "pause" => Ok(Command::Pause),
            "resume" => Ok(Command::Resume),
            "seek" => {
                if let Some(pos_str) = order.parameters.get(0) {
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
                if let Some(level_str) = order.parameters.get(0) {
                    if let Ok(level) = level_str.parse::<f32>() {
                        Ok(Command::Volume { level })
                    } else {
                        Err(CommandParseError::InvalidParameters)
                    }
                } else {
                    Err(CommandParseError::InvalidParameters)
                }
            }
            "speed" => {
                if let Some(factor_str) = order.parameters.get(0) {
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
