use core::marker::PhantomData;

use wasm4_sys::*;

pub struct Controls {
    pub mouse: Mouse,
    pub gamepad: Gamepad,
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
            buttons: [
                buttons & MOUSE_LEFT != 0,
                buttons & MOUSE_RIGHT != 0,
                buttons & MOUSE_MIDDLE != 0,
            ]
        }
    }
}
pub struct MouseState {
    pub x: i16,
    pub y: i16,
    pub buttons: [bool; 3],
}

pub struct Gamepad(PhantomData<*const ()>);
impl Gamepad {
    pub(crate) unsafe fn new_() -> Self {
        Self(PhantomData)
    }
    pub fn state(&self, id: usize) -> GamepadState {
        let gamepad = unsafe {
            [*GAMEPAD1, *GAMEPAD2, *GAMEPAD3, *GAMEPAD4]
        }[id];

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
