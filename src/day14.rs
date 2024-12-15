#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let input_size = input.len();
    let mut index = 2;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    const L: u32 = 103;
    const C: u32 = 101;
    for _ in 0..500 {
        // let pc = {
        //     let limit = b',';
        //     let mut n = input[index] as i32 - 48;
        //     index += 1;
        //     while input[index] != limit {
        //         n = 10*n + input[index] as i32 - 48;
        //         index += 1;
        //     }
        //     n
        // };

        // index += 1;

        // let pl = {
        //     let limit = b' ';
        //     let mut n = input[index] as i32 - 48;
        //     index += 1;
        //     while input[index] != limit {
        //         n = 10*n + input[index] as i32 - 48;
        //         index += 1;
        //     }
        //     n
        // };

        // index += 3;

        // let vc = {
        //     let limit = b',';
        //     let sign = if input[index] == b'-'{ index += 1; -1 } else { 1 };
        //     let mut n = input[index] as i32 - 48;
        //     index += 1;
        //     while input[index] != limit {
        //         n = 10*n + input[index] as i32 - 48;
        //         index += 1;
        //     }
        //     sign * n
        // };

        // index += 1;

        // let vl = {
        //     let limit = b'\n';
        //     let sign = if input[index] == b'-'{ index += 1; -1 } else { 1 };
        //     let mut n = input[index] as i32 - 48;
        //     index += 1;
        //     while index < input_size && input[index] != limit {
        //         n = 10*n + input[index] as i32 - 48;
        //         index += 1;
        //     }
        //     sign * n
        // };
        // index += 3;

        let pc = {
            let limit = b',';
            let mut n = input[index] as u32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as u32 - 48;
                index += 1;
            }
            n
        };

        index += 1;

        let pl = {
            let limit = b' ';
            let mut n = input[index] as u32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as u32 - 48;
                index += 1;
            }
            n
        };

        index += 3;

        let vc = {
            let limit = b',';
            if input[index] == b'-'{
                index += 1;
                let mut n = 0;
                while input[index] != limit {
                    n = 10*n + input[index] as u32 - 48;
                    index += 1;
                }
                C - n
            }
            else {
                let mut n = 0;
                while input[index] != limit {
                    n = 10*n + input[index] as u32 - 48;
                    index += 1;
                }
                n
            }
        };

        index += 1;

        let vl = {
            let limit = b'\n';
            if input[index] == b'-'{
                index += 1;
                let mut n = 0;
                while index < input_size && input[index] != limit {
                    n = 10*n + input[index] as u32 - 48;
                    index += 1;
                }
                L - n
            }
            else {
                let mut n = 0;
                while index < input_size && input[index] != limit {
                    n = 10*n + input[index] as u32 - 48;
                    index += 1;
                }
                n
            }
        };
        index += 3;

        let ec = (pc + vc*100)%C;
        let el = (pl + vl*100)%L;

        // let ec = (pc + vc*100).rem_euclid(101);
        // let el = (pl + vl*100).rem_euclid(103);

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
pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    const SAMPLE_SIZE: usize = 120;
    let mut samples_lines = [(0u32, 0u32);SAMPLE_SIZE];
    let mut samples_cols = [(0u32, 0u32);SAMPLE_SIZE];
    let mut index = 2;
    const L: u32 = 103;
    const C: u32 = 101;
    for (sl, sc) in samples_lines.iter_mut().zip(samples_cols.iter_mut()) {
        sc.0 = {
            let limit = b',';
            let mut n = input[index] as u32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as u32 - 48;
                index += 1;
            }
            n
        };

        index += 1;

        sl.0 = {
            let limit = b' ';
            let mut n = input[index] as u32 - 48;
            index += 1;
            while input[index] != limit {
                n = 10*n + input[index] as u32 - 48;
                index += 1;
            }
            n
        };

        index += 3;

        sc.1 = {
            let limit = b',';
            if input[index] == b'-'{
                index += 1;
                let mut n = 0;
                while input[index] != limit {
                    n = 10*n + input[index] as u32 - 48;
                    index += 1;
                }
                C - n
            }
            else {
                let mut n = 0;
                while input[index] != limit {
                    n = 10*n + input[index] as u32 - 48;
                    index += 1;
                }
                n
            }
        };

        index += 1;

        sl.1 = {
            let limit = b'\n';
            if input[index] == b'-'{
                index += 1;
                let mut n = 0;
                while input[index] != limit {
                    n = 10*n + input[index] as u32 - 48;
                    index += 1;
                }
                L - n
            }
            else {
                let mut n = 0;
                while input[index] != limit {
                    n = 10*n + input[index] as u32 - 48;
                    index += 1;
                }
                n
            }
        };
        index += 3;
    }

    let mod_l = {
        let mut m = 0;
        let mut m_count = 0;
        for t in 1..L {
            let mut v = [0;L as usize];
            let mut maxi_value = 0;
            for s in samples_lines {
                let e: usize = ((s.0 + s.1*t) % L) as usize;
                v[e]+=1;
                if v[e] > maxi_value {
                    maxi_value = v[e];
                }
            }
            if m_count < maxi_value {
                m_count = maxi_value;
                m = t;
            }
        }
        m
    };

    {
        let mut m = 0;
        let mut m_count = 0;
        let mut t = mod_l;
        for _ in 1..C {
            t+=L;
            let mut v = [0;C as usize];
            let mut maxi_value = 0;
            for s in samples_cols {
                let e: usize = ((s.0 + s.1*t) % C) as usize;
                v[e]+=1;
                if v[e] > maxi_value {
                    maxi_value = v[e];
                }
            }
            if m_count < maxi_value {
                m_count = maxi_value;
                m = t;
            }
        }
        m
    }

    // let n = 60;
    // let mut t = n * L + mod_l;
    // while t % C != mod_c {
    //     t += L;
    // }
    // const T:usize = L*C;
    // if t > T { t-=T; }
    // mod_c
}
