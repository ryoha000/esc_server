use easy_scraper::Pattern;
extern crate reqwest;

pub async fn get_all_games(header: reqwest::header::HeaderMap) {
    println!("{:?}", header);
    let pat = Pattern::new(
        r#"
<tr>
    <th>{{id}}</th>
    <th>{{gamename}}</th>
    <th>{{furigana}}</th>
    <th>{{sellday}}</th>
    <th>{{brandname}}</th>
    <th>{{median}}</th>
    <th>{{stdev}}</th>
    <th>{{creater}}</th>
    <th>{{kansouurl}}</th>
    <th>{{checked}}</th>
    <th>{{hanbaisuu}}</th>
    <th>{{average2}}</th>
    <th>{{median2}}</th>
    <th>{{count2}}</th>
    <th>{{comike}}</th>
    <th>{{shoukai}}</th>
    <th>{{model}}</th>
    <th>{{checked2}}</th>
    <th>{{erogame}}</th>
    <th>{{galge}}</th>
    <th>{{elfics}}</th>
    <th>{{banner_url}}</th>
    <th>{{admin_checked}}</th>
    <th>{{max2}}</th>
    <th>{{min2}}</th>
    <th>{{gyutto_enc}}</th>
    <th>{{gyutto_id}}</th>
    <th>{{dmm}}</th>
    <th>{{dmm_genre}}</th>
    <th>{{dmm_genre_2}}</th>
    <th>{{erogametokuten}}</th>
    <th>{{total_play_time_median}}</th>
    <th>{{time_before_understanding_fun_median}}</th>
    <th>{{dlsite_id}}</th>
    <th>{{dlsite_domain}}</th>
    <th>{{the_number_of_uid_which_input_pov}}</th>
    <th>{{the_number_of_uid_which_input_play}}</th>
    <th>{{total_pov_enrollment_of_a}}</th>
    <th>{{total_pov_enrollment_of_b}}</th>
    <th>{{total_pov_enrollment_of_c}}</th>
    <th>{{trial_url}}</th>
    <th>{{trial_h}}</th>
    <th>{{http_response_code}}</th>
    <th>{{okazu}}</th>
    <th>{{axis_of_soft_or_hard}}</th>
    <th>{{trial_url_update_time}}</th>
    <th>{{genre}}</th>
    <th>{{twitter}}</th>
    <th>{{erogetrailers}}</th>
    <th>{{tourokubi}}</th>
    <th>{{digiket}}</th>
    <th>{{dmm_sample_image_count}}</th>
    <th>{{dlsite_sample_image_count}}</th>
    <th>{{gyutto_sample_image_count}}</th>
    <th>{{digiket_sample_image_count}}</th>
    <th>{{twitter_search}}</th>
    <th>{{tgfrontier}}</th>
    <th>{{gamemeter}}</th>
    <th>{{twitter_data_widget_id}}</th>
    <th>{{twitter_data_widget_id_before}}</th>
    <th>{{twitter_data_widget_id_official}}</th>
    <th>{{masterup}}</th>
    <th>{{masterup_tourokubi}}</th>
    <th>{{steam}}</th>
    <th>{{dlsite_rental}}</th>
    <th>{{dmm_subsc}}</th>
    <th>{{surugaya_1}}</th>
    <th>{{surugaya_2}}</th>
    <th>{{surugaya_1_back_image}}</th>
    <th>{{surugaya_2_back_image}}</th>
    <th>{{count_all}}</th>
</tr>
"#,
    )
    .unwrap();

    let client = reqwest::Client::builder()
        .default_headers(header)
        .build()
        .unwrap();

    let res = client.get("https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/usersql_exec.php?sql_id=2726")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let ms = pat.matches(&res);
    println!("{:#?}", ms);
}
