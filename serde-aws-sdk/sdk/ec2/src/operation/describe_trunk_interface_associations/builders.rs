// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_trunk_interface_associations::_describe_trunk_interface_associations_output::DescribeTrunkInterfaceAssociationsOutputBuilder;

pub use crate::operation::describe_trunk_interface_associations::_describe_trunk_interface_associations_input::DescribeTrunkInterfaceAssociationsInputBuilder;

/// Fluent builder constructing a request to `DescribeTrunkInterfaceAssociations`.
///
/// <note>
/// <p>This API action is currently in <b>limited preview only</b>. If you are interested in using this feature, contact your account manager.</p>
/// </note>
/// <p>Describes one or more network interface trunk associations.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeTrunkInterfaceAssociationsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsInputBuilder
            }
impl DescribeTrunkInterfaceAssociationsFluentBuilder {
    /// Creates a new `DescribeTrunkInterfaceAssociations`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociations, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsError>
    >{
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsOutput, aws_smithy_http::result::SdkError<crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_trunk_interface_associations().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_trunk_interface_associations::paginator::DescribeTrunkInterfaceAssociationsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_trunk_interface_associations::paginator::DescribeTrunkInterfaceAssociationsPaginator{
        crate::operation::describe_trunk_interface_associations::paginator::DescribeTrunkInterfaceAssociationsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `AssociationIds`.
    ///
    /// To override the contents of this collection use [`set_association_ids`](Self::set_association_ids).
    ///
    /// <p>The IDs of the associations.</p>
    pub fn association_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.association_ids(input.into());
        self
    }
    /// <p>The IDs of the associations.</p>
    pub fn set_association_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_association_ids(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>gre-key</code> - The ID of a trunk interface association.</p> </li>
    /// <li> <p> <code>interface-protocol</code> - The interface protocol. Valid values are <code>VLAN</code> and <code>GRE</code>.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>gre-key</code> - The ID of a trunk interface association.</p> </li>
    /// <li> <p> <code>interface-protocol</code> - The interface protocol. Valid values are <code>VLAN</code> and <code>GRE</code>.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
