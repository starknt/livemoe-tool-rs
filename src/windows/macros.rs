#[macro_export]
macro_rules! TEXT {
    ($x:expr) => {
        {
            use std::ffi::OsStr;
            use std::iter::once;
            use std::os::windows::ffi::OsStrExt;

            let wide: Vec<u16> = OsStr::new($x).encode_wide().chain(once(0)).collect();

            wide.as_ptr()
        }
    };
}