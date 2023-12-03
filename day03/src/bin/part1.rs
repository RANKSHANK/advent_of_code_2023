fn main() {
    let input = include_str!("input.txt");
    println!("Sum: {}", part1(input));
    let start = std::time::Instant::now();
    for _ in 0..9999 {
        part1(input);
    }
    println!("Avg run time of {:?}", start.elapsed() / 10000);
}

fn part1(input : &str) -> String {
    let bytes : &[u8] = input.as_bytes();
    let mut index : usize = 0;
    let mut sum : usize = 0;
    loop {
        if bytes[index] == b'\n' {
            break;
        }
        index += 1;
    }
    let leng : usize = index + 1;
    let mut line_start : usize = leng; // Need to track start since \n delims final vals

    loop {
        match bytes[index] {
            b'.' | b'0'..=b'9' => {
            },
            b'\n' => {
                line_start = index + 1;
                if line_start == bytes.len() {
                    break; //where eof :c
                }
            },
            _ => {
                // SEARCH UPPER
                let scan_index : usize = index - leng - 1;
                let mut skip : usize = 0;
                match bytes[scan_index] {
                    b'0'..=b'9' => {
                        let line_start = line_start - leng;
                        let mut val : usize = (bytes[scan_index] - b'0') as usize;
                        let mut num_scan = scan_index - 1;
                        let mut mult = 10;
                        skip = 1;
                        // UPPER LEFT
                        loop {
                            match bytes[num_scan] {
                                b'0'..=b'9' => {
                                    val += (bytes[num_scan] - b'0') as usize * mult;
                                    mult *= 10;
                                },
                                _ => {
                                    break;
                                },
                            }
                            if num_scan <= line_start {
                                break
                            }
                            num_scan -= 1;
                        }
                        let mut num_scan = scan_index + 1;
                        loop {
                            match bytes[num_scan] {
                                b'0'..=b'9' => {
                                    skip = 2;
                                    val *= 10;
                                    val += (bytes[num_scan] - b'0') as usize; 
                                },
                                _ => {
                                    break;
                                },
                            }
                            num_scan += 1;
                        }
                        sum += val;
                    },
                    _ => {},
                }
                // UPPER MIDDLE
                if skip == 0 {
                    let scan_index : usize = scan_index + 1;
                    match bytes[scan_index] {
                        b'0'..=b'9' => {
                            let mut val : usize = (bytes[scan_index] - b'0') as usize;
                            let mut num_scan = scan_index + 1;
                            loop {
                                match bytes[num_scan] {
                                    b'0'..=b'9' => {
                                        val *= 10;
                                        val += (bytes[num_scan] - b'0') as usize; 
                                    },
                                    _ => {
                                        break;
                                    },
                                }
                                num_scan += 1;
                            }
                            sum += val;
                            skip = 2;
                        },
                        _ => {},
                    }
                }
                // UPPER RIGHT
                if skip < 2 {
                    let scan_index : usize = scan_index + 2;
                    match bytes[scan_index] {
                        b'0'..=b'9' => {
                            let mut val : usize = (bytes[scan_index] - b'0') as usize;
                            let mut num_scan = scan_index + 1;
                            loop {
                                match bytes[num_scan] {
                                    b'0'..=b'9' => {
                                        val *= 10;
                                        val += (bytes[num_scan] - b'0') as usize; 
                                    },
                                    _ => {
                                        break;
                                    },
                                }
                                num_scan += 1;
                            }
                            sum += val;
                        },
                        _ => {},
                    }
                }


                // SEARCH LOWER 

                let scan_index : usize = index + leng - 1;
                let mut skip : usize = 0;
                // LOWER LEFT
                match bytes[scan_index] {
                    b'0'..=b'9' => {
                        let line_start = line_start - leng;
                        let mut val : usize = (bytes[scan_index] - b'0') as usize;
                        let mut num_scan = scan_index - 1;
                        let mut mult = 10;
                        skip = 1;
                        loop {
                            match bytes[num_scan] {
                                b'0'..=b'9' => {
                                    val += (bytes[num_scan] - b'0') as usize * mult;
                                    mult *= 10;
                                },
                                _ => {
                                    break;
                                },
                            }
                            if num_scan <= line_start {
                                break;
                            }
                            num_scan -= 1;
                        }
                        let mut num_scan = scan_index + 1;
                        loop {
                            match bytes[num_scan] {
                                b'0'..=b'9' => {
                                    skip = 2;
                                    val *= 10;
                                    val += (bytes[num_scan] - b'0') as usize; 
                                },
                                _ => {
                                    break;
                                },
                            }
                            num_scan += 1;
                        }
                        sum += val;
                    },
                    _ => {},
                }
                // LOWER MIDDLE
                if skip == 0 {
                    let scan_index : usize = scan_index + 1;
                    match bytes[scan_index] {
                        b'0'..=b'9' => {
                            let mut val : usize = (bytes[scan_index] - b'0') as usize;
                            let mut num_scan = scan_index + 1;
                            loop {
                                match bytes[num_scan] {
                                    b'0'..=b'9' => {
                                        val *= 10;
                                        val += (bytes[num_scan] - b'0') as usize; 
                                    },
                                    _ => {
                                        break;
                                    },
                                }
                                num_scan += 1;
                            }
                            sum += val;
                            skip = 2;
                        },
                        _ => {},
                    }
                }
                // LOWER RIGHT
                if skip < 2 {
                    let scan_index : usize = scan_index + 2;
                    match bytes[scan_index] {
                        b'0'..=b'9' => {
                            let mut val : usize = (bytes[scan_index] - b'0') as usize;
                            let mut num_scan = scan_index + 1;
                            loop {
                                match bytes[num_scan] {
                                    b'0'..=b'9' => {
                                        val *= 10;
                                        val += (bytes[num_scan] - b'0') as usize; 
                                    },
                                    _ => {
                                        break;
                                    },
                                }
                                num_scan += 1;
                            }
                            sum += val;
                        },
                        _ => {},
                    }
                }


                // SEARCH LEFT 
                let mut scan_index : usize = index - 1;
                let mut mult : usize = 1;
                loop {
                    match bytes[scan_index] {
                        b'0'..=b'9' => {
                            sum += (bytes[scan_index] - b'0') as usize * mult;
                            scan_index -= 1;
                            if scan_index == 0 {
                                break;
                            }
                            mult *= 10;
                        },
                        _ => {
                            break;
                        },
                    }
                }

                // SEARCH RIGHT
                let mut scan_index : usize = index + 1;
                match bytes[scan_index] {
                    b'0'..=b'9' => {
                        let mut val : usize = (bytes[scan_index] - b'0') as usize;
                        scan_index += 1;
                        loop {
                            match bytes[scan_index] {
                                b'0'..=b'9' => {
                                    val *= 10;
                                    val += (bytes[scan_index] - b'0') as usize;
                                    scan_index += 1;
                                },
                                _ => {
                                    break;
                                },
                            }
                        }
                        sum += val;

                    },
                    _ => {},
                }
            },
        }
        index += 1
    }
    return sum.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("test1.txt");
        let result = part1(input);
        assert_eq!(result, "4361".to_string());
    }
}
