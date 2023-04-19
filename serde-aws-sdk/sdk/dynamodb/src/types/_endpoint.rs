// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An endpoint information details.</p>
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
pub struct Endpoint {
    /// <p>IP address of the endpoint.</p>
    #[doc(hidden)]
    pub address: std::option::Option<std::string::String>,
    /// <p>Endpoint cache time to live (TTL) value.</p>
    #[doc(hidden)]
    pub cache_period_in_minutes: i64,
}
impl Endpoint {
    /// <p>IP address of the endpoint.</p>
    pub fn address(&self) -> std::option::Option<&str> {
        self.address.as_deref()
    }
    /// <p>Endpoint cache time to live (TTL) value.</p>
    pub fn cache_period_in_minutes(&self) -> i64 {
        self.cache_period_in_minutes
    }
}
impl Endpoint {
    /// Creates a new builder-style object to manufacture [`Endpoint`](crate::types::Endpoint).
    pub fn builder() -> crate::types::builders::EndpointBuilder {
        crate::types::builders::EndpointBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Endpoint;
/// A builder for [`Endpoint`](crate::types::Endpoint).
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
pub struct EndpointBuilder {
    pub(crate) address: std::option::Option<std::string::String>,
    pub(crate) cache_period_in_minutes: std::option::Option<i64>,
}
impl EndpointBuilder {
    /// <p>IP address of the endpoint.</p>
    pub fn address(mut self, input: impl Into<std::string::String>) -> Self {
        self.address = Some(input.into());
        self
    }
    /// <p>IP address of the endpoint.</p>
    pub fn set_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.address = input;
        self
    }
    /// <p>Endpoint cache time to live (TTL) value.</p>
    pub fn cache_period_in_minutes(mut self, input: i64) -> Self {
        self.cache_period_in_minutes = Some(input);
        self
    }
    /// <p>Endpoint cache time to live (TTL) value.</p>
    pub fn set_cache_period_in_minutes(mut self, input: std::option::Option<i64>) -> Self {
        self.cache_period_in_minutes = input;
        self
    }
    /// Consumes the builder and constructs a [`Endpoint`](crate::types::Endpoint).
    pub fn build(self) -> crate::types::Endpoint {
        crate::types::Endpoint {
            address: self.address,
            cache_period_in_minutes: self.cache_period_in_minutes.unwrap_or_default(),
        }
    }
}
