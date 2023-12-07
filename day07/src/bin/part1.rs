fn main() {
    // let input = include_str!("test1.txt");
    // println!("Test Sum: {}", part1(input));
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
    const SCORE_MASK : u64 = 0b111111111111111111111;
    const FLAG_MASKS : [u64; 13] = [// vim macros go BRR
        (1 << 47) | (1 << 34) | (1 << 21), // 2
        (1 << 48) | (1 << 35) | (1 << 22), // 3
        (1 << 49) | (1 << 36) | (1 << 23), // 4
        (1 << 50) | (1 << 37) | (1 << 24), // 5
        (1 << 51) | (1 << 38) | (1 << 25), // 6
        (1 << 52) | (1 << 39) | (1 << 26), // 7
        (1 << 53) | (1 << 40) | (1 << 27), // 8
        (1 << 54) | (1 << 41) | (1 << 28), // 9
        (1 << 55) | (1 << 42) | (1 << 29), // T
        (1 << 56) | (1 << 43) | (1 << 30), // J
        (1 << 57) | (1 << 44) | (1 << 31), // Q
        (1 << 58) | (1 << 45) | (1 << 32), // K
        (1 << 59) | (1 << 46) | (1 << 33), // A
    ];
    const ONES_MASKS : [u64; 13] = [ 
        1 << 21, // 2
        1 << 22, // 3
        1 << 23, // 4
        1 << 24, // 5
        1 << 25, // 6
        1 << 26, // 7
        1 << 27, // 8
        1 << 28, // 9
        1 << 29, // T
        1 << 30, // J
        1 << 31, // Q
        1 << 32, // K
        1 << 33, // A
    ];
    const TWOS_MASKS : [u64; 13] = [
        1 << 34, // 2
        1 << 35, // 3
        1 << 36, // 4
        1 << 37, // 5
        1 << 38, // 6
        1 << 39, // 7
        1 << 40, // 8
        1 << 41, // 9
        1 << 42, // T
        1 << 43, // J
        1 << 44, // Q
        1 << 45, // K 
        1 << 46, // A
    ];
    const FOURS_MASKS : [u64; 13] = [
        1 << 47, // 2
        1 << 48, // 3
        1 << 49, // 4
        1 << 50, // 5
        1 << 51, // 6
        1 << 52, // 7
        1 << 53, // 8
        1 << 54, // 9
        1 << 55, // T
        1 << 56, // J
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
        for idx in 0..5 {
            let mut c = bytes[idx + index];
            match c {
                b'T' => {
                    c = 8;
                },
                b'J' => {
                    c = 9;
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
                    c = c - b'2';
                },
            }
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

// I stopped reading the rules at poker and scored based on hand strength
fn actual_poker_pt1(input : &str) -> String{
    let bytes : &[u8] = input.as_bytes();
    const SCORE_MASK : u64 = 0b111111111111111111111;
    const TYPE_MASK : u64 = 111 << 60; // save for debugging
    const FLAG_MASKS : [u64; 13] = [// vim macros go BRR
        (1 << 47) | (1 << 34) | (1 << 21), // 2
        (1 << 48) | (1 << 35) | (1 << 22), // 3
        (1 << 49) | (1 << 36) | (1 << 23), // 4
        (1 << 50) | (1 << 37) | (1 << 24), // 5
        (1 << 51) | (1 << 38) | (1 << 25), // 6
        (1 << 52) | (1 << 39) | (1 << 26), // 7
        (1 << 53) | (1 << 40) | (1 << 27), // 8
        (1 << 54) | (1 << 41) | (1 << 28), // 9
        (1 << 55) | (1 << 42) | (1 << 29), // T
        (1 << 56) | (1 << 43) | (1 << 30), // J
        (1 << 57) | (1 << 44) | (1 << 31), // Q
        (1 << 58) | (1 << 45) | (1 << 32), // K
        (1 << 59) | (1 << 46) | (1 << 33), // A
    ];
    const ONES_MASKS : [u64; 13] = [ 
        1 << 21, // 2
        1 << 22, // 3
        1 << 23, // 4
        1 << 24, // 5
        1 << 25, // 6
        1 << 26, // 7
        1 << 27, // 8
        1 << 28, // 9
        1 << 29, // T
        1 << 30, // J
        1 << 31, // Q
        1 << 32, // K
        1 << 33, // A
    ];
    const TWOS_MASKS : [u64; 13] = [
        1 << 34, // 2
        1 << 35, // 3
        1 << 36, // 4
        1 << 37, // 5
        1 << 38, // 6
        1 << 39, // 7
        1 << 40, // 8
        1 << 41, // 9
        1 << 42, // T
        1 << 43, // J
        1 << 44, // Q
        1 << 45, // K 
        1 << 46, // A
    ];
    const FOURS_MASKS : [u64; 13] = [
        1 << 47, // 2
        1 << 48, // 3
        1 << 49, // 4
        1 << 50, // 5
        1 << 51, // 6
        1 << 52, // 7
        1 << 53, // 8
        1 << 54, // 9
        1 << 55, // T
        1 << 56, // J
        1 << 57, // Q
        1 << 58, // K 
        1 << 59, // A
    ];
    let mut hands : Vec<u64> = Vec::new();
    let mut index : usize = 0;
    'outer: loop {
        let mut hand : u64 = 0;
        let mut hand_type = 0;
        for idx in index..index + 5 {
            let mut c = bytes[idx];
            match c {
                b'T' => {
                    c = 8;
                },
                b'J' => {
                    c = 9;
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
                    c = c - b'2';
                },
            }
            let packed : u64 = hand & FLAG_MASKS[c as usize];
            if packed & ONES_MASKS[c as usize] == 0 {
                hand |= ONES_MASKS[c as usize];
            } else {
                if packed & TWOS_MASKS[c as usize] == 0 {
                    hand_type += 1;
                    hand |= TWOS_MASKS[c as usize];
                } else {
                    hand_type += 2;
                    hand |= FOURS_MASKS[c as usize];
                }
            }
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
        // if idx < 10 || idx > 990{
        //     println!("TYPE\n    AKQJT98765432AKQJT98765432AKQJT98765432");
        //     for i in 0..64 {
        //         print!("{}", (hand >> (63 - i) & 1));
        //     }
        //     println!(" {} x {}", hand & SCORE_MASK, idx + 1);
        // }
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
        assert_eq!(part1(input), "6440");
    }

    #[test]
    fn test2() {
        let input = include_str!("test2.txt");
        assert_eq!(part1(input), "14605");
    }

    #[test]
    fn test3() {
        let input = include_str!("test3.txt");
        assert_eq!(part1(input), "1053");
    }
}
