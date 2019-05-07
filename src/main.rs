/*
    Copyright © 2019 Alastair Feille
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/
#![feature(lang_items)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
fn _start() -> !
{
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {}
}
