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
    const SKIP : usize = 11;
    let mut product : usize = 1;
    let mut time_index: usize = SKIP;
    let mut dist_index: usize = SKIP + bytes.len() / 2; 
    let time_char : u8 = bytes[time_index];
    let mut time_val : usize = match time_char {
        b' ' => 0,
        _ => (time_char - b'0') as usize,
    };
    let dist_char : u8 = bytes[dist_index];
    let mut dist_val : usize = match dist_char {
        b' ' => 0,
        _ => (dist_char - b'0') as usize,
    };
    time_index += 1;
    dist_index += 1;
    loop {
        let time_char : u8 = bytes[time_index];
        match time_char {
            b' ' => {
            },
            b'\n' => {
              break;  
            },
            _ => {
                time_val *= 10;
                time_val += (time_char - b'0') as usize;
            },
        }
        let dist_char : u8 = bytes[dist_index];
        if dist_char != b' ' {
            dist_val *= 10;
            dist_val += (dist_char - b'0') as usize;
        }
        time_index += 1;
        dist_index += 1;
    }
    let time_val : f64 = time_val as f64;
    let dist_val : f64 = dist_val as f64;
    let discriminant : f64 = time_val * time_val - 4.0 * dist_val;
    let discriminant : f64 = discriminant.sqrt();
    let roots: [f64; 2] = [
        ((time_val - discriminant) * 0.5).floor(),
        ((time_val + discriminant) * 0.5).ceil() - 1.0,
    ];
    let val : usize = (roots[1] - roots[0]) as usize;
    product *= val;
    return product.to_string();
    
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let input = include_str!("test1.txt");
        assert_eq!(part2(input), "71503");
    }

}
