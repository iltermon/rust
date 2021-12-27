
use std::error::Error;

use reqwest;
use soup::prelude::*;

fn main() {
    let response = reqwest::blocking::get("https://www.etsy.com/shop/DigitalBoutiqueFinds")
        .expect("Failed to get response");
    let soup = Soup::from_reader(response);
    let some_text = soup.unwrap();
    println!("{:?}", some_text.tag("h1").attr("class", "wt-text-caption wt-no-wrap").);

}