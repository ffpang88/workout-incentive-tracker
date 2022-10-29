#[macro_use] extern crate rocket;

mod data;

#[get("/")]
async fn index() -> String {
    let output = match data::fetch_incentives(10, 0).await
    {
        Ok(result) => result,
        Err(error) => match error {
            data::Error::DBError(db_err) => format!("db error! {}", db_err),
        }
    };

    output
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
