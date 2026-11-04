use crate::policykit1::Subject;
use std::collections::HashMap;
use zbus::OwnedVariant;

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
        let session_id = OwnedVariant::from(zbus::Str::from(val.session_id.as_str()));
        Subject {
            subject_kind: "unix-session".to_string(),
            subject_details: HashMap::from_iter([("session-id".to_string(), session_id)]),
        }
    }
}
