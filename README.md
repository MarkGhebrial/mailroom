# Mailroom

An email server written in pure Rust.

This is very much a work in progress and is currently lacking most basic functionality.

## What works (not necessarily stable or complete!):
- My implementation of the POP3 *protocol*
   - Can fool email clients into showing a message in the user's inbox.
      - Tested with Mozilla Thunderbird.
   - Doesn't currently work with TLS or STARTTLS.
   - **Very** minimal; missing a lot of features
- The configuration file
   - Parsed with serde, then stored in a global static variable
- Logging with Log4rs

## What's missing / To do (in order of priority):
- A database to store emails in (kinda important!)
- TLS support
   - Automatically get certificates from Let's Encrypt?
- SMTP support
- IMAP support

# Setup

Set an environment variable called `CONFIG_PATH` to the path of `config.toml`

## Migration

Run `DATABASE_URL=sqlite://sqlite.db sea-orm-cli migrate refresh`

# License

Mailroom - A mail server written in Rust

Copyright (C) 2022 Mark Ghebrial

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.  If not, see https://www.gnu.org/licenses/.