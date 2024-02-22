use windows::Gaming::Input::{self, Gamepad};

fn main() {
    println!("Hello, world!");

    for gamepad in Gamepad::Gamepads() {
        println!("Gamepad: {:?}", gamepad);
    }
}
