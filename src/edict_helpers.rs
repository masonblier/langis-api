
/// splits a string by /, ignoring any / within ( ) parentheses
pub fn split_by_outer_slashes(text: &str) -> Vec<String> {
    // collects list of extracted part strings
    let mut collected_parts = Vec::<String>::new();
    // holds buffer of characters from current part
    let mut part_buf = String::new();
    // balanced parenthesis depth
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

    // fail if unbalanced
    if paren_count > 0 {
        panic!(format!("unbalanced parantheses in split_by_outer_slashes text: {}", text));
    }

    // collect last remaining part_buf
    if part_buf.len() > 0 {
        collected_parts.push(part_buf);
    }

    collected_parts
}

/// extracts ( ) groups while ignoring inner parentheses groups
/// returns remainder string with all ( ) groups removed
pub fn extract_outer_paren_groups(text: &str) -> (String, Vec<String>) {
    // holds array of extracted paren_group strings
    let mut collected_groups = Vec::<String>::new();
    // holds remainder string not containing groups text
    let mut rem_str = String::new();
    // holds buffer of matched characters from current group
    let mut group_buf = String::new();
    // balanced parenthesis depth
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
            // push character to group buf regardless
            group_buf.push(c);

            // if exited last outer paren, push and reset
            if paren_count == 0 {
                collected_groups.push(group_buf);
                group_buf = String::new();
            }

        // not within parentheses
        } else {
            // collect in rem_str
            rem_str.push(c);
        }
    }

    // fail if unbalanced
    if paren_count > 0 {
        panic!(format!("unbalanced parantheses in extract_outer_paren_groups text: {}", text));
    }

    // collect last remaining group_buf
    if group_buf.len() > 0 {
        collected_groups.push(group_buf);
    }

    (rem_str, collected_groups)
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

    #[test]
    fn test_extract_outer_paren_groups() {
        let input = "sim(pl/e) test";
        let (rem,groups) = extract_outer_paren_groups(input);
        assert_eq!(rem, "sim test");
        assert_eq!(groups, vec!["(pl/e)"]);


        let input = "multi(ple) group(s)";
        let (rem,groups) = extract_outer_paren_groups(input);
        assert_eq!(rem, "multi group");
        assert_eq!(groups, vec!["(ple)","(s)"]);

        let input = "ne(st)ing and と(some (unicode/ユニコード))";
        let (rem,groups) = extract_outer_paren_groups(input);
        assert_eq!(rem, "neing and と");
        assert_eq!(groups, vec!["(st)","(some (unicode/ユニコード))"]);
    }
}