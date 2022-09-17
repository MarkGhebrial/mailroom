# Mailroom

An email server written in pure Rust.

This is very much a work in progress and is currently lacking most basic functionality.

## What works (not necessarily stable or complete!):
 - My implementation of the POP3 *protocol*
     - Can fool email clients into showing a message in the user's inbox.
        - Tested with Mozilla Thunderbird.
     - Doesn't currently work with TLS or STARTTLS.
     - **Very** minimal; missing a lot of features

## What's missing / To do (in order of priority):
 - A database to store emails in (kinda important!)
 - A proper configuration method
 - Proper event logging
 - TLS support
    - Automatically get certificates from Let's Encrypt?
 - SMTP support
 - IMAP support

# License

Mailroom - A mail server written in Rust

Copyright (C) 2022 Mark Ghebrial

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.  If not, see https://www.gnu.org/licenses/.