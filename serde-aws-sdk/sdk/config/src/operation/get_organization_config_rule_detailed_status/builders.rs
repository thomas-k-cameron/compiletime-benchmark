// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_organization_config_rule_detailed_status::_get_organization_config_rule_detailed_status_output::GetOrganizationConfigRuleDetailedStatusOutputBuilder;

pub use crate::operation::get_organization_config_rule_detailed_status::_get_organization_config_rule_detailed_status_input::GetOrganizationConfigRuleDetailedStatusInputBuilder;

/// Fluent builder constructing a request to `GetOrganizationConfigRuleDetailedStatus`.
///
/// <p>Returns detailed status for each member account within an organization for a given organization Config rule.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetOrganizationConfigRuleDetailedStatusFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_organization_config_rule_detailed_status::builders::GetOrganizationConfigRuleDetailedStatusInputBuilder
            }
impl GetOrganizationConfigRuleDetailedStatusFluentBuilder {
    /// Creates a new `GetOrganizationConfigRuleDetailedStatus`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatus, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput, aws_smithy_http::result::SdkError<crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusError>>
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
    ///     let deserialized_parameters: crate::operation::get_organization_config_rule_detailed_status::builders::GetOrganizationConfigRuleDetailedStatusInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_organization_config_rule_detailed_status().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_organization_config_rule_detailed_status::builders::GetOrganizationConfigRuleDetailedStatusInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_organization_config_rule_detailed_status::paginator::GetOrganizationConfigRuleDetailedStatusPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::get_organization_config_rule_detailed_status::paginator::GetOrganizationConfigRuleDetailedStatusPaginator{
        crate::operation::get_organization_config_rule_detailed_status::paginator::GetOrganizationConfigRuleDetailedStatusPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of your organization Config rule for which you want status details for member accounts.</p>
    pub fn organization_config_rule_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.organization_config_rule_name(input.into());
        self
    }
    /// <p>The name of your organization Config rule for which you want status details for member accounts.</p>
    pub fn set_organization_config_rule_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_organization_config_rule_name(input);
        self
    }
    /// <p>A <code>StatusDetailFilters</code> object.</p>
    pub fn filters(mut self, input: crate::types::StatusDetailFilters) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A <code>StatusDetailFilters</code> object.</p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<crate::types::StatusDetailFilters>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum number of <code>OrganizationConfigRuleDetailedStatus</code> returned on each page. If you do not specify a number, Config uses the default. The default is 100.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of <code>OrganizationConfigRuleDetailedStatus</code> returned on each page. If you do not specify a number, Config uses the default. The default is 100.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}