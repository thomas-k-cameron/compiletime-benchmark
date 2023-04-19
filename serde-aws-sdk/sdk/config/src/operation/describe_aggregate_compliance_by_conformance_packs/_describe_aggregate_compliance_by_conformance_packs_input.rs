// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct DescribeAggregateComplianceByConformancePacksInput {
    /// <p>The name of the configuration aggregator.</p>
    #[doc(hidden)]
    pub configuration_aggregator_name: std::option::Option<std::string::String>,
    /// <p>Filters the result by <code>AggregateConformancePackComplianceFilters</code> object.</p>
    #[doc(hidden)]
    pub filters: std::option::Option<crate::types::AggregateConformancePackComplianceFilters>,
    /// <p>The maximum number of conformance packs compliance details returned on each page. The default is maximum. If you specify 0, Config uses the default. </p>
    #[doc(hidden)]
    pub limit: i32,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeAggregateComplianceByConformancePacksInput {
    /// <p>The name of the configuration aggregator.</p>
    pub fn configuration_aggregator_name(&self) -> std::option::Option<&str> {
        self.configuration_aggregator_name.as_deref()
    }
    /// <p>Filters the result by <code>AggregateConformancePackComplianceFilters</code> object.</p>
    pub fn filters(
        &self,
    ) -> std::option::Option<&crate::types::AggregateConformancePackComplianceFilters> {
        self.filters.as_ref()
    }
    /// <p>The maximum number of conformance packs compliance details returned on each page. The default is maximum. If you specify 0, Config uses the default. </p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeAggregateComplianceByConformancePacksInput {
    /// Creates a new builder-style object to manufacture [`DescribeAggregateComplianceByConformancePacksInput`](crate::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksInput).
    pub fn builder() -> crate::operation::describe_aggregate_compliance_by_conformance_packs::builders::DescribeAggregateComplianceByConformancePacksInputBuilder{
        crate::operation::describe_aggregate_compliance_by_conformance_packs::builders::DescribeAggregateComplianceByConformancePacksInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksInput;
/// A builder for [`DescribeAggregateComplianceByConformancePacksInput`](crate::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksInput).
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
pub struct DescribeAggregateComplianceByConformancePacksInputBuilder {
    pub(crate) configuration_aggregator_name: std::option::Option<std::string::String>,
    pub(crate) filters:
        std::option::Option<crate::types::AggregateConformancePackComplianceFilters>,
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl DescribeAggregateComplianceByConformancePacksInputBuilder {
    /// <p>The name of the configuration aggregator.</p>
    pub fn configuration_aggregator_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.configuration_aggregator_name = Some(input.into());
        self
    }
    /// <p>The name of the configuration aggregator.</p>
    pub fn set_configuration_aggregator_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.configuration_aggregator_name = input;
        self
    }
    /// <p>Filters the result by <code>AggregateConformancePackComplianceFilters</code> object.</p>
    pub fn filters(
        mut self,
        input: crate::types::AggregateConformancePackComplianceFilters,
    ) -> Self {
        self.filters = Some(input);
        self
    }
    /// <p>Filters the result by <code>AggregateConformancePackComplianceFilters</code> object.</p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<crate::types::AggregateConformancePackComplianceFilters>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The maximum number of conformance packs compliance details returned on each page. The default is maximum. If you specify 0, Config uses the default. </p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The maximum number of conformance packs compliance details returned on each page. The default is maximum. If you specify 0, Config uses the default. </p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAggregateComplianceByConformancePacksInput`](crate::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksInput).
    pub fn build(self) -> Result<crate::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::describe_aggregate_compliance_by_conformance_packs::DescribeAggregateComplianceByConformancePacksInput {
                configuration_aggregator_name: self.configuration_aggregator_name
                ,
                filters: self.filters
                ,
                limit: self.limit
                    .unwrap_or_default()
                ,
                next_token: self.next_token
                ,
            }
        )
    }
}
