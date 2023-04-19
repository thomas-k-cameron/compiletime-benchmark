// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The credit option for CPU usage of a T instance.</p>
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
pub struct CreditSpecificationRequest {
    /// <p>The credit option for CPU usage of a T instance.</p>
    /// <p>Valid values: <code>standard</code> | <code>unlimited</code> </p>
    #[doc(hidden)]
    pub cpu_credits: std::option::Option<std::string::String>,
}
impl CreditSpecificationRequest {
    /// <p>The credit option for CPU usage of a T instance.</p>
    /// <p>Valid values: <code>standard</code> | <code>unlimited</code> </p>
    pub fn cpu_credits(&self) -> std::option::Option<&str> {
        self.cpu_credits.as_deref()
    }
}
impl CreditSpecificationRequest {
    /// Creates a new builder-style object to manufacture [`CreditSpecificationRequest`](crate::types::CreditSpecificationRequest).
    pub fn builder() -> crate::types::builders::CreditSpecificationRequestBuilder {
        crate::types::builders::CreditSpecificationRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CreditSpecificationRequest;
/// A builder for [`CreditSpecificationRequest`](crate::types::CreditSpecificationRequest).
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
pub struct CreditSpecificationRequestBuilder {
    pub(crate) cpu_credits: std::option::Option<std::string::String>,
}
impl CreditSpecificationRequestBuilder {
    /// <p>The credit option for CPU usage of a T instance.</p>
    /// <p>Valid values: <code>standard</code> | <code>unlimited</code> </p>
    pub fn cpu_credits(mut self, input: impl Into<std::string::String>) -> Self {
        self.cpu_credits = Some(input.into());
        self
    }
    /// <p>The credit option for CPU usage of a T instance.</p>
    /// <p>Valid values: <code>standard</code> | <code>unlimited</code> </p>
    pub fn set_cpu_credits(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cpu_credits = input;
        self
    }
    /// Consumes the builder and constructs a [`CreditSpecificationRequest`](crate::types::CreditSpecificationRequest).
    pub fn build(self) -> crate::types::CreditSpecificationRequest {
        crate::types::CreditSpecificationRequest {
            cpu_credits: self.cpu_credits,
        }
    }
}
