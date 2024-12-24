macro_rules! mp {
    ($v:ident $op:tt $shift:literal) => ((($v $op $shift) ^ $v) & (16777216-1))
}

macro_rules! single_cycle {
    ($v:ident) => {
        $v=mp!($v<<6);
        $v=mp!($v>>5);
        $v=mp!($v<<11);
    }
}


#[aoc(day22, part1)]
pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut index = 0;
    let mut tot = 0;
    while input.len() > index {
        let mut v = 0;
        while input.len() > index && input[index] != b'\n' {
            v = v*10 + (input[index] - b'0') as u64;
            index+=1;
        }
        index+=1;
        for _ in 0..2000 {
            single_cycle!(v);
        }
        tot += v;
    }
    tot
}

use std::collections::HashMap;
#[aoc(day22, part2)]
pub fn part2(input: &str) -> u16 {
    let mut map: HashMap<(i8,i8,i8,i8),(u16,u16)> = HashMap::new();
    let input = input.as_bytes();
    let mut index = 0;
    let mut i = 0;
    while input.len() > index {
        let mut v = 0;
        i+=1;
        while input.len() > index && input[index] != b'\n' {
            v = v*10 + (input[index] - b'0') as u64;
            index+=1;
        }
        index+=1;
        let p0 = (v%10) as i8;

        single_cycle!(v);
        let p1 = (v%10) as i8;
        let mut d0 = p1 - p0;

        single_cycle!(v);
        let p2 = (v%10) as i8;
        let mut d1 = p2 - p1;

        single_cycle!(v);
        let mut p3 = (v%10) as i8;
        let mut d2 = p3 - p2;

        for _ in 3..2000 {
            single_cycle!(v);
            let p4 = (v%10) as i8;
            let d3 = p4 - p3;

            let key = (d0, d1, d2, d3);
            d0 = d1;
            d1 = d2;
            d2 = d3;
            p3 = p4;
            map.entry(key).and_modify(|v| {
                if v.0 < i {
                    *v = (i, v.1+p4 as u16)
                }
            }).or_insert((i, p4 as u16));
        }
    }
    map.into_values().map(|v|v.1).max().unwrap()
}
