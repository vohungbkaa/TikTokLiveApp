pub enum OrderStatus {
    Draft,
    PendingInfo,
    Confirmed,
    Printed,
    Exported,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Draft => "draft",
            OrderStatus::PendingInfo => "pending_info",
            OrderStatus::Confirmed => "confirmed",
            OrderStatus::Printed => "printed",
            OrderStatus::Exported => "exported",
            OrderStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Option<OrderStatus> {
        match s {
            "draft" => Some(OrderStatus::Draft),
            "pending_info" => Some(OrderStatus::PendingInfo),
            "confirmed" => Some(OrderStatus::Confirmed),
            "printed" => Some(OrderStatus::Printed),
            "exported" => Some(OrderStatus::Exported),
            "cancelled" => Some(OrderStatus::Cancelled),
            _ => None,
        }
    }

    pub fn can_transition_to(&self, next: &OrderStatus) -> bool {
        match (self, next) {
            (OrderStatus::Draft, OrderStatus::Confirmed) => true,
            (OrderStatus::Draft, OrderStatus::Cancelled) => true,
            (OrderStatus::Draft, OrderStatus::PendingInfo) => true,
            (OrderStatus::PendingInfo, OrderStatus::Confirmed) => true,
            (OrderStatus::PendingInfo, OrderStatus::Cancelled) => true,
            (OrderStatus::Confirmed, OrderStatus::Printed) => true,
            (OrderStatus::Confirmed, OrderStatus::Exported) => true,
            (OrderStatus::Confirmed, OrderStatus::Cancelled) => true,
            (OrderStatus::Printed, OrderStatus::Exported) => true,
            (OrderStatus::Exported, OrderStatus::Printed) => true,
            _ => false,
        }
    }
}
