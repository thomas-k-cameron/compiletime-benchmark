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
pub struct DescribeAddressTransfersInput {
    /// <p>The allocation IDs of Elastic IP addresses.</p>
    #[doc(hidden)]
    pub allocation_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of address transfers to return in one page of results.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DescribeAddressTransfersInput {
    /// <p>The allocation IDs of Elastic IP addresses.</p>
    pub fn allocation_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.allocation_ids.as_deref()
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of address transfers to return in one page of results.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeAddressTransfersInput {
    /// Creates a new builder-style object to manufacture [`DescribeAddressTransfersInput`](crate::operation::describe_address_transfers::DescribeAddressTransfersInput).
    pub fn builder(
    ) -> crate::operation::describe_address_transfers::builders::DescribeAddressTransfersInputBuilder
    {
        crate::operation::describe_address_transfers::builders::DescribeAddressTransfersInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_address_transfers::DescribeAddressTransfersInput;
/// A builder for [`DescribeAddressTransfersInput`](crate::operation::describe_address_transfers::DescribeAddressTransfersInput).
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
pub struct DescribeAddressTransfersInputBuilder {
    pub(crate) allocation_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DescribeAddressTransfersInputBuilder {
    /// Appends an item to `allocation_ids`.
    ///
    /// To override the contents of this collection use [`set_allocation_ids`](Self::set_allocation_ids).
    ///
    /// <p>The allocation IDs of Elastic IP addresses.</p>
    pub fn allocation_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.allocation_ids.unwrap_or_default();
        v.push(input.into());
        self.allocation_ids = Some(v);
        self
    }
    /// <p>The allocation IDs of Elastic IP addresses.</p>
    pub fn set_allocation_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.allocation_ids = input;
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of address transfers to return in one page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of address transfers to return in one page of results.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAddressTransfersInput`](crate::operation::describe_address_transfers::DescribeAddressTransfersInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_address_transfers::DescribeAddressTransfersInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_address_transfers::DescribeAddressTransfersInput {
                allocation_ids: self.allocation_ids,
                next_token: self.next_token,
                max_results: self.max_results,
                dry_run: self.dry_run,
            },
        )
    }
}