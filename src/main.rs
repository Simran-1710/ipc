mod ipc_trait;
mod response;
mod mediator;

use ipc_trait::IPC;
use response::Response;
use mediator::IPCMediator;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let ipc =  Arc::new(IPCMediator::new(10));

    let ipc_sub = Arc::clone(&ipc);
    
    let result = ipc_sub.subscribe::<String>("id1", Some(3)).await;
    match result {
        Ok(response) => match response {
            Response::Success(data) => println!("Received message: {}", data),
            Response::Timeout => println!("Timeout occurred while waiting for the message."),
            Response::DecodeFailure => println!("Failed to decode the received message."),
        },
        Err(e) => println!("Error occurred during subscription: {:?}", e),
    }


    sleep(Duration::from_millis(10000)).await;

    println!("After sleep");

    let ipc_pub = Arc::clone(&ipc);
    let publisher = tokio::spawn(async move {
        let payload = "Resp message";
        let result = ipc_pub.publish("id1", &payload).await;

        match result {
            Ok(_) => println!("Message published successfully!"),
            Err(e) => println!("Failed to publish message: {:?}", e),
        }
    });

    let _ = publisher.await;
}
