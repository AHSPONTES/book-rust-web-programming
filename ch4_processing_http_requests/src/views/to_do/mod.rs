use super::path::Path;
use actix_web::web;

mod create;
mod edit;
mod get;
mod utils;

pub fn item_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: String::from("/item"),
    };

    app.route(
        &base_path.define(String::from("/create/{title}")),
        web::post().to(create::create),
    );

    app.route(
        &base_path.define(String::from("/get")),
        web::get().to(get::get),
    );

    app.route(
        &base_path.define(String::from("/edit")),
        web::put().to(edit::edit),
    );
}
