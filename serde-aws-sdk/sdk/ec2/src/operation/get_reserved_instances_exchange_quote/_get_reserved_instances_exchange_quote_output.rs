// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of GetReservedInstancesExchangeQuote.</p>
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
pub struct GetReservedInstancesExchangeQuoteOutput {
    /// <p>The currency of the transaction.</p>
    #[doc(hidden)]
    pub currency_code: std::option::Option<std::string::String>,
    /// <p>If <code>true</code>, the exchange is valid. If <code>false</code>, the exchange cannot be completed.</p>
    #[doc(hidden)]
    pub is_valid_exchange: std::option::Option<bool>,
    /// <p>The new end date of the reservation term.</p>
    #[doc(hidden)]
    pub output_reserved_instances_will_expire_at: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The total true upfront charge for the exchange.</p>
    #[doc(hidden)]
    pub payment_due: std::option::Option<std::string::String>,
    /// <p>The cost associated with the Reserved Instance.</p>
    #[doc(hidden)]
    pub reserved_instance_value_rollup: std::option::Option<crate::types::ReservationValue>,
    /// <p>The configuration of your Convertible Reserved Instances.</p>
    #[doc(hidden)]
    pub reserved_instance_value_set:
        std::option::Option<std::vec::Vec<crate::types::ReservedInstanceReservationValue>>,
    /// <p>The cost associated with the Reserved Instance.</p>
    #[doc(hidden)]
    pub target_configuration_value_rollup: std::option::Option<crate::types::ReservationValue>,
    /// <p>The values of the target Convertible Reserved Instances.</p>
    #[doc(hidden)]
    pub target_configuration_value_set:
        std::option::Option<std::vec::Vec<crate::types::TargetReservationValue>>,
    /// <p>Describes the reason why the exchange cannot be completed.</p>
    #[doc(hidden)]
    pub validation_failure_reason: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetReservedInstancesExchangeQuoteOutput {
    /// <p>The currency of the transaction.</p>
    pub fn currency_code(&self) -> std::option::Option<&str> {
        self.currency_code.as_deref()
    }
    /// <p>If <code>true</code>, the exchange is valid. If <code>false</code>, the exchange cannot be completed.</p>
    pub fn is_valid_exchange(&self) -> std::option::Option<bool> {
        self.is_valid_exchange
    }
    /// <p>The new end date of the reservation term.</p>
    pub fn output_reserved_instances_will_expire_at(
        &self,
    ) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.output_reserved_instances_will_expire_at.as_ref()
    }
    /// <p>The total true upfront charge for the exchange.</p>
    pub fn payment_due(&self) -> std::option::Option<&str> {
        self.payment_due.as_deref()
    }
    /// <p>The cost associated with the Reserved Instance.</p>
    pub fn reserved_instance_value_rollup(
        &self,
    ) -> std::option::Option<&crate::types::ReservationValue> {
        self.reserved_instance_value_rollup.as_ref()
    }
    /// <p>The configuration of your Convertible Reserved Instances.</p>
    pub fn reserved_instance_value_set(
        &self,
    ) -> std::option::Option<&[crate::types::ReservedInstanceReservationValue]> {
        self.reserved_instance_value_set.as_deref()
    }
    /// <p>The cost associated with the Reserved Instance.</p>
    pub fn target_configuration_value_rollup(
        &self,
    ) -> std::option::Option<&crate::types::ReservationValue> {
        self.target_configuration_value_rollup.as_ref()
    }
    /// <p>The values of the target Convertible Reserved Instances.</p>
    pub fn target_configuration_value_set(
        &self,
    ) -> std::option::Option<&[crate::types::TargetReservationValue]> {
        self.target_configuration_value_set.as_deref()
    }
    /// <p>Describes the reason why the exchange cannot be completed.</p>
    pub fn validation_failure_reason(&self) -> std::option::Option<&str> {
        self.validation_failure_reason.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetReservedInstancesExchangeQuoteOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetReservedInstancesExchangeQuoteOutput {
    /// Creates a new builder-style object to manufacture [`GetReservedInstancesExchangeQuoteOutput`](crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteOutput).
    pub fn builder() -> crate::operation::get_reserved_instances_exchange_quote::builders::GetReservedInstancesExchangeQuoteOutputBuilder{
        crate::operation::get_reserved_instances_exchange_quote::builders::GetReservedInstancesExchangeQuoteOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteOutput;
/// A builder for [`GetReservedInstancesExchangeQuoteOutput`](crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteOutput).
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
pub struct GetReservedInstancesExchangeQuoteOutputBuilder {
    pub(crate) currency_code: std::option::Option<std::string::String>,
    pub(crate) is_valid_exchange: std::option::Option<bool>,
    pub(crate) output_reserved_instances_will_expire_at:
        std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) payment_due: std::option::Option<std::string::String>,
    pub(crate) reserved_instance_value_rollup: std::option::Option<crate::types::ReservationValue>,
    pub(crate) reserved_instance_value_set:
        std::option::Option<std::vec::Vec<crate::types::ReservedInstanceReservationValue>>,
    pub(crate) target_configuration_value_rollup:
        std::option::Option<crate::types::ReservationValue>,
    pub(crate) target_configuration_value_set:
        std::option::Option<std::vec::Vec<crate::types::TargetReservationValue>>,
    pub(crate) validation_failure_reason: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetReservedInstancesExchangeQuoteOutputBuilder {
    /// <p>The currency of the transaction.</p>
    pub fn currency_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.currency_code = Some(input.into());
        self
    }
    /// <p>The currency of the transaction.</p>
    pub fn set_currency_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.currency_code = input;
        self
    }
    /// <p>If <code>true</code>, the exchange is valid. If <code>false</code>, the exchange cannot be completed.</p>
    pub fn is_valid_exchange(mut self, input: bool) -> Self {
        self.is_valid_exchange = Some(input);
        self
    }
    /// <p>If <code>true</code>, the exchange is valid. If <code>false</code>, the exchange cannot be completed.</p>
    pub fn set_is_valid_exchange(mut self, input: std::option::Option<bool>) -> Self {
        self.is_valid_exchange = input;
        self
    }
    /// <p>The new end date of the reservation term.</p>
    pub fn output_reserved_instances_will_expire_at(
        mut self,
        input: aws_smithy_types::DateTime,
    ) -> Self {
        self.output_reserved_instances_will_expire_at = Some(input);
        self
    }
    /// <p>The new end date of the reservation term.</p>
    pub fn set_output_reserved_instances_will_expire_at(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.output_reserved_instances_will_expire_at = input;
        self
    }
    /// <p>The total true upfront charge for the exchange.</p>
    pub fn payment_due(mut self, input: impl Into<std::string::String>) -> Self {
        self.payment_due = Some(input.into());
        self
    }
    /// <p>The total true upfront charge for the exchange.</p>
    pub fn set_payment_due(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.payment_due = input;
        self
    }
    /// <p>The cost associated with the Reserved Instance.</p>
    pub fn reserved_instance_value_rollup(mut self, input: crate::types::ReservationValue) -> Self {
        self.reserved_instance_value_rollup = Some(input);
        self
    }
    /// <p>The cost associated with the Reserved Instance.</p>
    pub fn set_reserved_instance_value_rollup(
        mut self,
        input: std::option::Option<crate::types::ReservationValue>,
    ) -> Self {
        self.reserved_instance_value_rollup = input;
        self
    }
    /// Appends an item to `reserved_instance_value_set`.
    ///
    /// To override the contents of this collection use [`set_reserved_instance_value_set`](Self::set_reserved_instance_value_set).
    ///
    /// <p>The configuration of your Convertible Reserved Instances.</p>
    pub fn reserved_instance_value_set(
        mut self,
        input: crate::types::ReservedInstanceReservationValue,
    ) -> Self {
        let mut v = self.reserved_instance_value_set.unwrap_or_default();
        v.push(input);
        self.reserved_instance_value_set = Some(v);
        self
    }
    /// <p>The configuration of your Convertible Reserved Instances.</p>
    pub fn set_reserved_instance_value_set(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ReservedInstanceReservationValue>>,
    ) -> Self {
        self.reserved_instance_value_set = input;
        self
    }
    /// <p>The cost associated with the Reserved Instance.</p>
    pub fn target_configuration_value_rollup(
        mut self,
        input: crate::types::ReservationValue,
    ) -> Self {
        self.target_configuration_value_rollup = Some(input);
        self
    }
    /// <p>The cost associated with the Reserved Instance.</p>
    pub fn set_target_configuration_value_rollup(
        mut self,
        input: std::option::Option<crate::types::ReservationValue>,
    ) -> Self {
        self.target_configuration_value_rollup = input;
        self
    }
    /// Appends an item to `target_configuration_value_set`.
    ///
    /// To override the contents of this collection use [`set_target_configuration_value_set`](Self::set_target_configuration_value_set).
    ///
    /// <p>The values of the target Convertible Reserved Instances.</p>
    pub fn target_configuration_value_set(
        mut self,
        input: crate::types::TargetReservationValue,
    ) -> Self {
        let mut v = self.target_configuration_value_set.unwrap_or_default();
        v.push(input);
        self.target_configuration_value_set = Some(v);
        self
    }
    /// <p>The values of the target Convertible Reserved Instances.</p>
    pub fn set_target_configuration_value_set(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TargetReservationValue>>,
    ) -> Self {
        self.target_configuration_value_set = input;
        self
    }
    /// <p>Describes the reason why the exchange cannot be completed.</p>
    pub fn validation_failure_reason(mut self, input: impl Into<std::string::String>) -> Self {
        self.validation_failure_reason = Some(input.into());
        self
    }
    /// <p>Describes the reason why the exchange cannot be completed.</p>
    pub fn set_validation_failure_reason(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.validation_failure_reason = input;
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
    /// Consumes the builder and constructs a [`GetReservedInstancesExchangeQuoteOutput`](crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteOutput).
    pub fn build(self) -> crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteOutput{
        crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteOutput {
            currency_code: self.currency_code
            ,
            is_valid_exchange: self.is_valid_exchange
            ,
            output_reserved_instances_will_expire_at: self.output_reserved_instances_will_expire_at
            ,
            payment_due: self.payment_due
            ,
            reserved_instance_value_rollup: self.reserved_instance_value_rollup
            ,
            reserved_instance_value_set: self.reserved_instance_value_set
            ,
            target_configuration_value_rollup: self.target_configuration_value_rollup
            ,
            target_configuration_value_set: self.target_configuration_value_set
            ,
            validation_failure_reason: self.validation_failure_reason
            ,
            _request_id: self._request_id,
        }
    }
}
