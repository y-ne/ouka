use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn root() -> Html<String> {
    let template = IndexTemplate;
    Html(template.render().unwrap())
}
