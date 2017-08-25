//   git-dit - the distributed issue tracker for git
//   Copyright (C) 2017 Matthias Beyer <mail@beyermatthias.de>
//   Copyright (C) 2017 Julian Ganz <neither@nut.email>
//
//   This program is free software; you can redistribute it and/or modify
//   it under the terms of the GNU General Public License version 2 as
//   published by the Free Software Foundation.
//

use libgitdit::Issue;
use libgitdit::trailer::filter::{TrailerFilter, ValueMatcher};
use libgitdit::trailer::{TrailerValue, spec};
use std::str::FromStr;

use error::*;
use error::ErrorKind as EK;
use gitext::{RemotePriorization, ReferrencesExt};
use system::{Abortable, IteratorExt};


/// Filter specification
///
/// This type represents a filter rule for a single piece of metadata.
///
pub struct FilterSpec<'a> {
    /// Metadata to filter
    metadata: spec::TrailerSpec<'a>,
    /// Matcher for the value
    matcher: ValueMatcher,
}

impl<'a> FromStr for FilterSpec<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(2, ':');

        let metadata = parts
            .next()
            .and_then(|name| match name {
                "status"    => Some(spec::ISSUE_STATUS_SPEC.clone()),
                "type"      => Some(spec::ISSUE_TYPE_SPEC.clone()),
                _           => None,
            })
            .ok_or_else(|| Error::from_kind(EK::MalformedFilterSpec(s.to_owned())))?;

        let value = parts
            .next()
            .map(TrailerValue::from_slice)
            .ok_or_else(|| Error::from_kind(EK::MalformedFilterSpec(s.to_owned())))?;

        Ok(FilterSpec {metadata: metadata, matcher: ValueMatcher::Equals(value)})
    }
}


/// Metadata filter
///
pub struct MetadataFilter<'a> {
    prios: &'a RemotePriorization,
    trailers: Vec<TrailerFilter<'a>>,
}

impl<'a> MetadataFilter<'a> {
    /// Create a new metadata filter
    ///
    pub fn new<I>(prios: &'a RemotePriorization, spec: I) -> Self
        where I: IntoIterator<Item = FilterSpec<'a>>
    {
        MetadataFilter {
            prios: prios,
            trailers: spec
                .into_iter()
                .map(|spec| TrailerFilter::new(spec.metadata, spec.matcher))
                .collect(),
        }
    }

    /// Create an empty metadata filter
    ///
    /// The filter will not filter out any issues.
    ///
    pub fn empty(prios: &'a RemotePriorization) -> Self {
        MetadataFilter {
            prios: prios,
            trailers: Vec::new(),
        }
    }

    /// Filter an issue
    ///
    pub fn filter(&self, issue: &Issue) -> bool {
        // NOTE: if we ever add the filters crate as a dependency, this method
        //       may be transferred to an implementatio nof the Filter trait
        use git2::ObjectType;
        use libgitdit::iter::MessagesExt;
        use std::collections::HashMap;

        // Filtering may be expensive, so it makes sense to return early if the
        // filter is empty.
        if self.trailers.is_empty() {
            return true;
        }

        // Get the head reference
        let head = issue
            .heads()
            .abort_on_err()
            .select_ref(self.prios)
            .map(|head| head.peel(ObjectType::Commit).unwrap_or_abort().id());

        // Accumulate all the metadata we care about
        let acc: HashMap<_, _> = head
            .into_iter()
            .flat_map(|head| issue.messages_from(head).abort_on_err())
            .accumulate_trailers(self.trailers.iter().map(|i| i.spec()));

        // Compute whether all constraints are met
        self.trailers
            .iter()
            .all(|spec| spec.matches(&acc))
    }
}

