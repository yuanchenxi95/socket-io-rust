pub fn convert_char_to_number(c: char) -> Option<u8> {
    let n = c.to_digit(10)?;
    if n > 9 {
        return None;
    }
    Some(n as u8)
}
