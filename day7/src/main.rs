use std::io::BufRead;

const NUMBERS: &str = "1234567890";

use nom::combinator::map_res;
use nom::multi::many0;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::multi::separated_list0;
use nom::bytes::complete::take_while;
use nom::Parser;

fn sp<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let chars = " \n\t\r";
    take_while(move |c: char| chars.contains(c))(input)
}

macro_rules! ws {
    ($x:expr) => { tuple((sp, $x, sp)).map(|x| x.1) }
}

fn parse_number<'a>(input: &'a str) -> IResult<&'a str, u64>  {
    map_res(take_while(|x| NUMBERS.contains(x)), |s: &str| s.parse::<u64>())(input)
}

fn parse_commalist<'a>(input: &'a str) -> IResult<&'a str, Vec<u64>>  {
    separated_list0(ws!(tag(",")), parse_number)(input)
}

fn get_updated_fuel_cost(dist: i64) -> i64 {
    (dist * (dist + 1))/2
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    
    let line = lines.next().unwrap();
    let line = line.unwrap();
    let crabs = parse_commalist(&line);
    let crabs: Vec<i64> = crabs.unwrap().1.iter().map(|x| *x as i64).collect();
    
    let mut crab_pos_min = i64::MAX;
    let mut crab_pos_max = i64::MIN;
    for i in crabs.iter() {
        if i > &crab_pos_max {
            crab_pos_max = *i
        } 
        if i < &crab_pos_min {
            crab_pos_min = *i
        }
    }
    
    let mut best_pos = -1;
    let mut best_fuel = i64::MAX;
    
    for target in crab_pos_min..crab_pos_max {
        let mut total_fuel = 0;
        for crab in crabs.iter() {
            let fuel = (crab - target).abs();
            total_fuel += fuel;
        }
        if total_fuel < best_fuel {
            best_pos = target;
            best_fuel = total_fuel;
        }
    }
    
    println!("best pos: {} best fuel: {}",best_pos,best_fuel);
    
    let mut best_pos = -1;
    let mut best_fuel = i64::MAX;
    
    for target in crab_pos_min..crab_pos_max {
        let mut total_fuel = 0;
        for crab in crabs.iter() {
            let dist = (crab - target).abs();
            total_fuel += get_updated_fuel_cost(dist);
        }
        if total_fuel < best_fuel {
            best_pos = target;
            best_fuel = total_fuel;
        }
    }
    
    println!("best pos: {} best fuel (UDATED): {}",best_pos,best_fuel);
}
