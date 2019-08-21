//! The git_credentials crate provides types that help to implement git-credential helpers.
//!
//! The format is documented here: https://git-scm.com/docs/git-credential
use log::{debug, warn};
use std::io::{BufRead, BufReader, Read, Write};
use url::Url;

/// Holds the values of all parameters supported by git-credential
#[derive(Debug)]
pub struct GitCredential {
    /// The url field is treated specially by git-credential.
    /// Setting the url coresponds to setting all the other fields as part of the url.
    ///
    /// The url has the following format: `<protocol>://<username>:<password>@<host>/<path>`.
    pub url: Option<Url>, // TODO: Use URL type here?
    /// The protocol over which the credential will be used (e.g., `https`).
    pub protocol: Option<String>,
    /// The remote hostname for a network credential (e.g., `example.com`).
    pub host: Option<String>,
    /// The path with which the credential will be used. E.g., for accessing a remote https repository, this will be the repository’s path on the server.
    pub path: Option<String>,
    /// The credential’s username, if we already have one (e.g., from a URL, from the user, or from a previously run helper).
    pub username: Option<String>,
    /// The credential’s password, if we are asking it to be stored.
    pub password: Option<String>,
}

impl Default for GitCredential {
    /// Creates a new GitCredential struct with all values set to None
    fn default() -> GitCredential {
        GitCredential {
            url: None,
            protocol: None,
            host: None,
            path: None,
            username: None,
            password: None,
        }
    }
}

impl GitCredential {
    /// Read the git-credential values from a Reader like stdin
    ///
    /// ```
    /// use git_credential::GitCredential;
    ///
    /// let s = "username=me\npassword=%sec&ret!\n\n".as_bytes();
    ///
    /// let g = GitCredential::from_reader(s).unwrap();
    ///
    /// assert_eq!(g.username.unwrap(), "me");
    /// assert_eq!(g.password.unwrap(), "%sec&ret!");
    /// ```
    pub fn from_reader(source: impl Read) -> Result<GitCredential, ()> {
        //TODO: Implement error
        let mut gc = GitCredential::default();
        let source = BufReader::new(source);
        for line in source.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                // An empty line means we are done
                // TODO: Make sure an empty line exists in the end
                break;
            }
            match line.split_terminator('=').collect::<Vec<&str>>().as_slice() {
                [key, value] => {
                    debug!("Reading line with: {} = {}", key, value);
                    let value = value.to_string();
                    let key = key.to_owned(); // TODO: Why is this needed?
                    match key {
                        "url" => {
                            gc.url = {
                                let value = Url::parse(&value).unwrap();
                                Some(value)
                            }
                        }
                        "protocol" => gc.protocol = Some(value),
                        "host" => gc.host = Some(value),
                        "path" => gc.path = Some(value),
                        "username" => gc.username = Some(value),
                        "password" => gc.password = Some(value),
                        _ => warn!("Unknown key: {} = {}", &key, &value),
                    };
                }
                _ => warn!("Invalid line: {}", &line),
            };
        }
        Ok(gc)
    }

    /// Writes the git-credentials values to a Writer like stdout
    ///
    /// ```
    /// use git_credential::GitCredential;
    ///
    /// let mut g = GitCredential::default();
    /// g.username = Some("me".into());
    /// g.password = Some("%sec&ret!".into());
    ///
    /// let mut v: Vec<u8> = Vec::new();
    ///
    /// g.to_writer(&mut v).unwrap();
    ///
    /// assert_eq!("username=me\npassword=%sec&ret!\n\n", String::from_utf8(v).unwrap());
    /// ```
    pub fn to_writer(&self, mut sink: impl Write) -> Result<(), ()> {
        // The url filed is written first, this allows the other fields to override
        // parts of the url
        if let Some(url) = &self.url {
            writeln!(sink, "url={}", url).unwrap();
        }
        if let Some(protocol) = &self.protocol {
            writeln!(sink, "protocol={}", protocol).unwrap();
        }
        if let Some(host) = &self.host {
            writeln!(sink, "host={}", host).unwrap();
        }
        if let Some(path) = &self.path {
            writeln!(sink, "path={}", path).unwrap();
        }
        if let Some(username) = &self.username {
            writeln!(sink, "username={}", username).unwrap();
        }
        if let Some(password) = &self.password {
            writeln!(sink, "password={}", password).unwrap();
        }

        // One empty line in the end
        writeln!(sink).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{GitCredential, Url};
    #[test]
    fn read_from_reader() {
        let s = "username=me\npassword=%sec&ret!\nprotocol=https\nhost=example.com\npath=myproject.git\nurl=https://example.com/myproject.git\n\n".as_bytes();
        let g = GitCredential::from_reader(s).unwrap();
        assert_eq!(g.username.unwrap(), "me");
        assert_eq!(g.password.unwrap(), "%sec&ret!");
        assert_eq!(g.protocol.unwrap(), "https");
        assert_eq!(g.host.unwrap(), "example.com");
        assert_eq!(g.path.unwrap(), "myproject.git");
        assert_eq!(
            g.url.unwrap(),
            Url::parse("https://example.com/myproject.git").unwrap()
        );
    }

    #[test]
    fn write_to_writer() {
        let s = "url=https://example.com/myproject.git\nprotocol=https\nhost=example.com\npath=myproject.git\nusername=me\npassword=%sec&ret!\n\n";
        let mut g = GitCredential::default();
        g.username = Some("me".into());
        g.password = Some("%sec&ret!".into());
        g.url = Some(Url::parse("https://example.com/myproject.git").unwrap());
        g.protocol = Some("https".into());
        g.host = Some("example.com".into());
        g.path = Some("myproject.git".into());
        let mut v: Vec<u8> = Vec::new();
        g.to_writer(&mut v).unwrap();
        assert_eq!(s, String::from_utf8(v).unwrap());
    }

    #[test]
    fn read_and_write_adain() {
        let s = "url=https://example.com/myproject.git\nprotocol=https\nhost=example.com\npath=myproject.git\nusername=me\npassword=%sec&ret!\n\n";
        let g = GitCredential::from_reader(s.as_bytes()).unwrap();
        let mut v: Vec<u8> = Vec::new();
        g.to_writer(&mut v).unwrap();
        assert_eq!(s, String::from_utf8(v).unwrap());
    }
}
