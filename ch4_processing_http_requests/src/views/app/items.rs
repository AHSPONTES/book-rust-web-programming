use super::content_loader::read_file;
use actix_web::HttpResponse;

pub async fn items() -> HttpResponse {
    let mut html_data = read_file("./templates/main.html");
    let javascript_data = read_file("./javascript/main.js");
    let css_base_data = read_file("./css/base.css");
    let css_main_data = read_file("./css/main.css");

    html_data = html_data.replace("JAVASCRIPT;", &javascript_data);
    html_data = html_data.replace("{{BASE_CSS}}", &css_base_data);
    html_data = html_data.replace("{{CSS}}", &css_main_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
