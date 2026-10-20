//-----------------------------------------------------------------------------
// Copyright (c) 2026, Oracle and/or its affiliates.
//
// This software is dual-licensed to you under the Universal Permissive License
// (UPL) 1.0 as shown at https://oss.oracle.com/licenses/upl and Apache License
// 2.0 as shown at http://www.apache.org/licenses/LICENSE-2.0. You may choose
// either license.
//
// If you elect to accept the software under the Apache License, Version 2.0,
// the following applies:
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// external_auth.rs
//
// Defines the methods available for authenticating to the database without a
// user name and password.
//-----------------------------------------------------------------------------

/// Method used to authenticate to the database instead of supplying a user
/// name and a password. `Debug` is deliberately not implemented so that no
/// credential contained in it can be printed.
#[non_exhaustive]
pub enum ExternalAuth {
    /// Bearer access token (OAuth 2.0, OCI IAM or Microsoft Entra ID). The
    /// token is sent to the database in place of a password verifier and
    /// requires the tcps protocol.
    AccessToken(String),
}
