use std::io::Empty;

use reqwest;
use scraper::{Html, Selector};
use umya_spreadsheet::{self, helper::coordinate::column_index_from_string};
fn main() {
    println!(
        "{}",
        get_sales("https://www.etsy.com/shop/DigitalBoutiqueFinds")
    );
    get_links();
}

fn to_unsinged(i: usize) -> u32 {
    return i.to_string().parse::<u32>().unwrap();
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
    let workbook = umya_spreadsheet::reader::xlsx::read(&path).unwrap();
    let sheet = workbook.get_sheet_by_name("Sayfa1").unwrap();
    let mut started = false;
    let mut i = 0;
    while true {
        if sheet
            .get_cell_value_by_column_and_row(1, to_unsinged(i))
            .get_value()
            == ""
        {
            started = false;
        } else if sheet
            .get_cell_value_by_column_and_row(1, to_unsinged(i))
            .get_value()
            != ""
        {
            started = true
        } else if started
            && sheet
                .get_cell_value_by_column_and_row(1, to_unsinged(i))
                .get_value()
                == ""
        {
            break;
        }

        println!(
            "{}",
            sheet
                .get_cell_value_by_column_and_row(1, to_unsinged(i))
                .get_value()
        );
        i = i + 1;
    }
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
