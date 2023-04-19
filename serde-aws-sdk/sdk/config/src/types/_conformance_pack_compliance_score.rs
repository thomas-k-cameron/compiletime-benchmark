// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A compliance score is the percentage of the number of compliant rule-resource combinations in a conformance pack compared to the number of total possible rule-resource combinations in the conformance pack. This metric provides you with a high-level view of the compliance state of your conformance packs. You can use it to identify, investigate, and understand the level of compliance in your conformance packs.</p>
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
pub struct ConformancePackComplianceScore {
    /// <p>Compliance score for the conformance pack. Conformance packs with no evaluation results will have a compliance score of <code>INSUFFICIENT_DATA</code>.</p>
    #[doc(hidden)]
    pub score: std::option::Option<std::string::String>,
    /// <p>The name of the conformance pack.</p>
    #[doc(hidden)]
    pub conformance_pack_name: std::option::Option<std::string::String>,
    /// <p>The time that the conformance pack compliance score was last updated.</p>
    #[doc(hidden)]
    pub last_updated_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl ConformancePackComplianceScore {
    /// <p>Compliance score for the conformance pack. Conformance packs with no evaluation results will have a compliance score of <code>INSUFFICIENT_DATA</code>.</p>
    pub fn score(&self) -> std::option::Option<&str> {
        self.score.as_deref()
    }
    /// <p>The name of the conformance pack.</p>
    pub fn conformance_pack_name(&self) -> std::option::Option<&str> {
        self.conformance_pack_name.as_deref()
    }
    /// <p>The time that the conformance pack compliance score was last updated.</p>
    pub fn last_updated_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_updated_time.as_ref()
    }
}
impl ConformancePackComplianceScore {
    /// Creates a new builder-style object to manufacture [`ConformancePackComplianceScore`](crate::types::ConformancePackComplianceScore).
    pub fn builder() -> crate::types::builders::ConformancePackComplianceScoreBuilder {
        crate::types::builders::ConformancePackComplianceScoreBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ConformancePackComplianceScore;
/// A builder for [`ConformancePackComplianceScore`](crate::types::ConformancePackComplianceScore).
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
pub struct ConformancePackComplianceScoreBuilder {
    pub(crate) score: std::option::Option<std::string::String>,
    pub(crate) conformance_pack_name: std::option::Option<std::string::String>,
    pub(crate) last_updated_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl ConformancePackComplianceScoreBuilder {
    /// <p>Compliance score for the conformance pack. Conformance packs with no evaluation results will have a compliance score of <code>INSUFFICIENT_DATA</code>.</p>
    pub fn score(mut self, input: impl Into<std::string::String>) -> Self {
        self.score = Some(input.into());
        self
    }
    /// <p>Compliance score for the conformance pack. Conformance packs with no evaluation results will have a compliance score of <code>INSUFFICIENT_DATA</code>.</p>
    pub fn set_score(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.score = input;
        self
    }
    /// <p>The name of the conformance pack.</p>
    pub fn conformance_pack_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.conformance_pack_name = Some(input.into());
        self
    }
    /// <p>The name of the conformance pack.</p>
    pub fn set_conformance_pack_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.conformance_pack_name = input;
        self
    }
    /// <p>The time that the conformance pack compliance score was last updated.</p>
    pub fn last_updated_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.last_updated_time = Some(input);
        self
    }
    /// <p>The time that the conformance pack compliance score was last updated.</p>
    pub fn set_last_updated_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_time = input;
        self
    }
    /// Consumes the builder and constructs a [`ConformancePackComplianceScore`](crate::types::ConformancePackComplianceScore).
    pub fn build(self) -> crate::types::ConformancePackComplianceScore {
        crate::types::ConformancePackComplianceScore {
            score: self.score,
            conformance_pack_name: self.conformance_pack_name,
            last_updated_time: self.last_updated_time,
        }
    }
}
