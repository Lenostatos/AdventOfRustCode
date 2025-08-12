pub mod grid;
pub mod window_iter;

pub mod input {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::path::Path;

    pub fn lines_of<P: AsRef<Path>>(path: P) -> impl Iterator<Item = String> {
        let f = File::open(path).unwrap();
        let f = BufReader::new(f);

        f.lines().map(|l| l.unwrap())
    }
}
