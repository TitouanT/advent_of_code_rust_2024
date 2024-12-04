fn parse_input(input: &str) -> Vec<&str> {
    let mut lines = vec![];
    for l in input.split("\n") {
        if l.len() > 1 {
            lines.push(l);
        }
    }
    lines
}
fn check_word_matches(word: &str, input: &[&str], origin: (usize, usize), direction: (i32, i32), offset: i32) -> bool {
    let (line, col) = origin;
    let (dl, dc) = direction;
    let line = line as i32 - dl * offset;
    let col  = col as i32  - dc * offset;
    let nlines = input.len() as i32;
    let ncols = input[0].len() as i32;

    for (index, letter) in word.chars().enumerate() {
        let index = index as i32;
        let cur_line = line + dl * index;
        let cur_col = col + dc * index;
        if cur_line < 0 || cur_line >= nlines || cur_col < 0 || cur_col >= ncols {
            return false;
        }
        let Some(cur_letter) = input[cur_line as usize].chars().nth(cur_col as usize)
        else {
            return false;
        };
        if letter != cur_letter {
            return false
        }
    }
    true
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let input = parse_input(input);
    let mut nxmas = 0;
    for (i_line, &line) in input.iter().enumerate() {
        for (i_col, col) in line.chars().enumerate() {
            if col != 'X' {
                continue
            }
            for dline in -1..=1 {
                for dcol in -1..=1 {
                    if dline == 0 && dcol == 0 {
                        continue
                    }
                    nxmas += check_word_matches("XMAS", &input, (i_line, i_col), (dline, dcol), 0i32) as i32;
                }
            }
        }
    }
    nxmas
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let input = parse_input(input);
    let mut nxmas = 0;
    for (i_line, &line) in input.iter().enumerate() {
        for (i_col, col) in line.chars().enumerate() {
            if col != 'A' {
                continue
            }
            let mut mas = 0;
            for dline in [-1,1] {
                for dcol in [-1,1] {
                    mas += check_word_matches("MAS", &input, (i_line, i_col), (dline, dcol), 1i32) as i32;
                }
            }
            if mas == 2 {
                nxmas += 1;
            }
        }
    }
    nxmas
}
