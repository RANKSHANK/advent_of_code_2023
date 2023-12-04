use std::collections::VecDeque; // This spelling is killing my soul

fn main() {
    let input = include_str!("test1.txt");
    println!("Test Sum: {}", part2(input));
    let input = include_str!("input1.txt");
    println!("Sum: {}", part2(input));
    let start = std::time::Instant::now();
    for _ in 0..9999 {
        part2(input);
    }
    println!("Avg run time of {:?}", start.elapsed() / 10000);
}

fn part2(input : &str) -> String{
    let bytes : &[u8] = input.as_bytes();
    let mut cards: VecDeque<usize> = VecDeque::new();
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
    let num_wins = index - win_start;
    let draw_start : usize = index + 2;
    index += 24;
    loop {
        if bytes[index] == b'\n' {
            index+= 1;
            break
        }
        index+= 1;
    }

    let num_draws : usize = index - draw_start;
    for game in 0..(bytes.len() / (index)) {
        let mult : usize = match cards.pop_front() {
            Some(n) => n + 1,
            None => 1,
        };
        sum += mult;
        let mut flags : [bool; 100] = [false; 100];
        let mut score : usize = 0;
        let line_start : usize = game * index;
        let win_start : usize = line_start + win_start;
        for seg in (win_start..(win_start + num_wins)).step_by(3) {
            let c10 : u8 = bytes[seg];
            let c1 : usize = (bytes[seg + 1] - b'0') as usize;
            if c10 != b' ' {
                flags[((c10 - b'0') as usize) * 10 + c1] = true;
            } else {
                flags[c1] = true;
            }
        }
        let draw_start = line_start + draw_start;
        for seg in (draw_start..(draw_start + num_draws)).step_by(3) {
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
        for next_card in 0..score {
            if next_card < cards.len() {
               cards[next_card] += mult; 
            } else {
                cards.push_back(mult);
            }
        }
    }
        return sum.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test3() {
        let input = include_str!("test1.txt");
        assert_eq!(part2(input), "30");
    }

    #[test]
    fn test2() {
        let input = include_str!("test2.txt");
        assert_eq!(part2(input), "4");
    }
}
