use crate::error::{Error, Result};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};


//++ Work for the db to handle
#[derive(Serialize, Debug, Clone)]
pub struct Ticket {
    id: u64,
    title: String,
}
#[derive(Deserialize, Debug)]
pub struct CreateTicket {
    title: String
}

//--Work for the db to handle

//++Model controller
#[derive(Clone)]
pub struct ModelController {
    //Change Vec<Option<Ticket>>> because the vector will grow to infinity and thats not good
    store_ticket: Arc<Mutex<Vec<Option<Ticket>>>>
}

//Constructor ModelController
impl ModelController {
    pub async fn new() -> Result<Self> {

        Ok(
            Self {
                store_ticket: Arc::default(),
            }
        )
    }
}

//CRUD implementation
impl ModelController {
    pub async fn create_ticket(&self, create_ticket: CreateTicket) -> Result<Ticket> {

        let mut ticket_storage = self.store_ticket.lock().unwrap();
        let id = ticket_storage.len() as u64;
        let ticket = Ticket {
            id,
            title: create_ticket.title
        };

        ticket_storage.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_ticket(&self) -> Result<Vec<Ticket>> {

        let storage = self.store_ticket.lock().unwrap();

        let tickets = storage.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {

        let mut store = self.store_ticket.lock().unwrap();

        let ticket_delete = store.get_mut(id as usize).and_then(|x| x.take());

        ticket_delete.ok_or(Error::DeleteTicketIdNotFound {id})

    }
}

//--Model controller

