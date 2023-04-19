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
pub struct DescribePendingAggregationRequestsInput {
    /// <p>The maximum number of evaluation results returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
    #[doc(hidden)]
    pub limit: i32,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribePendingAggregationRequestsInput {
    /// <p>The maximum number of evaluation results returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribePendingAggregationRequestsInput {
    /// Creates a new builder-style object to manufacture [`DescribePendingAggregationRequestsInput`](crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsInput).
    pub fn builder() -> crate::operation::describe_pending_aggregation_requests::builders::DescribePendingAggregationRequestsInputBuilder{
        crate::operation::describe_pending_aggregation_requests::builders::DescribePendingAggregationRequestsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsInput;
/// A builder for [`DescribePendingAggregationRequestsInput`](crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsInput).
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
pub struct DescribePendingAggregationRequestsInputBuilder {
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl DescribePendingAggregationRequestsInputBuilder {
    /// <p>The maximum number of evaluation results returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The maximum number of evaluation results returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
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
    /// Consumes the builder and constructs a [`DescribePendingAggregationRequestsInput`](crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsInput).
    pub fn build(self) -> Result<crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsInput {
                limit: self.limit
                    .unwrap_or_default()
                ,
                next_token: self.next_token
                ,
            }
        )
    }
}
