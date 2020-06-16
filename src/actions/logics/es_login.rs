use easy_scraper::Pattern;
extern crate reqwest;

pub struct ESSession {
    pub user_id: String,
    pub php_sess_id: String,
}

pub async fn es_login(user_id: &str, password: &str) {
    let params = [("fLoginID", user_id), ("fPassword", password)];
    let client = reqwest::Client::new();
    let res = client.post("https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/loginExe_ver2.php")
        .form(&params)
        .send()
        .await
        .unwrap();
    let set_cookie_iter = res.headers();
    println!("{:?}", set_cookie_iter)
}