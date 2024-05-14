// Library for concurrent I/O resource management using reactor pattern.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2024 by
//     Lorenz Leutgeb
//
// Copyright 2024 Lorenz Leutgeb
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Poll engine provided by the [`polling`] crate.

use std::collections::VecDeque;
use std::io::{self, Error};
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Arc;
use std::time::Duration;

use polling::Event;

use crate::poller::{IoFail, IoType, Poll, Waker, WakerRecv, WakerSend};
use crate::{ResourceId, ResourceIdGenerator};

pub struct Poller {
	poller: polling::Poller,
}

impl Poller {
	pub fn new() -> std::io::Result<Self> {
		Ok(Self {
			poller: polling::Poller::new()?
		})
	}
}

impl Poll for Poller {
    type Waker;

    fn register_waker(&mut self, fd: &impl AsRawFd) {
	unsafe {
		self.poller.add(fd, Event::readable(7));
	}
    }

    fn register(&mut self, fd: &impl AsRawFd, interest: IoType) -> ResourceId {
	todo!()
    }

    fn unregister(&mut self, id: ResourceId) {
	todo!()
    }

    fn set_interest(&mut self, id: ResourceId, interest: IoType) -> bool {
	todo!()
    }

    fn poll(&mut self, timeout: Option<Duration>) -> io::Result<usize> {
	todo!()
    }
}