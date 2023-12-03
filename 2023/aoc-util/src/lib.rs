use std::fs;

pub fn load_input(path: &str) -> Vec<String> {
    let contents_res = fs::read_to_string(path);
    let contents = match contents_res {
        Ok(c) => c,
        Err(e) => panic!("Failed to open {path}: {e}"),
    };

    contents.split("\n").map(|s| s.to_string()).collect()
}
