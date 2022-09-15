# Rusty Mail Server

An email server written in pure Rust.

This is very much a work in progress and is currently lacking many crucial features.

## What works (not necessarily stable or complete!):
 - My implementation of the POP3 *protocol*
     - Can fool email clients into showing a message in the user's inbox.
        - Tested with Mozilla Thunderbird.
     - Doesn't currently work with TLS or STARTTLS.
     - **Very** minimal; missing a lot of features

## What's missing / To do (in order of priority):
 - A database to store emails in (kinda important!)
 - A proper configuration method
 - TLS support
    - Automatically get certificates from Let's Encrypt?
 - SMTP support
 - IMAP support
