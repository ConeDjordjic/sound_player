mod command;
mod order;
mod sound_player;
mod sound_player_manager;
use env_logger::Env;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tungstenite::{Message, connect};

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    env_logger::Builder::from_env(
        Env::default().default_filter_or("sound_player_manager=debug,sound_player=debug"),
    )
    .init();

    let mut manager = sound_player_manager::SoundPlayerManager::new().unwrap();

    std::thread::spawn(move || {
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
        r.store(false, Ordering::SeqCst);
    });

    let (mut ws, _resp) = connect("ws://127.0.0.1:9001").unwrap();
    log::info!("Connected to server");

    while running.load(Ordering::SeqCst) {
        match ws.read() {
            Ok(msg) => match msg {
                Message::Text(txt) => {
                    let order: crate::order::Order = serde_json::from_str(&txt).unwrap();
                    log::info!("Received order: {:?}", order);

                    let response = manager.process_order(order);

                    let json = serde_json::to_string(&response).unwrap();

                    if let Err(e) = ws.send(tungstenite::protocol::Message::Text(
                        tungstenite::Utf8Bytes::from(json),
                    )) {
                        log::error!("Failed to send response: {}", e);
                        break;
                    }
                }
                Message::Close(_) => {
                    log::info!("Server closed connection");
                    break;
                }
                _ => {}
            },
            Err(e) => {
                log::error!("WebSocket error: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    ws.close(None).unwrap();
    println!("Shutdown complete");
}
