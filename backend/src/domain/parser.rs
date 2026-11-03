use super::set::IntSet;

pub const MAX_INPUT_CHARS: usize = 512;
pub const MAX_ELEMENTS: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub code: &'static str,
    pub message: String,
}

impl ParseError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

struct Scanner {
    chars: Vec<char>,
    index: usize,
}

impl Scanner {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            index: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.index += 1;
        Some(ch)
    }

    fn skip_ws(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.index += 1;
        }
    }

    fn position(&self) -> usize {
        self.index + 1
    }

    fn finished_after_ws(&mut self) -> bool {
        self.skip_ws();
        self.peek().is_none()
    }
}

/// Grammatik für endliche Mengen ganzer Zahlen:
///
/// ```text
/// menge     ::= "∅" | "{" [ eintrag ("," eintrag)* ] "}"
/// eintrag   ::= ganzzahl
/// ganzzahl  ::= "-" ? ziffer+
/// ```
///
/// Leerzeichen sind um Klammern, Kommas und Zahlen erlaubt.
/// `{}` und `∅` sind die leere Menge. Duplikate werden entfernt.
/// Verschachtelte Mengen, Dezimalzahlen und andere Zeichen sind ungültig.
pub fn parse_int_set(input: &str) -> Result<IntSet, ParseError> {
    if input.chars().count() > MAX_INPUT_CHARS {
        return Err(ParseError::new(
            "too_long",
            format!("Die Eingabe darf höchstens {MAX_INPUT_CHARS} Zeichen lang sein."),
        ));
    }

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseError::new(
            "empty",
            "Die Eingabe ist leer. Erwartet wird ∅, {} oder eine Menge wie {-2,0,3}.",
        ));
    }
    if trimmed == "∅" || trimmed == "{}" {
        return Ok(IntSet::empty());
    }

    let mut scanner = Scanner::new(trimmed);
    scanner.skip_ws();
    if scanner.peek() != Some('{') {
        return Err(ParseError::new(
            "syntax",
            "Eine Menge muss mit «{» beginnen oder als ∅ bzw. {} geschrieben sein.",
        ));
    }
    scanner.bump();
    scanner.skip_ws();

    let mut set = IntSet::empty();
    let mut listed = 0usize;

    if scanner.peek() == Some('}') {
        scanner.bump();
        if !scanner.finished_after_ws() {
            return Err(trailing(&scanner));
        }
        return Ok(set);
    }

    loop {
        if scanner.peek() == Some('{') || scanner.peek() == Some('∅') {
            return Err(ParseError::new(
                "nested",
                "Verschachtelte Mengen kann dieser Parser nicht lesen. Erlaubt sind nur ganze Zahlen.",
            ));
        }
        let value = parse_integer(&mut scanner)?;
        listed += 1;
        if listed > MAX_ELEMENTS {
            return Err(ParseError::new(
                "too_many",
                format!("Eine Eingabe darf höchstens {MAX_ELEMENTS} Einträge enthalten."),
            ));
        }
        set.insert(value);
        scanner.skip_ws();
        match scanner.peek() {
            Some(',') => {
                scanner.bump();
                scanner.skip_ws();
                if scanner.peek() == Some('}') {
                    return Err(ParseError::new(
                        "syntax",
                        format!(
                            "Nach einem Komma fehlt eine Zahl (Position {}).",
                            scanner.position()
                        ),
                    ));
                }
            }
            Some('}') => {
                scanner.bump();
                break;
            }
            Some(ch) => {
                return Err(ParseError::new(
                    "syntax",
                    format!(
                        "Unerwartetes Zeichen «{ch}» an Position {}. Erwartet wird ein Komma oder «}}».",
                        scanner.position()
                    ),
                ));
            }
            None => {
                return Err(ParseError::new(
                    "syntax",
                    "Die schliessende Klammer «}» fehlt.",
                ));
            }
        }
    }

    if !scanner.finished_after_ws() {
        return Err(trailing(&scanner));
    }
    Ok(set)
}

fn trailing(scanner: &Scanner) -> ParseError {
    ParseError::new(
        "syntax",
        format!(
            "Nach der Menge stehen weitere Zeichen (ab Position {}).",
            scanner.position()
        ),
    )
}

fn parse_integer(scanner: &mut Scanner) -> Result<i64, ParseError> {
    scanner.skip_ws();
    let mut negative = false;
    if scanner.peek() == Some('+') {
        return Err(ParseError::new(
            "syntax",
            format!(
                "Ein führendes Plus ist nicht erlaubt (Position {}). Schreibe die Zahl ohne «+».",
                scanner.position()
            ),
        ));
    }
    if scanner.peek() == Some('-') {
        negative = true;
        scanner.bump();
    }
    if !scanner.peek().is_some_and(|ch| ch.is_ascii_digit()) {
        let found = scanner.peek().unwrap_or(' ');
        return Err(ParseError::new(
            "syntax",
            format!(
                "Erwartet wird eine ganze Zahl an Position {}, gefunden «{found}».",
                scanner.position()
            ),
        ));
    }

    let mut value: i64 = 0;
    while scanner.peek().is_some_and(|ch| ch.is_ascii_digit()) {
        let Some(ch) = scanner.bump() else {
            break;
        };
        let digit = i64::from(ch as u8 - b'0');
        value = value.checked_mul(10).and_then(|v| v.checked_add(digit)).ok_or_else(|| {
            ParseError::new(
                "overflow",
                "Eine Zahl liegt ausserhalb des erlaubten Bereichs für ganze 64-Bit-Zahlen.",
            )
        })?;
    }

    if negative {
        value = value.checked_neg().ok_or_else(|| {
            ParseError::new(
                "overflow",
                "Eine Zahl liegt ausserhalb des erlaubten Bereichs für ganze 64-Bit-Zahlen.",
            )
        })?;
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_empty_and_normalizes_duplicates_and_order() {
        assert_eq!(parse_int_set("∅").unwrap(), IntSet::empty());
        assert_eq!(parse_int_set(" {} ").unwrap(), IntSet::empty());
        assert_eq!(parse_int_set("{ }").unwrap(), IntSet::empty());
        let set = parse_int_set(" { 3, 1, 2, 2, 1 } ").unwrap();
        assert_eq!(set.elements(), vec![1, 2, 3]);
        assert_eq!(set.to_canonical(), "{1,2,3}");
    }

    #[test]
    fn parses_negative_numbers_and_leading_zeros() {
        let set = parse_int_set("{-2,0,3}").unwrap();
        assert_eq!(set.elements(), vec![-2, 0, 3]);
        assert_eq!(parse_int_set("{00,-0,01}").unwrap().elements(), vec![0, 1]);
    }

    #[test]
    fn rejects_invalid_syntax_nested_and_limits() {
        assert_eq!(parse_int_set("").unwrap_err().code, "empty");
        assert_eq!(parse_int_set("{1,}").unwrap_err().code, "syntax");
        assert_eq!(parse_int_set("{,1}").unwrap_err().code, "syntax");
        assert_eq!(parse_int_set("1,2,3").unwrap_err().code, "syntax");
        assert_eq!(parse_int_set("{1 2}").unwrap_err().code, "syntax");
        assert_eq!(parse_int_set("{1.5}").unwrap_err().code, "syntax");
        assert_eq!(parse_int_set("{+1}").unwrap_err().code, "syntax");
        assert_eq!(parse_int_set("{1,{2}}").unwrap_err().code, "nested");
        assert_eq!(parse_int_set("{1,2} extra").unwrap_err().code, "syntax");
        assert_eq!(parse_int_set("{1,2").unwrap_err().code, "syntax");

        let long = "x".repeat(MAX_INPUT_CHARS + 1);
        assert_eq!(parse_int_set(&long).unwrap_err().code, "too_long");

        let many = format!(
            "{{{}}}",
            (0..=MAX_ELEMENTS)
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        assert_eq!(parse_int_set(&many).unwrap_err().code, "too_many");

        let huge = format!("{{{}1}}", "9".repeat(40));
        assert_eq!(parse_int_set(&huge).unwrap_err().code, "overflow");
    }
}
