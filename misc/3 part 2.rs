use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut data: Vec<Vec<u8>> = vec![];
    for line in lines {
        data.push(line.unwrap().bytes().collect());
    }
    
    let mut matching: Vec<Vec<u8>> = data;
    
    let mut pos = 0;
    while matching.len() > 1 {
        let onecount = matching.iter().filter(|x| x[pos] == b'1').count();
        let most_common = if (onecount) >= (matching.len() - onecount) {b'1'} else {b'0'};
        let newmatch = matching.iter().filter(
            |x| x[pos] == most_common
        ).map(|x| x.clone()).collect();
        matching = newmatch;
        pos += 1;
    }
    println!("{:?}",String::from_utf8(matching[0].clone()))
}
