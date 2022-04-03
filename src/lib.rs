#[derive(Clone, Copy, Debug)]
pub enum ClientOrigin {
  TestsCLI,
  ProseAppMacOS,
  ProseAppIOS,
  ProseAppAndroid,
  ProseAppWindows,
  ProseAppLinux,
  ProseAppWeb,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
  #[error("The user does not exist.")]
  NoSuchUser,
  #[error("Invalid credentials.")]
  InvalidCredentials,
}

pub struct UserCredentials {
  pub jid: String,
}

/// Create a new TodoEntry from the given string.
///
pub fn login(
  jid: String,
  password: String,
  _origin: ClientOrigin,
) -> Result<UserCredentials, LoginError> {
  if jid != "hello@prose.org" {
    return Err(LoginError::NoSuchUser);
  }

  if password != "password" {
    return Err(LoginError::InvalidCredentials);
  }

  Ok(UserCredentials { jid })
}

uniffi_macros::include_scaffolding!("uniffi_test");
