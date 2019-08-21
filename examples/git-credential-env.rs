// Copyright 2019 Pascal Bach.
//
// SPDX-License-Identifier:	Apache-2.0 or MIT

/// A simple git credential helper that
/// provides the credentials from the two environment variables
/// `GIT_USER` and `GIT_PASS`.
///
/// Just copy the resulting binary `git-credential-env` into `PATH` and
/// configure it using `git config credential.helper env`.
use git_credential::GitCredential;
use std::env;

fn main() {
    let mut gc = GitCredential::default();

    // If we can't read a variable just ignore it.
    gc.username = env::var("GIT_USER").ok();
    gc.password = env::var("GIT_PASS").ok();

    let out = std::io::stdout();

    gc.to_writer(out)
        .expect("Something went wrong writing the credentials!");
}
