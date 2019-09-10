# Git Credential

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fbachp%2Fgit-credential-rs%2Fbadge%3Fref%3Dmaster&style=flat)](https://actions-badge.atrox.dev/bachp/git-credential-rs/goto?ref=master)
[![Crate](https://img.shields.io/crates/v/rand.svg)](https://crates.io/crates/git-credential))
[![API](https://docs.rs/git-credential/badge.svg)](https://docs.rs/git-credential)

A Rust library that provides types that help to implement git-credential helpers.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
git-credential = "*"
```

This crates provides types that are able to parse input and produce output in the format
described in [git-credential[1] ](https://git-scm.com/docs/git-credential).

The following shows an example on how create a `GitCredential` struct
from an input, modify it and write it back to an output:

```rust
use git_credential::GitCredential;

let input = "username=me\npassword=%sec&ret!\n\n".as_bytes();
let mut output: Vec<u8> = Vec::new();

let mut g = GitCredential::from_reader(input).unwrap();

assert_eq!(g.username.unwrap(), "me");
assert_eq!(g.password.unwrap(), "%sec&ret!");

g.username = Some("you".into());
g.password = Some("easy".into());

g.to_writer(&mut output).unwrap();

assert_eq!("username=you\npassword=easy\n\n", String::from_utf8(output).unwrap())
```

See the [API documentation](https://docs.rs/git-credential) for more details.

# License

Rand is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
