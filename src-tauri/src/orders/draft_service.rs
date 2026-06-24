use crate::selling_modes::models::OrderClaimResult;
use sqlx::SqlitePool;
use uuid::Uuid;
use super::models::{Order, OrderItem, OrderClaim};
use super::repository::OrderRepository;
use crate::customers::identity::CustomerIdentityResolver;

pub struct OrderDraftService;

impl OrderDraftService {
    pub async fn apply_claim_to_order(pool: &SqlitePool, claim: &OrderClaimResult, price: f64) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        // Extract available customer info
        let user_id = claim.user_id.as_deref();
        // Fallback to resolving the customer using identity resolver
        let customer_id = CustomerIdentityResolver::resolve(&mut tx, user_id, None, None).await?;
        
        let status_str = match claim.status {
            crate::selling_modes::models::ClaimStatus::Confirmed => "confirmed",
            crate::selling_modes::models::ClaimStatus::OutOfStock => "out_of_stock",
            crate::selling_modes::models::ClaimStatus::Won => "won",
            crate::selling_modes::models::ClaimStatus::Lost => "lost",
            crate::selling_modes::models::ClaimStatus::Waitlist => "waitlist",
            crate::selling_modes::models::ClaimStatus::PendingInfo => "pending_info",
            crate::selling_modes::models::ClaimStatus::NeedsReview => "needs_review",
            crate::selling_modes::models::ClaimStatus::Ignored => "ignored",
        }.to_string();

        let db_claim = OrderClaim {
            id: claim.claim_id.clone(),
            session_id: claim.session_id.clone(),
            event_id: claim.event_id.clone(),
            customer_id: customer_id.clone(),
            sku: claim.sku.clone().unwrap_or_default(),
            variant_id: None,
            qty: claim.quantity,
            confidence: None,
            status: status_str,
            created_at: chrono::Utc::now(),
        };

        sqlx::query(
            r#"
            INSERT INTO order_claims (id, session_id, event_id, customer_id, sku, variant_id, qty, confidence, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&db_claim.id)
        .bind(&db_claim.session_id)
        .bind(&db_claim.event_id)
        .bind(&db_claim.customer_id)
        .bind(&db_claim.sku)
        .bind(&db_claim.variant_id)
        .bind(db_claim.qty)
        .bind(db_claim.confidence)
        .bind(&db_claim.status)
        .bind(db_claim.created_at)
        .execute(&mut *tx)
        .await?;

        if claim.status == crate::selling_modes::models::ClaimStatus::Confirmed ||
           claim.status == crate::selling_modes::models::ClaimStatus::Won {
            let order = OrderRepository::get_order_by_customer_session(&mut tx, &customer_id, &claim.session_id).await?;
            
            let order_id = if let Some(existing_order) = order {
                existing_order.id
            } else {
                let new_order_id = Uuid::new_v4().to_string();
                let new_order = Order {
                    id: new_order_id.clone(),
                    session_id: claim.session_id.clone(),
                    customer_id: customer_id.clone(),
                    status: "draft".to_string(),
                    total_amount: 0.0,
                    created_at: chrono::Utc::now(),
                };
                OrderRepository::create_order(&mut tx, &new_order).await?;
                new_order_id
            };

            let item = OrderItem {
                id: Uuid::new_v4().to_string(),
                order_id: order_id.clone(),
                claim_id: Some(claim.claim_id.clone()),
                sku: claim.sku.clone().unwrap_or_default(),
                variant_id: None,
                qty: claim.quantity,
                price,
                created_at: chrono::Utc::now(),
            };

            OrderRepository::create_order_item(&mut tx, &item).await?;

            let total: f64 = sqlx::query_scalar::<_, f64>(
                "SELECT COALESCE(SUM(qty * price), 0.0) FROM order_items WHERE order_id = ?"
            )
            .bind(&order_id)
            .fetch_one(&mut *tx)
            .await?;

            OrderRepository::update_order_total(&mut tx, &order_id, total).await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
