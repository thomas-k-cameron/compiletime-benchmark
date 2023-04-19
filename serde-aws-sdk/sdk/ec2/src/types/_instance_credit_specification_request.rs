// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the credit option for CPU usage of a burstable performance instance.</p>
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
pub struct InstanceCreditSpecificationRequest {
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The credit option for CPU usage of the instance.</p>
    /// <p>Valid values: <code>standard</code> | <code>unlimited</code> </p>
    /// <p>T3 instances with <code>host</code> tenancy do not support the <code>unlimited</code> CPU credit option.</p>
    #[doc(hidden)]
    pub cpu_credits: std::option::Option<std::string::String>,
}
impl InstanceCreditSpecificationRequest {
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The credit option for CPU usage of the instance.</p>
    /// <p>Valid values: <code>standard</code> | <code>unlimited</code> </p>
    /// <p>T3 instances with <code>host</code> tenancy do not support the <code>unlimited</code> CPU credit option.</p>
    pub fn cpu_credits(&self) -> std::option::Option<&str> {
        self.cpu_credits.as_deref()
    }
}
impl InstanceCreditSpecificationRequest {
    /// Creates a new builder-style object to manufacture [`InstanceCreditSpecificationRequest`](crate::types::InstanceCreditSpecificationRequest).
    pub fn builder() -> crate::types::builders::InstanceCreditSpecificationRequestBuilder {
        crate::types::builders::InstanceCreditSpecificationRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::InstanceCreditSpecificationRequest;
/// A builder for [`InstanceCreditSpecificationRequest`](crate::types::InstanceCreditSpecificationRequest).
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
pub struct InstanceCreditSpecificationRequestBuilder {
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) cpu_credits: std::option::Option<std::string::String>,
}
impl InstanceCreditSpecificationRequestBuilder {
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The credit option for CPU usage of the instance.</p>
    /// <p>Valid values: <code>standard</code> | <code>unlimited</code> </p>
    /// <p>T3 instances with <code>host</code> tenancy do not support the <code>unlimited</code> CPU credit option.</p>
    pub fn cpu_credits(mut self, input: impl Into<std::string::String>) -> Self {
        self.cpu_credits = Some(input.into());
        self
    }
    /// <p>The credit option for CPU usage of the instance.</p>
    /// <p>Valid values: <code>standard</code> | <code>unlimited</code> </p>
    /// <p>T3 instances with <code>host</code> tenancy do not support the <code>unlimited</code> CPU credit option.</p>
    pub fn set_cpu_credits(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cpu_credits = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceCreditSpecificationRequest`](crate::types::InstanceCreditSpecificationRequest).
    pub fn build(self) -> crate::types::InstanceCreditSpecificationRequest {
        crate::types::InstanceCreditSpecificationRequest {
            instance_id: self.instance_id,
            cpu_credits: self.cpu_credits,
        }
    }
}
