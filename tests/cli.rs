#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn grep_test() {
        let mut result = Vec::new();
        let f = File::open("test.txt").expect("could not read file");
        let content = BufReader::new(f);

        unix_commands::grep(content, "10", &mut result);
        assert_eq!(result, b"10\n");
    }

    #[test]
    fn cat_test() {
        let mut result = Vec::new();
        let f = File::open("test.txt").expect("could not read file");
        let content = BufReader::new(f);

        unix_commands::cat(content, &mut result);
        assert_eq!(result, b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n");
    }
}
