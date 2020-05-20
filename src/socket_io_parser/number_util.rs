pub fn convert_char_to_number(c: char) -> Option<u8> {
    let n = c.to_digit(10)?;
    if n > 9 {
        return None;
    }
    Some(n as u8)
}

#[cfg(test)]
mod tests {
    use crate::socket_io_parser::number_util::convert_char_to_number;

    #[test]
    fn convert_char_to_number_test() {
        let num_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for ii in 0..10 {
            assert_eq!(ii as u8, convert_char_to_number(num_chars[ii]).unwrap());
        }
    }

    fn convert_char_to_number_test_none() {
        assert!(convert_char_to_number('a').is_none());
        assert!(convert_char_to_number('b').is_none());
        assert!(convert_char_to_number('å').is_none());
    }
}
