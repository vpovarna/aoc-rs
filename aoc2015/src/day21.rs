use itertools::{iproduct, Itertools};

#[allow(dead_code)]
pub fn run() {
    let boss_gear = Gear {
        damage: 8,
        armor: 1,
        cost: 0,
    };
    let player_hp = 100;
    let boss_hp = 104;

    let all_gears = player_gears();


    println!("{}", cheapest_gear_that_win(&all_gears, player_hp, &boss_gear, boss_hp));
    println!("{}", expensive_gear_that_loose(&all_gears, player_hp, &boss_gear, boss_hp));
}

#[allow(dead_code)]
fn cheapest_gear_that_win(all_gears: &Vec<Gear>, player_hp: i8, boss_stats: &Gear, boss_hp: i8) -> u16 {
    all_gears.iter()
        .filter(|current_gear| will_win(current_gear, player_hp, boss_stats, boss_hp))
        .map(|current_gear| current_gear.cost)
        .min()
        .unwrap()

    // all_gears.sort_by_key(|s| s.cost);
    //
    // all_gears.iter()
    //     .find(|current_gear| {
    //         will_win(current_gear, player_hp, boss_stats, boss_hp)
    //     })
    //     .map(|current_gear| current_gear.cost)
    //     .unwrap()
}

fn expensive_gear_that_loose(all_gears: &Vec<Gear>, player_hp: i8, boss_stats: &Gear, boss_hp: i8) -> u16 {
    all_gears.iter()
        .filter(|current_gear| !will_win(current_gear, player_hp, boss_stats, boss_hp))
        .map(|current_gear| current_gear.cost)
        .max()
        .unwrap()
}

#[allow(dead_code)]
fn will_win(player1_gear: &Gear, mut hp1: i8, player2_gear: &Gear, mut hp2: i8) -> bool {
    while hp1 > 0 && hp2 > 0 {
        let ch1_damage = (player1_gear.damage - player2_gear.armor).max(1);
        hp2 -= ch1_damage;
        if hp2 <= 0 {
            return true;
        }
        let ch2_damage = (player2_gear.damage - player1_gear.armor).max(1);
        hp1 -= ch2_damage;
        if hp1 <= 0 {
            return false;
        }
    }
    panic!("both characters have hp under 0")
}

// fn will_win(stats1: &Gear, hp1: i8, stats2: &Gear, hp2: i8) -> bool {
//     if hp1 <= 0 {
//         false
//     } else if hp2 <= 0 {
//         true
//     } else {
//         let new_hp2 = hp2 - (stats1.damage - stats2.armor).max(1);
//         !will_win(stats2, new_hp2, stats1, hp1)
//     }
// }

#[derive(Copy, Clone, Debug)]
struct Gear {
    damage: i8,
    armor: i8,
    cost: u16,
}

impl Gear {
    fn empty() -> Self {
        Self {
            damage: 0,
            armor: 0,
            cost: 0,
        }
    }

    fn combine(self, other: &Self) -> Self {
        Self {
            damage: self.damage + other.damage,
            armor: self.armor + other.armor,
            cost: self.cost + other.cost,
        }
    }
}

fn player_gears() -> Vec<Gear> {
    let weapons = vec![
        Gear { damage: 4, armor: 0, cost: 8 },
        Gear { damage: 5, armor: 0, cost: 10 },
        Gear { damage: 6, armor: 0, cost: 25 },
        Gear { damage: 7, armor: 0, cost: 40 },
        Gear { damage: 8, armor: 0, cost: 74 },
    ];
    let armors = vec![
        Gear { damage: 0, armor: 1, cost: 13 },
        Gear { damage: 0, armor: 2, cost: 31 },
        Gear { damage: 0, armor: 3, cost: 53 },
        Gear { damage: 0, armor: 4, cost: 75 },
        Gear { damage: 0, armor: 5, cost: 102 },
    ];
    let rings = vec![
        Gear { damage: 1, armor: 0, cost: 25 },
        Gear { damage: 2, armor: 0, cost: 50 },
        Gear { damage: 3, armor: 0, cost: 100 },
        Gear { damage: 0, armor: 1, cost: 20 },
        Gear { damage: 0, armor: 2, cost: 40 },
        Gear { damage: 0, armor: 3, cost: 80 },
    ];

    // generate all the combinations between two ring
    let two_rings: Vec<Gear> = rings.iter()
        .combinations(2)
        .map(|c| {
            c[0].combine(&c[1])
        })
        .collect_vec();

    // generate all rings combination
    let rings_layout = [two_rings, rings, vec![Gear::empty()]].concat();
    let armor_layouts = [armors, vec![Gear::empty()]].concat();

    // let mut all_possible_gears: Vec<Gear> = vec![];
    //
    // for weapon in &weapons {
    //     for ring in &rings_layout {
    //         for armor in &armor_layouts {
    //             let combined_gear = weapon.combine(&ring).combine(&armor);
    //             all_possible_gears.push(combined_gear);
    //         }
    //     }
    // }

    // Elegant solution:
    let all_possible_gears = iproduct!(&weapons, &armor_layouts, &rings_layout)
        .map(|(w, a, r)| {
            w.combine(a).combine(r)
        }).collect_vec();
    // println!("{:?}", all_possible_gears);
    // println!("{:?}", all_possible_gears.len());

    all_possible_gears
}
