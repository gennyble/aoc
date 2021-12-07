use std::{str::FromStr, time::Instant};

use aoc2021::day_parse;

fn main() {
    let mut world: LanternWorld = day_parse!();

    let before = Instant::now();
    for _ in 0..80 {
        world.tick();
    }
    let time_80 = before.elapsed();

    println!(
        "Fish after 80 days {} [{} nanosec]",
        world.count(),
        time_80.as_nanos()
    );

    let before = Instant::now();
    for _ in 0..(256 - 80) {
        world.tick();
    }
    let time_total = time_80 + before.elapsed();

    println!(
        "Fish after 256 days {} [{} nanosec]",
        world.count(),
        time_total.as_nanos()
    );
}

struct LanternWorld {
    fish: [usize; 9],
}

impl LanternWorld {
    pub fn tick(&mut self) {
        let tmp = self.fish[0];
        self.fish[0] = self.fish[1];
        self.fish[1] = self.fish[2];
        self.fish[2] = self.fish[3];
        self.fish[3] = self.fish[4];
        self.fish[4] = self.fish[5];
        self.fish[5] = self.fish[6];
        self.fish[6] = self.fish[7] + tmp;
        self.fish[7] = self.fish[8];
        self.fish[8] = tmp;
    }

    pub fn count(&self) -> usize {
        self.fish.iter().sum()
    }
}

impl FromStr for LanternWorld {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fish = [0; 9];
        let lanterns: Vec<usize> = s
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        for l in lanterns {
            fish[l] += 1;
        }

        Ok(Self { fish })
    }
}
