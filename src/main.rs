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

fn capture_area(file_number: u32, left_top: (f64, f64), right_bottom: (f64, f64)) {
    let screen = Screen::from_point(0, 0).expect("Couldn't find screen from point");

    let x = left_top.0 as i32;
    let y = left_top.1 as i32;
    let width = (right_bottom.0 - left_top.0) as u32;
    let height = (right_bottom.1 - left_top.1) as u32;

    // 指定されたエリアのスクリーンショットを取得
    let screenshot = screen
        .capture_area(x, y, width, height)
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

    // 左上の座標が記録されるのを待つ
    println!("左上の座標をクリックしてください。");
    let left_top;
    loop {
        thread::sleep(Duration::from_millis(100));
        if let Some(pos) = *click_position.lock().unwrap() {
            left_top = pos;
            break;
        }
    }

    // クリック位置をリセット
    *click_position.lock().unwrap() = None;

    // 右下の座標が記録されるのを待つ
    println!("右下の座標をクリックしてください。");
    let right_bottom;
    loop {
        thread::sleep(Duration::from_millis(100));
        if let Some(pos) = *click_position.lock().unwrap() {
            right_bottom = pos;
            break;
        }
    }

    // クリック位置をリセット
    *click_position.lock().unwrap() = None;

    // 画面の送りボタンの座標が記録されるのを待つ
    println!("画面の戻るボタンをクリックしてください。");
    let prev_button_pos;
    loop {
        thread::sleep(Duration::from_millis(100));
        if let Some(pos) = *click_position.lock().unwrap() {
            prev_button_pos = pos;
            break;
        }
    }

    // クリック位置をリセット
    *click_position.lock().unwrap() = None;

    // 画面の送りボタンの座標が記録されるのを待つ
    println!("画面の送りボタンをクリックしてください。");
    let next_button_pos;
    loop {
        thread::sleep(Duration::from_millis(100));
        if let Some(pos) = *click_position.lock().unwrap() {
            next_button_pos = pos;
            break;
        }
    }

    // ページを最初に戻す
    click_at_position(prev_button_pos.0, prev_button_pos.1);

    // 指定されたページ数だけスクリーンショットを取得
    for file_number in 0..page_count {
        println!("{}ページ目のスクリーンショットを取得中...", file_number + 1);
        capture_area(file_number, left_top, right_bottom);
        click_at_position(next_button_pos.0, next_button_pos.1);
        thread::sleep(Duration::from_millis(300));
    }
}
