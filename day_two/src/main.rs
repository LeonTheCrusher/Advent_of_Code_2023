use std::fs::read_to_string;

fn main() {
    let content: String = read_to_string("test.txt").unwrap();

    println!("Easy: {}", easy(&content));
    println!("Hard: {}", hard(&content));
}

fn easy(string: &String) -> i32 {
    let mut sum = 0;

    let mut game = 1;

    for line in string.lines() {
        let mut possible = true;

        let game_outcomes = line.split(":").collect::<Vec<&str>>();
        let rounds = game_outcomes[1].split(";").collect::<Vec<&str>>();

        for i in 0..rounds.len() {
            let cubes_pulled = rounds[i].split(",").collect::<Vec<&str>>();

            for j in 0..cubes_pulled.len() {
                let cubes_by_color = cubes_pulled[j].split(" ").collect::<Vec<&str>>();

                let value: u32 = cubes_by_color[1].parse().unwrap();
                let cube_color = cubes_by_color[2];

                if cube_color == "red" {
                    if value > 12 {
                        possible = false;
                    }
                } else if cube_color == "green" {
                    if value > 13 {
                        possible = false;
                    }
                } else if cube_color == "blue" {
                    if value > 14 {
                        possible = false;
                    }
                }
            }
        }

        if possible == true {
            sum += game;
        }
        game += 1;
    }
    sum
}

fn hard(string: &String) -> i32 {
    let mut sum = 0;

    for line in string.lines() {
        let (mut red, mut green, mut blue) = (0, 0, 0);

        let game_outcomes = line.split(":").collect::<Vec<&str>>();
        let rounds = game_outcomes[1].split(";").collect::<Vec<&str>>();

        for i in 0..rounds.len() {
            let cubes_pulled = rounds[i].split(",").collect::<Vec<&str>>();

            for j in 0..cubes_pulled.len() {
                let cubes_by_color = cubes_pulled[j].split(" ").collect::<Vec<&str>>();

                let value: i32 = cubes_by_color[1].parse().unwrap();
                let cube_color = cubes_by_color[2];

                if cube_color == "red" && value > red {
                    red = value;
                } else if cube_color == "green" && value > green {
                    green = value;
                } else if cube_color == "blue" && value > blue {
                    blue = value;
                }
            }
        }
        // println!("{red}, {green}, {blue}");
        sum += red * green * blue;
    }

    return sum;
}
