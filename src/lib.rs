#[macro_use]
extern crate lazy_static;
extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

lazy_static! {
    static ref DICT_CN_NUM: HashMap<u8, (&'static str, &'static str)> = vec![
        (0, ("零", "零")),
        (1, ("一", "壹")),
        (2, ("二", "贰")),
        (3, ("三", "叁")),
        (4, ("四", "肆")),
        (5, ("五", "伍")),
        (6, ("六", "陆")),
        (7, ("七", "柒")),
        (8, ("八", "捌")),
        (9, ("九", "玖")),
    ]
    .into_iter()
    .collect();
    static ref DICT_CN_SEC: HashMap<u8, (&'static str, &'static str)> = vec![
        (0, ("", "")),
        (1, ("十", "拾")),
        (2, ("百", "佰")),
        (3, ("千", "仟")),
        (4, ("万", "万")),
        (5, ("亿", "亿")),
    ]
    .into_iter()
    .collect();
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Dict {
    Normal,
    Capitalized,
}

impl Dict {
    fn get_num(&self, num: u8) -> &str {
        let temp = DICT_CN_NUM[&num];
        match *self {
            Dict::Normal => temp.0,
            Dict::Capitalized => temp.1,
        }
    }

    fn get_sec(&self, num: u8) -> &str {
        let temp = DICT_CN_SEC[&num];
        match *self {
            Dict::Normal => temp.0,
            Dict::Capitalized => temp.1,
        }
    }
}

fn convert(num: usize, dict: Dict) -> String {
    fn get_digits(num: usize) -> Vec<u8> {
        let mut tmp = num;
        (0..((num as f64).log10() + 1.0) as usize)
            .map(|_| {
                let tmp_inner = (tmp % 10) as u8;
                tmp /= 10;
                tmp_inner
            })
            .into_iter()
            .collect()
    }

    fn parse_section(digits: &[u8], dict: Dict, order: usize) -> String {
        // Recursion for minor sections (tens of thousands section)
        fn parse_minor_section(digits: &[u8], dict: Dict, minor_order: usize) -> String {
            // Divide the sections
            let minor_slice_length = if digits.len() > 4 { 4 } else { digits.len() };
            let minor_rest = if (digits.len() - minor_slice_length) > 0 { parse_minor_section (&digits[minor_slice_length..], dict, minor_order + minor_slice_length) } else { "".to_owned() };

            // Section suffix
            let minor_suffix = dict.get_sec(4).repeat(minor_order / 4);

            // Replacing
            let mut digits_replaced: Vec<String> = (0..minor_slice_length).map(|minor_position| {
                dict.get_num(digits[minor_position]).to_owned() + if digits[minor_position] != 0 { dict.get_sec(minor_position as u8) } else { "" }
            }).into_iter().collect();
            digits_replaced.reverse();
            let mut combined = digits_replaced.into_iter().collect::<String>();

            // Remove trailing zeros
            let trailing_zero = dict.get_num(0).to_owned() + dict.get_num(0);
            while combined.contains(&trailing_zero) {
                combined = combined.replace(&trailing_zero, dict.get_num(0));
            }
            if combined.ends_with(dict.get_num(0)) { combined.pop(); };

            // Return result
            (minor_rest + &combined + &minor_suffix)
        }

        // Recursion (hundreds of millions section)
        let slice_length = if digits.len() > 8 { 8 } else { digits.len() };
        let rest = if (digits.len() - slice_length) > 0 {
            parse_section(&digits[slice_length..], dict, order + slice_length)
        } else {
            "".to_owned()
        };

        // Get suffix
        let suffix = dict.get_sec(5).repeat(order / 8);

        // Get result for minor sections
        let minor = parse_minor_section(&digits[..slice_length], dict, 0);

        (rest + &minor + &suffix)
    }

    let digits: Vec<u8> = get_digits(num);
    let mut result = parse_section(&digits[..], dict, 0);

    // Remove leadning `one` when the number starts with 10
    if result.starts_with(&(dict.get_num(1).to_owned() + dict.get_sec(1))) {
        result = result.replacen(dict.get_num(1), "", 1)
    };

    result
}

#[wasm_bindgen]
pub fn convert_s(num: usize) -> String {
    convert(num, Dict::Normal)
}

#[wasm_bindgen]
pub fn convert_t(num: usize) -> String {
    convert(num, Dict::Capitalized)
}
