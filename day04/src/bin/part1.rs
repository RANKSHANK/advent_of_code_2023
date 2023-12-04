fn main() {
    let input = include_str!("input1.txt");
    println!("Sum: {}", part1(input));
    let start = std::time::Instant::now();
    for _ in 0..9999 {
        part1(input);
    }
    println!("Avg run time of {:?}", start.elapsed() / 10000);
}

fn part1(input : &str) -> String{
    let bytes : &[u8] = input.as_bytes();
    let mut sum : usize = 0;
    let mut index : usize = 6;
    loop {
        if bytes[index] == b':'{
            break;
        }
        index += 1;
    }
    let win_start : usize = index + 2;
    index += 16;
    loop {
        if bytes[index] == b'|' {
            break;
        }
        index += 1;
    }
    let num_wins = (index - win_start) / 3;
    let draw_start : usize = index + 2;
    index += 24;
    loop {
        if bytes[index] == b'\n' {
            index+= 1;
            break
        }
        index+= 1;
    }

    let num_draws : usize = (index - draw_start) / 3;
    for line in 0..(bytes.len() / (index)) {
        let mut flags : [bool; 100] = [false; 100];
        let mut score : usize = 0;
        let line_start : usize = line * index;
        let win_start : usize = line_start + win_start;
        for win in 0..num_wins {
            let seg = win_start + (win * 3);
            let c10 : u8 = bytes[seg];
            let c1 : usize = (bytes[seg + 1] - b'0') as usize;
            if c10 != b' ' {
                flags[((c10 - b'0') as usize) * 10 + c1] = true;
            } else {
                flags[c1] = true;
            }
        }
        let draw_start = line_start + draw_start;
        for draw in 0..num_draws {
            let seg = draw_start + (draw * 3);
            let c10 : u8 = bytes[seg];
            let c1 : usize = (bytes[seg + 1] - b'0') as usize;
            if c10 != b' ' {
                if flags[((c10 - b'0') as usize) * 10 + c1] {
                    score += 1;
                }
            } else {
                if flags[c1] {
                    score += 1;
                }
            }
        }
        if score != 0 {
            sum += 1 << (score - 1)
        }
    }
        return sum.to_string();
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("test1.txt");
        assert_eq!(part1(input), "13");
    }

    #[test]
    fn test2() {
        let input = include_str!("test2.txt");
        assert_eq!(part1(input), "1024");
    }
}
