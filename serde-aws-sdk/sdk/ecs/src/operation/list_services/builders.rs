// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_services::_list_services_output::ListServicesOutputBuilder;

pub use crate::operation::list_services::_list_services_input::ListServicesInputBuilder;

/// Fluent builder constructing a request to `ListServices`.
///
/// <p>Returns a list of services. You can filter the results by cluster, launch type, and scheduling strategy.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListServicesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_services::builders::ListServicesInputBuilder,
}
impl ListServicesFluentBuilder {
    /// Creates a new `ListServices`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_services::ListServices,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_services::ListServicesError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::list_services::ListServicesOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_services::ListServicesError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::list_services::builders::ListServicesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.list_services().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::list_services::builders::ListServicesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_services::paginator::ListServicesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_services::paginator::ListServicesPaginator {
        crate::operation::list_services::paginator::ListServicesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to use when filtering the <code>ListServices</code> results. If you do not specify a cluster, the default cluster is assumed.</p>
    pub fn cluster(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cluster(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to use when filtering the <code>ListServices</code> results. If you do not specify a cluster, the default cluster is assumed.</p>
    pub fn set_cluster(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cluster(input);
        self
    }
    /// <p>The <code>nextToken</code> value returned from a <code>ListServices</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it is possible the number of results to be fewer than <code>maxResults</code>.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value returned from a <code>ListServices</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it is possible the number of results to be fewer than <code>maxResults</code>.</p> <note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of service results that <code>ListServices</code> returned in paginated output. When this parameter is used, <code>ListServices</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListServices</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListServices</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of service results that <code>ListServices</code> returned in paginated output. When this parameter is used, <code>ListServices</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListServices</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListServices</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The launch type to use when filtering the <code>ListServices</code> results.</p>
    pub fn launch_type(mut self, input: crate::types::LaunchType) -> Self {
        self.inner = self.inner.launch_type(input);
        self
    }
    /// <p>The launch type to use when filtering the <code>ListServices</code> results.</p>
    pub fn set_launch_type(mut self, input: std::option::Option<crate::types::LaunchType>) -> Self {
        self.inner = self.inner.set_launch_type(input);
        self
    }
    /// <p>The scheduling strategy to use when filtering the <code>ListServices</code> results.</p>
    pub fn scheduling_strategy(mut self, input: crate::types::SchedulingStrategy) -> Self {
        self.inner = self.inner.scheduling_strategy(input);
        self
    }
    /// <p>The scheduling strategy to use when filtering the <code>ListServices</code> results.</p>
    pub fn set_scheduling_strategy(
        mut self,
        input: std::option::Option<crate::types::SchedulingStrategy>,
    ) -> Self {
        self.inner = self.inner.set_scheduling_strategy(input);
        self
    }
}
