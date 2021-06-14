use super::{literal::Literal, tag::Tag, LexicalHandle, LexicalType, PreviewableIter};

impl LexicalHandle<'_> {
    fn new(data: &str) -> LexicalHandle {
        LexicalHandle {
            data: PreviewableIter::new(data),
        }
    }

    fn load_next(&mut self) -> Option<LexicalType> {
        Tag::load_next(&mut self.data)
            .and_then(|f| Some(LexicalType::Tag(f)))
            .or_else(|| {
                Literal::read_next(&mut self.data).and_then(|f| Some(LexicalType::Literal(f)))
            })
    }
}

impl Iterator for LexicalHandle<'_> {
    type Item = LexicalType;

    fn next(&mut self) -> Option<Self::Item> {
        self.load_next()
    }
}

#[cfg(test)]
mod lextical {
    use super::*;

    #[test]
    fn tast_load() {
        let mut lex = LexicalHandle::new(
            r#"<text line="true">这是自动添加换行符结尾的文本行</text>
        <text>这是没有自动添加换行符结尾的文本行<endl/></text>
        <? 这里面是注释 ?>
        <text>
        内部格式会被忽略，除非使用符号标记: 换行<sign s="\n"/>
        或者<endl/>也能换行
        符号可以是任意符号，比如<sign s="<">
        可以是任何可能会被识别为标记的符号或者字符，长度不限<endl/>
        可以设置复读次数
        <endl/><sign s="-" repeat="6"/><endl/>
        长度为6的分割线
        </text>"#,
        );

        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
        println!("{:?}", lex.next());
    }
}
