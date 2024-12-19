use core::marker::PhantomData;

use wasm4_sys::*;

pub struct Controls {
    pub mouse: Mouse,
    pub gamepads: [Gamepad; 4],
}

pub struct Mouse(PhantomData<*const ()>);
impl Mouse {
    pub(crate) unsafe fn new_() -> Self {
        Self(PhantomData)
    }
    pub fn state(&self) -> MouseState {
        let buttons = unsafe { *MOUSE_BUTTONS };
        let x = unsafe { *MOUSE_X };
        let y = unsafe { *MOUSE_Y };

        MouseState {
            x,
            y,
            buttons: MouseButtons {
                left: buttons & MOUSE_LEFT != 0,
                right: buttons & MOUSE_RIGHT != 0,
                middle: buttons & MOUSE_MIDDLE != 0,
            }
        }
    }
}
pub struct MouseState {
    pub x: i16,
    pub y: i16,
    pub buttons: MouseButtons,
}
pub struct MouseButtons {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

pub struct Gamepad(*const u8);
impl Gamepad {
    pub(crate) unsafe fn new_(gamepad: *const u8) -> Self {
        Self(gamepad)
    }
    pub fn state(&self) -> GamepadState {
        let gamepad = unsafe { *self.0 };

        GamepadState {
            buttons: [
                gamepad & BUTTON_1 != 0,
                gamepad & BUTTON_2 != 0,
            ],
            dpad: Directions {
                up: gamepad & BUTTON_UP != 0,
                down: gamepad & BUTTON_DOWN != 0,
                left: gamepad & BUTTON_LEFT != 0,
                right: gamepad & BUTTON_RIGHT != 0,
            }
        }
    }
}
pub struct GamepadState {
    pub buttons: [bool; 2],
    pub dpad: Directions,
}
impl GamepadState {
    pub fn clean(self) -> Self {
        GamepadState {
            buttons: self.buttons,
            dpad: Directions { 
                up: if self.dpad.up ^ self.dpad.down { self.dpad.up } else { false },
                down: if self.dpad.up ^ self.dpad.down { self.dpad.down } else { false },
                left: if self.dpad.left ^ self.dpad.right { self.dpad.left } else { false },
                right: if self.dpad.left ^ self.dpad.right { self.dpad.right } else { false },
            },
        }
    }
}
pub struct Directions {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
