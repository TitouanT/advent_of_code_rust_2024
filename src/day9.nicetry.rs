// const N: usize = 10000;
const N: usize = 20000/2;
// const N: usize = 20/2;

#[derive(Copy, Clone, Debug)]
struct BlocStack {
    n: usize,
    stack: [usize;N],
}

impl BlocStack {
    fn push(&mut self, elt: usize) {
        self.stack[self.n] = elt;
        self.n += 1;
    }
}
#[derive(Copy, Clone, Debug)]
struct Bloc {
    id: usize,
    loc: usize,
    len: usize,
}
impl Bloc {
    fn get_id(&mut self, id: usize) {
        self.id = id
    }
}
fn read_input(input: &str) -> ([Bloc;N], [Bloc;N-1]) {
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

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
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

    let mut checksum: u64 = 0;
    for file in &files[1..] {
        let a = file.loc;
        let b = a + file.len;
        checksum += (((b*(b-1)-a*(a-1))/2) as u64)*(file.id as u64);
    }
    while fs_space < fs_file {
        checksum -= (fs_file as u64 - fs_space as u64)*(file_index as u64);
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

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let (files, mut spaces) = read_input(input);
    let mut checksum: u64 = 0;
    for file in &files[1..] {
        let a = file.loc;
        let b = a + file.len;
        checksum += (((b*(b-1)-a*(a-1))/2) as u64)*(file.id as u64);
    }

    let mut stacks = [BlocStack{n: 0, stack: [&Bloc{id:0, len:0, loc:0};N]};10];
    for (i, space) in spaces.iter().enumerate().rev() {
        // for stack in stacks.iter().take(space.len + 1).skip(1) {
        // }
        for istack in 1..=space.len {
            // let mut stack = &mut stacks[istack];
            // stack.push(&space);
            stacks[istack].push(i);

        }
    }

    println!("{:?}", spaces[0]);
    spaces[0].get_id(42);
    println!("{:?}", stacks[1].stack[stacks[1].n-1]);


    checksum
}
