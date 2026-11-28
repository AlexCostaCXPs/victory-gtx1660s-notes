// VIctory: this file is part of VIctory, distributed under the GNU General Public License, version 3 or later (GPL-3.0-or-later). See the LICENSE file at the root of the repository.
//! One frame's D3D12 work recorded and run again, to find out what it costs
//! the card and to try changes on it in seconds (docs/replay.md).
//!
//! [`record`] hooks the host's D3D12 objects (`hook`) and keeps what a frame
//! needs; `snap` writes one frame down; [`play`] runs it. `wire` is what a
//! capture holds, `pso` pipeline descriptions, `states` the states textures
//! are in, `edit` the experiments, `gpu` helpers.

mod edit;
mod gpu;
mod hook;
pub mod play;
mod pso;
pub mod record;
mod snap;
mod states;
mod wire;
