use super::PreviewableIter;

pub fn clear_space(data: &mut PreviewableIter) -> Option<()> {
    loop {
        match data.preview() {
            Some(ch) => {
                if !ch.is_whitespace() {
                    break Some(());
                } else {
                    data.next()?;
                }
            }
            None => break None,
        }
    }
}

pub fn check_next_sign(sign: char, moving: bool, data: &mut PreviewableIter) -> Option<bool> {
    let next = data.preview()?;
    if next == sign && moving {
        data.next()?;
    }
    Some(next == sign)
}
