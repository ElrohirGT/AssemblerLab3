pub fn decimal_to_hex(number: u32) -> String {
    let stack = vec![];
    let mut n = number;
    while (n != 0) {
        stack.push(n % 16);
        let a = (n / 16) as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::decimal_to_hex;

    #[test]
    fn decimal_to_hex_works() {
        let a = 45;
        let actual = decimal_to_hex(a);
        assert_eq!(actual, "2D".to_string());

        let a = 267;
        let actual = decimal_to_hex(a);
        assert_eq!(actual, "10B".to_string());
    }
}
