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
pub struct DescribeMultiRegionAccessPointOperationInput {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The request token associated with the request you want to know about. This request token is returned as part of the response when you make an asynchronous request. You provide this token to query about the status of the asynchronous action.</p>
    #[doc(hidden)]
    pub request_token_arn: std::option::Option<std::string::String>,
}
impl DescribeMultiRegionAccessPointOperationInput {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The request token associated with the request you want to know about. This request token is returned as part of the response when you make an asynchronous request. You provide this token to query about the status of the asynchronous action.</p>
    pub fn request_token_arn(&self) -> std::option::Option<&str> {
        self.request_token_arn.as_deref()
    }
}
impl DescribeMultiRegionAccessPointOperationInput {
    /// Creates a new builder-style object to manufacture [`DescribeMultiRegionAccessPointOperationInput`](crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationInput).
    pub fn builder() -> crate::operation::describe_multi_region_access_point_operation::builders::DescribeMultiRegionAccessPointOperationInputBuilder{
        crate::operation::describe_multi_region_access_point_operation::builders::DescribeMultiRegionAccessPointOperationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationInput;
/// A builder for [`DescribeMultiRegionAccessPointOperationInput`](crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationInput).
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
pub struct DescribeMultiRegionAccessPointOperationInputBuilder {
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) request_token_arn: std::option::Option<std::string::String>,
}
impl DescribeMultiRegionAccessPointOperationInputBuilder {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The request token associated with the request you want to know about. This request token is returned as part of the response when you make an asynchronous request. You provide this token to query about the status of the asynchronous action.</p>
    pub fn request_token_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.request_token_arn = Some(input.into());
        self
    }
    /// <p>The request token associated with the request you want to know about. This request token is returned as part of the response when you make an asynchronous request. You provide this token to query about the status of the asynchronous action.</p>
    pub fn set_request_token_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.request_token_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeMultiRegionAccessPointOperationInput`](crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationInput).
    pub fn build(self) -> Result<crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationInput {
                account_id: self.account_id
                ,
                request_token_arn: self.request_token_arn
                ,
            }
        )
    }
}
