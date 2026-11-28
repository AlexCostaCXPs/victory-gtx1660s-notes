// VIctory: this file is part of VIctory, distributed under the GNU General Public License, version 3 or later (GPL-3.0-or-later). See the LICENSE file at the root of the repository.
fn main() {
    println!("cargo:rustc-link-arg-cdylib=/DELAYLOAD:victory-probe-delayed.dll");
    println!("cargo:rustc-link-lib=delayimp");
}
