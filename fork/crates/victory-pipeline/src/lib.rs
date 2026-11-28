// VIctory: this file is part of VIctory, distributed under the GNU General Public License, version 3 or later (GPL-3.0-or-later). See the LICENSE file at the root of the repository.
//! Offline conversion of the user's game data into host formats.
//!
//! Everything here runs on the user's machine, reads the user's files and
//! writes into the user's cache. Nothing produced here is ever committed:
//! derived data is still game data (docs/asset-policy.md).
//!
//! The unit of work is a [`Converter`]: bytes in, bytes out, deterministic.
//! Results are stored in a content-addressed [`Cache`] keyed by converter
//! id, converter version and input bytes, so re-running the pipeline after
//! a game update or a converter fix only redoes what changed.

mod cache;

pub use cache::{Cache, CacheKey};

/// A deterministic transformation of one input into one output.
pub trait Converter: Send + Sync {
    /// Stable identifier, e.g. `"texture.detile"`. Part of the cache key.
    fn id(&self) -> &'static str;
    /// Bump whenever the output for a given input changes.
    fn version(&self) -> u32;
    fn convert(&self, input: &[u8]) -> anyhow::Result<Vec<u8>>;
}
