mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod p1 {
    use super::*;

    /*
    #[wasm_bindgen]
    pub struct P1Solution {
        parsed: String,
    }

    #[wasm_bindgen]
    impl P1Solution {
        #[wasm_bindgen(getter)]
        pub fn parsed(&self) -> String {
            self.parsed.clone()
        }
    }
    */

    #[wasm_bindgen]
    pub fn solvep1(input: String) -> Result<(), JsValue> {
        utils::set_panic_hook();

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let parsed = parse(&input);
        let p1parsed = document.get_element_by_id("p1parsed").expect("missing p1parsed");
        p1parsed.set_text_content(Some(&format!("{:?}", parsed)));

        let totals = totals(&parsed);
        let p1totals= document.get_element_by_id("p1totals").expect("missing p1totals");
        p1totals.set_text_content(Some(&format!("{:?}", totals)));

        let p1solution = document.get_element_by_id("p1solution").unwrap();
        p1solution.set_text_content(Some(&format!("{}", totals.iter().max().unwrap())));

        Ok(())
    }

    pub fn parse(s: &str)-> Vec<Option<u32>> {
        s.lines().map(str::trim).map(|l| if l.is_empty() {
            None
        } else {
            Some(u32::from_str_radix(l, 10).unwrap())
        }).collect()
    }

    pub fn totals(input: &[Option<u32>]) -> Vec<u32> {
        let groups = input.split(Option::is_none);
        let mut res = Vec::new();
        for g in groups {
            res.push(g.iter().map(|n| *n.as_ref().unwrap()).sum());
        }
        res
    }

    #[cfg(test)]
    mod p1test {
        use super::*;

        #[test]
        fn p1_example() {
            //assert_eq!(&p1solution(), "oops");
        }
    }
}
