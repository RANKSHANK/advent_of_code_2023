fn main() {
    let input = include_str!("test1.txt");
    println!("Test Sum: {}", part1(input));
    let input = include_str!("input1.txt");
    println!("Sum: {}", part1(input));
    let start = std::time::Instant::now();
    for _ in 0..9999 {
        part1(input);
    }
    println!("Avg run time of {:?}", start.elapsed() / 10000);
}

fn part1(input : &str) -> String{
    const SKIPS : [usize; 8] = [ 7, 20 , 24, 25, 20, 26, 29, 26];
    let bytes : &[u8] = input.as_bytes();
    let mut index : usize = SKIPS[0];
    let mut val : usize =  0;
    let mut seeds : Vec<usize> = Vec::new();
    loop {
        let c = bytes[index];
        match c {
            b' ' => {
                seeds.push(val);
                val = 0;
            },
            b'\n' => {
                seeds.push(val);
                break;
            },
            _ => {
                val *= 10;
                val += (c - b'0') as usize;
            },
        }
        index += 1;
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
                    for idx in 0..seeds.len() {
                        if inverted[idx] != inverter {
                            let seed : usize = seeds[idx];
                            if seed < max && seed >= src {
                                inverted[idx] = inverter;
                                seeds[idx] = (dest as isize + seed as isize - src as isize) as usize;
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
    
    return match seeds.iter().min() {
        Some(seed) => seed.to_string(),
        None => "0".to_string(),
    };
    
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("test1.txt");
        assert_eq!(part1(input), "35");
    }

}
