#![allow(unused)]

use::anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()>{

    let httpc = httpc_test::new_client("http://localhost:8080")?;
    httpc.do_get("/hello2/Viktor").await?.print().await?;
   // httpc.do_get("/templates/errorpage.html").await?.print().await?;

    let login = httpc.do_post(
        "/api/login",
        json!({
            "username": "Viktor",
            "pw": "test"
        })
    );

    login.await?.print().await?;


    let create_ticket = httpc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket 1"
        }
        )
    );

    create_ticket.await?.print().await?;

    let ticket_list = httpc.do_get(
        "/api/tickets",

    );

   // httpc.do_delete("/api/tickets/1").await?.print().await?;


    ticket_list.await?.print().await?;

    let ticket_replace = httpc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        })
    );

    ticket_replace.await?.print().await?;



    Ok(())
}