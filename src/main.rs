#![no_std]
#![no_main]
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(target_os = "linux")]
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    libc::system(b"xdg-open https://www.youtube.com/watch?v=dQw4w9WgXcQ\0".as_ptr() as _);
    libc::exit(0)
}

#[cfg(target_os = "windows")]
#[no_mangle]
unsafe extern "C" fn mainCRTStartup() -> ! {
    winapi::um::shellapi::ShellExecuteA(
        core::ptr::null_mut(),
        b"open\0".as_ptr() as _,
        b"https://www.youtube.com/watch?v=dQw4w9WgXcQ\0".as_ptr() as _,
        core::ptr::null(),
        core::ptr::null(),
        winapi::um::winuser::SW_SHOW,
    );
    winapi::um::processthreadsapi::ExitProcess(0);
    core::hint::unreachable_unchecked()
}

#[cfg(target_os = "macos")]
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    libc::system(b"open https://www.youtube.com/watch?v=dQw4w9WgXcQ\0".as_ptr() as _);
    libc::exit(0)
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
