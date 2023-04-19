// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_organization_conformance_pack_statuses::_describe_organization_conformance_pack_statuses_output::DescribeOrganizationConformancePackStatusesOutputBuilder;

pub use crate::operation::describe_organization_conformance_pack_statuses::_describe_organization_conformance_pack_statuses_input::DescribeOrganizationConformancePackStatusesInputBuilder;

/// Fluent builder constructing a request to `DescribeOrganizationConformancePackStatuses`.
///
/// <p>Provides organization conformance pack deployment status for an organization. </p> <note>
/// <p>The status is not considered successful until organization conformance pack is successfully deployed in all the member accounts with an exception of excluded accounts.</p>
/// <p>When you specify the limit and the next token, you receive a paginated response. Limit and next token are not applicable if you specify organization conformance pack names. They are only applicable, when you request all the organization conformance packs.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeOrganizationConformancePackStatusesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_organization_conformance_pack_statuses::builders::DescribeOrganizationConformancePackStatusesInputBuilder
            }
impl DescribeOrganizationConformancePackStatusesFluentBuilder {
    /// Creates a new `DescribeOrganizationConformancePackStatuses`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatuses, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesOutput, aws_smithy_http::result::SdkError<crate::operation::describe_organization_conformance_pack_statuses::DescribeOrganizationConformancePackStatusesError>>
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
    ///     let deserialized_parameters: crate::operation::describe_organization_conformance_pack_statuses::builders::DescribeOrganizationConformancePackStatusesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_organization_conformance_pack_statuses().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_organization_conformance_pack_statuses::builders::DescribeOrganizationConformancePackStatusesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_organization_conformance_pack_statuses::paginator::DescribeOrganizationConformancePackStatusesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_organization_conformance_pack_statuses::paginator::DescribeOrganizationConformancePackStatusesPaginator{
        crate::operation::describe_organization_conformance_pack_statuses::paginator::DescribeOrganizationConformancePackStatusesPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `OrganizationConformancePackNames`.
    ///
    /// To override the contents of this collection use [`set_organization_conformance_pack_names`](Self::set_organization_conformance_pack_names).
    ///
    /// <p>The names of organization conformance packs for which you want status details. If you do not specify any names, Config returns details for all your organization conformance packs. </p>
    pub fn organization_conformance_pack_names(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.organization_conformance_pack_names(input.into());
        self
    }
    /// <p>The names of organization conformance packs for which you want status details. If you do not specify any names, Config returns details for all your organization conformance packs. </p>
    pub fn set_organization_conformance_pack_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_organization_conformance_pack_names(input);
        self
    }
    /// <p>The maximum number of OrganizationConformancePackStatuses returned on each page. If you do no specify a number, Config uses the default. The default is 100. </p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of OrganizationConformancePackStatuses returned on each page. If you do no specify a number, Config uses the default. The default is 100. </p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
