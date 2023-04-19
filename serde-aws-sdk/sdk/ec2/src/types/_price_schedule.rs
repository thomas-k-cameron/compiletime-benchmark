// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the price for a Reserved Instance.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PriceSchedule {
    /// <p>The current price schedule, as determined by the term remaining for the Reserved Instance in the listing.</p>
    /// <p>A specific price schedule is always in effect, but only one price schedule can be active at any time. Take, for example, a Reserved Instance listing that has five months remaining in its term. When you specify price schedules for five months and two months, this means that schedule 1, covering the first three months of the remaining term, will be active during months 5, 4, and 3. Then schedule 2, covering the last two months of the term, will be active for months 2 and 1.</p>
    #[doc(hidden)]
    pub active: std::option::Option<bool>,
    /// <p>The currency for transacting the Reserved Instance resale. At this time, the only supported currency is <code>USD</code>.</p>
    #[doc(hidden)]
    pub currency_code: std::option::Option<crate::types::CurrencyCodeValues>,
    /// <p>The fixed price for the term.</p>
    #[doc(hidden)]
    pub price: std::option::Option<f64>,
    /// <p>The number of months remaining in the reservation. For example, 2 is the second to the last month before the capacity reservation expires.</p>
    #[doc(hidden)]
    pub term: std::option::Option<i64>,
}
impl PriceSchedule {
    /// <p>The current price schedule, as determined by the term remaining for the Reserved Instance in the listing.</p>
    /// <p>A specific price schedule is always in effect, but only one price schedule can be active at any time. Take, for example, a Reserved Instance listing that has five months remaining in its term. When you specify price schedules for five months and two months, this means that schedule 1, covering the first three months of the remaining term, will be active during months 5, 4, and 3. Then schedule 2, covering the last two months of the term, will be active for months 2 and 1.</p>
    pub fn active(&self) -> std::option::Option<bool> {
        self.active
    }
    /// <p>The currency for transacting the Reserved Instance resale. At this time, the only supported currency is <code>USD</code>.</p>
    pub fn currency_code(&self) -> std::option::Option<&crate::types::CurrencyCodeValues> {
        self.currency_code.as_ref()
    }
    /// <p>The fixed price for the term.</p>
    pub fn price(&self) -> std::option::Option<f64> {
        self.price
    }
    /// <p>The number of months remaining in the reservation. For example, 2 is the second to the last month before the capacity reservation expires.</p>
    pub fn term(&self) -> std::option::Option<i64> {
        self.term
    }
}
impl PriceSchedule {
    /// Creates a new builder-style object to manufacture [`PriceSchedule`](crate::types::PriceSchedule).
    pub fn builder() -> crate::types::builders::PriceScheduleBuilder {
        crate::types::builders::PriceScheduleBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PriceSchedule;
/// A builder for [`PriceSchedule`](crate::types::PriceSchedule).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct PriceScheduleBuilder {
    pub(crate) active: std::option::Option<bool>,
    pub(crate) currency_code: std::option::Option<crate::types::CurrencyCodeValues>,
    pub(crate) price: std::option::Option<f64>,
    pub(crate) term: std::option::Option<i64>,
}
impl PriceScheduleBuilder {
    /// <p>The current price schedule, as determined by the term remaining for the Reserved Instance in the listing.</p>
    /// <p>A specific price schedule is always in effect, but only one price schedule can be active at any time. Take, for example, a Reserved Instance listing that has five months remaining in its term. When you specify price schedules for five months and two months, this means that schedule 1, covering the first three months of the remaining term, will be active during months 5, 4, and 3. Then schedule 2, covering the last two months of the term, will be active for months 2 and 1.</p>
    pub fn active(mut self, input: bool) -> Self {
        self.active = Some(input);
        self
    }
    /// <p>The current price schedule, as determined by the term remaining for the Reserved Instance in the listing.</p>
    /// <p>A specific price schedule is always in effect, but only one price schedule can be active at any time. Take, for example, a Reserved Instance listing that has five months remaining in its term. When you specify price schedules for five months and two months, this means that schedule 1, covering the first three months of the remaining term, will be active during months 5, 4, and 3. Then schedule 2, covering the last two months of the term, will be active for months 2 and 1.</p>
    pub fn set_active(mut self, input: std::option::Option<bool>) -> Self {
        self.active = input;
        self
    }
    /// <p>The currency for transacting the Reserved Instance resale. At this time, the only supported currency is <code>USD</code>.</p>
    pub fn currency_code(mut self, input: crate::types::CurrencyCodeValues) -> Self {
        self.currency_code = Some(input);
        self
    }
    /// <p>The currency for transacting the Reserved Instance resale. At this time, the only supported currency is <code>USD</code>.</p>
    pub fn set_currency_code(
        mut self,
        input: std::option::Option<crate::types::CurrencyCodeValues>,
    ) -> Self {
        self.currency_code = input;
        self
    }
    /// <p>The fixed price for the term.</p>
    pub fn price(mut self, input: f64) -> Self {
        self.price = Some(input);
        self
    }
    /// <p>The fixed price for the term.</p>
    pub fn set_price(mut self, input: std::option::Option<f64>) -> Self {
        self.price = input;
        self
    }
    /// <p>The number of months remaining in the reservation. For example, 2 is the second to the last month before the capacity reservation expires.</p>
    pub fn term(mut self, input: i64) -> Self {
        self.term = Some(input);
        self
    }
    /// <p>The number of months remaining in the reservation. For example, 2 is the second to the last month before the capacity reservation expires.</p>
    pub fn set_term(mut self, input: std::option::Option<i64>) -> Self {
        self.term = input;
        self
    }
    /// Consumes the builder and constructs a [`PriceSchedule`](crate::types::PriceSchedule).
    pub fn build(self) -> crate::types::PriceSchedule {
        crate::types::PriceSchedule {
            active: self.active,
            currency_code: self.currency_code,
            price: self.price,
            term: self.term,
        }
    }
}
