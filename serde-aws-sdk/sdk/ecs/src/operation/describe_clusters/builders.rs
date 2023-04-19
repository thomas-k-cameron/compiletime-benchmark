// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_clusters::_describe_clusters_output::DescribeClustersOutputBuilder;

pub use crate::operation::describe_clusters::_describe_clusters_input::DescribeClustersInputBuilder;

/// Fluent builder constructing a request to `DescribeClusters`.
///
/// <p>Describes one or more of your clusters.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeClustersFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_clusters::builders::DescribeClustersInputBuilder,
}
impl DescribeClustersFluentBuilder {
    /// Creates a new `DescribeClusters`.
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
            crate::operation::describe_clusters::DescribeClusters,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_clusters::DescribeClustersError,
        >,
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
        crate::operation::describe_clusters::DescribeClustersOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_clusters::DescribeClustersError,
        >,
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
    ///     let deserialized_parameters: crate::operation::describe_clusters::builders::DescribeClustersInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_clusters().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_clusters::builders::DescribeClustersInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Appends an item to `clusters`.
    ///
    /// To override the contents of this collection use [`set_clusters`](Self::set_clusters).
    ///
    /// <p>A list of up to 100 cluster names or full cluster Amazon Resource Name (ARN) entries. If you do not specify a cluster, the default cluster is assumed.</p>
    pub fn clusters(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.clusters(input.into());
        self
    }
    /// <p>A list of up to 100 cluster names or full cluster Amazon Resource Name (ARN) entries. If you do not specify a cluster, the default cluster is assumed.</p>
    pub fn set_clusters(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_clusters(input);
        self
    }
    /// Appends an item to `include`.
    ///
    /// To override the contents of this collection use [`set_include`](Self::set_include).
    ///
    /// <p>Determines whether to include additional information about the clusters in the response. If this field is omitted, this information isn't included.</p>
    /// <p>If <code>ATTACHMENTS</code> is specified, the attachments for the container instances or tasks within the cluster are included, for example the capacity providers.</p>
    /// <p>If <code>SETTINGS</code> is specified, the settings for the cluster are included.</p>
    /// <p>If <code>CONFIGURATIONS</code> is specified, the configuration for the cluster is included.</p>
    /// <p>If <code>STATISTICS</code> is specified, the task and service count is included, separated by launch type.</p>
    /// <p>If <code>TAGS</code> is specified, the metadata tags associated with the cluster are included.</p>
    pub fn include(mut self, input: crate::types::ClusterField) -> Self {
        self.inner = self.inner.include(input);
        self
    }
    /// <p>Determines whether to include additional information about the clusters in the response. If this field is omitted, this information isn't included.</p>
    /// <p>If <code>ATTACHMENTS</code> is specified, the attachments for the container instances or tasks within the cluster are included, for example the capacity providers.</p>
    /// <p>If <code>SETTINGS</code> is specified, the settings for the cluster are included.</p>
    /// <p>If <code>CONFIGURATIONS</code> is specified, the configuration for the cluster is included.</p>
    /// <p>If <code>STATISTICS</code> is specified, the task and service count is included, separated by launch type.</p>
    /// <p>If <code>TAGS</code> is specified, the metadata tags associated with the cluster are included.</p>
    pub fn set_include(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ClusterField>>,
    ) -> Self {
        self.inner = self.inner.set_include(input);
        self
    }
}
