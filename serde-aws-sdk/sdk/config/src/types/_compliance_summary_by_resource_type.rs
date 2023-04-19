// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The number of Amazon Web Services resources of a specific type that are compliant or noncompliant, up to a maximum of 100 for each.</p>
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
pub struct ComplianceSummaryByResourceType {
    /// <p>The type of Amazon Web Services resource.</p>
    #[doc(hidden)]
    pub resource_type: std::option::Option<std::string::String>,
    /// <p>The number of Amazon Web Services resources that are compliant or noncompliant, up to a maximum of 100 for each.</p>
    #[doc(hidden)]
    pub compliance_summary: std::option::Option<crate::types::ComplianceSummary>,
}
impl ComplianceSummaryByResourceType {
    /// <p>The type of Amazon Web Services resource.</p>
    pub fn resource_type(&self) -> std::option::Option<&str> {
        self.resource_type.as_deref()
    }
    /// <p>The number of Amazon Web Services resources that are compliant or noncompliant, up to a maximum of 100 for each.</p>
    pub fn compliance_summary(&self) -> std::option::Option<&crate::types::ComplianceSummary> {
        self.compliance_summary.as_ref()
    }
}
impl ComplianceSummaryByResourceType {
    /// Creates a new builder-style object to manufacture [`ComplianceSummaryByResourceType`](crate::types::ComplianceSummaryByResourceType).
    pub fn builder() -> crate::types::builders::ComplianceSummaryByResourceTypeBuilder {
        crate::types::builders::ComplianceSummaryByResourceTypeBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ComplianceSummaryByResourceType;
/// A builder for [`ComplianceSummaryByResourceType`](crate::types::ComplianceSummaryByResourceType).
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
pub struct ComplianceSummaryByResourceTypeBuilder {
    pub(crate) resource_type: std::option::Option<std::string::String>,
    pub(crate) compliance_summary: std::option::Option<crate::types::ComplianceSummary>,
}
impl ComplianceSummaryByResourceTypeBuilder {
    /// <p>The type of Amazon Web Services resource.</p>
    pub fn resource_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_type = Some(input.into());
        self
    }
    /// <p>The type of Amazon Web Services resource.</p>
    pub fn set_resource_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The number of Amazon Web Services resources that are compliant or noncompliant, up to a maximum of 100 for each.</p>
    pub fn compliance_summary(mut self, input: crate::types::ComplianceSummary) -> Self {
        self.compliance_summary = Some(input);
        self
    }
    /// <p>The number of Amazon Web Services resources that are compliant or noncompliant, up to a maximum of 100 for each.</p>
    pub fn set_compliance_summary(
        mut self,
        input: std::option::Option<crate::types::ComplianceSummary>,
    ) -> Self {
        self.compliance_summary = input;
        self
    }
    /// Consumes the builder and constructs a [`ComplianceSummaryByResourceType`](crate::types::ComplianceSummaryByResourceType).
    pub fn build(self) -> crate::types::ComplianceSummaryByResourceType {
        crate::types::ComplianceSummaryByResourceType {
            resource_type: self.resource_type,
            compliance_summary: self.compliance_summary,
        }
    }
}