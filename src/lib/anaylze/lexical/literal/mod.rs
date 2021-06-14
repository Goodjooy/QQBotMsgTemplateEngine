use super::{PreviewableIter, check_next_sign, clear_space};
#[derive(Debug)]
pub struct Literal(String);

impl Literal {
    pub fn read_next(data: &mut PreviewableIter) -> Option<Literal> {
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
mod literal{
    use super::*;

    #[test]
    fn test_read_literal() {
        let mut data=PreviewableIter::new(r#"1+1=2?"对的"<sign s="<">"#);
        let res=Literal::read_next(&mut data).unwrap();

        assert_eq!(res.0,r#"1+1=2?"对的""#);
        assert_eq!(data.next().unwrap(),'<');


    }
}