const ONE: usize = 0b11100110100100;
const TWO: usize = 0b100111011001110; 
const THREE: usize = 0b1001100111100010010000100;
const FOUR : usize = 0b101011101010010001;
const FIVE : usize = 0b101010001010100100;
const SIX : usize = 0b100100100010111; 
const SEVEN: usize = 0b1001000100101010010001101;
const EIGHT : usize = 0b10001000001100011110011;
const NINE : usize = 0b1101010000110100100;

fn main() {
    let input = include_str!("input1.txt");
    let start = std::time::Instant::now(); 
    for _ in 0..9999 {
        part2_fast(input);
    }
    println!("{:?}", start.elapsed() / 10000); 
}

fn part2_fast(input: &str) -> String {
    let mut accumulator : usize = 0;
    let mut first : usize;
    let mut last : usize = 0;
    let mut word : usize = 0;

    let mut iter = input.as_bytes().iter();
    'main: loop {
        loop {
            match iter.next() {
                Some(c) => {
                    match c {
                        b'1'..=b'9' => {
                            first = (c - b'0') as usize;
                            break;
                        },
                        b'a'..=b'z' => {
                            word = word << 5 | (c - b'a') as usize;
                            if word < ONE {
                                continue;
                            }
                            match word & 0b111111111111111 {
                                ONE => {
                                    first = 1;
                                    break;
                                },
                                SIX => {
                                    first = 6;
                                    break;
                                },
                                TWO => {
                                    first = 2;
                                    break;
                                },
                                _ => {
                                    if word < FIVE {
                                        continue;
                                    }
                                    match word & 0b11111111111111111111 {
                                        FIVE => {
                                            first = 5;
                                            break;
                                        },
                                        FOUR => {
                                            first = 4;
                                            break;
                                        },
                                        NINE => {
                                            first = 9;
                                            break;
                                        },
                                        _ => {
                                            if word < EIGHT {
                                                continue;
                                            }
                                            match word & 0b1111111111111111111111111 {
                                                EIGHT => {
                                                    first = 8;
                                                    break;
                                                },
                                                SEVEN => {
                                                    first = 7;
                                                    break;
                                                },
                                                THREE => {
                                                    first = 3;
                                                    break;
                                                },
                                                _ => {},
                                            };
                                        },
                                    }
                                },

                            }

                        },
                        _ => {
                            dbg!("{}", c);
                        },
                    }
                },
                None => {
                    break 'main;
                },
            };
        }
        word = 0;
        loop {
            match iter.next() {
                Some(c) => {
                    match c {
                        b'1'..=b'9' => {
                            word = 0;
                            last = (c - b'0') as usize;
                            continue;
                        },
                        b'a'..=b'z' => {
                            word = word << 5 | (c - b'a') as usize;
                            if word < ONE {
                                continue;
                            }
                            match word & 0b111111111111111 {
                                ONE => {
                                    last = 1;
                                    continue;
                                },
                                SIX => {
                                    last = 6;
                                    continue;
                                },
                                TWO => {
                                    last = 2;
                                    continue;
                                },
                                _ => {
                                    if word < FIVE {
                                        continue;
                                    }
                                    match word & 0b11111111111111111111 {
                                        FIVE => {
                                            last = 5;
                                            continue;
                                        },
                                        FOUR => {
                                            last = 4;
                                            continue;
                                        },
                                        NINE => {
                                            last = 9;
                                            continue;
                                        },
                                        _ => {
                                            if word < EIGHT {
                                                continue;
                                            }
                                            match word & 0b1111111111111111111111111 {
                                                EIGHT => {
                                                    last = 8;
                                                    continue;
                                                },
                                                SEVEN => {
                                                    last = 7;
                                                    continue;
                                                },
                                                THREE => {
                                                    last = 3;
                                                    continue;
                                                },
                                                _ => {},
                                            };
                                        },
                                    }
                                },

                            }

                        },
                        b'\n' | b'\0' => {
                            word = 0;
                            if last != 0 {
                                accumulator += first * 10 + last;
                                last = 0;
                                break;
                            }
                            accumulator += first * 10 + first;
                            break;
                        },
                        _ => {
                            dbg!("{}", c);
                        },
                    }
                },
                None => {
                    break 'main;
                },
            };
        }
    }

    // Parse through each char in the string, need to look for a fgetc equivalent
    return accumulator.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test4() {
        let input = include_str!("test3.txt");
        let result = part2_fast(input);
        assert_eq!(result, "281".to_string());
    }

    #[test]
    fn test_packed_nums() {
        let strs: [&str; 9] = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", ];
        let flags: [usize; 9] = [ ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, ];
        for i in 0..8 {
            let mut check : usize = 0;
            for c in strs[i].as_bytes() {
                check = check << 5 | (c - b'a') as usize;
            }
            assert_eq!(check, flags[i]);
        }
    }
}
