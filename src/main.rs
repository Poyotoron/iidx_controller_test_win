use windows::Gaming::Input::{self, Gamepad, RawGameController};

fn main() {
    println!("Hello, world!");

    // 箱コン以外はRawGameControllerで読むっぽい
    while let Ok(gamepad) = RawGameController::RawGameControllers() {
        println!("RawGameController: {:?}", gamepad); // なにこれ
        println!("RawGameController Size: {:?}", gamepad.Size()); // 接続中のコントローラーの台数

        // let reading = Gamepad::GetCurrentReading(&Gamepad);
    }
}
