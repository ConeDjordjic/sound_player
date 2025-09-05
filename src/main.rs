use std::env;
use std::fs::File;
use std::io::BufReader;

fn play_sound(sound_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let file = BufReader::new(File::open(sound_file)?);
    let sink = rodio::play(&stream_handle.mixer(), file)?;

    println!("Playing sound!");

    sink.sleep_until_end();
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Correct usage is: app_name sound_file.extension");
        return;
    }

    let sound_file = args.get(1).unwrap();

    match play_sound(sound_file) {
        Ok(_) => {
            println!("Sound played successfully!");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        Err(error) => {
            println!("Error playing sound: {}", error);
        }
    }
}
