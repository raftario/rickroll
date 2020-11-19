#![no_std]
#![no_main]
#![cfg_attr(windows, windows_subsystem = "windows")]

const URL: &[u8] = b"https://www.youtube.com/watch?v=dQw4w9WgXcQ\0";

#[cfg(target_os = "linux")]
const OPEN: &[u8] = b"xdg-open\0";
#[cfg(target_os = "macos")]
const OPEN: &[u8] = b"open\0";
#[cfg(target_os = "windows")]
const OPEN: &[u8] = b"open\0";

#[cfg(unix)]
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    syscall::platform::syscall3(
        syscall::platform::nr::EXECVE,
        b"/bin/sh\0".as_ptr() as _,
        [
            b"-c\0".as_ptr() as *const i8,
            OPEN.as_ptr() as *const i8,
            URL.as_ptr() as *const i8,
        ]
        .as_ptr() as _,
        core::ptr::null() as _,
    );
    syscall::platform::syscall1(syscall::platform::nr::EXIT, 0);
    core::hint::unreachable_unchecked()
}

#[cfg(windows)]
#[no_mangle]
unsafe extern "C" fn mainCRTStartup() -> ! {
    winapi::um::shellapi::ShellExecuteA(
        core::ptr::null_mut(),
        OPEN.as_ptr() as _,
        URL.as_ptr() as _,
        core::ptr::null(),
        core::ptr::null(),
        winapi::um::winuser::SW_SHOW,
    );
    winapi::um::processthreadsapi::ExitProcess(0);
    core::hint::unreachable_unchecked()
}

#[cfg(unix)]
#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    libc::exit(1)
}

#[cfg(windows)]
#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    winapi::um::processthreadsapi::ExitProcess(1);
    core::hint::unreachable_unchecked()
}
