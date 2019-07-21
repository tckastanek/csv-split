use std::str::Lines;

#[derive(Debug)]
pub struct CsvSplits<'a> {
    rem_lines: Lines<'a>,
    headers: &'a str,
    split_at_lines: usize,
}

impl<'a> CsvSplits<'a> {
    // TODO: Figure out if we could own this instead and consume the string
    pub fn new(file_string: &'a String, split_at_lines: usize) -> Self {
        let mut lines = file_string.lines();
        let headers = lines.next().unwrap();

        CsvSplits {
            rem_lines: lines,
            headers,
            split_at_lines,
        }
    }
}

impl<'a> Iterator for CsvSplits<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_string = String::new();

        for _ in 0..self.split_at_lines {
            match self.rem_lines.next() {
                None => (),
                Some(line_str) => {
                    if next_string.is_empty() {
                        // only add the headers if we have something to follow them
                        next_string.push_str(self.headers);
                        // Need to re append the stripped newline
                        next_string.push_str("\n");
                    }
                    next_string.push_str(line_str);
                    // Need to re append the stripped newline
                    next_string.push_str("\n");
                }
            }
        }

        if next_string.is_empty() {
            None
        } else {
            Some(next_string)
        }
    }
}
