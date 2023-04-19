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
pub struct SelectAggregateResourceConfigInput {
    /// <p>The SQL query SELECT command. </p>
    #[doc(hidden)]
    pub expression: std::option::Option<std::string::String>,
    /// <p>The name of the configuration aggregator.</p>
    #[doc(hidden)]
    pub configuration_aggregator_name: std::option::Option<std::string::String>,
    /// <p>The maximum number of query results returned on each page. </p>
    #[doc(hidden)]
    pub limit: i32,
    /// <p>The maximum number of query results returned on each page. Config also allows the Limit request parameter.</p>
    #[doc(hidden)]
    pub max_results: i32,
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl SelectAggregateResourceConfigInput {
    /// <p>The SQL query SELECT command. </p>
    pub fn expression(&self) -> std::option::Option<&str> {
        self.expression.as_deref()
    }
    /// <p>The name of the configuration aggregator.</p>
    pub fn configuration_aggregator_name(&self) -> std::option::Option<&str> {
        self.configuration_aggregator_name.as_deref()
    }
    /// <p>The maximum number of query results returned on each page. </p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// <p>The maximum number of query results returned on each page. Config also allows the Limit request parameter.</p>
    pub fn max_results(&self) -> i32 {
        self.max_results
    }
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl SelectAggregateResourceConfigInput {
    /// Creates a new builder-style object to manufacture [`SelectAggregateResourceConfigInput`](crate::operation::select_aggregate_resource_config::SelectAggregateResourceConfigInput).
    pub fn builder() -> crate::operation::select_aggregate_resource_config::builders::SelectAggregateResourceConfigInputBuilder{
        crate::operation::select_aggregate_resource_config::builders::SelectAggregateResourceConfigInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::select_aggregate_resource_config::SelectAggregateResourceConfigInput;
/// A builder for [`SelectAggregateResourceConfigInput`](crate::operation::select_aggregate_resource_config::SelectAggregateResourceConfigInput).
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
pub struct SelectAggregateResourceConfigInputBuilder {
    pub(crate) expression: std::option::Option<std::string::String>,
    pub(crate) configuration_aggregator_name: std::option::Option<std::string::String>,
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl SelectAggregateResourceConfigInputBuilder {
    /// <p>The SQL query SELECT command. </p>
    pub fn expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.expression = Some(input.into());
        self
    }
    /// <p>The SQL query SELECT command. </p>
    pub fn set_expression(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.expression = input;
        self
    }
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
    /// <p>The maximum number of query results returned on each page. </p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The maximum number of query results returned on each page. </p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>The maximum number of query results returned on each page. Config also allows the Limit request parameter.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of query results returned on each page. Config also allows the Limit request parameter.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`SelectAggregateResourceConfigInput`](crate::operation::select_aggregate_resource_config::SelectAggregateResourceConfigInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::select_aggregate_resource_config::SelectAggregateResourceConfigInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::select_aggregate_resource_config::SelectAggregateResourceConfigInput {
                expression: self.expression
                ,
                configuration_aggregator_name: self.configuration_aggregator_name
                ,
                limit: self.limit
                    .unwrap_or_default()
                ,
                max_results: self.max_results
                    .unwrap_or_default()
                ,
                next_token: self.next_token
                ,
            }
        )
    }
}
