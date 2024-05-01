use std::collections::HashMap;

use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

///! [BasicMetadata] holds all fields which are required to be set by the [Open Graph protocol] https://ogp.me/.
#[derive(Debug, Serialize, Deserialize)]
pub struct BasicMetadata {
    pub title: Option<String>,
    pub content_type: Option<String>,
    pub image: Option<String>,
    pub url: Option<String>,
}

///! The [ExpandedMetadata] is an expanded version of [BasicMetadata] which contains more options which are also frequently used on each site.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedMetadata {
    pub title: String,
    pub image: String,
    pub content_type: String,
    pub url: String,

    // Additional options which could be used on the website, for further meta tags create a pull request and add them here.
    pub locale: Option<String>,
    pub content: Option<String>,
    pub description: Option<String>,
    pub site_name: Option<String>,
    pub image_secure_url: Option<String>,
    pub image_height: Option<String>,
    pub image_width: Option<String>,
    pub image_alt: Option<String>,
    pub image_type: Option<String>,
}

pub async fn extract_from_url(url: &String) -> ExpandedMetadata {
    let html = fetch_url(url).await;
    let meta = extract_meta(html);
    return meta;
}

pub async fn fetch_url(url: &String) -> String {
    let c = Client::new();

    let res = c.get(url).send().await.unwrap();
    if res.status() != 200 {}
    return res.text().await.unwrap();
}

pub fn extract_meta(html: String) -> ExpandedMetadata {
    let document = Html::parse_document(&html);
    let selector = Selector::parse("meta").unwrap();
    let mt: Vec<String> = document
        .select(&selector)
        .map(|element| element.html())
        .collect();

    let new: Vec<String> = mt
        .iter()
        .filter(|e| e.contains("og:"))
        .map(|e| e.to_owned())
        .collect();

    let meta_tag_regex = Regex::new(r#"<meta\s+(?:property="(?P<property>[^"]+)"\s+content="(?P<content>[^"]+)"|content="(?P<content2>[^"]+)"\s+property="(?P<property2>[^"]+)")\s*/?>"#).unwrap();
    let mut properties: HashMap<String, String> = HashMap::new();

    for caps in meta_tag_regex.captures_iter(new.join("\n").as_str()) {
        let property = caps
            .name("property")
            .or(caps.name("property2"))
            .unwrap()
            .as_str();
        let content = caps
            .name("content")
            .or(caps.name("content2"))
            .unwrap()
            .as_str();
        properties.insert(property.to_string(), content.to_string());
    }

    ExpandedMetadata {
        title: properties["og:title"].clone(),
        image: properties["og:image"].clone(),
        content_type: properties["og:type"].clone(),
        url: properties["og:url"].clone(),

        locale: properties.get("og:locale").cloned(),
        content: properties.get("og:content").cloned(),
        description: properties.get("og:description").cloned(),
        site_name: properties.get("og:site_name").cloned(),
        image_secure_url: properties.get("og:image:secure_url").cloned(),
        image_height: properties.get("og:image:height").cloned(),
        image_width: properties.get("og:image:width").cloned(),
        image_alt: properties.get("og:image:alt").cloned(),
        image_type: properties.get("og:image:type").cloned(),
    }
}

#[tokio::test]
async fn test_extract_from_url() {
    let test_url = "https://shibaskitchen.de/wraps-gefuellt-mit-chicken-shawarma/".to_string();

    let x = extract_from_url(&test_url).await;
    println!("Res: {:?}", x);
}
