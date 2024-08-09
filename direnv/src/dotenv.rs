// Package dotenv implements the parsing of the .env format.
//
// There is no formal definition of the format but it has been introduced by
// https://github.com/bkeepers/dotenv which is thus canonical.
pub static LINE: &'static str = r#"(?x)
\A
\s*
(?:|\#.*|          # comment line
(?:export\s+)?    # optional export
([\w\.]+)         # key
(?:\s*=\s*|:\s+?) # separator
(                 # optional value begin
  '(?:\'|[^'])*'  #   single quoted value
  |               #   or
  "(?:\"|[^"])*"  #   double quoted value
  |               #   or
  [^\s\#\n]+       #   unquoted value
)?                # value end
\s*
(?:\#.*)?         # optional comment
)
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use potenv::parser::parse;
    use tracing_test::traced_test;

    #[test]
    fn test_foo() {
        let re = regex::Regex::new(LINE).expect("wtf");

        assert_eq!(re.is_match(r#"FOO="bar""#), true);
    }

    #[traced_test]
    #[test]
    fn test_bar() {
        let vars = parse(&"FOO='bar'", Some(PathBuf::new())).expect("THIS SHOULD WORK");

        vars.into_iter().for_each(|a| {
            println!("{:#?}", a);
        });

        assert_eq!(true, false);
        // .for_each(|(key, value)| {
        //     assert_eq!(key, "name");
        //     assert_eq!(value, "value");
        // })
    }
}
