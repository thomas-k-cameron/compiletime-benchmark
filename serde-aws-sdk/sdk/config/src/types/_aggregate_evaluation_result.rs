// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of an Config evaluation for an account ID and region in an aggregator. Provides the Amazon Web Services resource that was evaluated, the compliance of the resource, related time stamps, and supplementary information. </p>
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
pub struct AggregateEvaluationResult {
    /// <p>Uniquely identifies the evaluation result.</p>
    #[doc(hidden)]
    pub evaluation_result_identifier: std::option::Option<crate::types::EvaluationResultIdentifier>,
    /// <p>The resource compliance status.</p>
    /// <p>For the <code>AggregationEvaluationResult</code> data type, Config supports only the <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. Config does not support the <code>NOT_APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> value.</p>
    #[doc(hidden)]
    pub compliance_type: std::option::Option<crate::types::ComplianceType>,
    /// <p>The time when Config recorded the aggregate evaluation result.</p>
    #[doc(hidden)]
    pub result_recorded_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The time when the Config rule evaluated the Amazon Web Services resource.</p>
    #[doc(hidden)]
    pub config_rule_invoked_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Supplementary information about how the agrregate evaluation determined the compliance.</p>
    #[doc(hidden)]
    pub annotation: std::option::Option<std::string::String>,
    /// <p>The 12-digit account ID of the source account.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The source region from where the data is aggregated.</p>
    #[doc(hidden)]
    pub aws_region: std::option::Option<std::string::String>,
}
impl AggregateEvaluationResult {
    /// <p>Uniquely identifies the evaluation result.</p>
    pub fn evaluation_result_identifier(
        &self,
    ) -> std::option::Option<&crate::types::EvaluationResultIdentifier> {
        self.evaluation_result_identifier.as_ref()
    }
    /// <p>The resource compliance status.</p>
    /// <p>For the <code>AggregationEvaluationResult</code> data type, Config supports only the <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. Config does not support the <code>NOT_APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> value.</p>
    pub fn compliance_type(&self) -> std::option::Option<&crate::types::ComplianceType> {
        self.compliance_type.as_ref()
    }
    /// <p>The time when Config recorded the aggregate evaluation result.</p>
    pub fn result_recorded_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.result_recorded_time.as_ref()
    }
    /// <p>The time when the Config rule evaluated the Amazon Web Services resource.</p>
    pub fn config_rule_invoked_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.config_rule_invoked_time.as_ref()
    }
    /// <p>Supplementary information about how the agrregate evaluation determined the compliance.</p>
    pub fn annotation(&self) -> std::option::Option<&str> {
        self.annotation.as_deref()
    }
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The source region from where the data is aggregated.</p>
    pub fn aws_region(&self) -> std::option::Option<&str> {
        self.aws_region.as_deref()
    }
}
impl AggregateEvaluationResult {
    /// Creates a new builder-style object to manufacture [`AggregateEvaluationResult`](crate::types::AggregateEvaluationResult).
    pub fn builder() -> crate::types::builders::AggregateEvaluationResultBuilder {
        crate::types::builders::AggregateEvaluationResultBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AggregateEvaluationResult;
/// A builder for [`AggregateEvaluationResult`](crate::types::AggregateEvaluationResult).
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
pub struct AggregateEvaluationResultBuilder {
    pub(crate) evaluation_result_identifier:
        std::option::Option<crate::types::EvaluationResultIdentifier>,
    pub(crate) compliance_type: std::option::Option<crate::types::ComplianceType>,
    pub(crate) result_recorded_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) config_rule_invoked_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) annotation: std::option::Option<std::string::String>,
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) aws_region: std::option::Option<std::string::String>,
}
impl AggregateEvaluationResultBuilder {
    /// <p>Uniquely identifies the evaluation result.</p>
    pub fn evaluation_result_identifier(
        mut self,
        input: crate::types::EvaluationResultIdentifier,
    ) -> Self {
        self.evaluation_result_identifier = Some(input);
        self
    }
    /// <p>Uniquely identifies the evaluation result.</p>
    pub fn set_evaluation_result_identifier(
        mut self,
        input: std::option::Option<crate::types::EvaluationResultIdentifier>,
    ) -> Self {
        self.evaluation_result_identifier = input;
        self
    }
    /// <p>The resource compliance status.</p>
    /// <p>For the <code>AggregationEvaluationResult</code> data type, Config supports only the <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. Config does not support the <code>NOT_APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> value.</p>
    pub fn compliance_type(mut self, input: crate::types::ComplianceType) -> Self {
        self.compliance_type = Some(input);
        self
    }
    /// <p>The resource compliance status.</p>
    /// <p>For the <code>AggregationEvaluationResult</code> data type, Config supports only the <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. Config does not support the <code>NOT_APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> value.</p>
    pub fn set_compliance_type(
        mut self,
        input: std::option::Option<crate::types::ComplianceType>,
    ) -> Self {
        self.compliance_type = input;
        self
    }
    /// <p>The time when Config recorded the aggregate evaluation result.</p>
    pub fn result_recorded_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.result_recorded_time = Some(input);
        self
    }
    /// <p>The time when Config recorded the aggregate evaluation result.</p>
    pub fn set_result_recorded_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.result_recorded_time = input;
        self
    }
    /// <p>The time when the Config rule evaluated the Amazon Web Services resource.</p>
    pub fn config_rule_invoked_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.config_rule_invoked_time = Some(input);
        self
    }
    /// <p>The time when the Config rule evaluated the Amazon Web Services resource.</p>
    pub fn set_config_rule_invoked_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.config_rule_invoked_time = input;
        self
    }
    /// <p>Supplementary information about how the agrregate evaluation determined the compliance.</p>
    pub fn annotation(mut self, input: impl Into<std::string::String>) -> Self {
        self.annotation = Some(input.into());
        self
    }
    /// <p>Supplementary information about how the agrregate evaluation determined the compliance.</p>
    pub fn set_annotation(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.annotation = input;
        self
    }
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The 12-digit account ID of the source account.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The source region from where the data is aggregated.</p>
    pub fn aws_region(mut self, input: impl Into<std::string::String>) -> Self {
        self.aws_region = Some(input.into());
        self
    }
    /// <p>The source region from where the data is aggregated.</p>
    pub fn set_aws_region(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.aws_region = input;
        self
    }
    /// Consumes the builder and constructs a [`AggregateEvaluationResult`](crate::types::AggregateEvaluationResult).
    pub fn build(self) -> crate::types::AggregateEvaluationResult {
        crate::types::AggregateEvaluationResult {
            evaluation_result_identifier: self.evaluation_result_identifier,
            compliance_type: self.compliance_type,
            result_recorded_time: self.result_recorded_time,
            config_rule_invoked_time: self.config_rule_invoked_time,
            annotation: self.annotation,
            account_id: self.account_id,
            aws_region: self.aws_region,
        }
    }
}
