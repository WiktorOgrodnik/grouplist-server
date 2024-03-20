use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    
    let mut app = tide::new();
    app.at("/orders/shoes").post(orders_shoes);
    app.listen("127.0.0.1:8000").await?;

    Ok(())
}

async fn orders_shoes(mut req: Request<()>) -> tide::Result {
    let Animal {name, legs} = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}