use rusty_money::{iso, Money};
use serde::Deserialize;
use serde_aux::prelude::*;

use crate::de;

#[derive(Deserialize, Debug)]
pub enum OrderStatus {
    #[serde(rename = "preparing")]
    Preparing,

    #[serde(rename = "shipped")]
    Shipped,

    #[serde(rename = "delivered")]
    Delivered,
}

// Represents the current tracking
// information for an order
#[derive(Deserialize, Debug)]
pub struct OrderTracking {
    // The order ID that this tracking
    // corresponds to
    #[serde(rename = "order_id")]
    pub order_id: String,

    // The current status of the order
    //
    // "preparing"
    #[serde(rename = "status")]
    pub status: OrderStatus,

    // A description of the progress
    // of the order
    #[serde(rename = "desc")]
    pub description: String,

    // The current step/progress of the
    // order
    //
    // 0 => preparing
    // 1 => shipped
    // 2 => delivered
    #[serde(rename = "step")]
    pub step: usize,

    // The date at which the order
    // is scheduled for delivery
    #[serde(rename = "delivery_date")]
    pub delivery_date: String,

    // The number of boxes that the order
    // is packed in
    #[serde(
        rename = "number_box_needed",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub boxes: usize,

    // The total cost of the order
    #[serde(rename = "order_amount", deserialize_with = "de::money")]
    pub order_amount: Money<'static, iso::Currency>,

    // The number of stops prior
    // to delivering the order
    #[serde(rename = "stops_before")]
    pub stops_before: usize,

    // The estimated time at which
    // the order will be delivered
    #[serde(rename = "eta")]
    pub eta: String, // TODO

    // The name of the driver delivering
    // the order
    #[serde(rename = "driver_name")]
    pub driver_name: String,

    // The name of the delivery company
    // delivering the order
    #[serde(rename = "company_name")]
    pub company_name: String,

    // The phone number of the delivery company
    #[serde(rename = "formatted_company_phone_number")]
    pub company_phone: String,

    // The phone number of the pickup point
    #[serde(rename = "formatted_pup_phone_number")]
    pub pickup_phone: String,

    // The type of delivery scheduled
    // for the order
    //
    // "HD"   => Home Delivery
    // "PUP"  => Pickup
    #[serde(rename = "delivery_type")]
    pub delivery_type: String,

    // The address at which the order
    // will be delivered to
    #[serde(rename = "address")]
    pub address: String,

    // A short reminder on leaving
    // empty boxes outside
    #[serde(rename = "reminder")]
    pub reminder: String,
}

#[cfg(test)]
mod tests {
    use super::OrderTracking;

    #[test]
    fn test_deserialize() {
        let s = r#"
        {
          "status": "preparing",
          "step": 0,
          "delivery_date": "Sunday, May 12th, 2024",
          "order_id": "12345678",
          "number_box_needed": "1",
          "order_amount_label": "Total: ",
          "order_amount": "$130.31",
          "stops_before": 13,
          "eta": "15:15",
          "driver_name": "John",
          "company_name": "Yalla Go",
          "company_phone_number": "514-123-4567",
          "formatted_company_phone_number": "+15141234567",
          "pup_phone_number": "5141234567",
          "formatted_pup_phone_number": "+15141234567",
          "delivery_type": "HD",
          "address": "7070 Henri Julien Ave, Montréal",
          "reminder": "Don’t forget to leave any empty baskets from your previous order at your front door for our delivery driver to pick up.",
          "desc": "We’re preparing your order. Once it’s on its way, your delivery ETA will appear."
        }
        "#;

        let tracking: Result<OrderTracking, _> = serde_json::from_str(s);
        assert!(tracking.is_ok());
    }
}
