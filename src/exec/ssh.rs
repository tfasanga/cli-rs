use std::ffi::OsString;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Clone)]
pub struct SshCredentials {
    pub username: String,
    pub hostname: String,
    pub password: Option<String>,
    pub port: Option<u16>,
    pub private_key_file: Option<OsString>,
    pub public_key_file: Option<OsString>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct SshCredentialsBuilder {
    username: String,
    hostname: String,
    password: Option<String>,
    port: Option<u16>,
    private_key_file: Option<OsString>,
    public_key_file: Option<OsString>,
}

impl SshCredentials {
    #[must_use] pub fn builder(username: String, hostname: String) -> SshCredentialsBuilder {
        SshCredentialsBuilder {
            username,
            hostname,
            password: None,
            port: None,
            private_key_file: None,
            public_key_file: None,
        }
    }
}

impl SshCredentialsBuilder {
    #[must_use] pub fn build(self) -> SshCredentials {
        SshCredentials {
            username: self.username,
            hostname: self.hostname,
            password: self.password,
            port: self.port,
            private_key_file: self.private_key_file,
            public_key_file: self.public_key_file,
        }
    }

    #[must_use] pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    #[must_use] pub fn private_key_file(mut self, private_key_file:  OsString) -> Self {
        self.private_key_file = Some(private_key_file);
        self
    }

    #[must_use] pub fn public_key_file(mut self, public_key_file: OsString) -> Self {
        self.public_key_file = Some(public_key_file);
        self
    }

    #[must_use] pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
}
