// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Identifies an Amazon Web Services resource and indicates whether it complies with the Config rule that it was evaluated against.</p>
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
pub struct ExternalEvaluation {
    /// <p>The evaluated compliance resource type. Config accepts <code>AWS::::Account</code> resource type.</p>
    #[doc(hidden)]
    pub compliance_resource_type: std::option::Option<std::string::String>,
    /// <p>The evaluated compliance resource ID. Config accepts only Amazon Web Services account ID.</p>
    #[doc(hidden)]
    pub compliance_resource_id: std::option::Option<std::string::String>,
    /// <p>The compliance of the Amazon Web Services resource. The valid values are <code>COMPLIANT, NON_COMPLIANT, </code> and <code>NOT_APPLICABLE</code>.</p>
    #[doc(hidden)]
    pub compliance_type: std::option::Option<crate::types::ComplianceType>,
    /// <p>Supplementary information about the reason of compliance. For example, this task was completed on a specific date.</p>
    #[doc(hidden)]
    pub annotation: std::option::Option<std::string::String>,
    /// <p>The time when the compliance was recorded. </p>
    #[doc(hidden)]
    pub ordering_timestamp: std::option::Option<aws_smithy_types::DateTime>,
}
impl ExternalEvaluation {
    /// <p>The evaluated compliance resource type. Config accepts <code>AWS::::Account</code> resource type.</p>
    pub fn compliance_resource_type(&self) -> std::option::Option<&str> {
        self.compliance_resource_type.as_deref()
    }
    /// <p>The evaluated compliance resource ID. Config accepts only Amazon Web Services account ID.</p>
    pub fn compliance_resource_id(&self) -> std::option::Option<&str> {
        self.compliance_resource_id.as_deref()
    }
    /// <p>The compliance of the Amazon Web Services resource. The valid values are <code>COMPLIANT, NON_COMPLIANT, </code> and <code>NOT_APPLICABLE</code>.</p>
    pub fn compliance_type(&self) -> std::option::Option<&crate::types::ComplianceType> {
        self.compliance_type.as_ref()
    }
    /// <p>Supplementary information about the reason of compliance. For example, this task was completed on a specific date.</p>
    pub fn annotation(&self) -> std::option::Option<&str> {
        self.annotation.as_deref()
    }
    /// <p>The time when the compliance was recorded. </p>
    pub fn ordering_timestamp(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.ordering_timestamp.as_ref()
    }
}
impl ExternalEvaluation {
    /// Creates a new builder-style object to manufacture [`ExternalEvaluation`](crate::types::ExternalEvaluation).
    pub fn builder() -> crate::types::builders::ExternalEvaluationBuilder {
        crate::types::builders::ExternalEvaluationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ExternalEvaluation;
/// A builder for [`ExternalEvaluation`](crate::types::ExternalEvaluation).
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
pub struct ExternalEvaluationBuilder {
    pub(crate) compliance_resource_type: std::option::Option<std::string::String>,
    pub(crate) compliance_resource_id: std::option::Option<std::string::String>,
    pub(crate) compliance_type: std::option::Option<crate::types::ComplianceType>,
    pub(crate) annotation: std::option::Option<std::string::String>,
    pub(crate) ordering_timestamp: std::option::Option<aws_smithy_types::DateTime>,
}
impl ExternalEvaluationBuilder {
    /// <p>The evaluated compliance resource type. Config accepts <code>AWS::::Account</code> resource type.</p>
    pub fn compliance_resource_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.compliance_resource_type = Some(input.into());
        self
    }
    /// <p>The evaluated compliance resource type. Config accepts <code>AWS::::Account</code> resource type.</p>
    pub fn set_compliance_resource_type(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.compliance_resource_type = input;
        self
    }
    /// <p>The evaluated compliance resource ID. Config accepts only Amazon Web Services account ID.</p>
    pub fn compliance_resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.compliance_resource_id = Some(input.into());
        self
    }
    /// <p>The evaluated compliance resource ID. Config accepts only Amazon Web Services account ID.</p>
    pub fn set_compliance_resource_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.compliance_resource_id = input;
        self
    }
    /// <p>The compliance of the Amazon Web Services resource. The valid values are <code>COMPLIANT, NON_COMPLIANT, </code> and <code>NOT_APPLICABLE</code>.</p>
    pub fn compliance_type(mut self, input: crate::types::ComplianceType) -> Self {
        self.compliance_type = Some(input);
        self
    }
    /// <p>The compliance of the Amazon Web Services resource. The valid values are <code>COMPLIANT, NON_COMPLIANT, </code> and <code>NOT_APPLICABLE</code>.</p>
    pub fn set_compliance_type(
        mut self,
        input: std::option::Option<crate::types::ComplianceType>,
    ) -> Self {
        self.compliance_type = input;
        self
    }
    /// <p>Supplementary information about the reason of compliance. For example, this task was completed on a specific date.</p>
    pub fn annotation(mut self, input: impl Into<std::string::String>) -> Self {
        self.annotation = Some(input.into());
        self
    }
    /// <p>Supplementary information about the reason of compliance. For example, this task was completed on a specific date.</p>
    pub fn set_annotation(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.annotation = input;
        self
    }
    /// <p>The time when the compliance was recorded. </p>
    pub fn ordering_timestamp(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.ordering_timestamp = Some(input);
        self
    }
    /// <p>The time when the compliance was recorded. </p>
    pub fn set_ordering_timestamp(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.ordering_timestamp = input;
        self
    }
    /// Consumes the builder and constructs a [`ExternalEvaluation`](crate::types::ExternalEvaluation).
    pub fn build(self) -> crate::types::ExternalEvaluation {
        crate::types::ExternalEvaluation {
            compliance_resource_type: self.compliance_resource_type,
            compliance_resource_id: self.compliance_resource_id,
            compliance_type: self.compliance_type,
            annotation: self.annotation,
            ordering_timestamp: self.ordering_timestamp,
        }
    }
}
