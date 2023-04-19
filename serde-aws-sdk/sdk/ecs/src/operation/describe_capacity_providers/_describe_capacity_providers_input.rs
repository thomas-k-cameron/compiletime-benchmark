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
pub struct DescribeCapacityProvidersInput {
    /// <p>The short name or full Amazon Resource Name (ARN) of one or more capacity providers. Up to <code>100</code> capacity providers can be described in an action.</p>
    #[doc(hidden)]
    pub capacity_providers: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Specifies whether or not you want to see the resource tags for the capacity provider. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    #[doc(hidden)]
    pub include: std::option::Option<std::vec::Vec<crate::types::CapacityProviderField>>,
    /// <p>The maximum number of account setting results returned by <code>DescribeCapacityProviders</code> in paginated output. When this parameter is used, <code>DescribeCapacityProviders</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeCapacityProviders</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 10. If this parameter is not used, then <code>DescribeCapacityProviders</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeCapacityProviders</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeCapacityProvidersInput {
    /// <p>The short name or full Amazon Resource Name (ARN) of one or more capacity providers. Up to <code>100</code> capacity providers can be described in an action.</p>
    pub fn capacity_providers(&self) -> std::option::Option<&[std::string::String]> {
        self.capacity_providers.as_deref()
    }
    /// <p>Specifies whether or not you want to see the resource tags for the capacity provider. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    pub fn include(&self) -> std::option::Option<&[crate::types::CapacityProviderField]> {
        self.include.as_deref()
    }
    /// <p>The maximum number of account setting results returned by <code>DescribeCapacityProviders</code> in paginated output. When this parameter is used, <code>DescribeCapacityProviders</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeCapacityProviders</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 10. If this parameter is not used, then <code>DescribeCapacityProviders</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeCapacityProviders</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeCapacityProvidersInput {
    /// Creates a new builder-style object to manufacture [`DescribeCapacityProvidersInput`](crate::operation::describe_capacity_providers::DescribeCapacityProvidersInput).
    pub fn builder() -> crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersInputBuilder{
        crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_capacity_providers::DescribeCapacityProvidersInput;
/// A builder for [`DescribeCapacityProvidersInput`](crate::operation::describe_capacity_providers::DescribeCapacityProvidersInput).
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
pub struct DescribeCapacityProvidersInputBuilder {
    pub(crate) capacity_providers: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) include: std::option::Option<std::vec::Vec<crate::types::CapacityProviderField>>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl DescribeCapacityProvidersInputBuilder {
    /// Appends an item to `capacity_providers`.
    ///
    /// To override the contents of this collection use [`set_capacity_providers`](Self::set_capacity_providers).
    ///
    /// <p>The short name or full Amazon Resource Name (ARN) of one or more capacity providers. Up to <code>100</code> capacity providers can be described in an action.</p>
    pub fn capacity_providers(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.capacity_providers.unwrap_or_default();
        v.push(input.into());
        self.capacity_providers = Some(v);
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of one or more capacity providers. Up to <code>100</code> capacity providers can be described in an action.</p>
    pub fn set_capacity_providers(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.capacity_providers = input;
        self
    }
    /// Appends an item to `include`.
    ///
    /// To override the contents of this collection use [`set_include`](Self::set_include).
    ///
    /// <p>Specifies whether or not you want to see the resource tags for the capacity provider. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    pub fn include(mut self, input: crate::types::CapacityProviderField) -> Self {
        let mut v = self.include.unwrap_or_default();
        v.push(input);
        self.include = Some(v);
        self
    }
    /// <p>Specifies whether or not you want to see the resource tags for the capacity provider. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    pub fn set_include(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::CapacityProviderField>>,
    ) -> Self {
        self.include = input;
        self
    }
    /// <p>The maximum number of account setting results returned by <code>DescribeCapacityProviders</code> in paginated output. When this parameter is used, <code>DescribeCapacityProviders</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeCapacityProviders</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 10. If this parameter is not used, then <code>DescribeCapacityProviders</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of account setting results returned by <code>DescribeCapacityProviders</code> in paginated output. When this parameter is used, <code>DescribeCapacityProviders</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeCapacityProviders</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 10. If this parameter is not used, then <code>DescribeCapacityProviders</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeCapacityProviders</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeCapacityProviders</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeCapacityProvidersInput`](crate::operation::describe_capacity_providers::DescribeCapacityProvidersInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_capacity_providers::DescribeCapacityProvidersInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_capacity_providers::DescribeCapacityProvidersInput {
                capacity_providers: self.capacity_providers,
                include: self.include,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
