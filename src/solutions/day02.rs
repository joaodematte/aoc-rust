#[derive(Default)]
struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

fn exceeds_limit(number: usize, color: &str) -> bool {
    match color {
        "red" => number > 12,
        "green" => number > 13,
        "blue" => number > 14,
        _ => false,
    }
}

pub fn resolve_part_one(file_content: String) -> usize {
    let result = file_content
        .lines()
        .enumerate()
        .fold(0, |acc, (index, line)| {
            let valid = line
                .split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .flat_map(|turn| turn.split(", "))
                .all(|turn| {
                    let (parsed_value, color) = turn.split_once(" ").unwrap();

                    !exceeds_limit(parsed_value.parse().unwrap(), color)
                });

            if valid {
                return acc + index + 1;
            }

            acc
        });

    result
}

pub fn resolve_part_two(file_content: String) -> usize {
    let result = file_content.lines().fold(0, |acc, line| {
        let game_data = line.split_once(": ").unwrap().1;

        let turns = game_data
            .split("; ")
            .flat_map(|turn| turn.split(", "))
            .map(|turn| {
                let (value, color) = turn.split_once(" ").unwrap();
                Turn {
                    red: if color == "red" {
                        value.parse().unwrap()
                    } else {
                        0
                    },
                    green: if color == "green" {
                        value.parse().unwrap()
                    } else {
                        0
                    },
                    blue: if color == "blue" {
                        value.parse().unwrap()
                    } else {
                        0
                    },
                }
            });

        let fewer_set = turns.fold(Turn::default(), |fewer_set, turn| Turn {
            red: fewer_set.red.max(turn.red),
            green: fewer_set.green.max(turn.green),
            blue: fewer_set.blue.max(turn.blue),
        });

        acc + (fewer_set.red * fewer_set.green * fewer_set.blue)
    });

    result
}
