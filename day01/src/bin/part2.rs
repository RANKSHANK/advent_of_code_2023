fn main() {
    let input = include_str!("input1.txt");
    dbg!(part2(input));
}

fn part2(input: &str) -> String {
    let mut accumulator : i32 = 0;
    let mut first : i32 = -1;
    let mut last : i32 = -1;
    let mut word : u32 = 0;
    // cache 'a' as u32 since it gets used so much. Not sure if necessary
    let a : u32 = 'a' as u32;
    // Numbers are packed into u32s to be used as a hash array
    // For speed the loop would get unrolled and the hashes would be hardcoded
    let hashes : [[(u32, i32); 3]; 3] = [
        [ (pack_chars("three"), 3), (pack_chars("seven"), 7), (pack_chars("eight"), 8), ],
        [ (pack_chars("four"), 4), (pack_chars("five"), 5), (pack_chars("nine"), 9) ],
        [ (pack_chars("one"), 1), (pack_chars("six"), 6), (pack_chars("two"), 2), ],
    ];
    // The smallest hash is one, due to the number of chars and alphabetical order
    let min_word : u32 = hashes[2][0].0 - 1;
    // Parse through each char in the string, need to look for a fgetc equivalent
    'main: for c in input.chars() {
        // Check for numerical characters
        if c >= '0' && c <= '9' {
            // Resets the word set since numeric chars aren't allowed
            word = 0;
            // Ensure the first number is set before setting the last number
            if first == -1 {
                first = c.to_digit(10).unwrap() as i32;
                continue;
            }  
            last = c.to_digit(10).unwrap() as i32;
            continue;
        }
        // Check for letter chars
        if c >= 'a' && c <= 'z' {
            // shifts the word left by 5 and appends the char to the rhs
            word = (word << 5) | (c as u32 - a);
            // Skip instances where the word is smaller than the minimum hash
            if word > min_word {
                // mask for removing the extra bits
                let mut mask : u32 = 0b00000001111111111111111111111111;
                // u32 representing the masked word
                let mut check : u32 = word & mask;
                // loop through the hashes separated by no. char levels
                for level in hashes {
                    // check the hash at each level
                    for pair in level {
                        // Check if the hash matches
                        if check == pair.0 {
                            // Set the correct value and exit to the main loop
                            if first == -1 {
                                first = pair.1;
                                continue 'main;
                            }  
                            last = pair.1;
                            continue 'main;
                        }
                    }
                    // Shift the mask over by 1 char width (5 bits)
                    mask >>= 5;
                    // Mask the check hash
                    check &= mask;
                    // break out of the loop early when the hash falls below min
                    if check < min_word {
                        break;
                    }
                }
                continue;
            }
        }
        // On EOF or new line add the first and last vals to the accumulator
        if c == '\n' || c == '\0' {
            // Add first & last to the accumulator, taking into account single
            // digit lines
            if last == -1 {
                accumulator += (first * 10 ) + first;
            } else {
                accumulator += (first * 10) + last;
            }
            // Reset loop vars to their initial states
            word = 0;
            first = -1;
            last = -1;
        }
    }
    return accumulator.to_string();
}

// Takes a string and hashes it into a u32
// Max string length is 6, numbers max leng is 5 so no need for error handling
fn pack_chars(word: &str) -> u32 {
    let mut accumulator: u32 = 0;
    for c in word.chars() {
        // Shift the accumulator left by char width (5) and append the char to rhs
        accumulator = accumulator << 5 | (c as u32 - 'a' as u32);
    }
    return accumulator;
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
