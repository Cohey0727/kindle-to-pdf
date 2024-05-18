use rdev::{listen, Button, Event, EventType};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let current_position = Arc::new(Mutex::new((0.0, 0.0)));
    let click_position = Arc::new(Mutex::new(None));

    let current_position_clone = Arc::clone(&current_position);
    let click_position_clone = Arc::clone(&click_position);

    println!("画面の送りボタンをクリックしてください。");

    let handle = thread::spawn(move || {
        let callback = move |event: Event| match event.event_type {
            EventType::MouseMove { x, y } => {
                let mut position = current_position_clone.lock().unwrap();
                *position = (x, y);
            }
            EventType::ButtonPress(Button::Left) => {
                let position = current_position_clone.lock().unwrap();
                let mut click_pos = click_position_clone.lock().unwrap();
                *click_pos = Some(*position);
            }
            _ => {}
        };

        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    });

    loop {
        thread::sleep(Duration::from_millis(100));
        let click_pos = click_position.lock().unwrap();
        if let Some((x, y)) = *click_pos {
            println!("クリックされた位置: ({}, {})", x, y);
            break;
        }
    }

    handle.join().unwrap();
}
