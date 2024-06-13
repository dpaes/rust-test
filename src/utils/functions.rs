pub fn limpar_console() {
    return print!("{esc}c", esc = 27 as char);
}