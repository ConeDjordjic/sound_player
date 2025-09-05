mod sound_player_manager;
use sound_player_manager::{SoundPlayerError, SoundPlayerManager, SoundPlayerResult};

fn main() -> SoundPlayerResult<()> {
    env_logger::init();

    log::info!("Starting audio player test");
    // Set up the player
    let mut player = SoundPlayerManager::new()?;

    // Try to play a song
    println!("Loading songs/cone.mp3...");
    match player.play("songs/cone.mp3") {
        Ok(()) => println!("Playing successfully"),
        Err(SoundPlayerError::FileOpenError { file, source }) => {
            eprintln!("Can't open {}: {}", file, source);
            return Ok(());
        }
        Err(SoundPlayerError::DecodingError { file, source }) => {
            eprintln!("Can't decode {}: {}", file, source);
            return Ok(());
        }
        Err(SoundPlayerError::PlayError { file, source }) => {
            eprintln!("Playback failed for {}: {}", file, source);
            return Ok(());
        }
        Err(SoundPlayerError::StreamError(e)) => {
            eprintln!("Audio system error: {}", e);
            return Ok(());
        }
        Err(e) => {
            eprintln!("Something went wrong: {}", e);
            return Ok(());
        }
    }

    // Let it play for a bit
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Test pause
    println!("\nPausing...");
    if let Err(e) = player.pause() {
        eprintln!("Pause failed: {}", e);
    }
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Test resume
    println!("Resuming...");
    if let Err(e) = player.resume() {
        eprintln!("Resume failed: {}", e);
    }
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Test seeking
    println!("Jumping to 30 seconds...");
    if let Err(e) = player.seek(30) {
        match e {
            SoundPlayerError::SeekError { position, source } => {
                eprintln!("Can't seek to {}s: {}", position, source);
            }
            _ => eprintln!("Seek error: {}", e),
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Test volume
    println!("Setting volume to 50%...");
    if let Err(e) = player.volume(0.5) {
        match e {
            SoundPlayerError::InvalidVolume { volume } => {
                eprintln!("Bad volume {}, needs to be 0.0-1.0", volume);
            }
            _ => eprintln!("Volume error: {}", e),
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Test speed
    println!("Speeding up to 1.5x...");
    if let Err(e) = player.speed(1.5) {
        match e {
            SoundPlayerError::InvalidSpeed { speed } => {
                eprintln!("Bad speed {}, needs to be > 0.0", speed);
            }
            _ => eprintln!("Speed error: {}", e),
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(4));

    // Check status
    println!("\nStatus check:");
    println!("  Current song: {}", player.current_song());

    if let Ok(playing) = player.is_playing() {
        println!("  Playing: {}", playing);
    }

    if let Ok(volume) = player.get_volume() {
        println!("  Volume: {:.0}%", volume * 100.0);
    }

    if let Ok(paused) = player.is_paused() {
        println!("  Paused: {}", paused);
    }

    // Let it play a bit more
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Stop
    println!("\nStopping playback...");
    if let Err(e) = player.stop() {
        eprintln!("Stop failed: {}", e);
    }

    println!("Done.");
    Ok(())
}
