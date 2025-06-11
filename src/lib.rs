pub type Sand = Vec<Vec<i16>>;

pub fn init_sand(h: i128, w: i128) -> Sand{
    let mut s: Sand = Vec::new();
    for _y in 0..h-1{
        let mut line = Vec::new();
        for _x in 0..w-1{
            line.push(0)
        }
        s.push(line);
    }
    return s;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}