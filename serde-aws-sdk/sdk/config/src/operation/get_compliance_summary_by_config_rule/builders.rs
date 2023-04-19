// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_compliance_summary_by_config_rule::_get_compliance_summary_by_config_rule_output::GetComplianceSummaryByConfigRuleOutputBuilder;

pub use crate::operation::get_compliance_summary_by_config_rule::_get_compliance_summary_by_config_rule_input::GetComplianceSummaryByConfigRuleInputBuilder;

/// Fluent builder constructing a request to `GetComplianceSummaryByConfigRule`.
///
/// <p>Returns the number of Config rules that are compliant and noncompliant, up to a maximum of 25 for each.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetComplianceSummaryByConfigRuleFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_compliance_summary_by_config_rule::builders::GetComplianceSummaryByConfigRuleInputBuilder
            }
impl GetComplianceSummaryByConfigRuleFluentBuilder {
    /// Creates a new `GetComplianceSummaryByConfigRule`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRule, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleOutput, aws_smithy_http::result::SdkError<crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleError>>
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
    ///     let deserialized_parameters: crate::operation::get_compliance_summary_by_config_rule::builders::GetComplianceSummaryByConfigRuleInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_compliance_summary_by_config_rule().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_compliance_summary_by_config_rule::builders::GetComplianceSummaryByConfigRuleInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
}
