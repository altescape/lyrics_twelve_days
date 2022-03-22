fn main() {
    for day in 1..13 {
        day_intro(day);

        for gift_day in (1..(day + 1 )).rev() {
            gift(
                gift_day,
                if gift_day == 1 && day != 1 {
                    "and "
                } else {
                    ""
                },
            );
        }
    }
}

fn day_intro(n: u32) {
    let day = match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    println!(
        "\nOn the {} day of Christmas\nmy true love sent to me:",
        day
    );
}

fn gift(n: u32, prefix: &str) {
    let gift_text = match n {
        1 => "a partridge in a pear tree",
        2 => "two turtle-doves",
        3 => "three French hens",
        4 => "four calling birds",
        5 => "five golden rings",
        6 => "six geese a laying",
        7 => "seven swans a swimming",
        8 => "eight maids a milking",
        9 => "nine ladies dancing",
        10 => "ten lords a-leaping",
        11 => "eleven pipers piping",
        12 => "twelve drummers drumming",
        _ => "",
    };

    println!("{}{}", prefix, gift_text);
}
