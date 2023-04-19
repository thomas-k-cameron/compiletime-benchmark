// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct PurchaseHostReservationOutput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>The currency in which the <code>totalUpfrontPrice</code> and <code>totalHourlyPrice</code> amounts are specified. At this time, the only supported currency is <code>USD</code>.</p>
    #[doc(hidden)]
    pub currency_code: std::option::Option<crate::types::CurrencyCodeValues>,
    /// <p>Describes the details of the purchase.</p>
    #[doc(hidden)]
    pub purchase: std::option::Option<std::vec::Vec<crate::types::Purchase>>,
    /// <p>The total hourly price of the reservation calculated per hour.</p>
    #[doc(hidden)]
    pub total_hourly_price: std::option::Option<std::string::String>,
    /// <p>The total amount charged to your account when you purchase the reservation.</p>
    #[doc(hidden)]
    pub total_upfront_price: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl PurchaseHostReservationOutput {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The currency in which the <code>totalUpfrontPrice</code> and <code>totalHourlyPrice</code> amounts are specified. At this time, the only supported currency is <code>USD</code>.</p>
    pub fn currency_code(&self) -> std::option::Option<&crate::types::CurrencyCodeValues> {
        self.currency_code.as_ref()
    }
    /// <p>Describes the details of the purchase.</p>
    pub fn purchase(&self) -> std::option::Option<&[crate::types::Purchase]> {
        self.purchase.as_deref()
    }
    /// <p>The total hourly price of the reservation calculated per hour.</p>
    pub fn total_hourly_price(&self) -> std::option::Option<&str> {
        self.total_hourly_price.as_deref()
    }
    /// <p>The total amount charged to your account when you purchase the reservation.</p>
    pub fn total_upfront_price(&self) -> std::option::Option<&str> {
        self.total_upfront_price.as_deref()
    }
}
impl aws_http::request_id::RequestId for PurchaseHostReservationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PurchaseHostReservationOutput {
    /// Creates a new builder-style object to manufacture [`PurchaseHostReservationOutput`](crate::operation::purchase_host_reservation::PurchaseHostReservationOutput).
    pub fn builder(
    ) -> crate::operation::purchase_host_reservation::builders::PurchaseHostReservationOutputBuilder
    {
        crate::operation::purchase_host_reservation::builders::PurchaseHostReservationOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::purchase_host_reservation::PurchaseHostReservationOutput;
/// A builder for [`PurchaseHostReservationOutput`](crate::operation::purchase_host_reservation::PurchaseHostReservationOutput).
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
pub struct PurchaseHostReservationOutputBuilder {
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) currency_code: std::option::Option<crate::types::CurrencyCodeValues>,
    pub(crate) purchase: std::option::Option<std::vec::Vec<crate::types::Purchase>>,
    pub(crate) total_hourly_price: std::option::Option<std::string::String>,
    pub(crate) total_upfront_price: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl PurchaseHostReservationOutputBuilder {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The currency in which the <code>totalUpfrontPrice</code> and <code>totalHourlyPrice</code> amounts are specified. At this time, the only supported currency is <code>USD</code>.</p>
    pub fn currency_code(mut self, input: crate::types::CurrencyCodeValues) -> Self {
        self.currency_code = Some(input);
        self
    }
    /// <p>The currency in which the <code>totalUpfrontPrice</code> and <code>totalHourlyPrice</code> amounts are specified. At this time, the only supported currency is <code>USD</code>.</p>
    pub fn set_currency_code(
        mut self,
        input: std::option::Option<crate::types::CurrencyCodeValues>,
    ) -> Self {
        self.currency_code = input;
        self
    }
    /// Appends an item to `purchase`.
    ///
    /// To override the contents of this collection use [`set_purchase`](Self::set_purchase).
    ///
    /// <p>Describes the details of the purchase.</p>
    pub fn purchase(mut self, input: crate::types::Purchase) -> Self {
        let mut v = self.purchase.unwrap_or_default();
        v.push(input);
        self.purchase = Some(v);
        self
    }
    /// <p>Describes the details of the purchase.</p>
    pub fn set_purchase(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Purchase>>,
    ) -> Self {
        self.purchase = input;
        self
    }
    /// <p>The total hourly price of the reservation calculated per hour.</p>
    pub fn total_hourly_price(mut self, input: impl Into<std::string::String>) -> Self {
        self.total_hourly_price = Some(input.into());
        self
    }
    /// <p>The total hourly price of the reservation calculated per hour.</p>
    pub fn set_total_hourly_price(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.total_hourly_price = input;
        self
    }
    /// <p>The total amount charged to your account when you purchase the reservation.</p>
    pub fn total_upfront_price(mut self, input: impl Into<std::string::String>) -> Self {
        self.total_upfront_price = Some(input.into());
        self
    }
    /// <p>The total amount charged to your account when you purchase the reservation.</p>
    pub fn set_total_upfront_price(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.total_upfront_price = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PurchaseHostReservationOutput`](crate::operation::purchase_host_reservation::PurchaseHostReservationOutput).
    pub fn build(
        self,
    ) -> crate::operation::purchase_host_reservation::PurchaseHostReservationOutput {
        crate::operation::purchase_host_reservation::PurchaseHostReservationOutput {
            client_token: self.client_token,
            currency_code: self.currency_code,
            purchase: self.purchase,
            total_hourly_price: self.total_hourly_price,
            total_upfront_price: self.total_upfront_price,
            _request_id: self._request_id,
        }
    }
}
