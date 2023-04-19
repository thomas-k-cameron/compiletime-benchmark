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
pub struct DescribeConformancePackStatusInput {
    /// <p>Comma-separated list of conformance pack names.</p>
    #[doc(hidden)]
    pub conformance_pack_names: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    #[doc(hidden)]
    pub limit: i32,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeConformancePackStatusInput {
    /// <p>Comma-separated list of conformance pack names.</p>
    pub fn conformance_pack_names(&self) -> std::option::Option<&[std::string::String]> {
        self.conformance_pack_names.as_deref()
    }
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeConformancePackStatusInput {
    /// Creates a new builder-style object to manufacture [`DescribeConformancePackStatusInput`](crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusInput).
    pub fn builder() -> crate::operation::describe_conformance_pack_status::builders::DescribeConformancePackStatusInputBuilder{
        crate::operation::describe_conformance_pack_status::builders::DescribeConformancePackStatusInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusInput;
/// A builder for [`DescribeConformancePackStatusInput`](crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusInput).
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
pub struct DescribeConformancePackStatusInputBuilder {
    pub(crate) conformance_pack_names: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl DescribeConformancePackStatusInputBuilder {
    /// Appends an item to `conformance_pack_names`.
    ///
    /// To override the contents of this collection use [`set_conformance_pack_names`](Self::set_conformance_pack_names).
    ///
    /// <p>Comma-separated list of conformance pack names.</p>
    pub fn conformance_pack_names(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.conformance_pack_names.unwrap_or_default();
        v.push(input.into());
        self.conformance_pack_names = Some(v);
        self
    }
    /// <p>Comma-separated list of conformance pack names.</p>
    pub fn set_conformance_pack_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.conformance_pack_names = input;
        self
    }
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeConformancePackStatusInput`](crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusInput {
                conformance_pack_names: self.conformance_pack_names
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
