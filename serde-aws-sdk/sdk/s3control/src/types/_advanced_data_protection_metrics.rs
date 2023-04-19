// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The container element for Amazon S3 Storage Lens advanced data-protection metrics. Advanced data-protection metrics provide insights that you can use to perform audits and protect your data, for example replication rule counts within and across Regions.</p>
/// <p>For more information about S3 Storage Lens, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage_lens.html">Assessing your storage activity and usage with S3 Storage Lens</a> in the <i>Amazon S3 User Guide</i>. For a complete list of S3 Storage Lens metrics, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage_lens_metrics_glossary.html">S3 Storage Lens metrics glossary</a> in the <i>Amazon S3 User Guide</i>.</p>
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
pub struct AdvancedDataProtectionMetrics {
    /// <p>A container that indicates whether advanced data-protection metrics are enabled.</p>
    #[doc(hidden)]
    pub is_enabled: bool,
}
impl AdvancedDataProtectionMetrics {
    /// <p>A container that indicates whether advanced data-protection metrics are enabled.</p>
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}
impl AdvancedDataProtectionMetrics {
    /// Creates a new builder-style object to manufacture [`AdvancedDataProtectionMetrics`](crate::types::AdvancedDataProtectionMetrics).
    pub fn builder() -> crate::types::builders::AdvancedDataProtectionMetricsBuilder {
        crate::types::builders::AdvancedDataProtectionMetricsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AdvancedDataProtectionMetrics;
/// A builder for [`AdvancedDataProtectionMetrics`](crate::types::AdvancedDataProtectionMetrics).
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
pub struct AdvancedDataProtectionMetricsBuilder {
    pub(crate) is_enabled: std::option::Option<bool>,
}
impl AdvancedDataProtectionMetricsBuilder {
    /// <p>A container that indicates whether advanced data-protection metrics are enabled.</p>
    pub fn is_enabled(mut self, input: bool) -> Self {
        self.is_enabled = Some(input);
        self
    }
    /// <p>A container that indicates whether advanced data-protection metrics are enabled.</p>
    pub fn set_is_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.is_enabled = input;
        self
    }
    /// Consumes the builder and constructs a [`AdvancedDataProtectionMetrics`](crate::types::AdvancedDataProtectionMetrics).
    pub fn build(self) -> crate::types::AdvancedDataProtectionMetrics {
        crate::types::AdvancedDataProtectionMetrics {
            is_enabled: self.is_enabled.unwrap_or_default(),
        }
    }
}
