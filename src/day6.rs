#![allow(unused)]
struct Fish {
    new: bool,
    days_to_reproduce: u8,
}

impl Fish {
    fn create() -> Self {
        Self {
            new: true,
            days_to_reproduce: 8,
        }
    }
    fn from(init_days: u8) -> Self {
        Self {
            new: false,
            days_to_reproduce: init_days,
        }
    }

    fn decrement(&mut self) -> u64 {
        if self.days_to_reproduce == 0 {
            self.days_to_reproduce = 6;
            self.new = false;
            return 1;
        }
        self.days_to_reproduce -= 1;
        0
    }
}

fn part1() -> usize {
    let mut state = include_str!("../input/day6.txt")
        .split(",")
        .map(|n| {
            Fish::from(
                n.trim()
                    .parse()
                    .expect(format!("{} is not a valid number", n).as_str()),
            )
        })
        .collect::<Vec<Fish>>();
    for day in 0..80 {
        let num_fish_to_add: u64 = state.iter_mut().map(|f| f.decrement()).sum();
        for f in 0..num_fish_to_add {
            state.push(Fish::create());
        }
    }
    state.len()
}

fn part2() -> usize {
    let mut state = include_str!("../input/day6.txt")
        .split(",")
        .fold([0; 9], |mut map, n| {
            map[n.parse::<usize>().unwrap()] += 1;
            map
        });

    for day in 0..256 {
        let mut new_state: [usize; 9] = [0; 9];
        for (index, val) in state.iter().enumerate().rev() {
            let next_index = if index == 0 { 6 } else { index - 1 };
            new_state[next_index] += *val;
        }
        new_state[8] += state[0];
        state = new_state;
    }
    state.iter().sum()
}

pub fn exec() -> usize {
    println!("Day 6");
    part2()
}
