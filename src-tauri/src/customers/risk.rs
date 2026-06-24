use sqlx::{SqlitePool, Error};
use uuid::Uuid;
use super::models::CustomerRisk;

pub struct CustomerRiskService;

impl CustomerRiskService {
    pub async fn add_risk(
        pool: &SqlitePool,
        customer_id: &str,
        risk_type: &str,
        reason: Option<&str>,
        evidence: Option<&str>
    ) -> Result<String, Error> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        sqlx::query(
            "INSERT INTO customer_risks (id, customer_id, risk_type, reason, evidence, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(customer_id)
        .bind(risk_type)
        .bind(reason)
        .bind(evidence)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(id)
    }

    pub async fn get_risks(pool: &SqlitePool, customer_id: &str) -> Result<Vec<CustomerRisk>, Error> {
        let risks = sqlx::query_as::<_, CustomerRisk>(
            "SELECT id, customer_id, risk_type, reason, evidence, created_at FROM customer_risks WHERE customer_id = ?"
        )
        .bind(customer_id)
        .fetch_all(pool)
        .await?;
        Ok(risks)
    }
}
