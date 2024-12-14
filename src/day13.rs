#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
    let input = input.as_bytes();
    let n = input.len();
    let mut index = 0;
    let mut count = 0;
    for _ in 0..320 {
        let xa = (input[index+12] as i32)*10 + (input[index+13] as i32) - 528;
        let ya = (input[index+18] as i32)*10 + (input[index+19] as i32) - 528;
        index += 21;
        let xb = (input[index+12] as i32)*10 + (input[index+13] as i32) - 528;
        let yb = (input[index+18] as i32)*10 + (input[index+19] as i32) - 528;
        index += 30;
        let mut xp = input[index] as i32-48;
        index+=1;
        while input[index]!=44 {
            xp = 10*xp + (input[index]-48) as i32;
            index += 1;
        }
        index += 4;

        let mut yp = input[index] as i32 - 48;
        index += 1;
        while index < n && input[index]!=10 {
            yp = 10*yp + input[index] as i32 - 48;
            index += 1;
        }
        index += 2;

        let num_b = yp*xa - xp*ya;
        let den_b = yb*xa - xb*ya;

        let (t1, m1) = (num_b / den_b, num_b % den_b);
        let num_a = xp - xb*t1;
        let den_a = xa;
        let (t0, m0) = (num_a / den_a, num_a % den_a);
        if m1 == 0 && m0 == 0 {
            count += 3*t0 + t1;
        }
    }
    count
}
#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let input = input.as_bytes();
    let n = input.len();
    let mut index = 0;
    let mut count = 0;
    for _ in 0..320 {
        let xa = (input[index+12] as i64)*10 + (input[index+13] as i64) - 528;
        let ya = (input[index+18] as i64)*10 + (input[index+19] as i64) - 528;
        index += 21;
        let xb = (input[index+12] as i64)*10 + (input[index+13] as i64) - 528;
        let yb = (input[index+18] as i64)*10 + (input[index+19] as i64) - 528;
        index += 30;
        let mut xp = input[index] as i64-48;
        index+=1;
        while input[index]!=44 {
            xp = 10*xp + (input[index]-48) as i64;
            index += 1;
        }
        index += 4;

        let mut yp = input[index] as i64 - 48;
        index += 1;
        while index < n && input[index]!=10 {
            yp = 10*yp + input[index] as i64 - 48;
            index += 1;
        }
        index += 2;

        xp += 10_000_000_000_000;
        yp += 10_000_000_000_000;
        let num_b = yp*xa - xp*ya;
        let den_b = yb*xa - xb*ya;

        let (t1, m1) = (num_b / den_b, num_b % den_b);
        let num_a = xp - xb*t1;
        let den_a = xa;
        let (t0, m0) = (num_a / den_a, num_a % den_a);
        if m1 == 0 && m0 == 0 {
            count += 3*t0 + t1;
        }
    }
    count
}
