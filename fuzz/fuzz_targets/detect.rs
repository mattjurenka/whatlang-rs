#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate whatlang;

use whatlang::{detect};

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            match detect(s) {
                Some(info) => {
                    info.lang();
                    info.script();
                    info.confidence();
                    info.is_reliable();
                },
                _ => {},
            }
        },
        _ => {}
    };
});
