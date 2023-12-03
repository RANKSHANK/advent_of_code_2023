fn main() {
    let input = include_str!("input.txt");
    println!("Sum: {}", part2(input));
    let start = std::time::Instant::now();
    for _ in 0..9999 {
        part2(input);
    }
    println!("Avg run time of {:?}", start.elapsed() / 10000);
}

fn part2(input : &str) -> String {
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
            b'*' => {
                let mut gear: usize = 0;
                let mut teeth: usize = 0;
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
                        gear = val;
                        teeth = 1;
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
                            gear = val;
                            teeth = 1;
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
                            if teeth == 0 {
                                gear = val;
                            } else {
                                gear *= val;
                            }
                            teeth += 1;
                            
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
                        if teeth == 2 {
                            index += 1;
                            continue;
                        }
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
                        if teeth == 0 {
                            gear = val;
                        } else {
                            gear *= val;
                        }
                        teeth += 1;
                    },
                    _ => {},
                }
                // LOWER MIDDLE
                if skip == 0 {
                    let scan_index : usize = scan_index + 1;
                    match bytes[scan_index] {
                        b'0'..=b'9' => {
                            if teeth == 2 {
                                index += 1;
                                continue;
                            }
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
                            if teeth == 0 {
                                gear = val;
                            } else {
                                gear *= val;
                            }
                            teeth += 1;

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
                            if teeth == 2 {
                                index += 1;
                                continue;
                            }
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
                            if teeth == 0 {
                                gear = val;
                            } else {
                                gear *= val;
                            }
                            teeth += 1;
                        },
                        _ => {},
                    }
                }


                // SEARCH LEFT 
                let mut scan_index : usize = index - 1;
                match bytes[scan_index] {
                    b'0'..=b'9' => {
                        if teeth == 2 {
                            index += 1;
                            continue;
                        }
                        let mut val : usize = (bytes[scan_index] - b'0') as usize;
                        let mut mult : usize = 10;
                        loop {
                            scan_index -= 1;
                            match bytes[scan_index] {
                                b'0'..=b'9' => {
                                    if teeth == 2 {
                                        index += 1;
                                        continue;
                                    }
                                    val += (bytes[scan_index] - b'0') as usize * mult;
                                    mult *= 10;
                                },
                                _ => {
                                    break;
                                },
                            }
                        }
                        if teeth == 0 {
                            gear = val;
                        } else {
                            gear *= val;
                        }
                        teeth += 1;
                    },
                    _ => {},
                }


                // SEARCH RIGHT
                let mut scan_index : usize = index + 1;
                match bytes[scan_index] {
                    b'0'..=b'9' => {
                        if teeth == 2 {
                            index += 1;
                            continue;
                        }
                        let mut val : usize = (bytes[scan_index] - b'0') as usize;
                        loop {
                            scan_index += 1;
                            match bytes[scan_index] {
                                b'0'..=b'9' => {
                                    val *= 10;
                                    val += (bytes[scan_index] - b'0') as usize;
                                },
                                _ => {
                                    break;
                                },
                            }
                        }
                        if teeth == 0 {
                            gear = val;
                        } else {
                            gear *= val;
                        }
                        teeth += 1;

                    },
                    _ => {},
                }
                if teeth == 2 {
                    sum += gear;
                }
            },
            _ => {},
        }
        index += 1
    }
    return sum.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let input = include_str!("test1.txt");
        let result = part2(input);
        assert_eq!(result, "467835".to_string());
    }
}
