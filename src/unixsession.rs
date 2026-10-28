use zbus_polkit::policykit1::Subject;

#[derive(Debug, Clone)]
pub struct UnixSession {
    pub session_id: String,
}

impl UnixSession {
    pub fn new() -> Result<Self, crate::error::Error> {
        let id = nix::unistd::getpid();

        if let Ok(session_id) = systemd::login::get_session(Some(id.as_raw())) {
            return Ok(Self { session_id });
        }
        let uid = systemd::login::get_owner_uid(id.as_raw())?;
        let session_id = systemd::login::user_primary_session(uid)?;

        Ok(Self { session_id })
    }
}

impl From<UnixSession> for Subject {
    fn from(val: UnixSession) -> Self {
        Subject::new_for_unix_session(val.session_id)
    }
}
