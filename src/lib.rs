// Based on https://github.com/rust-lang/cargo/blob/099ad28104fe319f493dc42e0c694d468c65767d/src/cargo/lib.rs#L154-L178

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
fn isatty(stream: Stream) -> bool {
    extern crate kernel32;
    extern crate winapi;

    let handle = match stream {
        Stream::Stdout => winapi::winbase::STD_OUTPUT_HANDLE,
        Stream::Stderr => winapi::winbase::STD_ERROR_HANDLE,
    };

    unsafe {
        let handle = kernel32::GetStdHandle(handle);
        let mut out = 0;
        kernel32::GetConsoleMode(handle, &mut out) != 0
    }
}
