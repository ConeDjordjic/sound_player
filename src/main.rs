mod command;
mod sound_player;
mod sound_player_manager;
use env_logger::Env;
use sound_player_manager::*;

fn main() {
    env_logger::Builder::from_env(
        Env::default().default_filter_or("sound_player_manager=debug,sound_player=debug"),
    )
    .init();

    log::info!("Starting audio player test");

    let mut manager = SoundPlayerManager::new().unwrap();

    manager.process_command("play songs/cone.mp3");

    std::thread::sleep(std::time::Duration::from_secs(2));
}
