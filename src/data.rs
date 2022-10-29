
use surrealdb::Datastore;
use surrealdb::Session;
pub enum Error {
    DBError(surrealdb::Error),
}

impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Self {
        Error::DBError(err)
    }
}

pub async fn fetch_incentives(count: u32, page: u32) -> Result<String, Error> {
    let db = Datastore::new("tikv://localhost:8000").await?;

    let session = Session::for_kv().with_ns("test").with_db("test");
    let query = format!("SELECT * FROM incentives limit {} start {};", count, count * page); 
    let results = db.execute(query.as_str(), &session, None, false).await?;


    Ok(results[0].speed())
}