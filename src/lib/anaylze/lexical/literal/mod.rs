use crate::lib::anaylze::LoadNext;

use super::{check_next_sign, clear_space, PreviewableIter};
#[derive(Debug)]
pub struct Literal(String);

impl LoadNext<Literal> for Literal {
    fn load_next(data: &mut PreviewableIter) -> Option<Literal> {
        let mut literal = String::new();
        loop {
            clear_space(data)?;
            match check_next_sign('<', false, data) {
                Some(is) => {
                    if is {
                        break Some(Literal(literal));
                    } else {
                        literal.push(data.next()?);
                    }
                }
                None => break Some(Literal(literal)),
            }
        }
    }
}

#[cfg(test)]
mod literal {
    use super::*;

    #[test]
    fn test_read_literal() {
        let mut data = PreviewableIter::new(r#"1+1=2?"对的"<sign s="<">"#);
        let res = Literal::load_next(&mut data).unwrap();

        assert_eq!(res.0, r#"1+1=2?"对的""#);
        assert_eq!(data.next().unwrap(), '<');
    }
}
