// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary includes the name and status of the conformance pack.</p>
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
pub struct ConformancePackComplianceSummary {
    /// <p>The name of the conformance pack name.</p>
    #[doc(hidden)]
    pub conformance_pack_name: std::option::Option<std::string::String>,
    /// <p>The status of the conformance pack. The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code> and <code>INSUFFICIENT_DATA</code>.</p>
    #[doc(hidden)]
    pub conformance_pack_compliance_status:
        std::option::Option<crate::types::ConformancePackComplianceType>,
}
impl ConformancePackComplianceSummary {
    /// <p>The name of the conformance pack name.</p>
    pub fn conformance_pack_name(&self) -> std::option::Option<&str> {
        self.conformance_pack_name.as_deref()
    }
    /// <p>The status of the conformance pack. The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code> and <code>INSUFFICIENT_DATA</code>.</p>
    pub fn conformance_pack_compliance_status(
        &self,
    ) -> std::option::Option<&crate::types::ConformancePackComplianceType> {
        self.conformance_pack_compliance_status.as_ref()
    }
}
impl ConformancePackComplianceSummary {
    /// Creates a new builder-style object to manufacture [`ConformancePackComplianceSummary`](crate::types::ConformancePackComplianceSummary).
    pub fn builder() -> crate::types::builders::ConformancePackComplianceSummaryBuilder {
        crate::types::builders::ConformancePackComplianceSummaryBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ConformancePackComplianceSummary;
/// A builder for [`ConformancePackComplianceSummary`](crate::types::ConformancePackComplianceSummary).
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
pub struct ConformancePackComplianceSummaryBuilder {
    pub(crate) conformance_pack_name: std::option::Option<std::string::String>,
    pub(crate) conformance_pack_compliance_status:
        std::option::Option<crate::types::ConformancePackComplianceType>,
}
impl ConformancePackComplianceSummaryBuilder {
    /// <p>The name of the conformance pack name.</p>
    pub fn conformance_pack_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.conformance_pack_name = Some(input.into());
        self
    }
    /// <p>The name of the conformance pack name.</p>
    pub fn set_conformance_pack_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.conformance_pack_name = input;
        self
    }
    /// <p>The status of the conformance pack. The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code> and <code>INSUFFICIENT_DATA</code>.</p>
    pub fn conformance_pack_compliance_status(
        mut self,
        input: crate::types::ConformancePackComplianceType,
    ) -> Self {
        self.conformance_pack_compliance_status = Some(input);
        self
    }
    /// <p>The status of the conformance pack. The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code> and <code>INSUFFICIENT_DATA</code>.</p>
    pub fn set_conformance_pack_compliance_status(
        mut self,
        input: std::option::Option<crate::types::ConformancePackComplianceType>,
    ) -> Self {
        self.conformance_pack_compliance_status = input;
        self
    }
    /// Consumes the builder and constructs a [`ConformancePackComplianceSummary`](crate::types::ConformancePackComplianceSummary).
    pub fn build(self) -> crate::types::ConformancePackComplianceSummary {
        crate::types::ConformancePackComplianceSummary {
            conformance_pack_name: self.conformance_pack_name,
            conformance_pack_compliance_status: self.conformance_pack_compliance_status,
        }
    }
}