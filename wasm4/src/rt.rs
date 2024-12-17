pub trait Runtime {
    /// Called at the start of the game, before the first update.
    fn start(resources: Resources) -> Self;
    /// Called every frame, about 60 times per second.
    fn update(&mut self);
}

pub struct Resources {
    pub sound: crate::sound::Audio,
    pub framebuffer: crate::draw::Framebuffer,
    pub controls: crate::control::Controls,
}

#[doc(hidden)]
impl Resources {
    /// Can be called once.
    /// Called inside of the [`crate::main`] macro.
    pub unsafe fn new() -> Self {
        Resources {
            sound: crate::sound::Audio(()),
            framebuffer: crate::draw::Framebuffer::new_(),
            controls: crate::control::Controls {
                mouse: crate::control::Mouse::new_(),
                gamepads: [
                    crate::control::Gamepad::new_(wasm4_sys::GAMEPAD1),
                    crate::control::Gamepad::new_(wasm4_sys::GAMEPAD2),
                    crate::control::Gamepad::new_(wasm4_sys::GAMEPAD3),
                    crate::control::Gamepad::new_(wasm4_sys::GAMEPAD4),
                ],
            }
        }
    }
}

#[macro_export]
macro_rules! main {
    ($runtime:ty) => {
        static mut RUNTIME: core::mem::MaybeUninit<$runtime> = core::mem::MaybeUninit::uninit();

        #[no_mangle]
        unsafe extern "C" fn start() {
            // SAFETY: This call is described inside the doc comments for `Resources::new()`
            let res = unsafe { $crate::rt::Resources::new() };
            let rt = <$runtime as $crate::rt::Runtime>::start(res);
            // SAFETY: WASM-4 is single-threaded
            unsafe { RUNTIME = core::mem::MaybeUninit::new(rt) };
        }

        #[no_mangle]
        unsafe extern "C" fn update() {
            // SAFETY: WASM-4 is single-threaded. `update()` function is called after start by WASM-4 runtime
            let rt = unsafe { RUNTIME.assume_init_mut() };
            <$runtime as $crate::rt::Runtime>::update(rt);
        }
    };
}
