use rdev::{listen, simulate, Button, Event, EventType};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn click_at_position(x: f64, y: f64) {
    if let Err(error) = simulate(&EventType::MouseMove { x, y }) {
        println!("Error moving mouse: {:?}", error);
    }
    if let Err(error) = simulate(&EventType::ButtonPress(Button::Left)) {
        println!("Error pressing button: {:?}", error);
    }
    if let Err(error) = simulate(&EventType::ButtonRelease(Button::Left)) {
        println!("Error releasing button: {:?}", error);
    }
}

fn main() {
    let current_position = Arc::new(Mutex::new((0.0, 0.0)));
    let click_position = Arc::new(Mutex::new(None));

    let current_position_clone = Arc::clone(&current_position);
    let click_position_clone = Arc::clone(&click_position);

    println!("画面の送りボタンをクリックしてください。");

    thread::spawn(move || {
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

    // ユーザーがクリックして位置が記録されるのを待つ
    let mut click_pos = (0.0, 0.0);
    while click_position.lock().unwrap().is_none() {
        thread::sleep(Duration::from_millis(100));
    }
    if let Some((x, y)) = *click_position.lock().unwrap() {
        println!("クリックされた位置: ({}, {})", x, y);
        click_pos = (x, y);
    }

    // 連続クリックを実行
    loop {
        click_at_position(click_pos.0, click_pos.1);
        thread::sleep(Duration::from_millis(1000)); // 適宜クリック間隔を調整
    }
}
