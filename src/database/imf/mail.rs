use super::super::err::MailParseError;
use std::collections::HashMap;

/// Represents an email message.
#[derive(PartialEq, Debug)]
pub struct Mail {
    headers: HashMap<String, String>,
    content: String,
}

impl Mail {
    pub fn new(headers: HashMap<String, String>, content: String) -> Self {
        Self { headers, content }
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// Return the length of the message in octets.
    pub fn content_len(&self) -> usize {
        self.content.len()
    }
}

/// Convert a `String` into `Mail`
impl TryFrom<String> for Mail {
    type Error = MailParseError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // Split the headers and content
        let (header_str, content) = match string.split_once("\r\n\r\n") {
            Some(v) => v,
            None => (string.trim(), "")
        };

        let header_vec = unfold(header_str)?;

        let mut header_map: HashMap<String, String> = HashMap::new();
        for header in header_vec {
            let (name, body) = header.split_once(":").ok_or(MailParseError)?;

            header_map.insert(name.trim().to_owned(), body.trim().to_owned());
        }

        Ok(Self::new(header_map, content.to_owned()))
    }
}

/// "Unfold" long header fields as described in RFC 5322 section 2.2.3
fn unfold(headers: &str) -> Result<Vec<String>, MailParseError> {
    let mut out: Vec<String> = vec![];

    for line in headers.split("\r\n") {
        let trimmed = line.trim_start();

        if line == trimmed {
            // No leading whitespace was trimmed, therefore this is a new header
            out.push(line.to_owned())
        } else {
            let index = &out.len()-1;
            out.get_mut(index).ok_or(MailParseError)?.push_str(line);
        }
    }

    Ok(out)
}

#[test]
fn mail_parse() {
    let parsed: Mail =
        "From: John Doe <jdoe@machine.example>\r\nTo: Mary Smith <mary@example.net>\r\nSubject: Saying Hello\r\nDate: Fri, 21 Nov 1997 09:55:06 -0600\r\nMessage-ID: <1234@local.machine.example>\r\n\r\nThis is a message just to say hello.\r\nSo, \"Hello\"."
    .to_owned().try_into().unwrap();

    let mut headers: HashMap<String, String> = HashMap::new();

    let mut insert = |k: &str, v: &str| {
        headers.insert(k.trim().to_owned(), v.trim().to_owned());
    };

    insert("From","John Doe <jdoe@machine.example>");
    insert("To", "Mary Smith <mary@example.net>");
    insert("Subject", "Saying Hello");
    insert("Date", "Fri, 21 Nov 1997 09:55:06 -0600");
    insert("Message-ID", "<1234@local.machine.example>");

    let expected = Mail::new(
        headers,
        "This is a message just to say hello.\r\nSo, \"Hello\".".to_owned()
    );

    assert_eq!(
        parsed, expected
    );
}

#[test]
fn header_unfolding() {
    assert_eq!(
        unfold("Subject: This is\r\n a test."),
        Ok(vec!["Subject: This is a test.".to_owned()])
    );

    assert_eq!(
        unfold("Subject: This is\r\n a test. \r\nFoo: Bar\tBiz"),
        Ok(vec![
            "Subject: This is a test. ".to_owned(),
            "Foo: Bar\tBiz".to_owned()
        ])
    );
}