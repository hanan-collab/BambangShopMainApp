use std::thread;

use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::model::subscriber::{self, Subscriber};
use crate::repository::subscriber::SubscriberRepository;
use bambangshop::{compose_error_response, Result};
use rocket::http::Status;
use rocket::response::status::NotFound;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = &product_type_upper.as_str();
        let subscriber_result: Subscriber = SubscriberRepository::add(product_type_str, subscriber);
        return Ok(subscriber_result);
    }

    pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = &product_type_upper.as_str();
        let result: Option<Subscriber> = SubscriberRepository::delete(product_type_str, url);
        if result.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Subscriber Not Found"),
            ));
        }
        return Ok(result.unwrap());
    }
}
