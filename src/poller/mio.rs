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

//! Poll engine provided by the [`mio`] crate.

use std::os::fd::RawFd;
use std::sync::Arc;

use mio::event::Source;
use mio::unix::SourceFd;
use mio::Interest;

use crate::poller::Poll;
use crate::{ResourceId, ResourceIdGenerator};

use super::IoType;

const DEFAULT_CAPACITY: usize = 512;

impl Into<usize> for ResourceId {
    fn into(self) -> usize {
	self.0 as usize
    }
}

impl Into<mio::Token> for ResourceId {
    fn into(self) -> mio::Token {
	mio::Token(self.into())
    }
}

impl TryFrom<IoType> for mio::Interest {
    type Error = &'static str;

    fn try_from(value: IoType) -> Result<Self, Self::Error> {
	match (value.read, value.write) {
		(true, true) => Ok(mio::Interest::READABLE.add(mio::Interest::WRITABLE)),
		(true, false) => Ok(mio::Interest::READABLE),
		(false, true) => Ok(mio::Interest::WRITABLE),
		(false, false) => Err("none is not representable")
	}
    }
}

impl TryFrom<mio::Interest> for IoType {
    type Error = &'static str;

    fn try_from(interest: mio::Interest) -> Result<Self, Self::Error> {
	let removed = match interest.remove(Interest::READABLE) {
		None => None,
		Some(interest) => interest.remove(Interest::WRITABLE),
	};
	match removed {
		None =>
		Ok(IoType {
			read: interest.is_readable(),
			write: interest.is_readable(),
		}),
		Some(_) => Err("interest contains unsupported flags"),
	}
    }
}

struct Wrapper<'a> {
	poll: mio::Poll,
	id_gen: ResourceIdGenerator,
	sources: Vec<SourceFd<'a>>,
	events: mio::Events,
}

#[derive(Clone)]
pub struct Waker(Arc<mio::Waker>);

impl std::io::Read for Waker {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
	todo!()
    }
}

impl crate::poller::WakerSend for Waker {
    fn wake(&self) -> std::io::Result<()> {
	todo!()
    }
}

impl std::os::unix::io::AsRawFd for Waker {
    fn as_raw_fd(&self) -> RawFd { self.0.as_ref().as_raw_fd() }
}

impl crate::poller::WakerRecv for Waker {
    fn reset(&self) {
	todo!()
    }
}

impl crate::poller::Waker for Waker {
    type Send = Self;
    type Recv = Self;

    fn pair() -> Result<(Self::Send, Self::Recv), Error> {
        let waker = Arc::new(mio::Waker::new()?);
        Ok((Waker(waker.clone()), Waker(waker)))
    }
}

impl<'a> Poll for Wrapper<'a> {
    type Waker = super::Waker;

    fn register_waker(&mut self, fd: &impl std::os::unix::prelude::AsRawFd) {
	todo!()
    }

    fn register(&mut self, fd: &impl std::os::unix::prelude::AsRawFd, interest: super::IoType) -> ResourceId {
	let id = self.id_gen.next();
	let raw = fd.as_raw_fd().clone();
	let source = SourceFd(&'a raw);
	assert_eq!(self.sources.len(), id.into());
	self.sources.push(source);
	let source: &mut SourceFd = self.sources.get_mut(Into::<usize>::into(id)).unwrap();
	self.poll.registry().register(source, id.into(), interest.try_into().unwrap());
	id
    }

    fn unregister(&mut self, id: ResourceId) {
	let source: &mut SourceFd = self.sources.get_mut(Into::<usize>::into(id)).unwrap();
	self.poll.registry().deregister(source);
    }

    fn set_interest(&mut self, id: ResourceId, interest: super::IoType) -> bool {
	let source: &mut SourceFd = self.sources.get_mut(Into::<usize>::into(id)).unwrap();
	self.poll.registry().reregister(source, id.into(), interest.try_into().unwrap());
	true
    }

    fn poll(&mut self, timeout: Option<std::time::Duration>) -> std::io::Result<usize> {
	self.poll.poll(&mut self.events, timeout);
	if self.events.is_empty() {
		Ok(0)
	} else {
		Ok(self.events.iter().count())
	}
    }
}

impl Iterator for Wrapper {
    type Item;

    fn next(&mut self) -> Option<Self::Item> {
	todo!()
    }
}