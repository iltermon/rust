use chrono;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use umya_spreadsheet::{self, Worksheet};
fn main() {
    println!("1");
    let current_dir = std::env::current_dir().unwrap();
    let mut path_str = current_dir.clone();
    path_str.push("2.xlsx");
    println!("{:?}", chrono::offset::Local::now());
    let path = std::path::Path::new(&path_str);
    let workbook = umya_spreadsheet::reader::xlsx::read(&path).unwrap();
    let sheet = workbook.get_sheet_by_sheet_id("1").unwrap();
    let links_with_names = get_links_with_names(sheet);
    for i in 0..links_with_names[0].len() {
        println!("{}", get_sales(&links_with_names[0][i]));
        println!("{}", links_with_names[1][i]);
    }
    println!("{:?}", chrono::offset::Local::now());
}

fn to_unsinged(i: usize) -> u32 {
    return i.to_string().parse::<u32>().unwrap();
}
fn get_sales(link: &str) -> String {
    let response = reqwest::blocking::get(link).expect("Failed to get response");
    let html = Html::parse_document(&response.text().unwrap());
    let selector = Selector::parse(r#"span[class="wt-text-caption wt-no-wrap"]"#).unwrap();

    let sales = html
        .select(&selector)
        .next()
        .unwrap()
        .inner_html()
        .replace(",", "")
        .to_lowercase()
        .replace(" sales", "");

    return Regex::new(r"(<.*?>)")
        .unwrap()
        .replace_all(&sales, "")
        .to_string();
}

fn get_links_with_names(sheet: &Worksheet) -> Vec<Vec<String>> {
    let mut links: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut links_with_names: Vec<Vec<String>> = Vec::new();
    let mut started = false;
    let mut i = 0;
    loop {
        if !started
            && sheet
                .get_cell_value_by_column_and_row(1, to_unsinged(i))
                .get_value()
                .is_empty()
        {
            i += 1;
            continue;
        } else if !sheet
            .get_cell_value_by_column_and_row(1, to_unsinged(i))
            .get_value()
            .is_empty()
        {
            started = true
        } else if sheet
            .get_cell_value_by_column_and_row(1, to_unsinged(i))
            .get_value()
            .is_empty()
        {
            break;
        }

        links.push(
            sheet
                .get_cell_value_by_column_and_row(1, to_unsinged(i))
                .get_value()
                .to_string(),
        );
        names.push(
            sheet
                .get_cell_value_by_column_and_row(2, to_unsinged(i))
                .get_value()
                .to_string(),
        );

        i += 1;
    }
    links_with_names.push(links.clone());
    links_with_names.push(names.clone());

    return links_with_names;
}
