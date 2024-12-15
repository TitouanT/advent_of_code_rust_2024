#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let input_size = input.len();
    let mut index = 2;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for _ in 0..500 {
        let pc = {
            let limit = b',';
            let mut n = input[index] as i32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as i32 - 48;
                index += 1;
            }
            n
        };

        index += 1;

        let pl = {
            let limit = b' ';
            let mut n = input[index] as i32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as i32 - 48;
                index += 1;
            }
            n
        };

        index += 3;

        let vc = {
            let limit = b',';
            let sign = if input[index] == b'-'{ index += 1; -1 } else { 1 };
            let mut n = input[index] as i32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as i32 - 48;
                index += 1;
            }
            sign * n
        };

        index += 1;

        let vl = {
            let limit = b'\n';
            let sign = if input[index] == b'-'{ index += 1; -1 } else { 1 };
            let mut n = input[index] as i32 - 48;
            index += 1;
            while index < input_size && input[index] != limit {
                n = 10*n + input[index] as i32 - 48;
                index += 1;
            }
            sign * n
        };
        index += 3;

        let ec = (pc + vc*100).rem_euclid(101);
        let el = (pl + vl*100).rem_euclid(103);

        // slower 14%
        // match ec.cmp(&50) {
        //     Ordering::Less    => {
        //         match el.cmp(&51) {
        //             Ordering::Less    => q1 += 1,
        //             Ordering::Greater => q2 += 1,
        //             _ => {},
        //         }
        //     },
        //     Ordering::Greater => {
        //         match el.cmp(&51) {
        //             Ordering::Less    => q3 += 1,
        //             Ordering::Greater => q4 += 1,
        //             _ => {},
        //         }
        //     },
        //     _ => {},
        // }

        // about same
        // match ec {
        //     0..50 => {
        //         match el {
        //             0..51  => q1 += 1,
        //             52..103 => q2 += 1,
        //             _ => (),
        //         }
        //     },
        //     51..101 => {
        //         match el {
        //             0..51  => q3 += 1,
        //             52..103 => q4 += 1,
        //             _ => (),
        //         }
        //     },
        //     _ => (),
        // }

        // baseline
        if ec < 50 {
            if el < 51 {
                q1 += 1;
            }
            else if el > 51 {
                q2 += 1;
            }
        }
        else if ec > 50 {
            if el < 51 {
                q3 += 1;
            }
            else if el > 51 {
                q4 += 1;
            }
        }
    }
    q1 * q2 * q3 * q4
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    const SAMPLE_SIZE: usize = 90;
    let mut samples = [(0i32, 0i32, 0i32, 0i32);SAMPLE_SIZE];

    let mut index = 2;
    // for i in 0..SAMPLE_SIZE {
    for s in samples.iter_mut() {
        s.0 = {
            let limit = b',';
            let mut n = input[index] as i32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as i32 - 48;
                index += 1;
            }
            n
        };

        index += 1;

        s.2 = {
            let limit = b' ';
            let mut n = input[index] as i32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as i32 - 48;
                index += 1;
            }
            n
        };

        index += 3;

        s.1 = {
            let limit = b',';
            let sign = if input[index] == b'-'{ index += 1; -1 } else { 1 };
            let mut n = input[index] as i32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as i32 - 48;
                index += 1;
            }
            sign * n
        };

        index += 1;

        s.3 = {
            let limit = b'\n';
            let sign = if input[index] == b'-'{ index += 1; -1 } else { 1 };
            let mut n = input[index] as i32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as i32 - 48;
                index += 1;
            }
            sign * n
        };
        index += 3;
    }

    const L: usize = 103;
    let mut v = [[0;L];L];
    let mod_l = {
        const N: usize = L;
        let mut m = 0;
        let mut m_count = 0;
        for t in 1..N {
            // let mut v = [0;N];
            let mut maxi_value = 0;
            for s in samples {
                let e: usize = (s.2 + s.3*(t as i32)).rem_euclid(N as i32) as usize;
                v[t][e]+=1;
                if v[t][e] > maxi_value {
                    maxi_value = v[t][e];
                }
            }
            if m_count < maxi_value {
                m_count = maxi_value;
                m = t;
            }
        }
        m
    };

    const C: usize = 101;
    let mut v = [[0;C];C];
    let mod_c = {
        const N: usize = C;
        let mut m = 0;
        let mut m_count = 0;
        for t in 1..N {
            // let mut v = [0;N];
            let mut maxi_value = 0;
            for s in samples {
                let e: usize = (s.0 + s.1*(t as i32)).rem_euclid(N as i32) as usize;
                v[t][e]+=1;
                if v[t][e] > maxi_value {
                    maxi_value = v[t][e];
                }
            }
            if m_count < maxi_value {
                m_count = maxi_value;
                m = t;
            }
        }
        m
    };

    let mut n = 60;
    let mut t = n * L + mod_c;
    while t % C != mod_c {
        n += 1;
        t = n * L + mod_l;
    }
    t
}
