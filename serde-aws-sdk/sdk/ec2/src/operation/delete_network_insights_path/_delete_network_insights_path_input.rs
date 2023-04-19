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
pub struct DeleteNetworkInsightsPathInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The ID of the path.</p>
    #[doc(hidden)]
    pub network_insights_path_id: std::option::Option<std::string::String>,
}
impl DeleteNetworkInsightsPathInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the path.</p>
    pub fn network_insights_path_id(&self) -> std::option::Option<&str> {
        self.network_insights_path_id.as_deref()
    }
}
impl DeleteNetworkInsightsPathInput {
    /// Creates a new builder-style object to manufacture [`DeleteNetworkInsightsPathInput`](crate::operation::delete_network_insights_path::DeleteNetworkInsightsPathInput).
    pub fn builder() -> crate::operation::delete_network_insights_path::builders::DeleteNetworkInsightsPathInputBuilder{
        crate::operation::delete_network_insights_path::builders::DeleteNetworkInsightsPathInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_network_insights_path::DeleteNetworkInsightsPathInput;
/// A builder for [`DeleteNetworkInsightsPathInput`](crate::operation::delete_network_insights_path::DeleteNetworkInsightsPathInput).
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
pub struct DeleteNetworkInsightsPathInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) network_insights_path_id: std::option::Option<std::string::String>,
}
impl DeleteNetworkInsightsPathInputBuilder {
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
    /// <p>The ID of the path.</p>
    pub fn network_insights_path_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_insights_path_id = Some(input.into());
        self
    }
    /// <p>The ID of the path.</p>
    pub fn set_network_insights_path_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_insights_path_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteNetworkInsightsPathInput`](crate::operation::delete_network_insights_path::DeleteNetworkInsightsPathInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_network_insights_path::DeleteNetworkInsightsPathInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_network_insights_path::DeleteNetworkInsightsPathInput {
                dry_run: self.dry_run,
                network_insights_path_id: self.network_insights_path_id,
            },
        )
    }
}
