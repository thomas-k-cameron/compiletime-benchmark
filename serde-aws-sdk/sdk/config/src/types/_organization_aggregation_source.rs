// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This object contains regions to set up the aggregator and an IAM role to retrieve organization details.</p>
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
pub struct OrganizationAggregationSource {
    /// <p>ARN of the IAM role used to retrieve Amazon Web Services Organization details associated with the aggregator account.</p>
    #[doc(hidden)]
    pub role_arn: std::option::Option<std::string::String>,
    /// <p>The source regions being aggregated.</p>
    #[doc(hidden)]
    pub aws_regions: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>If true, aggregate existing Config regions and future regions.</p>
    #[doc(hidden)]
    pub all_aws_regions: bool,
}
impl OrganizationAggregationSource {
    /// <p>ARN of the IAM role used to retrieve Amazon Web Services Organization details associated with the aggregator account.</p>
    pub fn role_arn(&self) -> std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The source regions being aggregated.</p>
    pub fn aws_regions(&self) -> std::option::Option<&[std::string::String]> {
        self.aws_regions.as_deref()
    }
    /// <p>If true, aggregate existing Config regions and future regions.</p>
    pub fn all_aws_regions(&self) -> bool {
        self.all_aws_regions
    }
}
impl OrganizationAggregationSource {
    /// Creates a new builder-style object to manufacture [`OrganizationAggregationSource`](crate::types::OrganizationAggregationSource).
    pub fn builder() -> crate::types::builders::OrganizationAggregationSourceBuilder {
        crate::types::builders::OrganizationAggregationSourceBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::OrganizationAggregationSource;
/// A builder for [`OrganizationAggregationSource`](crate::types::OrganizationAggregationSource).
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
pub struct OrganizationAggregationSourceBuilder {
    pub(crate) role_arn: std::option::Option<std::string::String>,
    pub(crate) aws_regions: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) all_aws_regions: std::option::Option<bool>,
}
impl OrganizationAggregationSourceBuilder {
    /// <p>ARN of the IAM role used to retrieve Amazon Web Services Organization details associated with the aggregator account.</p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.role_arn = Some(input.into());
        self
    }
    /// <p>ARN of the IAM role used to retrieve Amazon Web Services Organization details associated with the aggregator account.</p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// Appends an item to `aws_regions`.
    ///
    /// To override the contents of this collection use [`set_aws_regions`](Self::set_aws_regions).
    ///
    /// <p>The source regions being aggregated.</p>
    pub fn aws_regions(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.aws_regions.unwrap_or_default();
        v.push(input.into());
        self.aws_regions = Some(v);
        self
    }
    /// <p>The source regions being aggregated.</p>
    pub fn set_aws_regions(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.aws_regions = input;
        self
    }
    /// <p>If true, aggregate existing Config regions and future regions.</p>
    pub fn all_aws_regions(mut self, input: bool) -> Self {
        self.all_aws_regions = Some(input);
        self
    }
    /// <p>If true, aggregate existing Config regions and future regions.</p>
    pub fn set_all_aws_regions(mut self, input: std::option::Option<bool>) -> Self {
        self.all_aws_regions = input;
        self
    }
    /// Consumes the builder and constructs a [`OrganizationAggregationSource`](crate::types::OrganizationAggregationSource).
    pub fn build(self) -> crate::types::OrganizationAggregationSource {
        crate::types::OrganizationAggregationSource {
            role_arn: self.role_arn,
            aws_regions: self.aws_regions,
            all_aws_regions: self.all_aws_regions.unwrap_or_default(),
        }
    }
}