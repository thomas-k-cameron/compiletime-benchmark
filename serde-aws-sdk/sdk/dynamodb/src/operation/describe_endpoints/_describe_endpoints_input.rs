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
pub struct DescribeEndpointsInput {}
impl DescribeEndpointsInput {
    /// Creates a new builder-style object to manufacture [`DescribeEndpointsInput`](crate::operation::describe_endpoints::DescribeEndpointsInput).
    pub fn builder() -> crate::operation::describe_endpoints::builders::DescribeEndpointsInputBuilder
    {
        crate::operation::describe_endpoints::builders::DescribeEndpointsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_endpoints::DescribeEndpointsInput;
/// A builder for [`DescribeEndpointsInput`](crate::operation::describe_endpoints::DescribeEndpointsInput).
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
pub struct DescribeEndpointsInputBuilder {}
impl DescribeEndpointsInputBuilder {
    /// Consumes the builder and constructs a [`DescribeEndpointsInput`](crate::operation::describe_endpoints::DescribeEndpointsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_endpoints::DescribeEndpointsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::describe_endpoints::DescribeEndpointsInput {})
    }
}
