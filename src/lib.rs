use std::{fmt::Write, io::Write as CharWriter};

use md5::Context;
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use js_sys::Array;

#[wasm_bindgen]
pub struct Md5Digest {
    hasher: Context,
}

#[wasm_bindgen]
impl Md5Digest {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Md5Digest {
        Md5Digest { hasher: Context::new() }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hasher.write(data).unwrap();
    }

    pub fn finalize(self) -> String {
        static CH: &'static [u8] = b"0123456789abcdef";
        let ans = self.hasher.compute().0;
        let mut out = String::with_capacity(ans.len() << 1);
        for &u in ans.iter() {
            out.write_char(CH[(u>>4)as usize].into()).unwrap();
            out.write_char(CH[(u &0xf) as usize].into()).unwrap();
        }
        out
    }

    pub fn finalize_bytes(self) -> Uint8Array {
        let ans = self.hasher.compute().0;
        let array: Array = ans.iter().map(|x| JsValue::from(*x as u8)).collect();
        Uint8Array::new(&array)
    }
}
