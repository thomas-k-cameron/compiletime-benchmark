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
pub struct ListQueryLoggingConfigsOutput {
    /// <p>An array that contains one <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_QueryLoggingConfig.html">QueryLoggingConfig</a> element for each configuration for DNS query logging that is associated with the current Amazon Web Services account.</p>
    #[doc(hidden)]
    pub query_logging_configs: std::option::Option<std::vec::Vec<crate::types::QueryLoggingConfig>>,
    /// <p>If a response includes the last of the query logging configurations that are associated with the current Amazon Web Services account, <code>NextToken</code> doesn't appear in the response.</p>
    /// <p>If a response doesn't include the last of the configurations, you can get more configurations by submitting another <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListQueryLoggingConfigs.html">ListQueryLoggingConfigs</a> request. Get the value of <code>NextToken</code> that Amazon Route 53 returned in the previous response and include it in <code>NextToken</code> in the next request.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListQueryLoggingConfigsOutput {
    /// <p>An array that contains one <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_QueryLoggingConfig.html">QueryLoggingConfig</a> element for each configuration for DNS query logging that is associated with the current Amazon Web Services account.</p>
    pub fn query_logging_configs(
        &self,
    ) -> std::option::Option<&[crate::types::QueryLoggingConfig]> {
        self.query_logging_configs.as_deref()
    }
    /// <p>If a response includes the last of the query logging configurations that are associated with the current Amazon Web Services account, <code>NextToken</code> doesn't appear in the response.</p>
    /// <p>If a response doesn't include the last of the configurations, you can get more configurations by submitting another <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListQueryLoggingConfigs.html">ListQueryLoggingConfigs</a> request. Get the value of <code>NextToken</code> that Amazon Route 53 returned in the previous response and include it in <code>NextToken</code> in the next request.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListQueryLoggingConfigsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListQueryLoggingConfigsOutput {
    /// Creates a new builder-style object to manufacture [`ListQueryLoggingConfigsOutput`](crate::operation::list_query_logging_configs::ListQueryLoggingConfigsOutput).
    pub fn builder(
    ) -> crate::operation::list_query_logging_configs::builders::ListQueryLoggingConfigsOutputBuilder
    {
        crate::operation::list_query_logging_configs::builders::ListQueryLoggingConfigsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_query_logging_configs::ListQueryLoggingConfigsOutput;
/// A builder for [`ListQueryLoggingConfigsOutput`](crate::operation::list_query_logging_configs::ListQueryLoggingConfigsOutput).
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
pub struct ListQueryLoggingConfigsOutputBuilder {
    pub(crate) query_logging_configs:
        std::option::Option<std::vec::Vec<crate::types::QueryLoggingConfig>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListQueryLoggingConfigsOutputBuilder {
    /// Appends an item to `query_logging_configs`.
    ///
    /// To override the contents of this collection use [`set_query_logging_configs`](Self::set_query_logging_configs).
    ///
    /// <p>An array that contains one <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_QueryLoggingConfig.html">QueryLoggingConfig</a> element for each configuration for DNS query logging that is associated with the current Amazon Web Services account.</p>
    pub fn query_logging_configs(mut self, input: crate::types::QueryLoggingConfig) -> Self {
        let mut v = self.query_logging_configs.unwrap_or_default();
        v.push(input);
        self.query_logging_configs = Some(v);
        self
    }
    /// <p>An array that contains one <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_QueryLoggingConfig.html">QueryLoggingConfig</a> element for each configuration for DNS query logging that is associated with the current Amazon Web Services account.</p>
    pub fn set_query_logging_configs(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::QueryLoggingConfig>>,
    ) -> Self {
        self.query_logging_configs = input;
        self
    }
    /// <p>If a response includes the last of the query logging configurations that are associated with the current Amazon Web Services account, <code>NextToken</code> doesn't appear in the response.</p>
    /// <p>If a response doesn't include the last of the configurations, you can get more configurations by submitting another <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListQueryLoggingConfigs.html">ListQueryLoggingConfigs</a> request. Get the value of <code>NextToken</code> that Amazon Route 53 returned in the previous response and include it in <code>NextToken</code> in the next request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>If a response includes the last of the query logging configurations that are associated with the current Amazon Web Services account, <code>NextToken</code> doesn't appear in the response.</p>
    /// <p>If a response doesn't include the last of the configurations, you can get more configurations by submitting another <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListQueryLoggingConfigs.html">ListQueryLoggingConfigs</a> request. Get the value of <code>NextToken</code> that Amazon Route 53 returned in the previous response and include it in <code>NextToken</code> in the next request.</p>
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
    /// Consumes the builder and constructs a [`ListQueryLoggingConfigsOutput`](crate::operation::list_query_logging_configs::ListQueryLoggingConfigsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_query_logging_configs::ListQueryLoggingConfigsOutput {
        crate::operation::list_query_logging_configs::ListQueryLoggingConfigsOutput {
            query_logging_configs: self.query_logging_configs,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
