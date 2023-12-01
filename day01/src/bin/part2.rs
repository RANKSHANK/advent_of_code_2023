fn main() {
    let input = include_str!("input1.txt");
    dbg!(part2(input));
}

fn part2(input: &str) -> String {
    let mut accumulator : i32 = 0;
    let mut first : i32 = -1;
    let mut last : i32 = -1;
    let mut word : u32 = 0;
    let a : u32 = 'a' as u32;
    let hashes : [[(u32, i32); 3]; 3] = [
        [ (pack_chars("three"), 3), (pack_chars("seven"), 7), (pack_chars("eight"), 8), ],
        [ (pack_chars("four"), 4), (pack_chars("five"), 5), (pack_chars("nine"), 9) ],
        [ (pack_chars("one"), 1), (pack_chars("two"), 2), (pack_chars("six"), 6), ],
    ];
    let min_word : u32 = hashes[2][0].0 - 1;
    for c in input.chars() {
        if c >= '0' && c <= '9' {
            word = 0;
            if first == -1 {
                first = c.to_digit(10).unwrap() as i32;
                continue;
            }  
            last = c.to_digit(10).unwrap() as i32;
            continue;
        }
        if c >= 'a' && c <= 'z' {
            word = (word << 5) | (c as u32 - a);
            if word >= min_word {
                let mut mask : u32 = 0b00000001111111111111111111111111;
                let mut check : u32 = word & mask;
                let mut val : i32 = -1;
                'outer: for level in hashes {
                    for pair in level {
                        if check == pair.0 {
                            val = pair.1;
                            break 'outer;
                        }
                    }
                    mask >>= 5;
                    check = check & mask;
                    if check < min_word {
                        break;
                    }
                }
                if val == -1 {
                    continue;
                }
                if first == -1 {
                    first = val;
                    continue;
                }  
                last = val;
                continue;
            }
        }
        if c == '\n' {
            if first == -1 {
                continue;
            }
            if last == -1 {
                accumulator += (first * 10 ) + first;
            } else {
                accumulator += (first * 10) + last;
            }
            word = 0;
            first = -1;
            last = -1;
            continue;
        }
    }
    if first != -1 {
        if last == -1 {
            accumulator += (first * 10 ) + first;
        } else {
            accumulator += (first * 10) + last;
        }
    }
    return accumulator.to_string();
}

fn pack_chars(word: &str) -> u32 {
    let mut ret : u32 = 0;
    for c in word.chars() {
        ret = ret << 5 | (c as u32 - 'a' as u32);
    }
    return ret;
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test3() {
        let input = include_str!("test3.txt");
        let result = part2(input);
        assert_eq!(result, "281".to_string());
    }
}
