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
    const SCORE_MASK : u64 = 0b111111111111111111111;
    const FLAG_MASKS : [u64; 13] = [// vim macros go BRR
        (1 << 47) | (1 << 34) | (1 << 21), // J
        (1 << 48) | (1 << 35) | (1 << 22), // 2
        (1 << 49) | (1 << 36) | (1 << 23), // 3
        (1 << 50) | (1 << 37) | (1 << 24), // 4
        (1 << 51) | (1 << 38) | (1 << 25), // 5
        (1 << 52) | (1 << 39) | (1 << 26), // 6
        (1 << 53) | (1 << 40) | (1 << 27), // 7
        (1 << 54) | (1 << 41) | (1 << 28), // 8
        (1 << 55) | (1 << 42) | (1 << 29), // 9
        (1 << 56) | (1 << 43) | (1 << 30), // T
        (1 << 57) | (1 << 44) | (1 << 31), // Q
        (1 << 58) | (1 << 45) | (1 << 32), // K
        (1 << 59) | (1 << 46) | (1 << 33), // A
    ];
    const ONES_MASKS : [u64; 13] = [ 
        1 << 21, // J
        1 << 22, // 2
        1 << 23, // 3
        1 << 24, // 4
        1 << 25, // 5
        1 << 26, // 6
        1 << 27, // 7
        1 << 28, // 8
        1 << 29, // 9
        1 << 30, // T
        1 << 31, // Q
        1 << 32, // K
        1 << 33, // A
    ];
    const TWOS_MASKS : [u64; 13] = [
        1 << 34, // J
        1 << 35, // 2
        1 << 36, // 3
        1 << 37, // 4
        1 << 38, // 5
        1 << 39, // 6
        1 << 40, // 7
        1 << 41, // 8
        1 << 42, // 9
        1 << 43, // T
        1 << 44, // Q
        1 << 45, // K 
        1 << 46, // A
    ];
    const FOURS_MASKS : [u64; 13] = [
        1 << 47, // J
        1 << 48, // 2
        1 << 49, // 3
        1 << 50, // 4
        1 << 51, // 5
        1 << 52, // 6
        1 << 53, // 7
        1 << 54, // 8
        1 << 55, // 9
        1 << 56, // T
        1 << 57, // Q
        1 << 58, // K 
        1 << 59, // A
    ];
    let mut hands : Vec<u64> = Vec::new();
    let mut index : usize = 0;
    'outer: loop {
        let mut hand : u64 = 0;
        let mut packed: u64 = 0;
        let mut hand_type = 0;
        let mut jokers = 0;
        for idx in 0..5 {
            let mut c = bytes[idx + index];
            match c {
                b'T' => {
                    c = 9;
                },
                b'J' => {
                    c = 0;
                },
                b'Q' => {
                    c = 10;
                },
                b'K' => {
                    c = 11;
                },
                b'A' => {
                    c = 12;
                },
                b'\n' | b'\0' => {
                    break 'outer;
                },
                _ => {
                    c = c - b'1';
                },
            }
            if c != 0 {
                let unpacked : u64 = packed & FLAG_MASKS[c as usize];
                if unpacked & ONES_MASKS[c as usize] == 0 {
                    packed |= ONES_MASKS[c as usize];
                } else {
                    if unpacked & TWOS_MASKS[c as usize] == 0 {
                        hand_type += 1;
                        packed |= TWOS_MASKS[c as usize];
                    } else {
                        hand_type += 2;
                        packed |= FOURS_MASKS[c as usize];
                    }
                }
                hand |= (c as u64) << (55 - idx * 4);
            } else {
                jokers += 1;
            }
        }
        for _ in 0..jokers {
            hand_type = match hand_type {
                1 => 3, // 1p -> 3ok
                2 => 4, // 2p -> fh
                3 => 5, // 3ok -> 4ok
                5 => 7, // 4ok -> 5ok
                7 => 7, // DAMN YOU JJJJJ
                _ => 1, // _ -> 1p
            };
        }
        index += 6;
        hand |= hand_type  << 60;
        let mut score : u64 = 0;
        loop {
            let c : u8 = bytes[index];
            index += 1;
            match c {
                b'\n' | b'\0' => {
                    break;
                },
                _ => {
                    score *= 10;
                    score += (c - b'0') as u64;
                },
            }
        }
        hand |= score;
        hands.push(hand);
        if index >= (bytes.len()){
            break;
        }
    }
    hands.sort_by(|a, b| a.cmp(b));
    let mut sum : u64 = 0;
    for (idx, hand) in hands.iter().enumerate() {
        sum += (idx + 1) as u64 * (hand & SCORE_MASK);
    };
    return sum.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("test1.txt");
        assert_eq!(part2(input), "5905");
    }

    #[test]
    fn test2() {
        let input = include_str!("test2.txt");
        assert_eq!(part2(input), "13774");
    }
}
