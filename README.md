# mailroom

An email server written in pure Rust.

This is very much a work in progress and is nowhere near completion.

Mailroom aims to be the simplest way to host <10 email accounts on a homelab server, not an enterprise-grade email solution. Everything should be self contained and simple, and setup should take less than 30 minutes.

## What works (not necessarily stable or complete!):
- POP3
   - Tested with Mozilla Thunderbird.
   - Since mailroom doesn't yet have any form of mailbox system, every POP3 message retrieval returns a hardcoded test message.
   - Actions such as message deletion do not do anything, but user authentication with a simple password works.
   - Doesn't currently work with TLS or STARTTLS.
   - **Very** minimal; missing a lot of features
- The configuration file
   - Parsed with serde, then stored in a global static variable
- Logging with Log4rs

## What I'm working on:
- SMTP support
   - This is kinda important if mailroom is actually supposed to work as an email server lol.
- A database to store user information and emails
   - Switching to SQLite from Postgres for ease of use.
   - Using [sea-orm](https://www.sea-ql.org/SeaORM/) as the ORM.

## What's missing / To do (in order of priority):
- TLS support
   - Automatically get certificates from Let's Encrypt?
- DKIM support for signing outgoing emails.
- Automatic SPF record generation?
- IMAP support
- Verification of incoming emails via DKIM and SPF.

## Notes for my future self
- DKIM
   - Verifies the authenticity of incoming emails with a hash and digital signature.
   - Mailroom should generate a DKIM signature for outgoing emails and verify the signatures of incoming ones.
   - DKIM public key is stored in the domain's DNS record.
   - https://www.cloudflare.com/learning/dns/dns-records/dns-dkim-record/
   - https://datatracker.ietf.org/doc/html/rfc6376/
- SPF
   - Lists the addresses authorized to send emails on behalf of the domain.
   - Domain list is a DNS record.
   - Mailroom should reject incoming messages that are not from the incoming domain's SPF list.
   - Pretty much mandatory if you want other email services to accept emails from your domain.
   - https://www.cloudflare.com/learning/dns/dns-records/dns-spf-record/
   - https://datatracker.ietf.org/doc/html/rfc7208
- DMARC
   - Instructs other domains how to deal with emails that fail DKIM and SPF.
   - https://www.cloudflare.com/learning/dns/dns-records/dns-dmarc-record/
   - https://datatracker.ietf.org/doc/html/rfc7489

# Setup

(optional) Set an environment variable called `CONFIG_PATH` to the path of `config.toml`.

## Migration

Run `DATABASE_URL=sqlite://sqlite.db sea-orm-cli migrate refresh`

## Generate models

`sea-orm-cli generate entity -u sqlite://sqlite.db -o src/database/models`

# License

Mailroom - A mail server written in Rust

Copyright (C) 2023-2024 Mark Ghebrial

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.  If not, see https://www.gnu.org/licenses/.