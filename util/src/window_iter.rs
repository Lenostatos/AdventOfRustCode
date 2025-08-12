pub struct WindowStrIter<'a> {
    str: &'a str,
    window_pos: usize,
    window_size: usize,
}

impl<'a> WindowStrIter<'a> {
    fn new(str: &'a str, window_size: usize) -> Self {
        if window_size > str.len() {
            panic!("window size should not be bigger than the string it's moved over");
        }
        WindowStrIter {
            str,
            window_pos: 0,
            window_size,
        }
    }
}

impl<'a> Iterator for WindowStrIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.window_pos += 1;

        // The `- 1` bit here is dealing with the fact that the iterator's initial window position can't be set to -1 since it is an unsigned integer.
        let window_start = self.window_pos - 1;
        let window_end = window_start + self.window_size;

        if window_end > self.str.len() {
            return None;
        } else {
            return Some(&self.str[window_start..window_end]);
        }
    }
}

pub struct WindowStrIntoIter<'a> {
    str: &'a str,
    window_size: usize,
}

impl<'a> WindowStrIntoIter<'a> {
    pub fn new(str: &'a str, window_size: usize) -> Self {
        WindowStrIntoIter { str, window_size }
    }
}

impl<'a> IntoIterator for WindowStrIntoIter<'a> {
    type Item = &'a str;

    type IntoIter = WindowStrIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        WindowStrIter::new(self.str, self.window_size)
    }
}
