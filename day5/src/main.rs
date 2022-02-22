const NUMBERS: &str = "1234567890";

use std::io::BufRead;


use nom::combinator::map_res;
use nom::multi::many0;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::multi::separated_list0;
use nom::bytes::complete::take_while;
use nom::Parser;

use nom::combinator::success;

fn sp<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let chars = " \n\t\r";
    take_while(move |c: char| chars.contains(c))(input)
}

macro_rules! ws {
    ($x:expr) => { tuple((sp, $x, sp)).map(|x| x.1) }
}

fn parse_number<'a>(input: &'a str) -> IResult<&'a str, i32>  {
    map_res(take_while(|x| NUMBERS.contains(x)), |s: &str| s.parse::<i32>())(input)
}

fn parse_pair<'a>(input: &'a str) -> IResult<&'a str, [[i32; 2]; 2]>  {
    tuple((ws!(parse_number), tag(","), ws!(parse_number), tag("->"), ws!(parse_number), tag(","), ws!(parse_number))).map(|x| [[x.0, x.2],[x.4,x.6]]).parse(input)
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    
    let lines: Vec<[[i32; 2]; 2]> = lines.map(|x| x.unwrap()).map(|x| parse_pair(&x).unwrap().1).collect();

    let vlines: Vec<&[[i32; 2]; 2]> = lines.iter().filter(|x| x[0][0] == x[1][0]).collect();
    let hlines: Vec<&[[i32; 2]; 2]> = lines.iter().filter(|x| x[0][1] == x[1][1]).collect();
    
    let dlines: Vec<&[[i32; 2]; 2]> = lines.iter().filter(|x| {
        println!("xdif: {} ydif: {}",x[1][0]-x[0][0],x[1][1]-x[0][1]);
        (x[1][0]-x[0][0]).abs() == (x[1][1]-x[0][1]).abs()
    }).collect();
    
    let mut x_max = 0;
    let mut y_max = 0;
    
    for i in lines.iter() {
        for point in i.iter() {
            if point[0] > x_max {
                x_max = point[0];
            }
            if point[1] > y_max {
                y_max = point[1];
            }
        }
    }
    
    let y_max = y_max;
    let x_max = x_max;
    
    let mut lines_at_point = vec![vec![0; y_max as usize + 1]; x_max as usize + 1];
    
    for line in vlines.iter() {
        let x = line[0][0];
        assert_eq!(line[0][0],line[1][0]);
        let y1 = line[0][1];
        let y2 = line[1][1];
        let (ymin,ymax) = if y1 > y2 {(y2,y1)} else {(y1,y2)};
        for y in ymin..(ymax+1) {
            lines_at_point[x as usize][y as usize] += 1;
        }
    }
    
    for line in hlines.iter() {
        let y = line[0][1];
        assert_eq!(line[0][1],line[1][1]);
        let x1 = line[1][0];
        let x2 = line[0][0];
        let (xmin,xmax) = if x1 > x2 {(x2,x1)} else {(x1,x2)};
        for x in xmin..(xmax+1) {
        //    println!("marking point");
            lines_at_point[x as usize][y as usize] += 1;
        }
    }
    
    let mut danger_points = 0;
    
    for col in lines_at_point.iter() {
        for cell in col.iter() {
            if *cell > 1 {
                danger_points += 1;
            }
        }
    }
     
    println!("at least 2 (h/v) lines overlap on: {}",danger_points);

    for line in dlines.iter() {
        let y1 = line[0][1];
        let y2 = line[1][1];
        let x1 = line[0][0];
        let x2 = line[1][0];
        
        let (xmin,xmax) = if x1 > x2 {(x2,x1)} else {(x1,x2)};
        let (y_at_xmin,y_at_xmax) = if x1 > x2 {(y2,y1)} else {(y1,y2)};
        
        let dydx = (y_at_xmin-y_at_xmax)/(xmin-xmax);
        
        
        for x in xmin..(xmax+1) {
            let i = x - xmin;
            let y = y_at_xmin + i * dydx;
            lines_at_point[x as usize][y as usize] += 1;
        }
    }
    
    let mut danger_points = 0;
    
    for col in lines_at_point.iter() {
        for cell in col.iter() {
            if *cell > 1 {
                danger_points += 1;
            }
        }
    }
     
    println!("at least 2 (h/v/d) lines overlap on: {}",danger_points);
}
