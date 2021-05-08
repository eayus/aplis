use core::sync::atomic::{AtomicBool, Ordering};

pub static KEYBOARD: Keyboard = Keyboard::new();

pub struct Key {
    pressed: AtomicBool,
}

pub struct Keyboard {
    pub letter_a: Key,
    pub letter_b: Key,
    pub letter_c: Key,
}


impl Key {
    const fn unpressed() -> Self {
        Key {
            pressed: AtomicBool::new(false),
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed.load(Ordering::Relaxed)
    }

    pub fn set_pressed(&self, pressed: bool) {
        self.pressed.store(pressed, Ordering::Relaxed);
    }
}


impl Keyboard {
    const fn new() -> Self {
        Keyboard {
            letter_a: Key::unpressed(),
            letter_b: Key::unpressed(),
            letter_c: Key::unpressed(),
        }
    }
}
