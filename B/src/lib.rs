pub fn hex_to_decimal(number: &str) -> u32 {
    number.chars().rev().enumerate().map(map_to_decimal).sum()
}

pub fn decimal_to_hex(number: u32) -> String {
    let mut stack = vec![];
    let mut n = number;
    while n != 0 {
        stack.push(n % 16);
        n = (n / 16) as u32;
    }

    stack.into_iter().rev().map(map_to_hex).collect()
}

pub fn map_to_decimal((power, digit): (usize, char)) -> u32 {
    let hex_factor = match digit {
        '0'..='9' => digit
            .to_digit(10)
            .expect("The number between 0-9 couldn't be converted to decimal!"),
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => unreachable!("Reached {}", digit),
    };
    hex_factor * 16u32.pow(power as u32)
}

pub fn map_to_hex(number: u32) -> String {
    match number {
        0..=9 => number.to_string(),
        10 => "A".to_string(),
        11 => "B".to_string(),
        12 => "C".to_string(),
        13 => "D".to_string(),
        14 => "E".to_string(),
        15 => "F".to_string(),
        _ => unreachable!("Reached {}", number),
    }
}

#[cfg(test)]
mod tests {
    use crate::{decimal_to_hex, hex_to_decimal};

    #[test]
    fn decimal_to_hex_works() {
        let a = 45;
        let actual = decimal_to_hex(a);
        assert_eq!(actual, "2D".to_string());

        let a = 267;
        let actual = decimal_to_hex(a);
        assert_eq!(actual, "10B".to_string());
    }

    #[test]
    fn hex_to_decimal_works() {
        let a = "2D";
        let actual = hex_to_decimal(a);
        assert_eq!(actual, 45);

        let a = "10B";
        let actual = hex_to_decimal(a);
        assert_eq!(actual, 267);
    }
}
