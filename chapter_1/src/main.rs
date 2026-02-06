fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0/5.0) + 32.0
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev: u32 = 0;
            let mut curr: u32 = 1;
            for _ in 2..=n {
                let next = curr + prev;
                prev = curr;
                curr = next;
            }
            curr
        }
    }
}

fn print_twelve_days_of_christmas() {
    let days = [
        "first","second","third","fourth","fifth","sixth",
        "seventh","eighth","ninth","tenth","eleventh","twelfth"
    ];
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas my true love sent to me:", days[i]);
        for j in (0..=i).rev() {
            if i != 0 && j == 0 {
                println!("and {}", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }
        println!();
    }
}

fn main() {
    println!("32°F = {}°C", fahrenheit_to_celsius(32.0));
    println!("30°C = {}°F", celsius_to_fahrenheit(32.0));
    println!("10th Fibonacci = {}", fibonacci(10));

    print_twelve_days_of_christmas();
}
