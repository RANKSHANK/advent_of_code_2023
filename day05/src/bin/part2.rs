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
    const SKIPS : [usize; 8] = [ 7, 20 , 24, 25, 20, 26, 29, 26];
    let bytes : &[u8] = input.as_bytes();
    let mut index : usize = SKIPS[0];
    let mut val : usize =  0;
    let mut seeds : Vec<[usize;2]> = Vec::new();
    'outer: loop {
        loop {
            let c = bytes[index];
            index += 1;
            match c {
                b' ' => {
                    seeds.push([val, 0]);
                    val = 0;
                    break;
                },
                _ => {
                    val *= 10;
                    val += (c - b'0') as usize;
                },
            }
        }
        let count = seeds.len() - 1;
        loop {
            let c = bytes[index];
            index += 1;
            match c {
                b' ' => {
                    seeds[count][1] = seeds[count][0] + val;
                    val = 0;
                    break;
                },
                b'\n' => {
                    seeds[count][1] = val;
                    break 'outer;
                },
                _ => {
                    val *= 10;
                    val += (c - b'0') as usize;
                },
            }
        }
    }
    let mut skip = 1;
    let mut inverter = true;
    loop {
        let mut inverted : Vec<bool> = vec!(!inverter; seeds.len());
        index += SKIPS[skip];
        loop {
            let mut range : usize = 0;
            let mut dest: usize = 0;
            let mut src: usize = 0;
            match bytes[index] {
                b'\n' => { break; },
                _ => {
                    loop {
                        let c = bytes[index];
                        index += 1;
                        match c {
                            b' ' => {
                                break;
                            },
                            _ => {
                                dest *= 10;
                                dest += (c - b'0') as usize;
                            },
                        }
                    }
                    loop {
                        let c = bytes[index];
                        index += 1;
                        match c {
                            b' ' => {
                                break;
                            },
                            _ => {
                                src *= 10;
                                src += (c - b'0') as usize;
                            },
                        }
                    }
                    loop {
                        let c = bytes[index];
                        index += 1;
                        match c {
                            b'\n' => {
                                break;
                            },
                            _ => {
                                range *= 10;
                                range += (c - b'0') as usize;
                            },
                        }
                    }
                    let max : usize = src + range;
                    let len : usize = inverted.len() - 1;
                    for idx in 0..len {
                        if inverted[idx] != inverter {
                            let init_low : usize = seeds[idx][0];
                            let init_high : usize = seeds[idx][1];
                            if init_low < max && init_high >= src {
                                if init_high > max {
                                    seeds[idx][1] = max - 1;
                                    seeds.push([max, init_high]);
                                    inverted.push(!inverter);
                                } 
                                if init_low < src {
                                    seeds[idx][0] = src;
                                    seeds.push([init_low, src - 1]);
                                    inverted.push(!inverter);
                                }
                                seeds[idx][0] = (dest as isize + seeds[idx][0] as isize - src as isize) as usize;
                                seeds[idx][1] = (dest as isize + seeds[idx][1] as isize - src as isize) as usize;
                                inverted[idx] = inverter;
                            }
                        }
                    }
                },
            }
        }
        if skip == 7 {
            break;
        }
        inverter = !inverter;
        index += 1;
        skip += 1;
    }
    let mut lowest : usize = usize::MAX;
    seeds.iter().for_each(|seed| if seed[0] < lowest { lowest = seed[0]; });
    return lowest.to_string();
    
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let input = include_str!("test1.txt");
        assert_eq!(part2(input), "46");
    }

}
