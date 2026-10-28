use axum::{extract::State, http::StatusCode, routing::get, Router};
use sqlx::MySqlPool;
use sqlx::FromRow;
use tower_http::services::ServeDir;

#[derive(sqlx::FromRow)]
struct City {
    #[sqlx(rename = "ID")]
    id: i32,
    #[sqlx(rename = "Name")]
    name: String,
    #[sqlx(rename = "Population")]
    population: i32,
    #[sqlx(rename = "District")]
    district: String,
    #[sqlx(rename = "CountryCode")]
    country_code: String
}

async fn database_check(State(pool): State<MySqlPool>) -> Result<&'static str, StatusCode> {
    let _: i32 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&pool)
        .await
        .map_err(|error| {
            eprintln!("MySQL check failed: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok("MySQL connection works")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await?;


    let cities: Vec<City> = sqlx::query_as::<_, City>(
        "SELECT ID, Name, Population, District, CountryCode FROM city WHERE population > 1000000"
    ).fetch_all(&pool).await?;

    println!("first city: {:?}", cities.first().unwrap().name);


    let api_routes = Router::new()
        .route("/hello", get(|| async { "Hello from Axum API!" }))
        .route("/db-check", get(database_check))
        .with_state(pool);

    let frontend_service = ServeDir::new("../client/dist")
        .not_found_service(ServeDir::new("../client/dist/index.html"));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(frontend_service);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
