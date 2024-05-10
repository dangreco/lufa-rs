use serde::Deserialize;
use std::collections::HashMap;

use crate::de;

mod item;
use item::*;

mod amounts;
use amounts::*;

mod recipe;
use recipe::*;

// An Order represents the user's current
// order on the platform. It contains information
// corresponding to the status of the order,
// the contents of the order, and the pricing
// of the order.
#[derive(Deserialize, Debug)]
pub struct Order {

    // The ID of the order
    #[serde(rename = "orderId")]
    pub id: String,

    // The status of the order
    #[serde(rename = "orderStatus")]
    pub status: String, // todo!

    // The date at which the order will be delivered
    #[serde(rename = "orderDate")]
    pub date: String, // todo!

    // A list of items in the order
    #[serde(rename = "orderDetails")]
    pub items: Vec<OrderItem>,

    // A list of meal kit recipes in the order
    #[serde(
        rename = "orderRecipes",
        deserialize_with = "de::deserialize_array_or_object"
    )]
    pub recipes: HashMap<usize, Recipe>,

    // The price breakdown of the order
    #[serde(rename = "checkoutAmounts")]
    pub amounts: CheckoutAmounts,
}

#[cfg(test)]
mod tests {
    use super::Order;

    #[test]
    fn test_deserialize() {
        let s = r#"
      {
        "success": true,
        "orderDetails": [
          {
            "p_id": "15304",
            "product_id": "15304",
            "order_details_id": "321886952",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "5.00",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "1.25",
            "avg_p_p_r": "1.25",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Analisa Lebanese Cucumbers (strong taste)",
            "s_name": "Lufa Farms Ville Saint-Laurent",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/10878_10878-Concombres-libanais-en-floraison-Les-Fermes-Lufa-Laval-.jpg",
            "cat_na": "Vegetables",
            "b_con": "0",
            "weight": "400",
            "c_wei": "0",
            "units": "sac",
            "unit": "g",
            "cat_weight": "0",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/10878_10878-Concombres-libanais-en-floraison-Les-Fermes-Lufa-Laval-.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/10878_10878-Concombres-libanais-en-floraison-Les-Fermes-Lufa-Laval-.jpg"
            }
          },
          {
            "p_id": "17161",
            "product_id": "17161",
            "order_details_id": "321886961",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "3.50",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "3.5",
            "avg_p_p_r": "3.5",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Green Kale (new variety)",
            "s_name": "Lufa Farms VSL Indoor Farm",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/6aa2b728-5198-4bba-a1ee-d4365784baac.jpg",
            "cat_na": "Vegetables",
            "b_con": "0",
            "weight": "100",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "0",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/6aa2b728-5198-4bba-a1ee-d4365784baac.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/6aa2b728-5198-4bba-a1ee-d4365784baac.jpg"
            }
          },
          {
            "p_id": "11994",
            "product_id": "11994",
            "order_details_id": "321886956",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "3.00",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "1.32",
            "avg_p_p_r": "1.32",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Brown Mushrooms",
            "s_name": "Essex",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/11994_-11994-Brown-Mushrooms-Essex.jpg",
            "cat_na": "Vegetables",
            "b_con": "0",
            "weight": "227",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "0",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/11994_-11994-Brown-Mushrooms-Essex.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/11994_-11994-Brown-Mushrooms-Essex.jpg"
            }
          },
          {
            "p_id": "16814",
            "product_id": "16814",
            "order_details_id": "321886945",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "2.25",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "3",
            "avg_p_p_r": "3",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Green Onions\t",
            "s_name": "Lufa Farms VSL Indoor Farm",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/ec341885-3a45-49cc-a876-95e1651be4ad.jpg",
            "cat_na": "Vegetables",
            "b_con": "0",
            "weight": "75",
            "c_wei": "0",
            "units": "portion",
            "unit": "g",
            "cat_weight": "0",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/ec341885-3a45-49cc-a876-95e1651be4ad.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/ec341885-3a45-49cc-a876-95e1651be4ad.jpg"
            }
          },
          {
            "p_id": "17817",
            "product_id": "17817",
            "order_details_id": "321886948",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "4.75",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "0.52",
            "avg_p_p_r": "0.52",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Salish Apples (4 to 6)",
            "s_name": "Warner's Farm",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/084dc84c-eea2-43b7-b734-e2f194dad5ce.jpg",
            "cat_na": "Fruits",
            "b_con": "0",
            "weight": "908",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "1",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/084dc84c-eea2-43b7-b734-e2f194dad5ce.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/084dc84c-eea2-43b7-b734-e2f194dad5ce.jpg"
            }
          },
          {
            "p_id": "17719",
            "product_id": "17719",
            "order_details_id": "321886946",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "1.75",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "0.74",
            "avg_p_p_r": "0.74",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Limes (3)",
            "s_name": "Acuifero Casma ",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/80a51f4b-0b1a-40e0-a856-b153217cb3f6.jpg",
            "cat_na": "Fruits",
            "b_con": "0",
            "weight": "235",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "1",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/80a51f4b-0b1a-40e0-a856-b153217cb3f6.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/80a51f4b-0b1a-40e0-a856-b153217cb3f6.jpg"
            }
          },
          {
            "p_id": "16443",
            "product_id": "16443",
            "order_details_id": "321886949",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "1.75",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "0.74",
            "avg_p_p_r": "0.74",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Limes (3)",
            "s_name": "Natural Mexico",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/80a51f4b-0b1a-40e0-a856-b153217cb3f6.jpg",
            "cat_na": "Fruits",
            "b_con": "0",
            "weight": "235",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "1",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/80a51f4b-0b1a-40e0-a856-b153217cb3f6.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/80a51f4b-0b1a-40e0-a856-b153217cb3f6.jpg"
            }
          },
          {
            "p_id": "17931",
            "product_id": "17931",
            "order_details_id": "322003855",
            "description": "",
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "5.75",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "1.15",
            "avg_p_p_r": "1.6",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Medium Ground Beef (fresh)",
            "s_name": "Boucherie B. Poirier",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/bf0de130-e612-4b42-9089-3997b8ad3563.jpg",
            "cat_na": "Meat",
            "b_con": "1",
            "weight": "500",
            "c_wei": "1",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "3",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/bf0de130-e612-4b42-9089-3997b8ad3563.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/bf0de130-e612-4b42-9089-3997b8ad3563.jpg"
            }
          },
          {
            "p_id": "12644",
            "product_id": "12644",
            "order_details_id": "321886944",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "15.50",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "2.21",
            "avg_p_p_r": "2.5",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Boneless Chicken Thighs (6, fresh)",
            "s_name": "Ferme des Voltigeurs",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/d761d43b-80e2-45d3-ab38-8191ad51f280.jpg",
            "cat_na": "Meat",
            "b_con": "1",
            "weight": "700",
            "c_wei": "1",
            "units": "sac",
            "unit": "g",
            "cat_weight": "3",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/d761d43b-80e2-45d3-ab38-8191ad51f280.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/d761d43b-80e2-45d3-ab38-8191ad51f280.jpg"
            }
          },
          {
            "p_id": "13378",
            "product_id": "13378",
            "order_details_id": "321886958",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "16.75",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "2.54",
            "avg_p_p_r": "2.54",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Pork & Coriander Dumplings (30, frozen)",
            "s_name": "Chef Su Dumplings",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/13378_13378-Pork---Coriander-Dumplings--frozen,-family-size--Chef-Su-Dumplings.jpg",
            "cat_na": "Pasta & Sauce",
            "b_con": "2",
            "weight": "660",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "5",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/13378_13378-Pork---Coriander-Dumplings--frozen,-family-size--Chef-Su-Dumplings.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/13378_13378-Pork---Coriander-Dumplings--frozen,-family-size--Chef-Su-Dumplings.jpg"
            }
          },
          {
            "p_id": "2917",
            "product_id": "2917",
            "order_details_id": "321886957",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "8.75",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": null,
            "avg_p_p_r": null,
            "avg_p_q": null,
            "avg_p_u": null,
            "quantity_in_basket": "1",
            "p_name": "Organic Free-Range Brown Eggs (12, large)",
            "s_name": "Nutri-Oeuf",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/2917_2917-Organic-Free-Range-Brown-Eggs--12,-large--Nutri-Oeufs.jpg",
            "cat_na": "Milk Products & Eggs",
            "b_con": "1",
            "weight": "780",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "6",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/2917_2917-Organic-Free-Range-Brown-Eggs--12,-large--Nutri-Oeufs.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/2917_2917-Organic-Free-Range-Brown-Eggs--12,-large--Nutri-Oeufs.jpg"
            }
          },
          {
            "p_id": "13357",
            "product_id": "13357",
            "order_details_id": "321886947",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "6.00",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "0.6",
            "avg_p_p_r": "0.6",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "ml",
            "quantity_in_basket": "1",
            "p_name": "10% Lactose-Free Coffee Cream",
            "s_name": "Nutrinor",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/c520236b-2a55-48d2-bcaa-bb49e4f870f5.jpg",
            "cat_na": "Milk Products & Eggs",
            "b_con": "1",
            "weight": "1000",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "ml",
            "cat_weight": "6",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/c520236b-2a55-48d2-bcaa-bb49e4f870f5.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/c520236b-2a55-48d2-bcaa-bb49e4f870f5.jpg"
            }
          },
          {
            "p_id": "10941",
            "product_id": "10941",
            "order_details_id": "321886950",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "6.25",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "3.12",
            "avg_p_p_r": "3.12",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Monterey Jack (lactose-free)",
            "s_name": "Fromagerie St-Guillaume",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/7760_Monterey-Jack--lactose-free----Fromagerie-St-Guillame.jpg",
            "cat_na": "Cheese",
            "b_con": "1",
            "weight": "200",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "7",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/7760_Monterey-Jack--lactose-free----Fromagerie-St-Guillame.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/7760_Monterey-Jack--lactose-free----Fromagerie-St-Guillame.jpg"
            }
          },
          {
            "p_id": "17350",
            "product_id": "17350",
            "order_details_id": "321886951",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "5.00",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "2.08",
            "avg_p_p_r": "2.08",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Cream Cheese",
            "s_name": "Riviera",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/7877db38-cab7-4111-80f5-2c7086fbc476.jpg",
            "cat_na": "Cheese",
            "b_con": "1",
            "weight": "240",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "7",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/7877db38-cab7-4111-80f5-2c7086fbc476.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/7877db38-cab7-4111-80f5-2c7086fbc476.jpg"
            }
          },
          {
            "p_id": "8945",
            "product_id": "8945",
            "order_details_id": "321886959",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "12.00",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "6.49",
            "avg_p_p_r": "6.49",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Buffalo Mozzarella ",
            "s_name": "Ferme Bufala Maciocia",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/8945_8945-Buffalo-Mozzarella-Ferme-Bufala-Maciocia-2.jpg",
            "cat_na": "Cheese",
            "b_con": "1",
            "weight": "185",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "7",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/8945_8945-Buffalo-Mozzarella-Ferme-Bufala-Maciocia-2.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/8945_8945-Buffalo-Mozzarella-Ferme-Bufala-Maciocia-2.jpg"
            }
          },
          {
            "p_id": "13763",
            "product_id": "13763",
            "order_details_id": "321886955",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "20.25",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "2.7",
            "avg_p_p_r": "2.7",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Butter Chicken (spicy)",
            "s_name": "Bagel Henri Bourassa",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/13763_13763-Butter-Chicken--family-size--Bagel-Henri-Bourassa.jpg",
            "cat_na": "Prepared Foods",
            "b_con": "1",
            "weight": "750",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "8",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/13763_13763-Butter-Chicken--family-size--Bagel-Henri-Bourassa.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/13763_13763-Butter-Chicken--family-size--Bagel-Henri-Bourassa.jpg"
            }
          },
          {
            "p_id": "10274",
            "product_id": "10274",
            "order_details_id": "321886954",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "4.25",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "2.12",
            "avg_p_p_r": "2.12",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Traditional Tortillas (7\", pack of 6, frozen)",
            "s_name": "Tilla' Tortilla",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/10274_10274-Traditional-Tortillas--7-22,-pack-of-6,-frozen--Tilla--Tortilla.jpg",
            "cat_na": "Bakery",
            "b_con": "2",
            "weight": "200",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "9",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/10274_10274-Traditional-Tortillas--7-22,-pack-of-6,-frozen--Tilla--Tortilla.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/10274_10274-Traditional-Tortillas--7-22,-pack-of-6,-frozen--Tilla--Tortilla.jpg"
            }
          },
          {
            "p_id": "16414",
            "product_id": "16414",
            "order_details_id": "321886953",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "4.00",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "0.59",
            "avg_p_p_r": "0.59",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "White Club Sandwich Bread (sliced)",
            "s_name": "Boulangerie Auger",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/04097804-85da-450b-af08-68f4f4a2c06d.jpg",
            "cat_na": "Bakery",
            "b_con": "0",
            "weight": "675",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "9",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/04097804-85da-450b-af08-68f4f4a2c06d.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/04097804-85da-450b-af08-68f4f4a2c06d.jpg"
            }
          },
          {
            "p_id": "1776",
            "product_id": "1776",
            "order_details_id": "321886960",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "3.50",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "0.77",
            "avg_p_p_r": "0.77",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Organic Firm Tofu",
            "s_name": "Aliments Horium",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/1776_1776-Organic-Firm-Tofu-Aliments-Horium.jpg",
            "cat_na": "Plant-Based Alternatives",
            "b_con": "1",
            "weight": "454",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "12",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/1776_1776-Organic-Firm-Tofu-Aliments-Horium.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/1776_1776-Organic-Firm-Tofu-Aliments-Horium.jpg"
            }
          },
          {
            "p_id": "17811",
            "product_id": "17811",
            "order_details_id": "321886962",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "0.00",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": "0",
            "avg_p_p_r": "0",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Gift: Analisa Lebanese Cucumbers ",
            "s_name": "Lufa Farms Ville Saint-Laurent",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/10878_10878-Concombres-libanais-en-floraison-Les-Fermes-Lufa-Laval-.jpg",
            "cat_na": "Gift",
            "b_con": "0",
            "weight": "400",
            "c_wei": "0",
            "units": "sac",
            "unit": "g",
            "cat_weight": "19",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/10878_10878-Concombres-libanais-en-floraison-Les-Fermes-Lufa-Laval-.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/10878_10878-Concombres-libanais-en-floraison-Les-Fermes-Lufa-Laval-.jpg"
            }
          },
          {
            "p_id": "3210",
            "product_id": "3210",
            "order_details_id": "321881949",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "6.00",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": null,
            "avg_p_p_r": null,
            "avg_p_q": null,
            "avg_p_u": null,
            "quantity_in_basket": "1",
            "p_name": "Home delivery",
            "s_name": "Lufa Farms Deliveries",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/3210_Basket-Launch---Delivery-Car---Winter-Scene-5.jpg",
            "cat_na": "For Internal Use",
            "b_con": "0",
            "weight": "0",
            "c_wei": "0",
            "units": "à l'unité",
            "unit": "g",
            "cat_weight": "20",
            "image_urls": {
              "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/3210_Basket-Launch---Delivery-Car---Winter-Scene-5.jpg",
              "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/3210_Basket-Launch---Delivery-Car---Winter-Scene-5.jpg"
            }
          }
        ],
        "orderId": "12345678",
        "orderStatus": "2",
        "orderDate": "Sunday, May 12th, 2024",
        "giftRecipient": "",
        "checkoutAmounts": {
          "total": "137.95",
          "subtotal": "130.00",
          "delivery_fees": "6,00",
          "remaining_balance": "1.05",
          "balance": "0.00",
          "consigne_amount": "0.00",
          "order_details": {
            "1776": {
              "price": "3.50",
              "quantity": "1",
              "row_total": 3.5
            },
            "2917": {
              "price": "8.75",
              "quantity": "1",
              "row_total": 8.75
            },
            "3210": {
              "price": "6.00",
              "quantity": "1",
              "row_total": 6
            },
            "8945": {
              "price": "12.00",
              "quantity": "1",
              "row_total": 12
            },
            "10274": {
              "price": "4.25",
              "quantity": "1",
              "row_total": 4.25
            },
            "10941": {
              "price": "6.25",
              "quantity": "1",
              "row_total": 6.25
            },
            "11994": {
              "price": "3.00",
              "quantity": "1",
              "row_total": 3
            },
            "12644": {
              "price": "15.50",
              "quantity": "1",
              "row_total": 15.5
            },
            "13357": {
              "price": "6.00",
              "quantity": "1",
              "row_total": 6
            },
            "13378": {
              "price": "16.75",
              "quantity": "1",
              "row_total": 16.75
            },
            "13763": {
              "price": "20.25",
              "quantity": "1",
              "row_total": 20.25
            },
            "15304": {
              "price": "5.00",
              "quantity": "1",
              "row_total": 5
            },
            "16414": {
              "price": "4.00",
              "quantity": "1",
              "row_total": 4
            },
            "16443": {
              "price": "1.75",
              "quantity": "1",
              "row_total": 1.75
            },
            "16814": {
              "price": "2.25",
              "quantity": "1",
              "row_total": 2.25
            },
            "17161": {
              "price": "3.50",
              "quantity": "1",
              "row_total": 3.5
            },
            "17350": {
              "price": "5.00",
              "quantity": "1",
              "row_total": 5
            },
            "17719": {
              "price": "1.75",
              "quantity": "1",
              "row_total": 1.75
            },
            "17811": {
              "price": "0.00",
              "quantity": "1",
              "row_total": 0
            },
            "17817": {
              "price": "4.75",
              "quantity": "1",
              "row_total": 4.75
            },
            "17931": {
              "price": "5.75",
              "quantity": "1",
              "row_total": 5.75
            }
          },
          "national_tax": "0.30",
          "provincial_tax": "0.60",
          "coupon_discount_amount": "0.00",
          "nb_item": 20,
          "basket_array": [
            85
          ],
          "order_donation": "0.00",
          "donation_discount": "0.00",
          "available_weekly": "0.00",
          "remaining_weekly": "0.00",
          "unformatted_total": 137.95,
          "unformatted_order_donation": "0.00",
          "unformatted_subtotal": 130,
          "unformatted_delivery_fees": 6,
          "unformatted_remaining_balance": 1.05,
          "unformatted_balance": 0,
          "unformatted_consigne_amount": 0,
          "unformatted_coupon_discount_amount": 0
        },
        "orderRecipes": [],
        "user_id": "123456"
      }
      "#;

        let order: Result<Order, _> = serde_json::from_str(s);
        assert!(order.is_ok());
    }
}
