pub fn get_dict_line<'a>(contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
            results.push(line);
    }

    return results;
}