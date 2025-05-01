use axum::response::Html;
use crate::handlers::render_template_list_html;

use crate::storage::list_template_names;

pub async fn render_template_list() -> Html<String> {
    let names = list_template_names();

    Html(render_template_list_html(&names, "user"))
}