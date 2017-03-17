// git-dit - the distributed issue tracker for git
// Copyright (C) 2016, 2017 Matthias Beyer <mail@beyermatthias.de>
// Copyright (C) 2016, 2017 Julian Ganz <neither@nut.email>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
extern crate git2;
extern crate regex;

pub mod error;
pub mod iter;
pub mod message;
pub mod repository;

mod first_parent_iter;

