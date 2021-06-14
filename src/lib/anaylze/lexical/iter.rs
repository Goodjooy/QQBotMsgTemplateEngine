use super::PreviewableIter;

impl Iterator for PreviewableIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.preview;
        self.preview = self.iter.next().or(Some('\0'))?;
        Self::preview_check(temp)
    }
}
impl<'a> PreviewableIter<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut t = PreviewableIter {
            preview: '\0',
            iter: data.chars(),
        };
        t.next();
        t
    }

    pub fn preview(&self) -> Option<char> {
        Self::preview_check(self.preview)
    }
    pub fn preview_check(data: char) -> Option<char> {
        if data == '\0' {
            None
        } else {
            Some(data)
        }
    }
}