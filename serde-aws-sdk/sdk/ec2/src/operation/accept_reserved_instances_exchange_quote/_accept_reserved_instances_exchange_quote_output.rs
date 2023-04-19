// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The result of the exchange and whether it was <code>successful</code>.</p>
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
pub struct AcceptReservedInstancesExchangeQuoteOutput {
    /// <p>The ID of the successful exchange.</p>
    #[doc(hidden)]
    pub exchange_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl AcceptReservedInstancesExchangeQuoteOutput {
    /// <p>The ID of the successful exchange.</p>
    pub fn exchange_id(&self) -> std::option::Option<&str> {
        self.exchange_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for AcceptReservedInstancesExchangeQuoteOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AcceptReservedInstancesExchangeQuoteOutput {
    /// Creates a new builder-style object to manufacture [`AcceptReservedInstancesExchangeQuoteOutput`](crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteOutput).
    pub fn builder() -> crate::operation::accept_reserved_instances_exchange_quote::builders::AcceptReservedInstancesExchangeQuoteOutputBuilder{
        crate::operation::accept_reserved_instances_exchange_quote::builders::AcceptReservedInstancesExchangeQuoteOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteOutput;
/// A builder for [`AcceptReservedInstancesExchangeQuoteOutput`](crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteOutput).
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
pub struct AcceptReservedInstancesExchangeQuoteOutputBuilder {
    pub(crate) exchange_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl AcceptReservedInstancesExchangeQuoteOutputBuilder {
    /// <p>The ID of the successful exchange.</p>
    pub fn exchange_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.exchange_id = Some(input.into());
        self
    }
    /// <p>The ID of the successful exchange.</p>
    pub fn set_exchange_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.exchange_id = input;
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
    /// Consumes the builder and constructs a [`AcceptReservedInstancesExchangeQuoteOutput`](crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteOutput).
    pub fn build(self) -> crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteOutput{
        crate::operation::accept_reserved_instances_exchange_quote::AcceptReservedInstancesExchangeQuoteOutput {
            exchange_id: self.exchange_id
            ,
            _request_id: self._request_id,
        }
    }
}
