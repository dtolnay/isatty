
pub fn stdout_isatty() -> bool {
    isatty(Stream::Stdout)
}

pub fn stderr_isatty() -> bool {
    isatty(Stream::Stderr)
}

enum Stream {
    Stdout,
    Stderr,
}


#[cfg(unix)]
fn isatty(stream: Stream) -> bool {
    extern crate libc;


    let fd = match stream {
        Stream::Stdout => libc::STDOUT_FILENO,
        Stream::Stderr => libc::STDERR_FILENO,
    };

    unsafe { libc::isatty(fd) != 0 }
}

#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
fn isatty(stream: Stream) -> bool {
    let handle = match stream {
        Stream::Stdout => winapi::winbase::STD_OUTPUT_HANDLE,
        Stream::Stderr => winapi::winbase::STD_ERROR_HANDLE,
    };
    unsafe {
        let handle = kernel32::GetStdHandle(handle);

        // check for msys/cygwin
        if is_cygwin_pty(handle) {
            return true;
        }


        let mut out = 0;
        kernel32::GetConsoleMode(handle, &mut out) != 0
    }
}

/// Returns true if there is an MSYS/cygwin tty on the given handle.
#[cfg(windows)]
fn is_cygwin_pty(handle: winapi::HANDLE) -> bool {
    // from https://github.com/BurntSushi/ripgrep/issues/94#issuecomment-261761687

    use std::ffi::OsString;
    use std::mem;
    use std::os::raw::c_void;
    use std::os::windows::ffi::OsStringExt;
    use std::slice;

    use kernel32::GetFileInformationByHandleEx;
    use winapi::fileapi::FILE_NAME_INFO;
    use winapi::minwinbase::FileNameInfo;
    use winapi::minwindef::MAX_PATH;

    unsafe {
        let size = mem::size_of::<FILE_NAME_INFO>();
        let mut name_info_bytes = vec![0u8; size + MAX_PATH];
        let res = GetFileInformationByHandleEx(handle,
                                               FileNameInfo,
                                               &mut *name_info_bytes as *mut _ as *mut c_void,
                                               name_info_bytes.len() as u32);
        if res == 0 {
            return true;
        }
        let name_info: FILE_NAME_INFO = *(name_info_bytes[0..size]
            .as_ptr() as *const FILE_NAME_INFO);
        let name_bytes = &name_info_bytes[size..size + name_info.FileNameLength as usize];
        let name_u16 = slice::from_raw_parts(name_bytes.as_ptr() as *const u16,
                                             name_bytes.len() / 2);
        let name = OsString::from_wide(name_u16)
            .as_os_str()
            .to_string_lossy()
            .into_owned();
        name.contains("msys-") || name.contains("-pty")
    }
}
