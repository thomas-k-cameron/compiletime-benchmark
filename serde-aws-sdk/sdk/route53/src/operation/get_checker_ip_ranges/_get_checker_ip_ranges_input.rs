// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Empty request.</p>
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
pub struct GetCheckerIpRangesInput {}
impl GetCheckerIpRangesInput {
    /// Creates a new builder-style object to manufacture [`GetCheckerIpRangesInput`](crate::operation::get_checker_ip_ranges::GetCheckerIpRangesInput).
    pub fn builder(
    ) -> crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesInputBuilder {
        crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_checker_ip_ranges::GetCheckerIpRangesInput;
/// A builder for [`GetCheckerIpRangesInput`](crate::operation::get_checker_ip_ranges::GetCheckerIpRangesInput).
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
pub struct GetCheckerIpRangesInputBuilder {}
impl GetCheckerIpRangesInputBuilder {
    /// Consumes the builder and constructs a [`GetCheckerIpRangesInput`](crate::operation::get_checker_ip_ranges::GetCheckerIpRangesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_checker_ip_ranges::GetCheckerIpRangesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_checker_ip_ranges::GetCheckerIpRangesInput {})
    }
}
