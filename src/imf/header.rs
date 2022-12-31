/// Represents a header in an Internet Message Format message
pub struct ImfHeader {
    pub name: HeaderName,
    pub body: HeaderBody,
}

/// Enumerates all the header names specified in RFC 5322
pub enum HeaderName {
    Date,
    From,
    Sender,
    ReplyTo,
    To,
    Cc,
    Bcc,
    MessageID,
    InReplyTo,
    References,
    Subject,
    Comments,
    Keywords,
    ResentDate,
    ResentFrom,
    ResentSender,
    ResentTo,
    ResentCc,
    ResentBcc,
    ResentMessageID,
    ReturnPath,
    Other(String),
}

impl From<&str> for HeaderName {
    fn from(string: &str) -> Self {
        use HeaderName::*;
        match string {
            "Date" => Date,
            "From" => From,
            "Sender" => Sender,
            "Reply-To" => ReplyTo,
            "To" => To,
            "Cc" => Cc,
            "Bcc" => Bcc,
            "Message-ID" => MessageID,
            "In-Reply-To" => InReplyTo,
            "References" => References,
            "Subject" => Subject,
            "Comments" => Comments,
            "Keywords" => Keywords,
            "Resent-Date" => ResentDate,
            "Resent-From" => ResentFrom,
            "Resent-Sender" => ResentSender,
            "Resent-To" => ResentTo,
            "Resent-Cc" => ResentCc,
            "Resent-Bcc" => ResentBcc,
            "Resent-Message-ID" => ResentMessageID,
            "Return-Path" => ReturnPath,
            s => Other(s.to_owned()),
        }
    }
}

pub enum HeaderBody {
    Unstructured(String),
}
