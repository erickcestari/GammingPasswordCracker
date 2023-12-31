use inputbot::{
    KeybdKey::{self, *},
    MouseButton::*,
};
use std::{io::BufRead, collections::HashMap};
use std::{fs::File, io, thread::sleep, time::Duration};

fn main() -> io::Result<()> {
    let file = File::open("./file/password.txt")?;

    let passwords: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

    CapsLockKey.bind(move || {
        while CapsLockKey.is_toggled() {
            LeftButton.press();
            LeftButton.release();

            sleep(Duration::from_millis(500));
        }
    });

    F2Key.bind(move || {
        for pswd in &passwords  {
            write_string(&pswd);
            LeftButton.press();
            LeftButton.release();
            sleep(Duration::from_millis(10));
        }
    });

    F1Key.bind(move || {
        inputbot::stop_handling_input_events();
        panic!("exit");
    });

    inputbot::handle_input_events(true);

    Ok(())
}

fn write_string(s: &str) {
    let map_keys = key_to_char_map();

    for c in s.chars() {
        if let Some(&key) = map_keys.get(&c) {
            key.press();
            key.release();
            sleep(Duration::from_millis(1));
        }
    }
}

fn key_to_char_map() -> std::collections::HashMap<char, KeybdKey> {
    let mut map = std::collections::HashMap::new();

    // Alphabets
    map.insert(KeybdKey::AKey, 'a');
    map.insert(KeybdKey::BKey, 'b');
    map.insert(KeybdKey::CKey, 'c');
    map.insert(KeybdKey::DKey, 'd');
    map.insert(KeybdKey::EKey, 'e');
    map.insert(KeybdKey::FKey, 'f');
    map.insert(KeybdKey::GKey, 'g');
    map.insert(KeybdKey::HKey, 'h');
    map.insert(KeybdKey::IKey, 'i');
    map.insert(KeybdKey::JKey, 'j');
    map.insert(KeybdKey::KKey, 'k');
    map.insert(KeybdKey::LKey, 'l');
    map.insert(KeybdKey::MKey, 'm');
    map.insert(KeybdKey::NKey, 'n');
    map.insert(KeybdKey::OKey, 'o');
    map.insert(KeybdKey::PKey, 'p');
    map.insert(KeybdKey::QKey, 'q');
    map.insert(KeybdKey::RKey, 'r');
    map.insert(KeybdKey::SKey, 's');
    map.insert(KeybdKey::TKey, 't');
    map.insert(KeybdKey::UKey, 'u');
    map.insert(KeybdKey::VKey, 'v');
    map.insert(KeybdKey::WKey, 'w');
    map.insert(KeybdKey::XKey, 'x');
    map.insert(KeybdKey::YKey, 'y');
    map.insert(KeybdKey::ZKey, 'z');

    // Numbers
    map.insert(KeybdKey::Numrow1Key, '1');
    map.insert(KeybdKey::Numrow2Key, '2');
    map.insert(KeybdKey::Numrow3Key, '3');
    map.insert(KeybdKey::Numrow4Key, '4');
    map.insert(KeybdKey::Numrow5Key, '5');
    map.insert(KeybdKey::Numrow6Key, '6');
    map.insert(KeybdKey::Numrow7Key, '7');
    map.insert(KeybdKey::Numrow8Key, '8');
    map.insert(KeybdKey::Numrow9Key, '9');
    map.insert(KeybdKey::Numrow0Key, '0');

    invert_map(&map)
}


fn invert_map(map: &HashMap<KeybdKey, char>) -> HashMap<char, KeybdKey> {
    map.iter().map(|(&k, &v)| (v, k)).collect()
}