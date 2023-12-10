use itertools::Itertools;

fn until_zero(list: Vec<i32>) -> i32 {
    if list.is_empty() {
        return 0;
    }

    if let Some(x) = list.iter().counts().get(&0) {
        if x == &list.len() {
            return 0;
        }
    }

    let a = list
        .iter()
        .enumerate()
        .filter_map(|(index, val)| {
            //
            if index == list.len() - 1 {
                None
            } else {
                Some(list[index + 1] - val)
            }
        })
        .collect_vec();
    //let a = list.into_iter().coalesce(|x, y| Ok(x - y))
    (&list).last().unwrap().to_owned() + until_zero(a)
}

fn main() {
    let input = include_str!("../data/data.txt");

    let sum = input
        .lines()
        .map(|line| {
            let a = line
                .split(" ")
                .map(|a| a.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            until_zero(a)
        })
        .inspect(|x| {
            dbg!(x);
        })
        .sum::<i32>();
    //.collect::<Vec<_>>()
    //.iter()
    //.map()

    //let a = until_zero(list);

    println!("{:?}", sum);
}
