use requests::Response;
use scraper::{Html, Selector};

fn main() {
    let response = get_website_document("https://egamersworld.com/dota2/team/whoops-Vy7SP9Uww");
    let raw_doc = response.text().unwrap();
    assert!(response.status_code().is_success());

    let doc = Html::parse_document(&raw_doc);
    let teams_selector = Selector::parse("#finished_list > div:nth-child(1) > a > div.match-half > div.match-inner .team_name").unwrap();
    let scores_selector = Selector::parse("#finished_list > div:nth-child(1) > a > div.match-half > div.match-inner > div.match-data > div > div span").unwrap();
    let teams = get_text_from_selector(&teams_selector, &doc);
    let scores = get_text_from_selector(&scores_selector, &doc);

    println!("Latest match {} vs {} was {}:{}", teams[0], teams[1], scores[0], scores[1]);
}

fn get_text_from_selector<'a>(selector: &Selector, doc: &'a Html) -> Vec<&'a str>{
    let mut values = Vec::new();
    for value in doc.select(&selector){
        let value_text = value.text().collect::<Vec<_>>();
        values.push(value_text[0].trim());
    }
    return values;
}

fn get_website_document(url: &str) -> Response{
    return requests::get(url).unwrap();
}
