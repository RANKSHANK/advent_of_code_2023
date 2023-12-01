fn main() {
    let input = include_str!("input1.txt");
    dbg!(part1(input));

}

fn part1(input: &str) -> String {
    let mut accumulator : i32 = 0;
    let mut first : i32 = -1;
    let mut last : i32 = -1;

    for c in input.chars() {
        if c >= '0' && c <= '9' {
            if first == -1 {
                first = c.to_digit(10).unwrap() as i32;
                continue;
            }  
            last = c.to_digit(10).unwrap() as i32;
            continue;
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

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test1() {
        let result = part1(
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, "142".to_string());
    }

    #[test]
    fn test2() { // forgetting \n followed by EOF got hands
        let input = include_str!("test2.txt");
        let result = part1(input);
        assert_eq!(result, "142".to_string());
    }

}
