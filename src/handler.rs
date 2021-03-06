use rocket_contrib::Json;
use padding;
use params;

#[get("/leftpad?<request>")]
fn leftpad(
    request: params::PaddingRequestUrl,
) -> Result<Json<padding::PaddingResponse>, &'static str> {

    Ok(Json(padding::left(&params::read_params(&request)?)))
}

#[get("/rightpad?<request>")]
fn leftpad_right(
    request: params::PaddingRequestUrl,
) -> Result<Json<padding::PaddingResponse>, &'static str> {
    Ok(Json(padding::right(&params::read_params(&request)?)))
}
