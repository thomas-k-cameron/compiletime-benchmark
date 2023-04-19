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
pub struct ListStoredQueriesOutput {
    /// <p>A list of <code>StoredQueryMetadata</code> objects.</p>
    #[doc(hidden)]
    pub stored_query_metadata:
        std::option::Option<std::vec::Vec<crate::types::StoredQueryMetadata>>,
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call this action again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>. </p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListStoredQueriesOutput {
    /// <p>A list of <code>StoredQueryMetadata</code> objects.</p>
    pub fn stored_query_metadata(
        &self,
    ) -> std::option::Option<&[crate::types::StoredQueryMetadata]> {
        self.stored_query_metadata.as_deref()
    }
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call this action again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListStoredQueriesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListStoredQueriesOutput {
    /// Creates a new builder-style object to manufacture [`ListStoredQueriesOutput`](crate::operation::list_stored_queries::ListStoredQueriesOutput).
    pub fn builder(
    ) -> crate::operation::list_stored_queries::builders::ListStoredQueriesOutputBuilder {
        crate::operation::list_stored_queries::builders::ListStoredQueriesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_stored_queries::ListStoredQueriesOutput;
/// A builder for [`ListStoredQueriesOutput`](crate::operation::list_stored_queries::ListStoredQueriesOutput).
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
pub struct ListStoredQueriesOutputBuilder {
    pub(crate) stored_query_metadata:
        std::option::Option<std::vec::Vec<crate::types::StoredQueryMetadata>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListStoredQueriesOutputBuilder {
    /// Appends an item to `stored_query_metadata`.
    ///
    /// To override the contents of this collection use [`set_stored_query_metadata`](Self::set_stored_query_metadata).
    ///
    /// <p>A list of <code>StoredQueryMetadata</code> objects.</p>
    pub fn stored_query_metadata(mut self, input: crate::types::StoredQueryMetadata) -> Self {
        let mut v = self.stored_query_metadata.unwrap_or_default();
        v.push(input);
        self.stored_query_metadata = Some(v);
        self
    }
    /// <p>A list of <code>StoredQueryMetadata</code> objects.</p>
    pub fn set_stored_query_metadata(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::StoredQueryMetadata>>,
    ) -> Self {
        self.stored_query_metadata = input;
        self
    }
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call this action again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call this action again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>. </p>
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
    /// Consumes the builder and constructs a [`ListStoredQueriesOutput`](crate::operation::list_stored_queries::ListStoredQueriesOutput).
    pub fn build(self) -> crate::operation::list_stored_queries::ListStoredQueriesOutput {
        crate::operation::list_stored_queries::ListStoredQueriesOutput {
            stored_query_metadata: self.stored_query_metadata,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
