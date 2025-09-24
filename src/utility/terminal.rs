pub fn clear_screen() {
    // ANSI escape code to clear the screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
}
