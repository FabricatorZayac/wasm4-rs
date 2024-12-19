#[macro_export]
macro_rules! tracef {
    ($($arg:tt)*) => {{
        $crate::trace($crate::format::format_no_std::show(
            $crate::format::BUF_SLICE.expect("Logger not initialized"),
            ::core::format_args!($($arg)*)
        ).unwrap());
    }};
}

pub use format_no_std;

static mut BUF_SLICE: Option<&mut [u8]> = None;

pub struct Log;
impl Log {
    pub(crate) fn new_() -> Self { Log }
    pub fn init(self, buf: &'static mut [u8]) {
        unsafe { BUF_SLICE = Some(buf) };
    }
}

