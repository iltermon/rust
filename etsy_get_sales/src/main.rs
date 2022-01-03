use reqwest;
use scraper::{Html, Selector};
use umya_spreadsheet::{self, Worksheet};
fn main() {
    let path_str = format!(
        "{}\\2.xlsx",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let path = std::path::Path::new(&path_str);

    let workbook = umya_spreadsheet::reader::xlsx::read(&path).unwrap();
    let sheet = workbook.get_sheet_by_name("Sayfa1").unwrap();
    println!(
        "{}",
        get_sales("https://www.etsy.com/shop/DigitalBoutiqueFinds")
    );
    for i in 0..get_links_with_names(sheet)[0].len() {
        println!("{}", get_links_with_names(sheet)[0][i]);
        println!("{}", get_links_with_names(sheet)[1][i]);
        break;
    }
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
        links_with_names.push(links.clone());
        links_with_names.push(names.clone());
        i += 1;
    }
    return links_with_names;
}
