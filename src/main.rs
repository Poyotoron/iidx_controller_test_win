use std::thread;
use std::time::Duration;
use std::{process::exit, sync::mpsc};

use windows::Gaming::Input::{GameControllerSwitchPosition, RawGameController};

// 接続中のコントローラーを取得して0番目を返す
fn connect() -> Result<RawGameController, &'static str> {
    let controller = RawGameController::RawGameControllers().unwrap();
    if controller.Size().unwrap() > 0 {
        Ok(controller.GetAt(0).unwrap())
    } else {
        Err("No controller found")
    }
}

// 動き始めはコントローラーを認識しないのでループで待つ
fn connect_wait() -> Result<RawGameController, &'static str> {
    loop {
        match connect() {
            Ok(controller) => return Ok(controller),
            Err(_) => std::thread::sleep(std::time::Duration::from_secs(1)),
        }
    }
}

fn main() {
    // 箱コン以外はRawGameControllerで読むっぽい
    // コントローラー接続
    let controller: RawGameController = connect_wait().unwrap();

    // コントローラー情報表示
    println!(
        "RawGameController At0 Name: {:?}",
        RawGameController::DisplayName(&controller)
    );
    println!(
        "RawGameController PID: {:?}",
        RawGameController::HardwareProductId(&controller)
    );
    println!(
        "RawGameController VID: {:?}",
        RawGameController::HardwareVendorId(&controller)
    );
    println!(
        "RawGameController Button: {:?}",
        RawGameController::ButtonCount(&controller)
    );
    println!(
        "RawGameController Axis: {:?}",
        RawGameController::AxisCount(&controller)
    );
    println!(
        "RawGameController Switch: {:?}",
        RawGameController::SwitchCount(&controller)
    );

    // スレッド間通信用チャンネル
    let (tx_counter, rx_counter) = mpsc::channel();
    let tx_counter_scratch = tx_counter.clone();
    let (tx_scratch, rx_scratch) = mpsc::channel();

    // キーカウント表示スレッド
    thread::spawn(move || {
        // カウントの初期化([1, 2, 3, 4, 5, 6, 7, 皿1, 皿2])
        let mut key_counter = vec![0, 0, 0, 0, 0, 0, 0, 0];

        loop {
            println!("{:?}", key_counter);

            for received in rx_counter.try_iter() {
                key_counter[received as usize] += 1;
            }

            thread::sleep(Duration::from_micros(500));
        }
    });

    // スクラッチ管理スレッド
    thread::spawn(move || {
        let mut prev_scratch_value = 0.0;
        let mut prev_scratch_active = false;
        let mut prev_scratch_dir = false;

        let mut counter = 0;

        // TODO: 初期状態のズレを無視する
        loop {
            let mut scratch_value: f32 = prev_scratch_value;
            let mut scratch_active = prev_scratch_active;
            let mut scratch_dir = prev_scratch_dir;

            for received in rx_scratch.try_iter() {
                scratch_value = received;
            }

            if prev_scratch_value != scratch_value {
                scratch_active = true;
                counter = 0;

                let scratch_diff = scratch_value - prev_scratch_value;
                if scratch_diff.abs() > 0.8 {
                    if scratch_diff < 0.0 {
                        scratch_dir = true;
                    } else {
                        scratch_dir = false;
                    }
                } else {
                    if scratch_diff > 0.0 {
                        scratch_dir = true;
                    } else {
                        scratch_dir = false;
                    }
                }
            } else {
                counter += 1;

                if counter > 50 {
                    scratch_active = false;
                }
            }

            if (scratch_active && !prev_scratch_active) || (scratch_dir != prev_scratch_dir) {
                tx_counter_scratch.send(7).unwrap();
            }

            prev_scratch_value = scratch_value;
            prev_scratch_active = scratch_active;
            prev_scratch_dir = scratch_dir;

            thread::sleep(Duration::from_micros(500));
        }
    });

    // 0.5ミリ秒ごとにコントローラーのボタンの状態を取って表示
    let prev_button_state = &mut [false; 16];
    let button_state = &mut [false; 16];
    let switch_state = &mut [GameControllerSwitchPosition::Center];
    let axis_state = &mut [0.0 as f64; 2];

    loop {
        std::thread::sleep(std::time::Duration::from_micros(100));
        let _buttons = RawGameController::GetCurrentReading(
            &controller,
            button_state,
            switch_state,
            axis_state,
        )
        .unwrap();
        // println!("RawGameController Buttons: {:?}", button_state);
        // println!("RawGameController Axis: {:?}", axis_state);

        for i in 0..button_state.len() {
            if button_state[i] && !prev_button_state[i] {
                tx_counter.send(i as u8).unwrap();
            }
        }

        if button_state[10] {
            exit(0);
        }

        tx_scratch.send(axis_state[0] as f32 * 2.0 - 1.0).unwrap();

        prev_button_state.copy_from_slice(button_state);
    }
}
