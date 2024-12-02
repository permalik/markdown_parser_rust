use crate::elements::{literals, structs::Token, tokens::Tokens};
use crate::utils::utils::string_to_static_str;
use regex::Regex;

pub fn lex(line_number: usize, line: &str, tokens: &mut Vec<Token>) {
    match line.chars().nth(0) {
        None => {
            let empty_line_literal = literals::EMPTY_LINE;
            if let Tokens::EmptyLine(empty_line) = empty_line_literal {
                if line == empty_line {
                    tokens.push(Token {
                        line_number,
                        name: "empty_line".to_string(),
                        kind: Tokens::EmptyLine(""),
                        value: "".to_string(),
                    });
                }
            }
        }
        Some(c) => match c {
            '#' => {
                fn verify_heading(level: usize, literal: Tokens) {
                    let expected = format!("{} ", &"#".repeat(level));
                    match literal {
                        Tokens::HeadingOne(h) => assert_eq!(h, expected),
                        Tokens::HeadingTwo(h) => assert_eq!(h, expected),
                        Tokens::HeadingThree(h) => assert_eq!(h, expected),
                        Tokens::HeadingFour(h) => assert_eq!(h, expected),
                        Tokens::HeadingFive(h) => assert_eq!(h, expected),
                        Tokens::HeadingSix(h) => assert_eq!(h, expected),
                        _ => unreachable!(),
                    }
                }

                let mut heading_level = 1;
                while heading_level < line.len() && line.chars().nth(heading_level) == Some('#') {
                    heading_level += 1;
                }

                if heading_level <= 6 {
                    let (token_kind, literal) = match heading_level {
                        1 => (Tokens::HeadingOne("# "), literals::HEADING_ONE),
                        2 => (Tokens::HeadingTwo("## "), literals::HEADING_TWO),
                        3 => (Tokens::HeadingThree("### "), literals::HEADING_THREE),
                        4 => (Tokens::HeadingFour("#### "), literals::HEADING_FOUR),
                        5 => (Tokens::HeadingFive("##### "), literals::HEADING_FIVE),
                        6 => (Tokens::HeadingSix("######"), literals::HEADING_SIX),
                        _ => unreachable!(),
                    };

                    verify_heading(heading_level, literal);

                    let heading_text = lex_element_text(line, heading_level);

                    tokens.push(Token {
                        line_number,
                        name: format!("h{}", heading_level),
                        kind: token_kind,
                        value: format!("{} {}", "#".repeat(heading_level), heading_text),
                    });
                }
            }
            '0'..='9' => {
                if line.len() > 3
                    && line.chars().nth(1) == Some('.')
                    && line.chars().nth(2) == Some(' ')
                {
                    let ordered_list_number_literal = literals::ORDERED_LIST_NUMBER;
                    if let Tokens::OrderedListNumber(ordered_list_number) =
                        ordered_list_number_literal
                    {
                        let re = Regex::new(ordered_list_number).expect("invalid regex");
                        re.is_match("8. ");
                    }

                    let ordered_list_number = line.chars().nth(0);
                    let ordered_list_text = lex_element_text(line, 3);

                    tokens.push(Token {
                        line_number,
                        name: "Ordered List".to_string(),
                        kind: Tokens::OrderedListNumber(string_to_static_str(format!(
                            "{}. ",
                            ordered_list_number.unwrap()
                        ))),
                        value: format!("{}. {}", ordered_list_number.unwrap(), ordered_list_text),
                    });
                } else {
                    lex_text(line_number, line, tokens);
                }
            }
            '-' => {
                let mut horizontal_rule_hyphen_level = 0;
                while horizontal_rule_hyphen_level < line.len()
                    && line.chars().nth(horizontal_rule_hyphen_level) == Some('-')
                {
                    horizontal_rule_hyphen_level += 1;
                }

                if horizontal_rule_hyphen_level == 3 && line.len() == 3 {
                    let horizontal_rule_hyphen_literal = literals::HORIZONTAL_RULE_HYPHEN;
                    if let Tokens::HorizontalRuleHyphen(horizontal_rule_hyphen) =
                        horizontal_rule_hyphen_literal
                    {
                        assert_eq!(horizontal_rule_hyphen, "---");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "horizontal_rule_hyphen".to_string(),
                        kind: Tokens::HorizontalRuleHyphen("---"),
                        value: format!("---"),
                    });
                    return;
                }
                let mut task_list_index = 0;
                let mut is_task_list = false;
                if line.chars().nth(task_list_index) == Some('-') {
                    task_list_index += 1;
                    if line.chars().nth(task_list_index) == Some(' ') {
                        task_list_index += 1;
                        if line.chars().nth(task_list_index) == Some('[') {
                            task_list_index += 1;
                            if line.chars().nth(task_list_index) == Some(' ') {
                                task_list_index += 1;
                                if line.chars().nth(task_list_index) == Some(']') {
                                    task_list_index += 1;
                                    if line.chars().nth(task_list_index) == Some(' ') {
                                        is_task_list = true;
                                    }
                                }
                            }
                        }
                    }
                }

                if is_task_list {
                    let task_list_literal = literals::TASK_LIST;
                    if let Tokens::TaskList(task_list) = task_list_literal {
                        assert_eq!(task_list, "- [ ] ");
                    }

                    let line_text = String::from(line);
                    let task_list_text = &line_text[6..line_text.len()];

                    tokens.push(Token {
                        line_number,
                        name: "task_list".to_string(),
                        kind: Tokens::TaskList("- [ ] "),
                        value: format!("{}{}", "- [ ] ".to_string(), task_list_text),
                    });
                    return;
                }

                let unordered_list_hyphen_literal = literals::UNORDERED_LIST_HYPHEN;
                if let Tokens::UnorderedListHyphen(unordered_list_hyphen) =
                    unordered_list_hyphen_literal
                {
                    assert_eq!(unordered_list_hyphen, "- ");
                }

                let line_text = String::from(line);
                let unordered_list_text = &line_text[2..line_text.len()];

                tokens.push(Token {
                    line_number,
                    name: "unordered_list_hyphen".to_string(),
                    kind: Tokens::UnorderedListHyphen("- "),
                    value: format!("{}{}", "- ".to_string(), unordered_list_text),
                });
                return;
            }
            '*' => {
                let mut horizontal_rule_asterisk_level = 0;
                while horizontal_rule_asterisk_level < line.len()
                    && line.chars().nth(horizontal_rule_asterisk_level) == Some('*')
                {
                    horizontal_rule_asterisk_level += 1;
                }

                if horizontal_rule_asterisk_level == 3 && line.len() == 3 {
                    let horizontal_rule_asterisk_literal = literals::HORIZONTAL_RULE_ASTERISK;
                    if let Tokens::HorizontalRuleAsterisk(horizontal_rule_asterisk) =
                        horizontal_rule_asterisk_literal
                    {
                        assert_eq!(horizontal_rule_asterisk, "***");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "horizontal_rule_asterisk".to_string(),
                        kind: Tokens::HorizontalRuleAsterisk("***"),
                        value: format!("***"),
                    });
                }
            }
            '_' => {
                let mut horizontal_rule_underscore_level = 0;
                while horizontal_rule_underscore_level < line.len()
                    && line.chars().nth(horizontal_rule_underscore_level) == Some('_')
                {
                    horizontal_rule_underscore_level += 1;
                }
                if horizontal_rule_underscore_level == 3 && line.len() == 3 {
                    let horizontal_rule_underscore_literal = literals::HORIZONTAL_RULE_UNDERSCORE;
                    if let Tokens::HorizontalRuleUnderscore(horizontal_rule_underscore) =
                        horizontal_rule_underscore_literal
                    {
                        assert_eq!(horizontal_rule_underscore, "___");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "horizontal_rule_underscore".to_string(),
                        kind: Tokens::HorizontalRuleUnderscore("___"),
                        value: format!("___"),
                    });
                }
            }
            ':' => {
                if line.chars().nth(1) == Some(' ') {
                    let definition_list_literal = literals::DEFINITION_LIST;
                    if let Tokens::DefinitionList(definition_list) = definition_list_literal {
                        assert_eq!(definition_list, ": ");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "definition_list".to_string(),
                        kind: Tokens::DefinitionList(": "),
                        value: format!(": "),
                    });
                }
            }
            '>' => {
                let blockquote_level = 0;
                if line.chars().nth(blockquote_level + 1) == Some(' ') {
                    let blockquote_literal = literals::BLOCKQUOTE;
                    if let Tokens::Blockquote(blockquote) = blockquote_literal {
                        assert_eq!(blockquote, "> ");
                    }

                    tokens.push(Token {
                        line_number,
                        name: "blockquote".to_string(),
                        kind: Tokens::Blockquote("> "),
                        value: format!("> "),
                    });
                }
            }
            '`' => {
                let c_block_literal = literals::C_BLOCK;
                if let Tokens::CBlock(c_block) = c_block_literal {
                    if line == c_block {
                        tokens.push(Token {
                            line_number,
                            name: "c_block".to_string(),
                            kind: Tokens::CBlock(c_block),
                            value: c_block.to_string(),
                        });
                        return;
                    }
                }

                let markdown_block_literal = literals::MARKDOWN_BLOCK;
                if let Tokens::MarkdownBlock(markdown_block) = markdown_block_literal {
                    if line == markdown_block {
                        tokens.push(Token {
                            line_number,
                            name: "markdown_block".to_string(),
                            kind: Tokens::MarkdownBlock(markdown_block),
                            value: markdown_block.to_string(),
                        });
                        return;
                    }
                }

                let code_block_literal = literals::CODE_BLOCK;
                if let Tokens::CodeBlock(code_block) = code_block_literal {
                    if line == code_block {
                        tokens.push(Token {
                            line_number,
                            name: "code_block".to_string(),
                            kind: Tokens::CodeBlock(code_block),
                            value: code_block.to_string(),
                        });
                        return;
                    }
                }

                if line.starts_with("```") {
                    tokens.push(Token {
                        line_number,
                        name: "general_code_block".to_string(),
                        kind: Tokens::CodeBlock("```"),
                        value: line.to_string(),
                    });
                    return;
                }
            }
            _ => {
                let mut is_line_break = false;
                if line.len() >= 2 {
                    let first_space_position = line.len() - 1;
                    let second_space_position = line.len() - 2;
                    if line.chars().nth(second_space_position) == Some(' ')
                        && line.chars().nth(first_space_position) == Some(' ')
                    {
                        let line_break_literal = literals::LINE_BREAK;
                        if let Tokens::LineBreak(line_break) = line_break_literal {
                            assert_eq!(line_break, "  ");
                        }

                        is_line_break = true;
                    }
                }

                if is_line_break {
                    let line_text = String::from(line);
                    let line_break_text_len = line_text.len() - 2;
                    let line_break_text = &line_text[0..line_break_text_len];

                    tokens.push(Token {
                        line_number,
                        name: "text".to_string(),
                        kind: Tokens::Text(String::from(line_break_text)),
                        value: String::from(line_break_text),
                    });

                    tokens.push(Token {
                        line_number,
                        name: "line_break".to_string(),
                        kind: Tokens::LineBreak("  "),
                        value: format!("  "),
                    });
                } else {
                    lex_text(line_number, line, tokens);
                }
            }
        },
    }
}

fn lex_element_text(line: &str, heading_level: usize) -> &str {
    &line[heading_level..]
}

fn lex_text(line_number: usize, line: &str, tokens: &mut Vec<Token>) {
    tokens.push(Token {
        line_number,
        name: "text".to_string(),
        kind: Tokens::Text(String::from(line)),
        value: String::from(line),
    });
}
