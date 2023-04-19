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
pub struct GetStoredQueryInput {
    /// <p>The name of the query.</p>
    #[doc(hidden)]
    pub query_name: std::option::Option<std::string::String>,
}
impl GetStoredQueryInput {
    /// <p>The name of the query.</p>
    pub fn query_name(&self) -> std::option::Option<&str> {
        self.query_name.as_deref()
    }
}
impl GetStoredQueryInput {
    /// Creates a new builder-style object to manufacture [`GetStoredQueryInput`](crate::operation::get_stored_query::GetStoredQueryInput).
    pub fn builder() -> crate::operation::get_stored_query::builders::GetStoredQueryInputBuilder {
        crate::operation::get_stored_query::builders::GetStoredQueryInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_stored_query::GetStoredQueryInput;
/// A builder for [`GetStoredQueryInput`](crate::operation::get_stored_query::GetStoredQueryInput).
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
pub struct GetStoredQueryInputBuilder {
    pub(crate) query_name: std::option::Option<std::string::String>,
}
impl GetStoredQueryInputBuilder {
    /// <p>The name of the query.</p>
    pub fn query_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.query_name = Some(input.into());
        self
    }
    /// <p>The name of the query.</p>
    pub fn set_query_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.query_name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetStoredQueryInput`](crate::operation::get_stored_query::GetStoredQueryInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_stored_query::GetStoredQueryInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_stored_query::GetStoredQueryInput {
            query_name: self.query_name,
        })
    }
}
