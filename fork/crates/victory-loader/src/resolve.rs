// VIctory: this file is part of VIctory, distributed under the GNU General Public License, version 3 or later (GPL-3.0-or-later). See the LICENSE file at the root of the repository.
//! How imports get their addresses.

use victory_pe::ImportSymbol;

/// Who provides a bound import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Provenance {
    /// Our implementation of the console API.
    Hle,
    /// The host's own implementation (e.g. Windows API sets that behave the
    /// same on console and PC).
    Host,
    /// A DLL the title ships, mapped like the executable.
    Title,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    Found {
        address: usize,
        provenance: Provenance,
    },
    /// Nobody implements it; the loader installs a reporting stub.
    Missing,
}

/// Supplies an address for each `dll!symbol` the title imports.
///
/// `dll` is passed exactly as written in the image; implementations should
/// compare case-insensitively.
pub trait ImportResolver {
    fn resolve(&self, dll: &str, symbol: &ImportSymbol) -> Resolution;
}
