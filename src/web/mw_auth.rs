use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use crate::web::AUTH_TOKEN;
use crate::error::{Result, Error};



pub async fn mw_require_auth<B>(
    cookies: Cookies,
    request: Request<B>,
    next: Next<B>)
    -> Result<Response> {

    println!("->> {:<12} - mw_require_auth ", "MIDDLEWARE");

    let auth = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    //TODO: Token component validation
    let (u_id, exp, sign) = auth
        .ok_or(Error::AuthFailNoAuthCookieFound)
        .and_then(parse_token)?;


    Ok(next.run(request).await)
}

//Parsing token of form user-[user-id].expirationofthetoken.signature
//Returns a tuple of form (userid: u64, expiration: String, signature: String)

fn parse_token(token: String) -> Result<(u64, String, String)>{

    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    )
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let u_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((u_id, exp.to_string(), sign.to_string()))

}