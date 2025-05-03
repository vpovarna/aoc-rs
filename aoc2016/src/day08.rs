use aoclib::read_lines;
use itertools::Itertools;

#[allow(dead_code)]
pub fn run() {
    let lines = read_lines("input/2016/day08.txt");
    let commands = parse_input(lines.clone());

    let mut screen = [[false; 50]; 6];
    apply_commands(&commands, &mut screen);

    println!("Part 1: {}", count_pixels(&screen));
    println!("Part 2:");
    print_screen(&screen);
}

#[derive(Debug)]
enum Command {
    Rect(usize, usize),
    RotateCol(usize, usize),
    RotateRow(usize, usize),
}

#[allow(dead_code)]
fn count_pixels(screen: &[[bool; 50]; 6]) -> usize {
    screen
        .map(|row| row.into_iter().filter(|&pixel| pixel).count())
        .iter()
        .sum()
}

#[allow(dead_code)]
fn print_screen(screen: &[[bool; 50]; 6]) {
    for x in 0..screen.len() {
        println!(
            "{}",
            screen[x]
                .map(|pixel| if pixel { "**" } else { "  " })
                .concat()
        );
    }
}

#[allow(dead_code)]
fn apply_commands(commands: &Vec<Command>, screen: &mut [[bool; 50]; 6]) {
    for command in commands {
        match command {
            &Command::Rect(yr, xr) => {
                for x in 0..xr {
                    for y in 0..yr {
                        screen[x][y] = true;
                    }
                }
            }
            &Command::RotateCol(idx, ammount) => {
                let mut col = screen.map(|row| row[idx]).into_iter().collect_vec();
                col.rotate_right(ammount);
                for x in 0..6 {
                    screen[x][idx] = col[x];
                }
            }
            &Command::RotateRow(idx, ammount) => {
                screen[idx].rotate_right(ammount);
            }
        }
    }
}

#[allow(dead_code)]
fn parse_input(input: Vec<String>) -> Vec<Command> {
    input
        .iter()
        .map(|line| {
            let parts = line.split(' ').collect_vec();

            if parts[0] == "rect" {
                let xy = parts[1].split('x').collect_vec();
                Command::Rect(xy[0].parse().unwrap(), xy[1].parse().unwrap())
            } else {
                let idx = parts[2].split('=').collect_vec()[1].parse().unwrap();
                let dir = parts[4].parse().unwrap();

                if parts[1] == "column" {
                    Command::RotateCol(idx, dir)
                } else {
                    Command::RotateRow(idx, dir)
                }
            }
        })
        .collect_vec()
}
