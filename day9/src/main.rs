use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::character::complete::alpha1;
use nom::character::complete::one_of;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;
use std::io::BufRead;

fn sp<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let chars = " \n\t\r";
    take_while(move |c: char| chars.contains(c))(input)
}

macro_rules! ws {
    ($x:expr) => {
        tuple((sp, $x, sp)).map(|x| x.1)
    };
}

fn parse_diget<'a>(input: &'a str) -> IResult<&'a str, u32> {
    one_of("0123456789")
        .map(|x| x.to_digit(10).unwrap())
        .parse(input)
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Vec<u32>> {
    many0(parse_diget).parse(input)
}

fn get_2d(a: &[Vec<u32>], x: isize, y: isize) -> Option<u32> {
    a.get::<usize>(x.try_into().ok()?)?
        .get::<usize>(y.try_into().ok()?)
        .map(|x| *x)
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let lines: Vec<Vec<u32>> = lines.map(|x| parse_line(&x.unwrap()).unwrap().1).collect();

    let maxx = lines.len();

    // Keep track of the total risk value for the answer to the first part
    let mut total_risk = 0;

    // To find basins we first find the low points, then expand basins from them.

    // 0 = no basin, other vaulues = basin
    let mut basin_buffer: Vec<Vec<u32>> = vec![];

    // initalize the basin buffer.
    for x in 0..maxx {
        let maxy = lines[x].len();
        basin_buffer.push(vec![0; maxy])
    }

    // The basin ids must be uniqe so we keep a counter
    let mut basin_id = 1;

    for x in 0..maxx {
        let maxy = lines[x].len();
        for y in 0..maxy {
            let x = x as isize;
            let y = y as isize;
            let mut min_surrounding = u32::MAX;

            min_surrounding = min_surrounding.min(get_2d(&lines[..], x + 1, y).unwrap_or(u32::MAX));
            min_surrounding = min_surrounding.min(get_2d(&lines[..], x - 1, y).unwrap_or(u32::MAX));
            min_surrounding = min_surrounding.min(get_2d(&lines[..], x, y + 1).unwrap_or(u32::MAX));
            min_surrounding = min_surrounding.min(get_2d(&lines[..], x, y - 1).unwrap_or(u32::MAX));

            if (min_surrounding > (lines[x as usize][y as usize] as u32)) {
                let hight = lines[x as usize][y as usize] as u32;
                let risk = hight + 1;
                total_risk += risk;
                basin_buffer[x as usize][y as usize] = basin_id;
                basin_id += 1;
            }
        }
    }

    println!("total risk: {}", total_risk);

    let mut do_flow = true;
    while do_flow {
        do_flow = false;
        println!("simulating flow.");
        println!("basin_buffer: {:?}", basin_buffer);
        for x in 0..maxx {
            let maxy = lines[x].len();
            for y in 0..maxy {
                let hight = lines[x][y];
                // calculate witch locations (x,y) will flow into
                let mut flows_into: Vec<(usize, usize)> = vec![];
                let surrounding: [[isize; 2]; 4] = [[-1, 0], [1, 0], [0, 1], [0, -1]];
                for d in surrounding {
                    let fx = (x as isize + d[0]);
                    let fy = (y as isize + d[1]);
                    let fhight = get_2d(&lines[..], fx, fy).unwrap_or(u32::MAX);
                    if fhight < hight {
                        flows_into.push((fx as usize, fy as usize));
                    }
                }
            
                let mut basins_flowed_into = vec![];
                
                for pid in flows_into.iter() {
                    let bid = basin_buffer[pid.0][pid.1];
                    if !basins_flowed_into.contains(&bid) {
                        basins_flowed_into.push(bid)
                    } 
                }
                
                if (basins_flowed_into.len() == 1 && hight != 9) {
                    let flows = flows_into[0];
                    if (basin_buffer[x][y] != basin_buffer[flows.0][flows.1]) {
                        do_flow = true;
                        basin_buffer[x][y] = basin_buffer[flows.0][flows.1];
                    }
                }
            }
        }
    }
    
    let mut sizes = vec![0; basin_id as usize];
    for x in 0..maxx {
        let maxy = lines[x].len();
        for y in 0..maxy {
           sizes[basin_buffer[x][y] as usize] += 1; 
        }
    }

    let mut sizes_excluding_zero = &mut sizes[1..];
    
    sizes_excluding_zero.sort();
    
    let top_3 = &sizes_excluding_zero[(sizes_excluding_zero.len() - 3)..];
    
    println!("product of 3 largest basins {} [{:?}]", top_3[0] * top_3[1] * top_3[2], top_3);
}
