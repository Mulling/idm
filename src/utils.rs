pub fn isatty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}
