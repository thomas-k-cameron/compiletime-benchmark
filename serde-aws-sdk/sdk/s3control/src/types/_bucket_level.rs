// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for the bucket-level configuration for Amazon S3 Storage Lens.</p>
/// <p>For more information about S3 Storage Lens, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage_lens.html">Assessing your storage activity and usage with S3 Storage Lens</a> in the <i>Amazon S3 User Guide</i>.</p>
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
pub struct BucketLevel {
    /// <p>A container for the bucket-level activity metrics for S3 Storage Lens.</p>
    #[doc(hidden)]
    pub activity_metrics: std::option::Option<crate::types::ActivityMetrics>,
    /// <p>A container for the prefix-level metrics for S3 Storage Lens. </p>
    #[doc(hidden)]
    pub prefix_level: std::option::Option<crate::types::PrefixLevel>,
    /// <p>A container for bucket-level advanced cost-optimization metrics for S3 Storage Lens.</p>
    #[doc(hidden)]
    pub advanced_cost_optimization_metrics:
        std::option::Option<crate::types::AdvancedCostOptimizationMetrics>,
    /// <p>A container for bucket-level advanced data-protection metrics for S3 Storage Lens.</p>
    #[doc(hidden)]
    pub advanced_data_protection_metrics:
        std::option::Option<crate::types::AdvancedDataProtectionMetrics>,
    /// <p>A container for bucket-level detailed status code metrics for S3 Storage Lens.</p>
    #[doc(hidden)]
    pub detailed_status_codes_metrics:
        std::option::Option<crate::types::DetailedStatusCodesMetrics>,
}
impl BucketLevel {
    /// <p>A container for the bucket-level activity metrics for S3 Storage Lens.</p>
    pub fn activity_metrics(&self) -> std::option::Option<&crate::types::ActivityMetrics> {
        self.activity_metrics.as_ref()
    }
    /// <p>A container for the prefix-level metrics for S3 Storage Lens. </p>
    pub fn prefix_level(&self) -> std::option::Option<&crate::types::PrefixLevel> {
        self.prefix_level.as_ref()
    }
    /// <p>A container for bucket-level advanced cost-optimization metrics for S3 Storage Lens.</p>
    pub fn advanced_cost_optimization_metrics(
        &self,
    ) -> std::option::Option<&crate::types::AdvancedCostOptimizationMetrics> {
        self.advanced_cost_optimization_metrics.as_ref()
    }
    /// <p>A container for bucket-level advanced data-protection metrics for S3 Storage Lens.</p>
    pub fn advanced_data_protection_metrics(
        &self,
    ) -> std::option::Option<&crate::types::AdvancedDataProtectionMetrics> {
        self.advanced_data_protection_metrics.as_ref()
    }
    /// <p>A container for bucket-level detailed status code metrics for S3 Storage Lens.</p>
    pub fn detailed_status_codes_metrics(
        &self,
    ) -> std::option::Option<&crate::types::DetailedStatusCodesMetrics> {
        self.detailed_status_codes_metrics.as_ref()
    }
}
impl BucketLevel {
    /// Creates a new builder-style object to manufacture [`BucketLevel`](crate::types::BucketLevel).
    pub fn builder() -> crate::types::builders::BucketLevelBuilder {
        crate::types::builders::BucketLevelBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::BucketLevel;
/// A builder for [`BucketLevel`](crate::types::BucketLevel).
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
pub struct BucketLevelBuilder {
    pub(crate) activity_metrics: std::option::Option<crate::types::ActivityMetrics>,
    pub(crate) prefix_level: std::option::Option<crate::types::PrefixLevel>,
    pub(crate) advanced_cost_optimization_metrics:
        std::option::Option<crate::types::AdvancedCostOptimizationMetrics>,
    pub(crate) advanced_data_protection_metrics:
        std::option::Option<crate::types::AdvancedDataProtectionMetrics>,
    pub(crate) detailed_status_codes_metrics:
        std::option::Option<crate::types::DetailedStatusCodesMetrics>,
}
impl BucketLevelBuilder {
    /// <p>A container for the bucket-level activity metrics for S3 Storage Lens.</p>
    pub fn activity_metrics(mut self, input: crate::types::ActivityMetrics) -> Self {
        self.activity_metrics = Some(input);
        self
    }
    /// <p>A container for the bucket-level activity metrics for S3 Storage Lens.</p>
    pub fn set_activity_metrics(
        mut self,
        input: std::option::Option<crate::types::ActivityMetrics>,
    ) -> Self {
        self.activity_metrics = input;
        self
    }
    /// <p>A container for the prefix-level metrics for S3 Storage Lens. </p>
    pub fn prefix_level(mut self, input: crate::types::PrefixLevel) -> Self {
        self.prefix_level = Some(input);
        self
    }
    /// <p>A container for the prefix-level metrics for S3 Storage Lens. </p>
    pub fn set_prefix_level(
        mut self,
        input: std::option::Option<crate::types::PrefixLevel>,
    ) -> Self {
        self.prefix_level = input;
        self
    }
    /// <p>A container for bucket-level advanced cost-optimization metrics for S3 Storage Lens.</p>
    pub fn advanced_cost_optimization_metrics(
        mut self,
        input: crate::types::AdvancedCostOptimizationMetrics,
    ) -> Self {
        self.advanced_cost_optimization_metrics = Some(input);
        self
    }
    /// <p>A container for bucket-level advanced cost-optimization metrics for S3 Storage Lens.</p>
    pub fn set_advanced_cost_optimization_metrics(
        mut self,
        input: std::option::Option<crate::types::AdvancedCostOptimizationMetrics>,
    ) -> Self {
        self.advanced_cost_optimization_metrics = input;
        self
    }
    /// <p>A container for bucket-level advanced data-protection metrics for S3 Storage Lens.</p>
    pub fn advanced_data_protection_metrics(
        mut self,
        input: crate::types::AdvancedDataProtectionMetrics,
    ) -> Self {
        self.advanced_data_protection_metrics = Some(input);
        self
    }
    /// <p>A container for bucket-level advanced data-protection metrics for S3 Storage Lens.</p>
    pub fn set_advanced_data_protection_metrics(
        mut self,
        input: std::option::Option<crate::types::AdvancedDataProtectionMetrics>,
    ) -> Self {
        self.advanced_data_protection_metrics = input;
        self
    }
    /// <p>A container for bucket-level detailed status code metrics for S3 Storage Lens.</p>
    pub fn detailed_status_codes_metrics(
        mut self,
        input: crate::types::DetailedStatusCodesMetrics,
    ) -> Self {
        self.detailed_status_codes_metrics = Some(input);
        self
    }
    /// <p>A container for bucket-level detailed status code metrics for S3 Storage Lens.</p>
    pub fn set_detailed_status_codes_metrics(
        mut self,
        input: std::option::Option<crate::types::DetailedStatusCodesMetrics>,
    ) -> Self {
        self.detailed_status_codes_metrics = input;
        self
    }
    /// Consumes the builder and constructs a [`BucketLevel`](crate::types::BucketLevel).
    pub fn build(self) -> crate::types::BucketLevel {
        crate::types::BucketLevel {
            activity_metrics: self.activity_metrics,
            prefix_level: self.prefix_level,
            advanced_cost_optimization_metrics: self.advanced_cost_optimization_metrics,
            advanced_data_protection_metrics: self.advanced_data_protection_metrics,
            detailed_status_codes_metrics: self.detailed_status_codes_metrics,
        }
    }
}
