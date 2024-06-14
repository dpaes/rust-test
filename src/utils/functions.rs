pub fn limpar_console() {
    return print!("{esc}c", esc = 27 as char);
}

// Converte uma string em uma response
pub fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// Converte uma string em um user
pub fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}