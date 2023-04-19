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
pub struct SelectResourceConfigInput {
    /// <p>The SQL query <code>SELECT</code> command.</p>
    #[doc(hidden)]
    pub expression: std::option::Option<std::string::String>,
    /// <p>The maximum number of query results returned on each page. </p>
    #[doc(hidden)]
    pub limit: i32,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl SelectResourceConfigInput {
    /// <p>The SQL query <code>SELECT</code> command.</p>
    pub fn expression(&self) -> std::option::Option<&str> {
        self.expression.as_deref()
    }
    /// <p>The maximum number of query results returned on each page. </p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl SelectResourceConfigInput {
    /// Creates a new builder-style object to manufacture [`SelectResourceConfigInput`](crate::operation::select_resource_config::SelectResourceConfigInput).
    pub fn builder(
    ) -> crate::operation::select_resource_config::builders::SelectResourceConfigInputBuilder {
        crate::operation::select_resource_config::builders::SelectResourceConfigInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::select_resource_config::SelectResourceConfigInput;
/// A builder for [`SelectResourceConfigInput`](crate::operation::select_resource_config::SelectResourceConfigInput).
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
pub struct SelectResourceConfigInputBuilder {
    pub(crate) expression: std::option::Option<std::string::String>,
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl SelectResourceConfigInputBuilder {
    /// <p>The SQL query <code>SELECT</code> command.</p>
    pub fn expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.expression = Some(input.into());
        self
    }
    /// <p>The SQL query <code>SELECT</code> command.</p>
    pub fn set_expression(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.expression = input;
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
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`SelectResourceConfigInput`](crate::operation::select_resource_config::SelectResourceConfigInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::select_resource_config::SelectResourceConfigInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::select_resource_config::SelectResourceConfigInput {
                expression: self.expression,
                limit: self.limit.unwrap_or_default(),
                next_token: self.next_token,
            },
        )
    }
}
