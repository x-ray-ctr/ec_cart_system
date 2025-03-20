use axum::{Router, routing::get, routing::post};
use std::sync::Arc;
use sqlx::PgPool;
use elasticsearch::Elasticsearch;
use crate::infrastructure::repository::postgres::PostgresProductRepository;
use crate::infrastructure::search::elasticsearch::ElasticsearchProductRepository;
use crate::infrastructure::payment_gateway::stripe::StripePaymentGateway;
use crate::usecase::product::service::ProductService;
use crate::usecase::cart::service::CartService;
use crate::usecase::order::service::OrderService;
use crate::usecase::payment::service::PaymentService;
use crate::usecase::shipment::service::ShipmentService;

#[tokio::main]
async fn main() {
    let db_pool = PgPool::connect("postgres://user:password@localhost/ec_db").await.unwrap();
    let es_client = Elasticsearch::default();
    let stripe_gateway = StripePaymentGateway::new();

    let product_repo = PostgresProductRepository::new(db_pool.clone());
    let product_service = Arc::new(ProductService::new(product_repo));

    let cart_repo = PostgresCartRepository::new(db_pool.clone());
    let cart_service = Arc::new(CartService::new(cart_repo));

    let order_repo = PostgresOrderRepository::new(db_pool.clone());
    let order_service = Arc::new(OrderService::new(order_repo, cart_repo));

    let payment_repo = PostgresPaymentRepository::new(db_pool.clone());
    let payment_service = Arc::new(PaymentService::new(payment_repo, stripe_gateway));

    let shipment_repo = PostgresShipmentRepository::new(db_pool.clone());
    let shipment_service = Arc::new(ShipmentService::new(shipment_repo));

    let app = Router::new()
        .route("/products", get(list_products))
        .route("/products/search", get(search_products))
        .route("/cart/:user_id", post(add_to_cart))
        .route("/order/:user_id", post(create_order))
        .route("/payment", post(process_payment))
        .route("/shipment/:order_id", post(ship_order));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
