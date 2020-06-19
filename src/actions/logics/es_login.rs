//extern crate reqwest;
use reqwest::header;
use anyhow::{Context, Result};

#[derive(Debug)]
struct HiddenForm {
    cookie: header::HeaderValue,
    token: String,
}

pub async fn es_login(user_id: &str, password: &str) -> Result<header::HeaderValue> {
    let hidden_form = get_token().await?;
    let params = [("fLoginID", user_id), ("fPassword", password), ("_token", &hidden_form.token), ("sorce_url", "/~ap2/ero/toukei_kaiseki/")];

    let mut headers = header::HeaderMap::new();
    headers.insert("cookie", hidden_form.cookie);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .with_context(|| "failed to create client")?;

    let res = client.post("https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/loginExe_ver2.php")
        .form(&params)
        .send()
        .await
        .with_context(|| "ErogameScape is not respond")?;

    let _cookie = res.headers().get_all("set-cookie").iter();

    if _cookie.size_hint().0 < 3 {
        anyhow::bail!("user_id or password is invalid")
    }

    let mut concat_cookie = String::new();
    for c in _cookie {
        println!("{:?}", c);
        let split_cookie: Vec<&str> = c.to_str().unwrap().split(";").collect();
        concat_cookie += split_cookie.get(0).with_context(|| "Can not get cookie")?;
        concat_cookie += "; ";
    }
    println!("{:?}", concat_cookie);
    Ok(header::HeaderValue::from_str(&concat_cookie).unwrap())
}

use scraper::{Html, Selector};

async fn get_token() -> Result<HiddenForm> {
    let res = reqwest::get("https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/login.php")
        .await?;

    let res_cookie = res.headers().get("set-cookie").with_context(|| "ErogameScape is not respond")?;

    let cookie = res_cookie.clone();

    let res_text = res.text().await.with_context(|| "ErogameScape is not respond")?;
    let fragment = Html::parse_fragment(&res_text);
    let input_selector = Selector::parse("input").unwrap();
    let input = fragment.select(&input_selector);

    for element in input {
        if let Some(name) = element.value().attr("name") {
            if name == "_token" {
                if let Some(value) = element.value().attr("value") {
                    return Ok(HiddenForm { cookie: cookie, token: value.to_string() })
                }
            }
        }
    }
    anyhow::bail!("Erogame Scape design is chenged")
}
