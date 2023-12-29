use inputbot::{KeySequence, KeybdKey::*, MouseButton::*};
use std::{thread::sleep, time::Duration};

fn main() {
    // Bind the number 1 key your keyboard to a function that types
    // "Hello, world!" when pressed.
    Numrow1Key.bind(|| KeySequence("Hello, world!").send());

    // Bind your caps lock key to a function that starts an autoclicker.
    CapsLockKey.bind(move || {
        while CapsLockKey.is_toggled() {
            LeftButton.press();
            LeftButton.release();

            sleep(Duration::from_millis(30));
        }
    });

    F1Key.bind(move || {
        inputbot::stop_handling_input_events();
        panic!("exit");
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events(true);

    
}
