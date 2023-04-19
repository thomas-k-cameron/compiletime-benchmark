// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The container element for Amazon S3 Storage Lens detailed status code metrics. Detailed status code metrics generate metrics for HTTP status codes, such as <code>200 OK</code>, <code>403 Forbidden</code>, <code>503 Service Unavailable</code> and others. </p>
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
pub struct DetailedStatusCodesMetrics {
    /// <p>A container that indicates whether detailed status code metrics are enabled.</p>
    #[doc(hidden)]
    pub is_enabled: bool,
}
impl DetailedStatusCodesMetrics {
    /// <p>A container that indicates whether detailed status code metrics are enabled.</p>
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}
impl DetailedStatusCodesMetrics {
    /// Creates a new builder-style object to manufacture [`DetailedStatusCodesMetrics`](crate::types::DetailedStatusCodesMetrics).
    pub fn builder() -> crate::types::builders::DetailedStatusCodesMetricsBuilder {
        crate::types::builders::DetailedStatusCodesMetricsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::DetailedStatusCodesMetrics;
/// A builder for [`DetailedStatusCodesMetrics`](crate::types::DetailedStatusCodesMetrics).
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
pub struct DetailedStatusCodesMetricsBuilder {
    pub(crate) is_enabled: std::option::Option<bool>,
}
impl DetailedStatusCodesMetricsBuilder {
    /// <p>A container that indicates whether detailed status code metrics are enabled.</p>
    pub fn is_enabled(mut self, input: bool) -> Self {
        self.is_enabled = Some(input);
        self
    }
    /// <p>A container that indicates whether detailed status code metrics are enabled.</p>
    pub fn set_is_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.is_enabled = input;
        self
    }
    /// Consumes the builder and constructs a [`DetailedStatusCodesMetrics`](crate::types::DetailedStatusCodesMetrics).
    pub fn build(self) -> crate::types::DetailedStatusCodesMetrics {
        crate::types::DetailedStatusCodesMetrics {
            is_enabled: self.is_enabled.unwrap_or_default(),
        }
    }
}
