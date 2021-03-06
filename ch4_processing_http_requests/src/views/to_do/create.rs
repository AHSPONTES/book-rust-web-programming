use crate::diesel;
use diesel::prelude::*;

use actix_web::HttpRequest;
use actix_web::Responder;

use super::utils::return_state;
use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;

pub async fn create(req: HttpRequest) -> impl Responder {
    let title = &req.match_info().get("title").unwrap().to_string()[..];
    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

    let connection = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(title))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title.to_string(), token.user_id);

        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&connection);
    }

    return_state(&token.user_id)
}
