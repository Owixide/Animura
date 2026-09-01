use windows::{
    Win32::Foundation::{E_FAIL, GetLastError},
    core::{Error, HRESULT},
};

pub fn get_error_win32() -> Error {
    unsafe {
        let errorcode = GetLastError().0;
        let ec = if errorcode == 0 {
            E_FAIL
        } else if errorcode as i32 <= 0 {
            HRESULT(errorcode as i32)
        } else {
            HRESULT(((errorcode & 0x0000_FFFF) | 0x8007_0000) as i32)
        };

        Error::from_hresult(ec)
    }
}
