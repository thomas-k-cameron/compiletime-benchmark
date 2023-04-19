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
pub struct ListResourceRequestsInput {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    /// <p>The default is <code>20</code>.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call this action again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The filter criteria to apply to the requests returned.</p>
    #[doc(hidden)]
    pub resource_request_status_filter:
        std::option::Option<crate::types::ResourceRequestStatusFilter>,
}
impl ListResourceRequestsInput {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    /// <p>The default is <code>20</code>.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call this action again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The filter criteria to apply to the requests returned.</p>
    pub fn resource_request_status_filter(
        &self,
    ) -> std::option::Option<&crate::types::ResourceRequestStatusFilter> {
        self.resource_request_status_filter.as_ref()
    }
}
impl ListResourceRequestsInput {
    /// Creates a new builder-style object to manufacture [`ListResourceRequestsInput`](crate::operation::list_resource_requests::ListResourceRequestsInput).
    pub fn builder(
    ) -> crate::operation::list_resource_requests::builders::ListResourceRequestsInputBuilder {
        crate::operation::list_resource_requests::builders::ListResourceRequestsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_resource_requests::ListResourceRequestsInput;
/// A builder for [`ListResourceRequestsInput`](crate::operation::list_resource_requests::ListResourceRequestsInput).
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
pub struct ListResourceRequestsInputBuilder {
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) resource_request_status_filter:
        std::option::Option<crate::types::ResourceRequestStatusFilter>,
}
impl ListResourceRequestsInputBuilder {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    /// <p>The default is <code>20</code>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    /// <p>The default is <code>20</code>.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call this action again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call this action again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The filter criteria to apply to the requests returned.</p>
    pub fn resource_request_status_filter(
        mut self,
        input: crate::types::ResourceRequestStatusFilter,
    ) -> Self {
        self.resource_request_status_filter = Some(input);
        self
    }
    /// <p>The filter criteria to apply to the requests returned.</p>
    pub fn set_resource_request_status_filter(
        mut self,
        input: std::option::Option<crate::types::ResourceRequestStatusFilter>,
    ) -> Self {
        self.resource_request_status_filter = input;
        self
    }
    /// Consumes the builder and constructs a [`ListResourceRequestsInput`](crate::operation::list_resource_requests::ListResourceRequestsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_resource_requests::ListResourceRequestsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::list_resource_requests::ListResourceRequestsInput {
                max_results: self.max_results,
                next_token: self.next_token,
                resource_request_status_filter: self.resource_request_status_filter,
            },
        )
    }
}
