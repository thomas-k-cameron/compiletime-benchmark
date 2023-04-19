// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for the information associated with a <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_PutMultiRegionAccessPoint.html">PutMultiRegionAccessPoint</a> request.</p>
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
pub struct PutMultiRegionAccessPointPolicyInput {
    /// <p>The name of the Multi-Region Access Point associated with the request.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The policy details for the <code>PutMultiRegionAccessPoint</code> request.</p>
    #[doc(hidden)]
    pub policy: std::option::Option<std::string::String>,
}
impl PutMultiRegionAccessPointPolicyInput {
    /// <p>The name of the Multi-Region Access Point associated with the request.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The policy details for the <code>PutMultiRegionAccessPoint</code> request.</p>
    pub fn policy(&self) -> std::option::Option<&str> {
        self.policy.as_deref()
    }
}
impl PutMultiRegionAccessPointPolicyInput {
    /// Creates a new builder-style object to manufacture [`PutMultiRegionAccessPointPolicyInput`](crate::types::PutMultiRegionAccessPointPolicyInput).
    pub fn builder() -> crate::types::builders::PutMultiRegionAccessPointPolicyInputBuilder {
        crate::types::builders::PutMultiRegionAccessPointPolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PutMultiRegionAccessPointPolicyInput;
/// A builder for [`PutMultiRegionAccessPointPolicyInput`](crate::types::PutMultiRegionAccessPointPolicyInput).
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
pub struct PutMultiRegionAccessPointPolicyInputBuilder {
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) policy: std::option::Option<std::string::String>,
}
impl PutMultiRegionAccessPointPolicyInputBuilder {
    /// <p>The name of the Multi-Region Access Point associated with the request.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the Multi-Region Access Point associated with the request.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The policy details for the <code>PutMultiRegionAccessPoint</code> request.</p>
    pub fn policy(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy = Some(input.into());
        self
    }
    /// <p>The policy details for the <code>PutMultiRegionAccessPoint</code> request.</p>
    pub fn set_policy(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy = input;
        self
    }
    /// Consumes the builder and constructs a [`PutMultiRegionAccessPointPolicyInput`](crate::types::PutMultiRegionAccessPointPolicyInput).
    pub fn build(self) -> crate::types::PutMultiRegionAccessPointPolicyInput {
        crate::types::PutMultiRegionAccessPointPolicyInput {
            name: self.name,
            policy: self.policy,
        }
    }
}
