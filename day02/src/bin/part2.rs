
fn main () {
    let input = include_str!("input1.txt");
    dbg!(part2(input));
}

enum Mode {
    GAME,
    COLOR,
}

fn part2(input: &str) -> String {
    let zero : u32 = '0' as u32;
    let mut mode : Mode = Mode::GAME;
    let mut val : u32 = 0;
    let mut success : u32 = 0; 
    let mut r : u32 = 0;
    let mut g : u32 = 0;
    let mut b : u32 = 0;
    for c in input.chars() {
        if c >= '0' && c <= '9' {
            val = val * 10 + (c as u32) - zero;
            continue;
        }
        match mode {
            Mode::GAME => {
                match c {
                    ':' => {
                        mode = Mode::COLOR;
                        r = 0;
                        g = 0;
                        b = 0;
                        val = 0;
                    },
                    _ => {
                    },
                }
                continue;
            }
            Mode::COLOR => {
                match c {
                    'r' => {
                        if val > r {
                            r = val;
                        }
                        val = 0;
                    },
                    'g' => {
                        if val > g {
                            g = val;
                        }
                        val = 0;
                    },
                    'b' => {
                        if val > b {
                            b = val;
                        }
                        val = 0;
                    },
                    ' ' => {},
                    '\n' => {
                        success += r * g * b;
                        mode = Mode::GAME;
                    },
                    '\0' => {
                        success += r * g * b;
                        break;
                    },
                    ',' => {
                        val = 0;
                    },
                    ';' => {
                        val = 0;
                    },
                    _ => {
                    },
                }
                continue;
            }
        }
    }

        success.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let input = include_str!("test1.txt");
        let result = part2(input);
        assert_eq!(result, "8".to_string());
    }
}
