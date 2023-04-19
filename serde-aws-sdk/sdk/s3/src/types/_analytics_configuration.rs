// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the configuration and any analyses for the analytics filter of an Amazon S3 bucket.</p>
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
pub struct AnalyticsConfiguration {
    /// <p>The ID that identifies the analytics configuration.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>The filter used to describe a set of objects for analyses. A filter must have exactly one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided, all objects will be considered in any analysis.</p>
    #[doc(hidden)]
    pub filter: std::option::Option<crate::types::AnalyticsFilter>,
    /// <p> Contains data related to access patterns to be collected and made available to analyze the tradeoffs between different storage classes. </p>
    #[doc(hidden)]
    pub storage_class_analysis: std::option::Option<crate::types::StorageClassAnalysis>,
}
impl AnalyticsConfiguration {
    /// <p>The ID that identifies the analytics configuration.</p>
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The filter used to describe a set of objects for analyses. A filter must have exactly one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided, all objects will be considered in any analysis.</p>
    pub fn filter(&self) -> std::option::Option<&crate::types::AnalyticsFilter> {
        self.filter.as_ref()
    }
    /// <p> Contains data related to access patterns to be collected and made available to analyze the tradeoffs between different storage classes. </p>
    pub fn storage_class_analysis(
        &self,
    ) -> std::option::Option<&crate::types::StorageClassAnalysis> {
        self.storage_class_analysis.as_ref()
    }
}
impl AnalyticsConfiguration {
    /// Creates a new builder-style object to manufacture [`AnalyticsConfiguration`](crate::types::AnalyticsConfiguration).
    pub fn builder() -> crate::types::builders::AnalyticsConfigurationBuilder {
        crate::types::builders::AnalyticsConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AnalyticsConfiguration;
/// A builder for [`AnalyticsConfiguration`](crate::types::AnalyticsConfiguration).
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
pub struct AnalyticsConfigurationBuilder {
    pub(crate) id: std::option::Option<std::string::String>,
    pub(crate) filter: std::option::Option<crate::types::AnalyticsFilter>,
    pub(crate) storage_class_analysis: std::option::Option<crate::types::StorageClassAnalysis>,
}
impl AnalyticsConfigurationBuilder {
    /// <p>The ID that identifies the analytics configuration.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.id = Some(input.into());
        self
    }
    /// <p>The ID that identifies the analytics configuration.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The filter used to describe a set of objects for analyses. A filter must have exactly one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided, all objects will be considered in any analysis.</p>
    pub fn filter(mut self, input: crate::types::AnalyticsFilter) -> Self {
        self.filter = Some(input);
        self
    }
    /// <p>The filter used to describe a set of objects for analyses. A filter must have exactly one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided, all objects will be considered in any analysis.</p>
    pub fn set_filter(mut self, input: std::option::Option<crate::types::AnalyticsFilter>) -> Self {
        self.filter = input;
        self
    }
    /// <p> Contains data related to access patterns to be collected and made available to analyze the tradeoffs between different storage classes. </p>
    pub fn storage_class_analysis(mut self, input: crate::types::StorageClassAnalysis) -> Self {
        self.storage_class_analysis = Some(input);
        self
    }
    /// <p> Contains data related to access patterns to be collected and made available to analyze the tradeoffs between different storage classes. </p>
    pub fn set_storage_class_analysis(
        mut self,
        input: std::option::Option<crate::types::StorageClassAnalysis>,
    ) -> Self {
        self.storage_class_analysis = input;
        self
    }
    /// Consumes the builder and constructs a [`AnalyticsConfiguration`](crate::types::AnalyticsConfiguration).
    pub fn build(self) -> crate::types::AnalyticsConfiguration {
        crate::types::AnalyticsConfiguration {
            id: self.id,
            filter: self.filter,
            storage_class_analysis: self.storage_class_analysis,
        }
    }
}
