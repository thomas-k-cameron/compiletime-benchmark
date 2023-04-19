// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for S3 Glacier job parameters.</p>
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
pub struct GlacierJobParameters {
    /// <p>Retrieval tier at which the restore will be processed.</p>
    #[doc(hidden)]
    pub tier: std::option::Option<crate::types::Tier>,
}
impl GlacierJobParameters {
    /// <p>Retrieval tier at which the restore will be processed.</p>
    pub fn tier(&self) -> std::option::Option<&crate::types::Tier> {
        self.tier.as_ref()
    }
}
impl GlacierJobParameters {
    /// Creates a new builder-style object to manufacture [`GlacierJobParameters`](crate::types::GlacierJobParameters).
    pub fn builder() -> crate::types::builders::GlacierJobParametersBuilder {
        crate::types::builders::GlacierJobParametersBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::GlacierJobParameters;
/// A builder for [`GlacierJobParameters`](crate::types::GlacierJobParameters).
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
pub struct GlacierJobParametersBuilder {
    pub(crate) tier: std::option::Option<crate::types::Tier>,
}
impl GlacierJobParametersBuilder {
    /// <p>Retrieval tier at which the restore will be processed.</p>
    pub fn tier(mut self, input: crate::types::Tier) -> Self {
        self.tier = Some(input);
        self
    }
    /// <p>Retrieval tier at which the restore will be processed.</p>
    pub fn set_tier(mut self, input: std::option::Option<crate::types::Tier>) -> Self {
        self.tier = input;
        self
    }
    /// Consumes the builder and constructs a [`GlacierJobParameters`](crate::types::GlacierJobParameters).
    pub fn build(self) -> crate::types::GlacierJobParameters {
        crate::types::GlacierJobParameters { tier: self.tier }
    }
}
