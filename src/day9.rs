const N: usize = 20000/2;

#[derive(Copy, Clone)]
struct BlocStack {
    n: usize,
    stack: [usize;N],
}

#[derive(Copy, Clone)]
struct Bloc {
    id: usize,
    loc: usize,
    len: usize,
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn read_input(input: &str) -> ([Bloc;N], [Bloc;N-1]) {
    let mut files = [Bloc {
        id: 0,
        loc: 0,
        len: 0,
    };N];
    let mut spaces = [Bloc {
        id: 0,
        loc: 0,
        len: 0,
    };N-1];
    let mut loc = 0;
    let mut file_index = 0;
    let mut space_index = 0;
    let mut is_file = true;
    for size_char in input[..(2*N-1)].chars() {
        if is_file {
            let len = (size_char as u8 - 48) as usize;
            let file = &mut files[file_index];
            file.id = file_index;
            file.loc = loc;
            file.len = len;
            loc += len;
            file_index += 1;
            is_file = false;
        }
        else {
            let len = (size_char as u8 - 48) as usize;
            if len > 0 {
                let space = &mut spaces[space_index];
                space.id = 0;
                space.loc = loc;
                space.len = len;
                loc += len;
                space_index += 1;
            }
            is_file = true;
        }
    }
    (files, spaces)
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_wrap(input: &str) -> usize {
    let (files, spaces) = read_input(input);

    let mut file_index = N-1;
    let (mut fs_file_min, mut fs_file) = {
        let file = &files[file_index];
        (file.loc, file.loc + file.len - 1)
    };

    let mut space_index = 0;
    let (mut fs_space_max, mut fs_space) = {
        let space = &spaces[space_index];
        (space.loc + space.len, space.loc)
    };

    let mut checksum: usize = 0;
    for file in &files[1..] {
        let a = file.loc;
        let b = a + file.len;
        checksum += ((b*(b-1)-a*(a-1))/2)*(file.id);
    }
    while fs_space < fs_file {
        checksum -= (fs_file - fs_space)*file_index;
        {
            fs_space += 1;
            if fs_space >= fs_space_max {
                space_index += 1;
                (fs_space_max, fs_space) = {
                    let space = &spaces[space_index];
                    (space.loc + space.len, space.loc)
                };
            }
        }
        {
            fs_file -= 1;
            if fs_file < fs_file_min {
                file_index -= 1;
                (fs_file_min, fs_file) = {
                    let file = &files[file_index];
                    (file.loc, file.loc + file.len - 1)
                };
            }
        }
    }
    checksum

}
#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    unsafe {
        part1_wrap(input)
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_wrap(input: &str) -> usize {
    let (files, mut spaces) = read_input(input);

    let mut stacks = [BlocStack{n: 0, stack: [0;N]};10];
    for (i, space) in spaces.iter().enumerate().rev() {
        for stack in stacks.iter_mut().take(space.len + 1).skip(1) {
            stack.stack[stack.n] = i;
            stack.n += 1;
        }
    }

    let mut checksum = 0;
    for file in files.iter().rev() {
        let stack = &mut stacks[file.len];
        let mut loc = file.loc;
        while stack.n > 0 {
            let free_space_index = stack.stack[stack.n-1];
            let space = &mut spaces[free_space_index];
            if space.len < file.len {
                stack.n -= 1;
                continue
            }
            if space.loc > file.loc {
                break
            }
            loc = space.loc;
            if space.len == file.len {
                stack.n -= 1;
            }
            space.loc += file.len;
            space.len -= file.len;
            break
        }
        checksum += {
            let a = loc;
            let b = loc + file.len;
            ((b*(b-1)-a*(a-1))/2)*file.id
        }
    }
    checksum

}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    unsafe {
        part2_wrap(input)
    }
}
