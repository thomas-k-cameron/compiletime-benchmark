// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator.</p>
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
pub struct AggregateComplianceCount {
    /// <p>The 12-digit account ID or region based on the GroupByKey value.</p>
    #[doc(hidden)]
    pub group_name: std::option::Option<std::string::String>,
    /// <p>The number of compliant and noncompliant Config rules.</p>
    #[doc(hidden)]
    pub compliance_summary: std::option::Option<crate::types::ComplianceSummary>,
}
impl AggregateComplianceCount {
    /// <p>The 12-digit account ID or region based on the GroupByKey value.</p>
    pub fn group_name(&self) -> std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The number of compliant and noncompliant Config rules.</p>
    pub fn compliance_summary(&self) -> std::option::Option<&crate::types::ComplianceSummary> {
        self.compliance_summary.as_ref()
    }
}
impl AggregateComplianceCount {
    /// Creates a new builder-style object to manufacture [`AggregateComplianceCount`](crate::types::AggregateComplianceCount).
    pub fn builder() -> crate::types::builders::AggregateComplianceCountBuilder {
        crate::types::builders::AggregateComplianceCountBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AggregateComplianceCount;
/// A builder for [`AggregateComplianceCount`](crate::types::AggregateComplianceCount).
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
pub struct AggregateComplianceCountBuilder {
    pub(crate) group_name: std::option::Option<std::string::String>,
    pub(crate) compliance_summary: std::option::Option<crate::types::ComplianceSummary>,
}
impl AggregateComplianceCountBuilder {
    /// <p>The 12-digit account ID or region based on the GroupByKey value.</p>
    pub fn group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.group_name = Some(input.into());
        self
    }
    /// <p>The 12-digit account ID or region based on the GroupByKey value.</p>
    pub fn set_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>The number of compliant and noncompliant Config rules.</p>
    pub fn compliance_summary(mut self, input: crate::types::ComplianceSummary) -> Self {
        self.compliance_summary = Some(input);
        self
    }
    /// <p>The number of compliant and noncompliant Config rules.</p>
    pub fn set_compliance_summary(
        mut self,
        input: std::option::Option<crate::types::ComplianceSummary>,
    ) -> Self {
        self.compliance_summary = input;
        self
    }
    /// Consumes the builder and constructs a [`AggregateComplianceCount`](crate::types::AggregateComplianceCount).
    pub fn build(self) -> crate::types::AggregateComplianceCount {
        crate::types::AggregateComplianceCount {
            group_name: self.group_name,
            compliance_summary: self.compliance_summary,
        }
    }
}
