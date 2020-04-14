
/// splits a string by /, ignoring any / within ( ) parentheses 
pub fn split_by_outer_slashes(text: &str) -> Vec<String> {
    let mut collected_parts = Vec::<String>::new();
    let mut part_buf = String::new();
    let mut paren_count = 0;

    // for each char in text
    for c in text.chars() {
        // incr paren_count if paren detected
        if c == '(' {
            paren_count += 1;
        }

        // if we are in a set of parentheses
        if paren_count >= 1 {
            if c == ')' {
                paren_count -= 1;
            }
            part_buf.push(c);

        // not within parentheses
        } else {
            if c == '/' {
                // split by /
                collected_parts.push(part_buf);
                part_buf = String::new();
            } else {
                part_buf.push(c);
            }
        }
    }

    // collect last remaining part_buf
    if part_buf.len() > 0 {
        collected_parts.push(part_buf);
    }

    collected_parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_by_outer_slashes() {
        let input = "a/sim(pl/e)/test";
        let output = split_by_outer_slashes(input);
        assert_eq!(output, vec!["a","sim(pl/e)","test"]);


        let input = "a test/with (some (unicode/ユニコード))";
        let output = split_by_outer_slashes(input);
        assert_eq!(output, vec!["a test","with (some (unicode/ユニコード))"]);
    }
}