//! Small `Copy` ids the trace and the later slices share (ai.md "Data model").
//!
//! Content names things by string (`sf.ai.method`, `sf.ai.drive`, `sf.ai.consideration` records);
//! names resolve to these indices **once, at load**, through registries the owning slice builds
//! (AI2: tasks, methods, operators; AI7: drives, considerations); [`DirectiveId`] is the exception, a
//! per-actor instance sequence the directive stack issues (AI4). Nothing in a tick
//! compares strings. Each id serializes as its bare number (`#[serde(transparent)]`) and displays
//! with a short prefix (`t3`, `m7`, ...) for console lines.

use std::fmt;

use serde::{Deserialize, Serialize};

macro_rules! id {
    ($(#[$doc:meta])* $name:ident($int:ty), $prefix:literal) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub $int);

        impl $name {
            /// The registry index.
            pub const fn index(self) -> $int {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!($prefix, "{}"), self.0)
            }
        }
    };
}

id!(
    /// A drive (`Directive`, `Combat`, `Flee`, ...): index into the drive registry (AI7).
    DriveId(u16), "d"
);
id!(
    /// A utility consideration (input + response curve + weight): index into the consideration
    /// registry (AI7). Named in a drive's score breakdown.
    ConsiderationId(u16), "c"
);
id!(
    /// An HTN task, compound or primitive: index into the task registry (AI2).
    TaskId(u32), "t"
);
id!(
    /// An HTN method (one way to decompose a compound task): index into the method library (AI2).
    MethodId(u32), "m"
);
id!(
    /// An operator (executor) a primitive task runs: index into the operator vocabulary (AI2, AI6).
    OperatorId(u16), "o"
);
id!(
    /// A directive instance on an actor's stack (AI4): what `done(id)` polls. Unique per actor.
    DirectiveId(u32), "dir"
);
