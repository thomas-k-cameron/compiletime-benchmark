// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_managed_prefix_list_associations::_get_managed_prefix_list_associations_output::GetManagedPrefixListAssociationsOutputBuilder;

pub use crate::operation::get_managed_prefix_list_associations::_get_managed_prefix_list_associations_input::GetManagedPrefixListAssociationsInputBuilder;

/// Fluent builder constructing a request to `GetManagedPrefixListAssociations`.
///
/// <p>Gets information about the resources that are associated with the specified managed prefix list.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetManagedPrefixListAssociationsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsInputBuilder
            }
impl GetManagedPrefixListAssociationsFluentBuilder {
    /// Creates a new `GetManagedPrefixListAssociations`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_managed_prefix_list_associations::GetManagedPrefixListAssociations, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_managed_prefix_list_associations::GetManagedPrefixListAssociationsError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_managed_prefix_list_associations::GetManagedPrefixListAssociationsOutput, aws_smithy_http::result::SdkError<crate::operation::get_managed_prefix_list_associations::GetManagedPrefixListAssociationsError>>
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
    ///     let deserialized_parameters: crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_managed_prefix_list_associations().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_managed_prefix_list_associations::paginator::GetManagedPrefixListAssociationsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::get_managed_prefix_list_associations::paginator::GetManagedPrefixListAssociationsPaginator{
        crate::operation::get_managed_prefix_list_associations::paginator::GetManagedPrefixListAssociationsPaginator::new(self.handle, self.inner)
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
    /// <p>The ID of the prefix list.</p>
    pub fn prefix_list_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.prefix_list_id(input.into());
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn set_prefix_list_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_prefix_list_id(input);
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
}
