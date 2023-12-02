use super::commons;

/**
--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look.
The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations,
you need to check all fifty stars by December 25th.
Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar;
the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough")
and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions")
and hang on did you just say the sky ("of course, where do you think snow comes from")
when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
 */

/**
--- Part One ---

As they're making the final adjustments, they discover that their calibration document (your puzzle input)
has been amended by a very young Elf who was apparently just excited to show off her art skills.
Consequently, the Elves are having trouble reading the values on the document.
The newly-improved calibration document consists of lines of text;
each line originally contained a specific calibration value that the Elves now need to recover.
On each line, the calibration value can be found by combining the first digit and the last digit (in that order)
to form a single two-digit number.

For example:
```
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
Consider your entire calibration document. What is the sum of all of the calibration values?
 */

const INPUT_FILE_NAME: &str = "day_1.txt";

fn first_part() {
    let file_content = commons::get_input(INPUT_FILE_NAME);
    let mut calibration_sum: i32 = 0;

    for line in file_content {
        let digits_in_line: Vec<char> = line.chars()
            .filter(|x| x.is_digit(10))
            .collect::<Vec<char>>();

        let first_digit: &char = digits_in_line.first().expect("At least one digit must be present");
        let last_digit: &char = digits_in_line.last().expect("At least one digit must be present");

        let calibration_digit = format!("{first_digit}{last_digit}")
            .parse::<i32>()
            .expect("At this point the string should be a valid number");

        calibration_sum += calibration_digit;
    }
    println!("{calibration_sum}") // 55488
}

/**
--- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters:
one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
Equipped with this new information, you now need to find the real first and last digit on each line.

For example:
```
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
What is the sum of all of the calibration values?
 */
fn second_part() {
    let file_content = commons::get_input(INPUT_FILE_NAME);

    let numbers: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut calibration_sum: i32 = 0;

    for line in file_content {
        let mut digits_in_line: Vec<char> = vec![];

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) { digits_in_line.push(c); }

            for (j, num) in numbers.iter().enumerate() {
                if line[i..].starts_with(num) {
                    digits_in_line.push((j + 1).to_string().parse().expect("j + 1 should be a valid number"))
                }
            }
        }

        let first_digit: &char = digits_in_line.first().expect("At least one digit must be present");
        let last_digit: &char = digits_in_line.last().expect("At least one digit must be present");

        let calibration_digit = format!("{first_digit}{last_digit}")
            .parse::<i32>()
            .expect("At this point the string should be a valid number");

        calibration_sum += calibration_digit;
    }
    println!("{calibration_sum}") // 55614
}

pub fn run() {
    first_part();
    second_part();
}