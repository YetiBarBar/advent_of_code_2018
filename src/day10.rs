use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{char, i32},
    combinator::map,
    multi::{many0, separated_list0},
    IResult, Parser,
};

#[derive(Debug)]
struct Star {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn parse_star(data: &str) -> IResult<&str, Vec<Star>> {
    separated_list0(
        char('\n'),
        map(
            (
                tag("position=<"),
                many0(char(' ')),
                i32,
                char(','),
                many0(char(' ')),
                i32,
                tag("> velocity=<"),
                many0(char(' ')),
                i32,
                tag(","),
                many0(char(' ')),
                i32,
                char('>'),
            ),
            |(_, _, x, _, _, y, _, _, vx, _, _, vy, _)| Star { x, y, vx, vy },
        ),
    )
    .parse(data)
}

fn main() {
    let stars = include_str!("../data/day_2018_10.data");
    let stars = parse_star(&stars).unwrap().1;

    // println!("{:#?}", stars);
    // let mut hmap = HashMap::new();
    // Find most probable position
    // for idx in 0..11_000 {
    //     // let mut delta_x = vec![];
    //     let mut delta_y = vec![];
    //     for star in &stars {
    //         delta_y.push((star.y + star.vy * idx).abs());
    //     }
    //     hmap.insert(idx, delta_y.iter().sum::<i32>() / delta_y.len() as i32);
    // }
    //
    // println!("{:?}", hmap.iter().min_by_key(|(_, &v)| v));

    // with code above, we could figure out that collinding should be around 10142

    // Let's display all from 10_100 to 10_200
    for idx in 10_000..12_200 {
        let mut hset = HashSet::new();
        for star in stars.iter() {
            hset.insert((star.x + idx * star.vx, star.y + idx * star.vy));
        }

        println!("***************              {}          ********", idx);
        // Let's assume 200 char wide by 20 char tall
        for y in -20..20 {
            for x in -100..100 {
                if hset.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!(" ");
                };
            }
            println!();
        }
    }
}
