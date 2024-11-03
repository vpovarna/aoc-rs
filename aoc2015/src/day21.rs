#[derive(Debug, Copy, Clone)]
struct Character<'a> {
    name: &'a str,
    armor: i8,
    damage: i8,
    hit: i8,
}

impl<'a> Character<'a> {
    fn new(name: &'a str, armor: i8, damage: i8, hit: i8) -> Character<'a> {
        return Character {
            name,
            armor,
            damage,
            hit,
        };
    }

    fn current_stats(self) {
        println!("{} remaining hit: {},", &self.name, &self.hit)
    }
}

fn solve(ch1: &mut Character, ch2: &mut Character) -> bool {
    while ch1.hit > 0 && ch2.hit > 0 {
        let ch1_damage = (ch1.damage - ch2.armor).max(1);
        ch2.hit -= ch1_damage;
        if ch2.hit <= 0 {
            return true
        }
        let ch2_damage = (ch2.damage - ch1.armor).max(1);
        ch1.hit -= ch2_damage;
        if ch1.hit <= 0 {
            return false
        }
        ch2.current_stats();
        ch1.current_stats();
        println!("=====")
    }
    panic!("both characters have hp under 0")
}

pub fn run() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2());
}

#[allow(dead_code)]
fn part1() -> usize {
    let mut player = Character {
        name: "player",
        armor: 5,
        damage: 5,
        hit: 8,
    };

    let mut boss = Character {
        name: "boss",
        armor: 2,
        damage: 7,
        hit: 12,
    };

    let t = solve(&mut player, &mut boss);
    println!("{}", t);
    1
}

#[allow(dead_code)]
fn part2() -> usize {
    1
}