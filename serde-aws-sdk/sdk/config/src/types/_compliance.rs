// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Indicates whether an Amazon Web Services resource or Config rule is compliant and provides the number of contributors that affect the compliance.</p>
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
pub struct Compliance {
    /// <p>Indicates whether an Amazon Web Services resource or Config rule is compliant.</p>
    /// <p>A resource is compliant if it complies with all of the Config rules that evaluate it. A resource is noncompliant if it does not comply with one or more of these rules.</p>
    /// <p>A rule is compliant if all of the resources that the rule evaluates comply with it. A rule is noncompliant if any of these resources do not comply.</p>
    /// <p>Config returns the <code>INSUFFICIENT_DATA</code> value when no evaluation results are available for the Amazon Web Services resource or Config rule.</p>
    /// <p>For the <code>Compliance</code> data type, Config supports only <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code> values. Config does not support the <code>NOT_APPLICABLE</code> value for the <code>Compliance</code> data type.</p>
    #[doc(hidden)]
    pub compliance_type: std::option::Option<crate::types::ComplianceType>,
    /// <p>The number of Amazon Web Services resources or Config rules that cause a result of <code>NON_COMPLIANT</code>, up to a maximum number.</p>
    #[doc(hidden)]
    pub compliance_contributor_count: std::option::Option<crate::types::ComplianceContributorCount>,
}
impl Compliance {
    /// <p>Indicates whether an Amazon Web Services resource or Config rule is compliant.</p>
    /// <p>A resource is compliant if it complies with all of the Config rules that evaluate it. A resource is noncompliant if it does not comply with one or more of these rules.</p>
    /// <p>A rule is compliant if all of the resources that the rule evaluates comply with it. A rule is noncompliant if any of these resources do not comply.</p>
    /// <p>Config returns the <code>INSUFFICIENT_DATA</code> value when no evaluation results are available for the Amazon Web Services resource or Config rule.</p>
    /// <p>For the <code>Compliance</code> data type, Config supports only <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code> values. Config does not support the <code>NOT_APPLICABLE</code> value for the <code>Compliance</code> data type.</p>
    pub fn compliance_type(&self) -> std::option::Option<&crate::types::ComplianceType> {
        self.compliance_type.as_ref()
    }
    /// <p>The number of Amazon Web Services resources or Config rules that cause a result of <code>NON_COMPLIANT</code>, up to a maximum number.</p>
    pub fn compliance_contributor_count(
        &self,
    ) -> std::option::Option<&crate::types::ComplianceContributorCount> {
        self.compliance_contributor_count.as_ref()
    }
}
impl Compliance {
    /// Creates a new builder-style object to manufacture [`Compliance`](crate::types::Compliance).
    pub fn builder() -> crate::types::builders::ComplianceBuilder {
        crate::types::builders::ComplianceBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Compliance;
/// A builder for [`Compliance`](crate::types::Compliance).
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
pub struct ComplianceBuilder {
    pub(crate) compliance_type: std::option::Option<crate::types::ComplianceType>,
    pub(crate) compliance_contributor_count:
        std::option::Option<crate::types::ComplianceContributorCount>,
}
impl ComplianceBuilder {
    /// <p>Indicates whether an Amazon Web Services resource or Config rule is compliant.</p>
    /// <p>A resource is compliant if it complies with all of the Config rules that evaluate it. A resource is noncompliant if it does not comply with one or more of these rules.</p>
    /// <p>A rule is compliant if all of the resources that the rule evaluates comply with it. A rule is noncompliant if any of these resources do not comply.</p>
    /// <p>Config returns the <code>INSUFFICIENT_DATA</code> value when no evaluation results are available for the Amazon Web Services resource or Config rule.</p>
    /// <p>For the <code>Compliance</code> data type, Config supports only <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code> values. Config does not support the <code>NOT_APPLICABLE</code> value for the <code>Compliance</code> data type.</p>
    pub fn compliance_type(mut self, input: crate::types::ComplianceType) -> Self {
        self.compliance_type = Some(input);
        self
    }
    /// <p>Indicates whether an Amazon Web Services resource or Config rule is compliant.</p>
    /// <p>A resource is compliant if it complies with all of the Config rules that evaluate it. A resource is noncompliant if it does not comply with one or more of these rules.</p>
    /// <p>A rule is compliant if all of the resources that the rule evaluates comply with it. A rule is noncompliant if any of these resources do not comply.</p>
    /// <p>Config returns the <code>INSUFFICIENT_DATA</code> value when no evaluation results are available for the Amazon Web Services resource or Config rule.</p>
    /// <p>For the <code>Compliance</code> data type, Config supports only <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code> values. Config does not support the <code>NOT_APPLICABLE</code> value for the <code>Compliance</code> data type.</p>
    pub fn set_compliance_type(
        mut self,
        input: std::option::Option<crate::types::ComplianceType>,
    ) -> Self {
        self.compliance_type = input;
        self
    }
    /// <p>The number of Amazon Web Services resources or Config rules that cause a result of <code>NON_COMPLIANT</code>, up to a maximum number.</p>
    pub fn compliance_contributor_count(
        mut self,
        input: crate::types::ComplianceContributorCount,
    ) -> Self {
        self.compliance_contributor_count = Some(input);
        self
    }
    /// <p>The number of Amazon Web Services resources or Config rules that cause a result of <code>NON_COMPLIANT</code>, up to a maximum number.</p>
    pub fn set_compliance_contributor_count(
        mut self,
        input: std::option::Option<crate::types::ComplianceContributorCount>,
    ) -> Self {
        self.compliance_contributor_count = input;
        self
    }
    /// Consumes the builder and constructs a [`Compliance`](crate::types::Compliance).
    pub fn build(self) -> crate::types::Compliance {
        crate::types::Compliance {
            compliance_type: self.compliance_type,
            compliance_contributor_count: self.compliance_contributor_count,
        }
    }
}
