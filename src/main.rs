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

    // 0.5ミリ秒ごとにコントローラーのボタンの状態を取って表示
    let button_state = &mut [false; 16];
    let switch_state = &mut [GameControllerSwitchPosition::Center];
    let axis_state = &mut [0.0 as f64; 2];

    loop {
        std::thread::sleep(std::time::Duration::from_micros(500));
        let _buttons = RawGameController::GetCurrentReading(
            &controller,
            button_state,
            switch_state,
            axis_state,
        )
        .unwrap();
        println!("RawGameController Buttons: {:?}", button_state);
        println!("RawGameController Axis: {:?}", axis_state);
    }
}
