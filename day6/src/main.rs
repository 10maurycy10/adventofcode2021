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

fn day(fishes:&Vec<u64>) -> Vec<u64> {
    // TODO this could be made much faster by keeping track of fishes with a counter of {0,1,2,3,4,5,6,7,8} instead of thousands of individal fishes.
    fishes.iter().map(|age| if *age == 0 {
        vec![6,8]
    } else {
        vec![age-1]
    }
    ).flatten().collect()
}

// much faster version of functions neaded for part 2.
fn day_binned(fishes:&[u64;10]) -> [u64;10] {
    let mut new = [0; 10];
    for i in 0..9 {
        new[i] = fishes[i + 1];
    }
    
    new[6] += fishes[0];
    new[8] += fishes[0];
    new
}

fn count_fishes(fishes: &[u64;10]) -> u64 {
    let mut sum = 0;
    for i in fishes.iter() {
        sum += i;
    }
    sum
}

fn bin_fishes(fishes: &Vec<u64>) -> [u64;10] {
    let mut binned = [0; 10];
    for fish in fishes.iter() {
        binned[*fish as usize] += 1
    }
    return binned
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    
    let line = lines.next().unwrap();
    let line = line.unwrap();
    let fish = parse_commalist(&line);
    let fish = fish.unwrap().1;

    let mut fish_binned = bin_fishes(&fish);
    
    for i in 0..257 {
        println!("fishes {:?}, fishes {}, day: {}",fish_binned,count_fishes(&fish_binned),i);
        let newfish_binned = day_binned(&fish_binned);
        fish_binned = newfish_binned;
    }
}
