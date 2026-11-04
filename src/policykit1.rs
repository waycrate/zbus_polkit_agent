/*
 * Copyright  © 2024 herby
 *
 * SPDX-License-Identifier: MIT
 * Permission is hereby granted, free of charge, to any
 * person obtaining a copy of this software and associated
 * documentation files (the "Software"), to deal in the
 * Software without restriction, including without
 * limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software
 * is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice
 * shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
 * ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 * TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
 * SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 * IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zbus::{OwnedVariant, Type};

/// This struct describes subjects such as UNIX processes. It is typically used to check if a given
/// process is authorized for an action.
///
/// The following kinds of subjects are known:
///
/// * Unix Process. `subject_kind` should be set to `unix-process` with keys `pid` (of type
///   `uint32`) and `start-time` (of type `uint64`).
///
/// * Unix Session. `subject_kind` should be set to `unix-session` with the key `session-id` (of
///   type `string`).
///
/// * System Bus Name. `subject_kind` should be set to `system-bus-name` with the key `name` (of
///   type `string`).
#[derive(Debug, Type, Serialize, Deserialize)]
pub struct Subject {
    /// The type of the subject.
    pub subject_kind: String,

    /// Details about the subject. Depending of the value of `subject_kind`, a set of well-defined
    /// key/value pairs are guaranteed to be available.
    pub subject_details: HashMap<String, OwnedVariant>,
}

/// This D-Bus interface is implemented by the /org/freedesktop/PolicyKit1/Authority object on the
/// well-known name org.freedesktop.PolicyKit1 on the system message bus.
#[zbus::proxy(
    interface = "org.freedesktop.PolicyKit1.Authority",
    default_service = "org.freedesktop.PolicyKit1",
    default_path = "/org/freedesktop/PolicyKit1/Authority"
)]
pub trait Authority {
    /// Register an authentication agent.
    ///
    /// Note that this should be called by same effective UID which will be passed to
    /// `AuthenticationAgentResponse2()`.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject to register the authentication agent for, typically a session
    /// subject.
    ///
    /// * `locale` - The locale of the authentication agent.
    ///
    /// * `object_path` - The object path of authentication agent object on the unique name of the
    /// caller.
    fn register_authentication_agent(
        &self,
        subject: &Subject,
        locale: &str,
        object_path: &str,
    ) -> zbus::Result<()>;
}
