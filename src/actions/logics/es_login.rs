extern crate reqwest;
use reqwest::header;

#[derive(Debug)]
struct HiddenForm {
    cookie: header::HeaderValue,
    token: String,
}

pub async fn es_login(user_id: &str, password: &str) -> header::HeaderMap {
    let hidden_form = get_token().await;
    let params = [("fLoginID", user_id), ("fPassword", password), ("_token", &hidden_form.token), ("sorce_url", "/~ap2/ero/toukei_kaiseki/")];

    let mut headers = header::HeaderMap::new();
    headers.insert("cookie", hidden_form.cookie);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = client.post("https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/loginExe_ver2.php")
        .form(&params)
        .send()
        .await
        .unwrap();

    let mut header_with_cookie = header::HeaderMap::new();
    let _cookie = res.headers().get_all("set-cookie").iter();
    let mut concat_cookie = String::new();
    for c in _cookie {
        println!("{:?}", c);
        let split_cookie: Vec<&str> = c.to_str().unwrap().split(";").collect();
        concat_cookie += split_cookie.get(0).unwrap();
        concat_cookie += "; ";
    }
    header_with_cookie.insert("cookie", header::HeaderValue::from_str(&concat_cookie).unwrap());
    println!("{:?}", header_with_cookie);
    header_with_cookie
}

use scraper::{Html, Selector};

async fn get_token() -> HiddenForm {
    let res = reqwest::get("https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/login.php")
        .await
        .unwrap();

    let res_cookie = res.headers().get("set-cookie").unwrap().clone();

    let res_text = res.text().await.unwrap();
    let fragment = Html::parse_fragment(&res_text);
    let input_selector = Selector::parse("input").unwrap();
    let input = fragment.select(&input_selector);

    for element in input {
        if let Some(name) = element.value().attr("name") {
            if name == "_token" {
                if let Some(value) = element.value().attr("value") {
                    return HiddenForm { cookie: res_cookie, token: value.to_string() }
                }
            }
        }
    }

    HiddenForm { cookie: res_cookie, token: String::from("") }
}

// "set-cookie":
// "PHPSESSID=4p0g2i65bmgc7t2lrr88prk38do0u45tag64h7tku8701epel4t2e5f9bisaokn9ldi1v9m6b0o2b6echh3033nook4gc5g9pham2augsp3rnqbc5v07as6glnd1lj5g;
// expires=Tue, 23-Jun-2020 09:08:31 GMT;
// Max-Age=604800;
// path=/; HttpOnly;
// SameSite=Lax",

// "set-cookie":
// "CONTENTS_VISIT=1;
// expires=Thu, 16-Jul-2020 09:08:31 GMT;
// Max-Age=2592000",

// "set-cookie":
// "user_id=ryoha;
// expires=Thu, 16-Jul-2020 09:08:31 GMT;
// Max-Age=2592000"