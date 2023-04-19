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
pub struct DescribeConfigurationAggregatorSourcesStatusOutput {
    /// <p>Returns an AggregatedSourceStatus object. </p>
    #[doc(hidden)]
    pub aggregated_source_status_list:
        std::option::Option<std::vec::Vec<crate::types::AggregatedSourceStatus>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeConfigurationAggregatorSourcesStatusOutput {
    /// <p>Returns an AggregatedSourceStatus object. </p>
    pub fn aggregated_source_status_list(
        &self,
    ) -> std::option::Option<&[crate::types::AggregatedSourceStatus]> {
        self.aggregated_source_status_list.as_deref()
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeConfigurationAggregatorSourcesStatusOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeConfigurationAggregatorSourcesStatusOutput {
    /// Creates a new builder-style object to manufacture [`DescribeConfigurationAggregatorSourcesStatusOutput`](crate::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusOutput).
    pub fn builder() -> crate::operation::describe_configuration_aggregator_sources_status::builders::DescribeConfigurationAggregatorSourcesStatusOutputBuilder{
        crate::operation::describe_configuration_aggregator_sources_status::builders::DescribeConfigurationAggregatorSourcesStatusOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusOutput;
/// A builder for [`DescribeConfigurationAggregatorSourcesStatusOutput`](crate::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusOutput).
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
pub struct DescribeConfigurationAggregatorSourcesStatusOutputBuilder {
    pub(crate) aggregated_source_status_list:
        std::option::Option<std::vec::Vec<crate::types::AggregatedSourceStatus>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeConfigurationAggregatorSourcesStatusOutputBuilder {
    /// Appends an item to `aggregated_source_status_list`.
    ///
    /// To override the contents of this collection use [`set_aggregated_source_status_list`](Self::set_aggregated_source_status_list).
    ///
    /// <p>Returns an AggregatedSourceStatus object. </p>
    pub fn aggregated_source_status_list(
        mut self,
        input: crate::types::AggregatedSourceStatus,
    ) -> Self {
        let mut v = self.aggregated_source_status_list.unwrap_or_default();
        v.push(input);
        self.aggregated_source_status_list = Some(v);
        self
    }
    /// <p>Returns an AggregatedSourceStatus object. </p>
    pub fn set_aggregated_source_status_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AggregatedSourceStatus>>,
    ) -> Self {
        self.aggregated_source_status_list = input;
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
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeConfigurationAggregatorSourcesStatusOutput`](crate::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusOutput).
    pub fn build(self) -> crate::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusOutput{
        crate::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusOutput {
            aggregated_source_status_list: self.aggregated_source_status_list
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
