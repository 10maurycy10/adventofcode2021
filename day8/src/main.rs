use std::io::BufRead;

const SEGMENTS: &str = "abcdef";

use nom::combinator::map_res;
use nom::multi::many0;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::multi::separated_list0;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::character::complete::alpha1;
use nom::Parser;
use std::collections::HashMap;

fn sp<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let chars = " \n\t\r";
    take_while(move |c: char| chars.contains(c))(input)
}

macro_rules! ws {
    ($x:expr) => { tuple((sp, $x, sp)).map(|x| x.1) }
}

fn parse_patern<'a>(input: &'a str) -> IResult<&'a str, Vec<char>>  {
    alpha1.map(|s: &str| s.chars().collect()).parse(input)
}

fn parse_wslist<'a>(input: &'a str) -> IResult<&'a str, Vec<Vec<char>>>  {
    many0(ws!(parse_patern))(input)
}

fn parse_entry<'a>(input: &'a str) -> IResult<&'a str, (Vec<Vec<char>>, [Vec<char>;4])>  {
    tuple((parse_wslist, ws!(tag("|")), parse_wslist)).map(|x| (x.0,x.2.try_into().unwrap())).parse(input)
}

fn seg_to_numeric(seg: char) -> u8 {
    match seg {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
        _ => panic!(),
    }
}

fn consists_of(lit: &[char], real: &[u8], is_posible: &mut [[bool;7]; 7]) {
    let mut lit_numeric = vec![];
    for seg in lit.iter() {
        lit_numeric.push(seg_to_numeric(*seg));
    }
    // for a intended segments...
    for real in real {
        // for all segments
        for i in 0..7 {
            // if not lit
            if (!lit_numeric.contains(&i)) {
                is_posible[*real as usize][i as usize] = false;
            }
        }
    }
}

const DIGETS: [[bool; 7]; 10] = [
    [true, true, true, false, true, true, true],    //0
    [false, false, true, false,false, true, false], //1
    [true, false, true, true, true, false, true],   //2
    [true, false, true, true, false, true, true],   //3
    [false, true, true, true, false, true, false],  //4
    [true, true, false, true, false, true, true],   //5
    [true, true, false, true, true, true ,true],    //6
    [true, false, true, false, false, true, false], //7
    [true, true, true, true, true, true, true],     //8
    [true, true, true, true, false, true, true]     //9
];

//  aaaa      0000
// b    c    1    2
// b    c    1    2
//  dddd      3333
// e    f    4    5
// e    f    4    5
//  gggg      6666
//  
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    
    let lines: Vec<(Vec<Vec<char>>, [Vec<char>;4])> = lines.map(|x| parse_entry(&x.unwrap()).unwrap().1).collect();
    
    let mut obv_counts: [u32; 10] = [0; 10];
    
    for line in lines.iter() {
        let display = &line.1;
        for d in display.iter() {
            match d.len() {
                2 => obv_counts[1] += 1,// 1
                4 => obv_counts[4] += 1,// 4
                3 => obv_counts[7] += 1,// 7
                7 => obv_counts[8] += 1,// 8
                _ => ()
            };
        }
    }
    
    let mut obv_sum = 0;
    for c in obv_counts.iter() {
        obv_sum += c;
    }
    println!("obvois number count: {}",obv_sum);
    
    for line in lines.iter() {
               
        let mut is_posible = [[true;7]; 7];
        
        let outputs = &line.0;
        
        for d in outputs.iter() {
            match d.len() {
                2 => consists_of(&d,&[2,5],&mut is_posible), // 1
                4 => consists_of(&d,&[1,2,3,5],&mut is_posible), // 4
                3 => consists_of(&d,&[0,2,5],&mut is_posible), //7
                7 => consists_of(&d,&[0,1,2,3,4,5,6],&mut is_posible), // 8
                _ => ()
            };
        }
        
        // for all digets
        for diget in DIGETS.iter() {
            // we check witch of the observed outputs it might be...
            let mut pos_outs = vec![];
            for out in outputs.iter() {
                // we check each segment individaly
                let mut cand = true;
                for orig_idx in 0..7 {
                    let possiblity = &is_posible[orig_idx];
                    for (out) in out.iter() {
                        if (!possiblity[seg_to_numeric(*out) as usize]) {
                            cand = true;
                        }
                    }
                }
                if cand {
                    pos_outs.push(out);
                }
            }
            if pos_outs.len() == 1 {
                let out = pos_outs[0];
                let real: Vec<u8> = diget.iter().enumerate().filter(|x| *x.1).map(|x| x.0 as u8).collect();
                consists_of(&out, &real, &mut is_posible);
                
            }
            //println!("{:?} : {:?}", diget, pos_outs);
        }
        
        println!("{:?}",is_posible)
 
    }
}
