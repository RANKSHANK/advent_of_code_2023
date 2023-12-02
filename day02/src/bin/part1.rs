fn main () {
    let input = include_str!("input1.txt");
    dbg!(part1(input, 12, 13, 14));
}

enum Mode {
    GAME,
    COLOR,
}

fn part1(input: &str, red: u32, green: u32, blue: u32) -> String {
    let zero : u32 = '0' as u32;
    let mut mode : Mode = Mode::GAME;
    let mut val : u32 = 0;
    let mut game : u32 = 0;
    let mut success : u32 = 0; 
    for c in input.chars() {
        if c >= '0' && c <= '9' {
            val = val * 10 + (c as u32) - zero;
            continue;
        }
        match mode {
            Mode::GAME => {
                match c {
                    ':' => {
                        game = val;
                        mode = Mode::COLOR;
                        val = 0;
                    },
                    _ => {
                        val = 0;
                    },
                }
                continue;
            }
            Mode::COLOR => {
                match c {
                    'r' => {
                        if val > red {
                            mode = Mode::GAME;
                        }
                        val = 0;
                    },
                    'g' => {
                        if val > green {
                            mode = Mode::GAME;
                        }
                        val = 0;
                    },
                    'b' => {
                        if val > blue {
                            mode = Mode::GAME;
                        }
                        val = 0;
                    },
                    ' ' => {},
                    '\n' => {
                        success += game;
                        mode = Mode::GAME;
                    },
                    '\0' => {
                        success += game;
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
    fn test1() {
        let input = include_str!("test1.txt");
        let result = part1(input, 12, 13, 14,);
        assert_eq!(result, "8".to_string());
    }
}
