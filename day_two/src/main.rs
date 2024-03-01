use std::fs::read_to_string;

fn main() {
    let content: String = read_to_string("games.txt").unwrap();

    println!("Easy: {}", easy(&content));
    println!("Hard: {}", hard(&content));
}

fn easy(string: &String) -> i32 {
    let mut sum = 0;

    let mut game = 1;

    for line in string.lines() {
        let parts = line.split(":");
        let collected_games = parts.collect::<Vec<&str>>();

        let mut possible = true;

        let collected_pulls = collected_games[1].split(";");
        let pulls = collected_pulls.collect::<Vec<&str>>();

        for i in 0..pulls.len() {
            let cubes = pulls[i].split(",");
            let collected_cubes = cubes.collect::<Vec<&str>>();

            for j in 0..collected_cubes.len() {
                let color = collected_cubes[j].split(" ");
                let collected_colors = color.collect::<Vec<&str>>();

                let value: u32 = collected_colors[1].parse().unwrap();
                let cube_color = collected_colors[2];

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

        let parts = line.split(":");
        let collected_games = parts.collect::<Vec<&str>>();

        let collected_pulls = collected_games[1].split(";");
        let pulls = collected_pulls.collect::<Vec<&str>>();

        for i in 0..pulls.len() {
            let cubes = pulls[i].split(",");
            let collected_cubes = cubes.collect::<Vec<&str>>();

            for j in 0..collected_cubes.len() {
                let color = collected_cubes[j].split(" ");
                let collected_colors = color.collect::<Vec<&str>>();

                let value: i32 = collected_colors[1].parse().unwrap();
                let cube_color = collected_colors[2];

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
