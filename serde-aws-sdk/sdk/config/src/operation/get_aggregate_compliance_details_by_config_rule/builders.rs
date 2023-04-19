// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_aggregate_compliance_details_by_config_rule::_get_aggregate_compliance_details_by_config_rule_output::GetAggregateComplianceDetailsByConfigRuleOutputBuilder;

pub use crate::operation::get_aggregate_compliance_details_by_config_rule::_get_aggregate_compliance_details_by_config_rule_input::GetAggregateComplianceDetailsByConfigRuleInputBuilder;

/// Fluent builder constructing a request to `GetAggregateComplianceDetailsByConfigRule`.
///
/// <p>Returns the evaluation results for the specified Config rule for a specific resource in a rule. The results indicate which Amazon Web Services resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule. </p> <note>
/// <p>The results can return an empty result page. But if you have a <code>nextToken</code>, the results are displayed on the next page.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetAggregateComplianceDetailsByConfigRuleFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_aggregate_compliance_details_by_config_rule::builders::GetAggregateComplianceDetailsByConfigRuleInputBuilder
            }
impl GetAggregateComplianceDetailsByConfigRuleFluentBuilder {
    /// Creates a new `GetAggregateComplianceDetailsByConfigRule`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRule, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleOutput, aws_smithy_http::result::SdkError<crate::operation::get_aggregate_compliance_details_by_config_rule::GetAggregateComplianceDetailsByConfigRuleError>>
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
    ///     let deserialized_parameters: crate::operation::get_aggregate_compliance_details_by_config_rule::builders::GetAggregateComplianceDetailsByConfigRuleInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_aggregate_compliance_details_by_config_rule().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_aggregate_compliance_details_by_config_rule::builders::GetAggregateComplianceDetailsByConfigRuleInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_aggregate_compliance_details_by_config_rule::paginator::GetAggregateComplianceDetailsByConfigRulePaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::get_aggregate_compliance_details_by_config_rule::paginator::GetAggregateComplianceDetailsByConfigRulePaginator{
        crate::operation::get_aggregate_compliance_details_by_config_rule::paginator::GetAggregateComplianceDetailsByConfigRulePaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the configuration aggregator.</p>
    pub fn configuration_aggregator_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.configuration_aggregator_name(input.into());
        self
    }
    /// <p>The name of the configuration aggregator.</p>
    pub fn set_configuration_aggregator_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_aggregator_name(input);
        self
    }
    /// <p>The name of the Config rule for which you want compliance information.</p>
    pub fn config_rule_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.config_rule_name(input.into());
        self
    }
    /// <p>The name of the Config rule for which you want compliance information.</p>
    pub fn set_config_rule_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_config_rule_name(input);
        self
    }
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The source region from where the data is aggregated.</p>
    pub fn aws_region(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.aws_region(input.into());
        self
    }
    /// <p>The source region from where the data is aggregated.</p>
    pub fn set_aws_region(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_aws_region(input);
        self
    }
    /// <p>The resource compliance status.</p> <note>
    /// <p>For the <code>GetAggregateComplianceDetailsByConfigRuleRequest</code> data type, Config supports only the <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. Config does not support the <code>NOT_APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> values.</p>
    /// </note>
    pub fn compliance_type(mut self, input: crate::types::ComplianceType) -> Self {
        self.inner = self.inner.compliance_type(input);
        self
    }
    /// <p>The resource compliance status.</p> <note>
    /// <p>For the <code>GetAggregateComplianceDetailsByConfigRuleRequest</code> data type, Config supports only the <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. Config does not support the <code>NOT_APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> values.</p>
    /// </note>
    pub fn set_compliance_type(
        mut self,
        input: std::option::Option<crate::types::ComplianceType>,
    ) -> Self {
        self.inner = self.inner.set_compliance_type(input);
        self
    }
    /// <p>The maximum number of evaluation results returned on each page. The default is 50. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of evaluation results returned on each page. The default is 50. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
