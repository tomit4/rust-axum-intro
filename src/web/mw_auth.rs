use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use crate::ctx::Ctx;
use axum::http::{Response, request::Parts};
use axum::extract::{FromRequestParts, Request};
use axum::middleware::Next;
use tower_cookies::{Cookie, Cookies};
use axum::body::Body;
use lazy_regex::regex_captures;
use async_trait::async_trait;

pub async fn mw_require_auth (
    // cookies: Cookies,
    ctx: Result<Ctx>,
    req: Request,
    next: Next
) -> Result<Response<Body>> {
    println!("->> {:12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    // This allows us to simply call the ctx function 
    // from_request_parts below to basically check
    // the auth cookie on any route we call it on 
    ctx?;

    // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: Real auth-token parsing & validation
    // auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    // let (_user_id, _exp, _sign) = auth_token
        // .ok_or(Error::AuthFailNoAuthTokenCookie)
        // .and_then(parse_token)?;

    // TODO: Token components validation (and the above destructured args would be used).

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver (
    // State(mc): State<ModelController>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>.
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Token components validations (usually expensive operation here).
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err()
        && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) 
    {
            cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}


// region:  --- Ctx Extractor (allows us to run )
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()

        /*
        // User the cookies extractor.
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // Parse token.
        let (user_id, _exp, _sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;
        // TODO: Token components validation.
        Ok(Ctx::new(user_id))
        */
    }
}
// endregion:  --- Ctx Extractor

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)

fn parse_token(token: String) -> Result<(u64, String, String)> {
    // each tuple () within the regex matches the destructured vales after _whole
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
