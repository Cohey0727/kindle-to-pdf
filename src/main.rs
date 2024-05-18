use rdev::{listen, simulate, Button, Event, EventType};
use screenshots::Screen;
use std::fs::{self};
use std::io::{self};
use std::path::Path;
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

fn capture_right_half(file_number: u32) {
    let screen = Screen::from_point(0, 0).expect("Couldn't find screen from point");
    let display = screen.display_info;
    let (w, h) = (display.width, display.height);

    // 右半分のスクリーンショットを取得
    let screenshot = screen
        .capture_area(
            (w / 2).try_into().unwrap(),
            0,
            (w / 2).try_into().unwrap(),
            h,
        )
        .expect("Couldn't capture screenshot");

    // outputsディレクトリを作成（存在しない場合）
    let output_dir = Path::new("outputs");
    if !output_dir.exists() {
        fs::create_dir(output_dir).expect("Couldn't create outputs directory");
    }

    // 指定されたファイル番号でファイルを保存
    let file_path = output_dir.join(format!("{:04}.png", file_number));
    screenshot
        .save(file_path)
        .expect("Couldn't save screenshot");
}
fn main() {
    println!("ページ数を入力してください:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let page_count: u32 = input.trim().parse().expect("Please enter a valid number");

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
    let mut position_recorded = false;
    let mut click_pos = (0.0, 0.0);
    while !position_recorded {
        thread::sleep(Duration::from_millis(100));
        let click_position_guard = click_position.lock().unwrap();
        if let Some((x, y)) = *click_position_guard {
            println!("クリックされた位置: ({}, {})", x, y);
            click_pos = (x, y);
            position_recorded = true;
        }
    }

    for file_number in 0..page_count {
        click_at_position(click_pos.0, click_pos.1);
        capture_right_half(file_number);
        thread::sleep(Duration::from_millis(1000));
    }
}
