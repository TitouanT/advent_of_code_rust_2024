const NPAT:usize = 600;

fn fp1_rec(line: &[u8], patterns: &[&[u8];NPAT], npat: usize, memo: &mut [Option<bool>;72], index:usize) -> bool {
    if let Some(ans) = memo[index]  {
        return ans
    }
    for pat in patterns.iter().take(npat) {
        let pat_end = index + pat.len();
        if pat_end <= line.len() && *pat == &line[index..pat_end] && fp1_rec(line, patterns, npat, memo, pat_end) {
            memo[index] = Some(true);
            return true;
        }
    }
    memo[index] = Some(false);
    false
}
fn fp1(line: &[u8], patterns: &[&[u8];NPAT], npat: usize) -> bool {
    let end = line.len();
    let mut states = [None;72];
    states[end] = Some(true);
    fp1_rec(line, patterns, npat, &mut states, 0)
    //
    // let mut states = [false;72];
    // states[0] = true;
    // let end = line.len();
    // let mut index = 0;
    // loop {
    //     while index < end && !states[index] {
    //         index += 1;
    //     }
    //     if index == end {
    //         return states[end];
    //     }
    //     for pat in patterns.iter().take(npat) {
    //         let pat_end = index + pat.len();
    //         if pat_end <= end && *pat == &line[index..pat_end] {
    //             states[pat_end] = true;
    //         }
    //     }
    //     index += 1;
    // }
}

// fn fp2_rec(line: &[u8], patterns: &[&[u8];NPAT], npat: usize, memo: &mut [Option<u64>;72], index:usize) -> u64 {
//     if let Some(ans) = memo[index]  {
//         return ans
//     }
//     let mut total = 0;
//     for pat in patterns.iter().take(npat) {
//         let pat_end = index + pat.len();
//         if pat_end <= line.len() && *pat == &line[index..pat_end] {
//             let child = fp2_rec(line, patterns, npat, memo, pat_end);
//             total += child;
//         }
//     }
//     memo[index] = Some(total);
//     total
// }

// fn fp2(line: &[u8], patterns: &[&[u8];NPAT], npat: usize) -> u64 {
//     let end = line.len();
//     let mut states = [None;72];
//     states[end] = Some(1);
//     fp2_rec(line, patterns, npat, &mut states, 0)
// }

fn fp2(line: &[u8], patterns: &[&[u8];NPAT], npat: usize) -> u64 {
    let mut states = [0;72];
    let end = line.len();
    let mut index = 0;
    states[0] = 1;
    loop {
        while index < end && states[index] == 0 {
            index += 1;
        }
        if index == end {
            return states[end];
        }
        let n = states[index];
        for pat in patterns.iter().take(npat) {
            let pat_end = index + pat.len();
            if pat_end <= end && *pat == &line[index..pat_end] {
                states[pat_end] += n;
            }
        }
        index += 1;
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let patterns = &mut [&input[..0];NPAT];
    let mut n_pat = 0;
    let mut index = 0;
    loop {
        let start = index;
        index += 1;
        while input[index] >= b'a' {
            index += 1;
        }
        patterns[n_pat] = &input[start..index];
        n_pat += 1;
        if input[index] == b'\n' {
            break
        }
        index += 2;
    }
    index += 2;
    let mut count = 0;
    let input_len = input.len();
    while index < input_len {
        let start = index;
        index += 1;
        while index < input_len && input[index] != b'\n' {
            index += 1;
        }
        let line = &input[start..index];
        if fp1(line, patterns, n_pat) {
            count += 1;
        }
        // // println!("{}", std::str::from_utf8(line).unwrap());
        index += 1;
    }
    count
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u64 {
    // println!("{}", input);
    let input = input.as_bytes();
    let patterns = &mut [&input[..0];NPAT];
    let mut n_pat = 0;
    let mut index = 0;
    loop {
        let start = index;
        index += 1;
        while input[index] >= b'a' {
            index += 1;
        }
        patterns[n_pat] = &input[start..index];
        n_pat += 1;
        if input[index] == b'\n' {
            break
        }
        index += 2;
    }
    index += 2;
    let mut count = 0;
    let input_len = input.len();
    while index < input_len {
        let start = index;
        index += 1;
        while index < input_len && input[index] != b'\n' {
            index += 1;
        }
        let line = &input[start..index];
        count += fp2(line, patterns, n_pat);
        index += 1;
    }
    count
}

