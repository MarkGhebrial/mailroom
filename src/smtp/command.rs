use email_address::EmailAddress;

enum SMTPCommand {
    // TODO: source routes
    MailFrom {
        sender: EmailAddress,
    },
    RcptTo {
        recipient: EmailAddress,
    },
    Data,

    // Verify and expand are optional and must be listed in an EHLO response (RFC 2821 3.5.2)
    /// `VRFY`; Verify that the user exists on the server
    Verify {
        address: String,
    },

    /// `EXPN`; List all the recipients on a mailing list
    Expand {
        mailing_list: String,
    },
}
