<!--
    Copyright © 2019 Alastair Feille
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->
# mutos
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](#outside-code)

An attempt at a microkernel written in Rust

# Building
(Requires cargo-xbuild, bootimage, and hjson)
```
./build.sh
```

# Running
(Requires qemu)
```
bootimage run
```

## License

Licenses are applied on a per-file basis. Check the header of each file to know which license applies.

If a newly created file is missing a license header, then the MPL license will apply. This is likely in error, so please contact me if you find a file missing license information.

### Original code

mutos is primarily distributed under the terms of the Mozilla Public License 2.0. All original code will be licensed under the MPL 2.0.

### Outside code

Most outside code is licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed under the license of the existing file. Newly created files should be licensed under the MPL 2.0.
