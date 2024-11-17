#[derive(Debug)]
pub enum Tokens {
    HeadingOne(&'static str),
    HeadingTwo(&'static str),
    HeadingThree(&'static str),
    HeadingFour(&'static str),
    HeadingFive(&'static str),
    HeadingSix(&'static str),
    OrderedListNumber(&'static str),
    UnorderedListHyphen(&'static str),
    Tasklist(&'static str),
    Blockquote(&'static str),
    HorizontalRuleHyphen(&'static str),
    HorizontalRuleAsterisk(&'static str),
    HorizontalRuleUnderscore(&'static str),
    EmptyLine(&'static str),
    LineBreak(&'static str),
    Text(String),
}
