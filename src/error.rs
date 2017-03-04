// git-dit - the distributed issue tracker for git
// Copyright (C) 2017 Matthias Beyer <mail@beyermatthias.de>
// Copyright (C) 2017 Julian Ganz <neither@nut.email>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

error_chain! {
    foreign_links {
        GitError(::git2::Error);
        GitDitError(::libgitdit::error::Error);
    }

    errors {
        WrappedGitError {
            description("TODO: Wrapped error")
            display("TODO: Wrapped error")
        }

        WrappedGitDitError {
            description("TODO: Wrapped error")
            display("TODO: Wrapped error")
        }
    }
}
