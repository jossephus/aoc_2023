use itertools::Itertools;

#[derive(Debug)]
struct BoatRace {
    time: i64,
    distance: i64,
}

impl BoatRace {
    pub fn max_beat_records(&self) -> usize {
        (0..self.time)
            .map(|val| (self.time - val) * val)
            .filter(|&x| x > self.distance)
            .count()
    }
}

fn main() {
    let input = r#"
Time:        48     87     69     81
Distance:   255   1288   1117   1623
    "#;

    //part1(input);

    part2(input);
}

fn part1(input: &str) {
    let lines = input.lines();
    let all_numbers = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split_once(":").unwrap().1.split_once(" ").unwrap().1)
        .map(|nums| {
            nums.split(" ")
                .filter_map(|x| x.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let (times, distances) = all_numbers.split_at(all_numbers.len() / 2);
    let time_to_distance = times
        .iter()
        .enumerate()
        .map(|(index, &val)| BoatRace {
            time: val,
            distance: distances.get(index).unwrap().to_owned(),
        })
        .collect::<Vec<_>>();

    dbg!(time_to_distance
        .iter()
        .fold(1, |acc, e| acc * e.max_beat_records()));
}

fn part2(input: &str) {
    let lines = input.lines();
    let all_numbers = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split_once(":").unwrap().1.split_once(" ").unwrap().1)
        .inspect(|num| {
            dbg!(num.trim());
        })
        .map(|nums| nums.split(" ").map(|x| x.trim()).join(""))
        .collect::<Vec<_>>();

    let boatRece = BoatRace {
        time: all_numbers.get(0).unwrap().parse::<i64>().unwrap(),
        distance: all_numbers.get(1).unwrap().parse::<i64>().unwrap(),
    };

    dbg!(boatRece.max_beat_records());
}
