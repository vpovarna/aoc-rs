// Sprinkles:    capacity 5,  durability -1, flavor 0, texture 0, calories 5
// PeanutButter: capacity -1, durability 3,  flavor 0, texture 0, calories 1
// Frosting:     capacity 0,  durability -1, flavor 4, texture 0, calories 6
// Sugar:        capacity -1, durability 0,  flavor 0, texture 2, calories 8

pub struct Day15Problem {
    part1_result: i32,
    part2_result: i32,
}

impl Day15Problem {
    pub fn new() -> Self {
        Self {
            part1_result: 0,
            part2_result: 0,
        }
    }

    pub fn solve(&mut self) {
        for sprinkles in 0..100 {
            for peanut_butter in 0..(100 - sprinkles) {
                for frosting in 0..(100 - sprinkles - peanut_butter) {
                    let sugar = 100 - sprinkles - peanut_butter - frosting;

                    if sugar < 0 {
                        continue;
                    }

                    let capacity = ((5 * sprinkles) - peanut_butter - sugar).max(0);
                    let durability = ((3 * peanut_butter) - sprinkles - frosting).max(0);
                    let flavor = (4 * frosting).max(0);
                    let texture = (2 * sugar).max(0);

                    if (capacity == 0) || (durability == 0) || (flavor == 0) || (texture == 0) {
                        continue;
                    }
                    let score = capacity * durability * flavor * texture;
                    self.part1_result = self.part1_result.max(score);

                    let calories = 5 * sprinkles + peanut_butter + 6 * frosting + 8 * sugar;
                    if calories == 500 {
                        self.part2_result = self.part2_result.max(score);
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let mut problem = Day15Problem::new();
    problem.solve();

    println!("{}", problem.part1_result);
    println!("{}", problem.part2_result);
}
