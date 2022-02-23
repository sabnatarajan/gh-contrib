#![allow(dead_code)]
use std::env;

use colored::Colorize;
use scraper::{Html,Selector};

#[derive(Debug, Clone)]
struct Contribution<'a> {
    date: &'a str,
    level: u8,
    count: u16,
}

const L0: (u8, u8, u8) = (22, 27, 34);
const L1: (u8, u8, u8) = (14, 68, 41);
const L2: (u8, u8, u8) = (0, 109, 50);
const L3: (u8, u8, u8) = (38, 166, 65);
const L4: (u8, u8, u8) = (57, 211, 83);

enum FillChars {
    FullBlock,
}

impl From<FillChars> for &str {
    fn from(char: FillChars) -> Self {
        match char {
            FillChars::FullBlock => "█",
        }
    }
}

fn main() {
    let mut args = env::args();
    let username = &args.nth(1).unwrap_or_else(|| "sabnatarajan".to_string());

    let url = format!("https://github.com/{}", username);
    let html = fetch_html(&url);

    let doc = Html::parse_document(html.as_str());
    let selector = Selector::parse(r#"rect[class="ContributionCalendar-day"]"#).unwrap();
    let mut contributions: Vec<Contribution> = vec![];
    for element in doc.select(&selector) {
        let value = element.value();
        let date = value.attr("data-date");
        if date.is_none() { continue; }

        let contribution = Contribution { 
            date: value.attr("data-date").unwrap(), 
            level: value.attr("data-level").unwrap_or("0").parse().unwrap(),
            count: value.attr("data-count").unwrap_or("0").parse().unwrap(),
        };
        contributions.push(contribution);
    }
    contributions.sort_by_key(|c| c.date);

    print_grid(contributions.to_vec());
}

fn print_char(char: &str, color: (u8, u8, u8)) {
    print!("{} ", char.truecolor(color.0, color.1, color.2));
}

fn fetch_html(url: &str) -> String {
    let resp = reqwest::blocking::get(url).ok();
    resp.unwrap().text().unwrap()
}

fn print_grid(contributions: Vec<Contribution>) {
    for day in 0..7 {
        for wk in 0..53 {
            let contrib = contributions.get(wk*7 + day).unwrap_or(&Contribution{date: "", level: u8::MAX, count: u16::MAX});
            let fill_char: &str = FillChars::FullBlock.into();
            match contrib.level {
                0 => print_char(fill_char,L0),
                1 => print_char(fill_char,L1),
                2 => print_char(fill_char,L2),
                3 => print_char(fill_char,L3),
                4 => print_char(fill_char,L4),
                _ => {},
            }
        }
        println!();
    }
}
