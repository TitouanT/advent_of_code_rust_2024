const N:usize = 16;

pub fn read_input(input: &str) -> ([u8;N], usize) {
    let input = input.as_bytes();
    let mut index = 12;
    let mut a = 0;
    while input[index] != b'\n' {
        a = 10*a + (input[index] - b'0') as usize;
        index += 1;
    }

    index += 39;
    let mut prg = [0u8;N];
    for i in prg.iter_mut() {
        *i = input[index] - b'0';
        index += 2;
    }
    (prg, a)
}

fn combo(a:usize, b:usize, c:usize, arg:u8) -> usize {
    if arg < 4 {
        arg as usize
    }
    else {
        [a,b,c][arg as usize - 4]
    }
}

pub fn get_one_byte(prg: &[u8;N], a: usize, ip: usize) -> (usize, usize, Option<u8>) {
    let mut a = a;
    let mut b = 0;
    let mut c = 0;
    let mut ip = ip;
    let mut output = None;
    loop {
        match prg[ip] {
            0 => a /= 2_usize.pow(combo(a, b, c, prg[ip+1]) as u32),
            1 => b ^= prg[ip+1] as usize,
            2 => b = combo(a,b,c,prg[ip+1])%8,
            3 => {
                return (a, (if a==0 {ip+2} else {prg[ip+1] as usize}), output);
            }
            4 => b ^= c,
            5 => output = Some((combo(a,b,c,prg[ip+1])%8) as u8),
            6 => b = a/2_usize.pow(combo(a, b, c, prg[ip+1]) as u32),
            7 => c = a/2_usize.pow(combo(a, b, c, prg[ip+1]) as u32),
            _ => unreachable!(),
        }
        ip+=2;
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let (prg, mut a) = read_input(input);
    let mut ip = 0;
    let mut ans = [0u8;32];
    let mut ans_size = 0;


    while ip < N {
        if let (new_a, new_ip, Some(output)) = get_one_byte(&prg, a, ip) {
            a = new_a;
            ip = new_ip;
            ans[ans_size] = output+b'0';
            ans[ans_size+1] = b',';
            ans_size += 2;
        }
        else {
            unreachable!();
        }
    }
    unsafe {
        std::str::from_utf8_unchecked(&ans[..ans_size-1]).to_string()
    }
}

// fn find_a_shift(prg: &[u8;N]) -> usize {
//     let mut a = 1;
//     while let (0, _, _) = get_one_byte(prg, a, 0) {
//         a*=2;
//     }
//     a
// }
// let shift = find_a_shift(&prg);

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let (prg, _) = read_input(input);
    let mut stack = [(0usize, 0u8, 0u8);N];
    let mut stack_size = 1;
    stack[0] = (0, 0, N as u8 - 1);
    let shift = 8;
    loop  {
        stack_size -= 1;
        let (a, bits, prg_to_output) = stack[stack_size];
        let big_a = a * shift + bits as usize;
        if let (_, _, Some(output)) = get_one_byte(&prg, big_a, 0) {
            if prg[prg_to_output as usize] == output {
                if prg_to_output == 0 {
                    return big_a;
                }
                if bits as usize + 1 < shift {
                    stack[stack_size] = (a, bits+1, prg_to_output);
                    stack_size += 1;
                }
                stack[stack_size] = (big_a, 0, prg_to_output-1);
                stack_size += 1;
            }
            else if bits as usize + 1 < shift {
                stack[stack_size] = (a, bits+1, prg_to_output);
                stack_size += 1;
            }
        }
    }
}
