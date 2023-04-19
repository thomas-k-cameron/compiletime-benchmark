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
pub struct ListStoredQueriesInput {
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of results to be returned with a single call.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
}
impl ListStoredQueriesInput {
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to be returned with a single call.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
}
impl ListStoredQueriesInput {
    /// Creates a new builder-style object to manufacture [`ListStoredQueriesInput`](crate::operation::list_stored_queries::ListStoredQueriesInput).
    pub fn builder(
    ) -> crate::operation::list_stored_queries::builders::ListStoredQueriesInputBuilder {
        crate::operation::list_stored_queries::builders::ListStoredQueriesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_stored_queries::ListStoredQueriesInput;
/// A builder for [`ListStoredQueriesInput`](crate::operation::list_stored_queries::ListStoredQueriesInput).
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
pub struct ListStoredQueriesInputBuilder {
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) max_results: std::option::Option<i32>,
}
impl ListStoredQueriesInputBuilder {
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to be returned with a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of results to be returned with a single call.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListStoredQueriesInput`](crate::operation::list_stored_queries::ListStoredQueriesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_stored_queries::ListStoredQueriesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::list_stored_queries::ListStoredQueriesInput {
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}