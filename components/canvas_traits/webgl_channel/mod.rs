/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(feature = "webgl_ipc")]
mod ipc;
#[cfg(feature = "webgl_ipc")]
pub use self::ipc::*;

#[cfg(not(feature = "webgl_ipc"))]
mod mpsc;
#[cfg(not(feature = "webgl_ipc"))]
pub use self::mpsc::*;
