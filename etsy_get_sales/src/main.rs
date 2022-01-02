use std::io::Empty;

use umya_spreadsheet;
use reqwest;
use scraper::{Html, Selector};
fn main() {
    println!(
        "{}",
        get_sales("https://www.etsy.com/shop/DigitalBoutiqueFinds")
    );
    get_links();
}

fn get_sales(link: &str) -> String {
    let response = reqwest::blocking::get(link).expect("Failed to get response");
    let html = Html::parse_document(&response.text().unwrap());
    let selector = Selector::parse(r#"a[rel="nofollow"]"#).unwrap();
    let sales = html
        .select(&selector)
        .next()
        .unwrap()
        .inner_html()
        .replace(",", "")
        .to_lowercase()
        .replace(" sales", "");
    return sales;
}

fn get_links() {
    let links: Vec<&str> = Vec::new();
    let path_str = format!(
        "{}\\2.xlsx",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let path = std::path::Path::new(&path_str);
    // println!("{}", path);
    let mut workbook = umya_spreadsheet::reader::xlsx::read(&path).unwrap();

    println!("{:?}", workbook.get_sheet_by_name("Sayfa1").unwrap().get_cell_value_by_column_and_row(1, 5).get_value());

    // let sheet_name= workbook.sheet_names()[0].clone();

    // let range = workbook.worksheet_range(&sheet_name).unwrap().expect("Failed to get range");

    // println!("{:?}", workbook);

    // if let Some(result) = iter.next() {
    //     let (label, value): (String, f64) = result?;
    //     assert_eq!(label, "celsius");
    //     assert_eq!(value, 22.2222);
    //     Ok(())
    // } else {
    //     Err(From::from("expected at least one record but got none"))
    // }
}
