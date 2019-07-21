use std::str::Lines;

#[derive(Debug)]
pub struct CsvSplits<'a> {
    rem_lines: Lines<'a>,
    headers: &'a str,
    split_at_lines: usize,
    file_name: FileNames<'a>,
}

impl<'a> CsvSplits<'a> {
    // TODO: Figure out if we could own this instead and consume the string
    pub fn new(file_string: &'a String, split_at_lines: usize, prefix: &'a str) -> Self {
        let mut lines = file_string.lines();
        let headers = lines.next().unwrap();

        CsvSplits {
            rem_lines: lines,
            headers,
            split_at_lines,
            file_name: FileNames::new(prefix),
        }
    }
}

impl<'a> Iterator for CsvSplits<'_> {
    // (file_data_string, file_name_string)
    type Item = (String, String);

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
            // only ever returns some...until re run out of usize
            let file_name_string = self.file_name.next().unwrap();
            Some((next_string, file_name_string))
        }
    }
}

// TODO: Eventually implement the more complex aa, ab, ac pattern `split` uses
#[derive(Debug)]
struct FileNames<'a> {
    prefix: &'a str,
    file_number: usize,
}

impl<'a> FileNames<'a> {
    fn new(prefix: &'a str) -> Self {
        FileNames { prefix, file_number: 1 }
    }
}

impl Iterator for FileNames<'_> {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        let next_file_name = format!("{}_{}", self.prefix, self.file_number);
        self.file_number = self.file_number + 1;
        
        // I suppose we _could_ check for overflow
        Some(next_file_name)
    }
}
