#![windows_subsystem = "windows"]
use fltk::{
    enums::{Align, Color, FrameType},
    prelude::*,
    *,
};
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use std::error::Error;

const BLUE: Color = Color::from_hex(0x42A5F5);
const SEL_BLUE: Color = Color::from_hex(0x2196F3);
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 600;
const DEV_URL: &str = "https://customer-survey-api-dev.lingyue-digital.com";
const INT_URL: &str = "https://customer-survey-api-int.lingyue-digital.com";
const STG_URL: &str = "https://customer-survey-api-stg.lingyue-digital.com";
const PROD_URL: &str = "https://customer-survey-api.lingyue-digital.com";

const SYSTEM_PATH: &str = "/customer/api/v1/survey/protected/v1/get-question-detail";


fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("查询问卷详情");

    let mut bar = frame::Frame::new(0, 0, WIDTH, 30, " 参数配置").with_align(Align::Center);

    let survey_id_label = frame::Frame::default()
        .with_size(60, 20)
        .below_of(&bar, 10)
        .with_label("surveyId:");

    let survey_id = input::Input::default()
        .with_size(200, 20)
        .right_of(&survey_id_label, 10);

    let business_id_label = frame::Frame::default()
        .with_size(60, 20)
        .right_of(&survey_id, 20)
        .with_label("businessId:");
    let business_id = input::Input::default()
        .with_size(150, 20)
        .right_of(&business_id_label, 10);

    let profiles_label = frame::Frame::default()
        .with_size(60, 20)
        .right_of(&business_id, 40)
        .with_label("运行环境:");

    let mut profiles = menu::Choice::default()
        .with_size(80, 20)
        .right_of(&profiles_label, 10);

    let mut bar_survey = frame::Frame::new(0, 100, WIDTH, 30, "问卷详情").with_align(Align::Center);
    let mut survey_buf = text::TextBuffer::default();
    let mut survey_txt = text::TextEditor::new(0, 140, WIDTH, 360, "");
    survey_txt.set_buffer(survey_buf.clone());

    let mut but = button::Button::new(WIDTH - 100, HEIGHT - 100, 60, 60, "查询");

    win.end();
    win.make_resizable(true);
    win.show();

    app::background(255, 255, 255);
    app::set_visible_focus(false);

    bar.set_frame(FrameType::FlatBox);
    bar.set_label_size(18);
    bar.set_label_color(Color::White);
    bar.set_color(BLUE);
    bar.draw(|b| {
        draw::set_draw_rgb_color(211, 211, 211);
        draw::draw_rectf(0, b.height(), b.width(), 3);
    });

    bar_survey.set_frame(FrameType::FlatBox);
    bar_survey.set_label_size(18);
    bar_survey.set_label_color(Color::White);
    bar_survey.set_color(BLUE);
    bar_survey.draw(|b| {
        draw::set_draw_rgb_color(211, 211, 211);
        draw::draw_rectf(0, b.height(), b.width(), 3);
    });
    profiles.add_choice("dev");
    profiles.add_choice("int");
    profiles.add_choice("stg");
    profiles.add_choice("prod");
    profiles.set_value(0);

    but.set_color(BLUE);
    but.set_selection_color(SEL_BLUE);
    but.set_label_color(Color::White);
    but.set_frame(FrameType::OShadowBox);

    let survey_id_value = survey_id.clone();
    let business_id_value = business_id.clone();

    but.set_callback(move |_| {
        let survey_id_value = survey_id_value.value();
        let business_id_value = business_id_value.value();
        let profiles_value = match profiles.value() {
            0 => DEV_URL,
            1 => INT_URL,
            2 => STG_URL,
            3 => PROD_URL,
            _ => "",
        };
        let response = request_data(survey_id_value, business_id_value, profiles_value).unwrap();
        // let result = serde_json::to_string_pretty(&response).unwrap();
        survey_buf.set_text(response.as_str())
    });

    app.run().unwrap();
}

fn request_data(
    survey_id: String,
    business_id: String,
    profile: &str,
) -> Result<String, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-user-agent",
        "pc;bmw-website;1.0.0(1);cn".parse().unwrap(),
    );
    headers.insert(
        "Content-Type",
        "application/json; charset=UTF-8".parse().unwrap(),
    );
    println!("{:?}", headers);
    let result = Client::new()
        .get(
            profile.to_owned()
                + SYSTEM_PATH
                + "?surveyId="
                + &survey_id
                + "&businessId="
                + &business_id,
        )
        .headers(headers)
        .send()
        .unwrap();
    Ok(result.text().unwrap())
}
