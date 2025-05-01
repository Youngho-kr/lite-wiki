use axum::response::Html;
use crate::auth::AuthUser;
use crate::handlers::render_template_list_html;

use crate::storage::list_template_names;

pub async fn render_template_list(
    AuthUser(username): AuthUser
) -> Html<String> {
    let names = list_template_names();

    Html(render_template_list_html(&names, &username))
}