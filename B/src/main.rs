use colored::Colorize;
use dectohex::{decimal_to_hex, hex_to_decimal};

fn main() {
    let title = r"
        .##.....##.########.##.....##.########..########..######.
        .##.....##.##........##...##..##.....##.##.......##....##
        .##.....##.##.........##.##...##.....##.##.......##......
        .#########.######......###....##.....##.######...##......
        .##.....##.##.........##.##...##.....##.##.......##......
        .##.....##.##........##...##..##.....##.##.......##....##
        .##.....##.########.##.....##.########..########..######.
    ";
    let instructions = format!(
        "{}: {}\n{}: {}\n{}",
        "Hex to decimal".yellow(),
        "AC1h",
        "Decimal to hex".yellow(),
        "3456d",
        "Write q to exit...".bold()
    );
    println!("{}\n{}", title.purple(), instructions);

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input from the terminal!");
        let converted = match input
            .trim()
            .chars()
            .last()
            .expect("The input doesn't have a final char!")
        {
            'q' => break,
            'h' => hex_to_decimal(&input[..(input.len() - 2)]).to_string(),
            'd' => decimal_to_hex(
                (&input[..(input.len() - 2)])
                    .parse()
                    .expect("The input is not a number!"),
            ),
            _ => {
                println!("{}", "Please check the format of the number!".red());
                continue;
            }
        };

        println!("{}", converted.blue());
    }
}
