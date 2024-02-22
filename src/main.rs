use windows::Gaming::Input::RawGameController;

fn main() {
    println!("Hello, world!");

    // 箱コン以外はRawGameControllerで読むっぽい
    while let Ok(gamepad) = RawGameController::RawGameControllers() {
        println!("RawGameController: {:?}", gamepad); // なにこれ
        println!("RawGameController Size: {:?}", gamepad.Size()); // 接続中のコントローラーの台数

        if gamepad.Size().unwrap() > 0 {
            println!("RawGameController At0: {:?}", gamepad.GetAt(0));

            println!(
                "RawGameController At0 Name: {:?}",
                RawGameController::DisplayName(&gamepad.GetAt(0).unwrap())
            );
            println!(
                "RawGameController PID: {:?}",
                RawGameController::HardwareProductId(&gamepad.GetAt(0).unwrap())
            );
            println!(
                "RawGameController VID: {:?}",
                RawGameController::HardwareVendorId(&gamepad.GetAt(0).unwrap())
            );
        }
    }
}
