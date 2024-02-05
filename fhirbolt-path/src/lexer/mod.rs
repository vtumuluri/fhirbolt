use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, take_while, take_while1, take_while_m_n},
    character::complete::char,
    combinator::{map, map_opt, map_res, value},
    sequence::{delimited, pair, preceded},
    IResult,
};

fn parse_unicode<'a>(input: &'a str) -> IResult<&'a str, char> {
    map_opt(
        map_res(
            preceded(
                char('u'),
                take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit()),
            ),
            move |hex| u32::from_str_radix(hex, 16),
        ),
        char::from_u32,
    )(input)
}

fn parse_escape<'a>(input: &'a str) -> IResult<&'a str, char> {
    preceded(
        char('\\'),
        alt((
            parse_unicode,
            value('\u{0C}', char('f')),
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('\'', char('\'')),
            value('`', char('`')),
        )),
    )(input)
}

fn parse_delimited<'a>(input: &'a str, delim: char) -> IResult<&'a str, String> {
    alt((
        delimited(
            char(delim),
            escaped_transform(
                take_while1(|c: char| c != delim && c != '\\'),
                '\\',
                alt((
                    parse_unicode,
                    value('\u{0C}', char('f')),
                    value('\n', char('n')),
                    value('\r', char('r')),
                    value('\t', char('t')),
                    value('\\', char('\\')),
                    value('/', char('/')),
                    value('\'', char('\'')),
                    value('`', char('`')),
                )),
            ),
            char(delim),
        ),
        map(preceded(char(delim), char(delim)), |_| String::from("")),
    ))(input)
}

// STRING
// : '\'' (ESC | .)*? '\''
// ;
fn parse_string(input: &str) -> IResult<&str, String> {
    parse_delimited(input, '\'')
}

// DELIMITEDIDENTIFIER
// : '`' (ESC | .)*? '`'
// ;
fn parse_delimited_identifier(input: &str) -> IResult<&str, String> {
    parse_delimited(input, '`')
}

// IDENTIFIER
// : ([A-Za-z] | '_')([A-Za-z0-9] | '_')*            // Added _ to support CQL (FHIR could constrain it out)
// ;
fn parse_id<'a>(input: &'a str) -> IResult<&'a str, String> {
    map(
        pair(
            take_while_m_n(1, 1, |c: char| c.is_ascii_alphabetic() || c == '_'),
            take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
        ),
        |tup| String::from(tup.0) + tup.1,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode() {
        assert_eq!(parse_unicode("uFF3FHello"), Ok(("Hello", '\u{FF3F}')));

        //Copy test cases from firely-net-sdk
        assert_eq!(parse_unicode("uface"), Ok(("", '\u{face}')));
        assert_eq!(parse_unicode("u0000"), Ok(("", '\u{0000}')));
        assert_eq!(parse_unicode("u09af"), Ok(("", '\u{09af}')));
        assert_eq!(parse_unicode("uffff"), Ok(("", '\u{ffff}')));

        assert!(parse_unicode("u").is_err());
        assert!(parse_unicode("u0").is_err());
        assert!(parse_unicode("u00").is_err());
        assert!(parse_unicode("u000").is_err());
        // TODO: As per Firely, this should fail but I dont see why
        assert_eq!(parse_unicode("u00000"), Ok(("0", '\x00')));
        assert!(parse_unicode("ugggg").is_err());
    }

    #[test]
    fn test_escape() {
        //Copy test cases from firely-net-sdk
        assert_eq!(parse_escape(r#"\uface"#), Ok(("", '龜')));
        assert_eq!(parse_escape(r#"\'"#), Ok(("", '\'')));
        assert_eq!(parse_escape(r#"\`"#), Ok(("", '`')));
        assert_eq!(parse_escape(r#"\\"#), Ok(("", '\\')));
        assert_eq!(parse_escape(r#"\/"#), Ok(("", '/')));
        assert_eq!(parse_escape(r#"\f"#), Ok(("", '\x0C')));
        assert_eq!(parse_escape(r#"\n"#), Ok(("", '\n')));
        assert_eq!(parse_escape(r#"\r"#), Ok(("", '\r')));
        assert_eq!(parse_escape(r#"\t"#), Ok(("", '\t')));

        assert!(parse_escape("\\").is_err());
        assert!(parse_escape(r#"\ugdef"#).is_err());
        // TODO: As per Firely, this should fail but I dont see why
        assert_eq!(parse_escape(r#"\u01234"#), Ok(("4", 'ģ')));
        assert!(parse_escape(r#"\x"#).is_err());
        assert!(parse_escape(r#"\b"#).is_err());
    }

    #[test]
    fn test_string() {
        assert_eq!(parse_string("\'Hello\'"), Ok(("", "Hello".to_owned())));
        assert_eq!(parse_string("'Hel\\\'lo'"), Ok(("", "Hel'lo".to_owned())));

        assert_eq!(
            parse_string(r#"'single quotes'"#),
            Ok(("", "single quotes".to_owned()))
        );
        assert_eq!(
            parse_string(r#"'""single quotes with doubles""'"#),
            Ok(("", r#"""single quotes with doubles"""#.to_owned()))
        );
        assert_eq!(
            parse_string(r#"'single \' quotes'"#),
            Ok(("", "single ' quotes".to_owned()))
        );
        assert_eq!(parse_string(r#"''"#), Ok(("", "".to_owned())));
        assert_eq!(
            parse_string(r#"'xxx \u0040 yyy \\\/\f\n\r\t zzz !@#$%^&*()_-=+[]{}|;:,.<>?`~'"#),
            Ok((
                "",
                "xxx @ yyy \\/\x0C\n\r\t zzz !@#$%^&*()_-=+[]{}|;:,.<>?`~".to_owned()
            ))
        );
        assert_eq!(
            parse_string(r#"'\\b(?<month>\\d{1,2})/(?<day>\\d{1,2})/(?<year>\\d{2,4})\\b'"#),
            Ok((
                "",
                r#"\b(?<month>\d{1,2})/(?<day>\d{1,2})/(?<year>\d{2,4})\b"#.to_owned()
            ))
        );

        // AssertParser.SucceedsMatch(parser, @,
        //                 @"\b(?<month>\d{1,2})/(?<day>\d{1,2})/(?<year>\d{2,4})\b");

        // AssertParser.FailsMatch(parser, @"'\q incorrect escape'");
        // AssertParser.FailsMatch(parser, @"""double quotes""");
        // AssertParser.FailsMatch(parser, @"no quotes");
        // AssertParser.FailsMatch(parser, @"""mixed quotes'");
    }

    #[test]
    fn test_delimited_identifier() {
        assert_eq!(
            parse_delimited_identifier(r#"`Hello`"#),
            Ok(("", "Hello".to_owned()))
        );
        assert_eq!(
            parse_delimited_identifier(r#"`Hel\`lo`"#),
            Ok(("", "Hel`lo".to_owned()))
        );
    }

    #[test]
    fn test_id() {
        assert_eq!(
            parse_id("_lastUpdated"),
            Ok(("", "_lastUpdated".to_owned()))
        );

        // Copy test cases from firely-net-sdk
        assert_eq!(parse_id("a"), Ok(("", "a".to_owned())));
        assert_eq!(parse_id("_"), Ok(("", "_".to_owned())));
        assert_eq!(parse_id("__"), Ok(("", "__".to_owned())));
        assert_eq!(
            parse_id("Abcdefghijklmnopqrstuvwxyz"),
            Ok(("", "Abcdefghijklmnopqrstuvwxyz".to_owned()))
        );
        assert_eq!(
            parse_id("_Abcdef_ghijklmnopqrstuvwxyz_"),
            Ok(("", "_Abcdef_ghijklmnopqrstuvwxyz_".to_owned()))
        );
        assert_eq!(
            parse_id("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            Ok(("", "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned()))
        );
        assert_eq!(
            parse_id("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            Ok((
                "",
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned()
            ))
        );
        assert_eq!(parse_id("a0123456789"), Ok(("", "a0123456789".to_owned())));
        assert_eq!(
            parse_id("_a0123456789"),
            Ok(("", "_a0123456789".to_owned()))
        );

        assert_eq!(
            parse_id("0toinfinity"),
            Err(nom::Err::Error(nom::error::Error::new(
                "0toinfinity",
                nom::error::ErrorKind::TakeWhileMN
            )))
        );
        assert!(parse_id("").is_err());
        assert!(parse_id("@").is_err());
        assert!(parse_id("#").is_err());
        assert!(parse_id("%").is_err());
        assert!(parse_id("$").is_err());
        assert!(parse_id("*").is_err());
        assert!(parse_id(".").is_err());
        assert!(parse_id("0").is_err());
        assert!(parse_id("0123456789").is_err());
        assert!(parse_id("2A").is_err());
    }
}
