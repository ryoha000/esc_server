use scraper;
use scraper::{Html};
extern crate reqwest;
use anyhow::{Context, Result};

pub struct ClientMaterial<'a> {
    pub url: &'a str,
    pub header: Option<reqwest::header::HeaderMap>,
    pub form: Option<[(&'a str, &'a str); 1]>
}

pub async fn setup_get_client<'a>(material: ClientMaterial<'a>) -> Result<String> {
    let mut client = reqwest::Client::new();

    if let Some(header) = material.header {
        client = reqwest::Client::builder()
            .default_headers(header)
            .build()
            .with_context(|| "Cannnot make client")?;
    }

    let raw_html = client.get(material.url)
        .send()
        .await
        .with_context(|| "ErogameScape is not respond")?
        .text()
        .await
        .with_context(|| "ErogameScape is not respond")?;

    Ok(raw_html)
}

pub async fn setup_post_client<'a>(material: ClientMaterial<'a>) -> Result<String> {
    let mut client = reqwest::Client::new();

    if let Some(header) = material.header {
        client = reqwest::Client::builder()
            .default_headers(header)
            .build()
            .with_context(|| "Cannnot make client")?;
    }

    let raw_html = client.post(material.url)
        .form(&material.form)
        .send()
        .await
        .with_context(|| "ErogameScape is not respond")?
        .text()
        .await
        .with_context(|| "ErogameScape is not respond")?;

    Ok(raw_html)
}

pub async fn execute_on_es(query: String) -> Result<scraper::html::Html> {
    let material = ClientMaterial {
        url: "https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/sql_for_erogamer_form.php",
        header: None,
        form: Some([("sql", &query)])
    };
    let text = setup_post_client(material).await?;

    Ok(Html::parse_fragment(&text))
}

pub fn option_bool_from_tf(b: String) -> Option<bool> {
    match &*b {
        "t" => Some(true),
        "f" => Some(false),
        _ => None
    }
}

pub fn option_i32_from_string(b: String) -> Option<i32> {
    match b.parse() {
        Ok(b) => Some(b),
        _ => None
    }
}

pub fn option_date_from_string(b: String) -> Option<chrono::NaiveDate> {
    match b.parse() {
        Ok(b) => Some(b),
        _ => None
    }
}

pub fn option_datetime_from_string(b: String) -> Option<chrono::NaiveDateTime> {
    match b.parse() {
        Ok(b) => Some(b),
        _ => None
    }
}

pub fn option_string_from_string(b: String) -> Option<String> {
    match &*b {
        "" => None,
        _ => Some(b)
    }
}

pub fn i32_from_string(b: String) -> i32 {
    match b.parse() {
        Ok(b) => b,
        _ => 0
    }
}
