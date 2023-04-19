// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The number of conformance packs that are compliant and noncompliant.</p>
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
pub struct AggregateConformancePackComplianceCount {
    /// <p>Number of compliant conformance packs.</p>
    #[doc(hidden)]
    pub compliant_conformance_pack_count: i32,
    /// <p>Number of noncompliant conformance packs.</p>
    #[doc(hidden)]
    pub non_compliant_conformance_pack_count: i32,
}
impl AggregateConformancePackComplianceCount {
    /// <p>Number of compliant conformance packs.</p>
    pub fn compliant_conformance_pack_count(&self) -> i32 {
        self.compliant_conformance_pack_count
    }
    /// <p>Number of noncompliant conformance packs.</p>
    pub fn non_compliant_conformance_pack_count(&self) -> i32 {
        self.non_compliant_conformance_pack_count
    }
}
impl AggregateConformancePackComplianceCount {
    /// Creates a new builder-style object to manufacture [`AggregateConformancePackComplianceCount`](crate::types::AggregateConformancePackComplianceCount).
    pub fn builder() -> crate::types::builders::AggregateConformancePackComplianceCountBuilder {
        crate::types::builders::AggregateConformancePackComplianceCountBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AggregateConformancePackComplianceCount;
/// A builder for [`AggregateConformancePackComplianceCount`](crate::types::AggregateConformancePackComplianceCount).
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
pub struct AggregateConformancePackComplianceCountBuilder {
    pub(crate) compliant_conformance_pack_count: std::option::Option<i32>,
    pub(crate) non_compliant_conformance_pack_count: std::option::Option<i32>,
}
impl AggregateConformancePackComplianceCountBuilder {
    /// <p>Number of compliant conformance packs.</p>
    pub fn compliant_conformance_pack_count(mut self, input: i32) -> Self {
        self.compliant_conformance_pack_count = Some(input);
        self
    }
    /// <p>Number of compliant conformance packs.</p>
    pub fn set_compliant_conformance_pack_count(mut self, input: std::option::Option<i32>) -> Self {
        self.compliant_conformance_pack_count = input;
        self
    }
    /// <p>Number of noncompliant conformance packs.</p>
    pub fn non_compliant_conformance_pack_count(mut self, input: i32) -> Self {
        self.non_compliant_conformance_pack_count = Some(input);
        self
    }
    /// <p>Number of noncompliant conformance packs.</p>
    pub fn set_non_compliant_conformance_pack_count(
        mut self,
        input: std::option::Option<i32>,
    ) -> Self {
        self.non_compliant_conformance_pack_count = input;
        self
    }
    /// Consumes the builder and constructs a [`AggregateConformancePackComplianceCount`](crate::types::AggregateConformancePackComplianceCount).
    pub fn build(self) -> crate::types::AggregateConformancePackComplianceCount {
        crate::types::AggregateConformancePackComplianceCount {
            compliant_conformance_pack_count: self
                .compliant_conformance_pack_count
                .unwrap_or_default(),
            non_compliant_conformance_pack_count: self
                .non_compliant_conformance_pack_count
                .unwrap_or_default(),
        }
    }
}
