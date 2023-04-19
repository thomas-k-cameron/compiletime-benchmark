// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::release_hosts::_release_hosts_output::ReleaseHostsOutputBuilder;

pub use crate::operation::release_hosts::_release_hosts_input::ReleaseHostsInputBuilder;

/// Fluent builder constructing a request to `ReleaseHosts`.
///
/// <p>When you no longer want to use an On-Demand Dedicated Host it can be released. On-Demand billing is stopped and the host goes into <code>released</code> state. The host ID of Dedicated Hosts that have been released can no longer be specified in another request, for example, to modify the host. You must stop or terminate all instances on a host before it can be released.</p>
/// <p>When Dedicated Hosts are released, it may take some time for them to stop counting toward your limit and you may receive capacity errors when trying to allocate new Dedicated Hosts. Wait a few minutes and then try again.</p>
/// <p>Released hosts still appear in a <code>DescribeHosts</code> response.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ReleaseHostsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::release_hosts::builders::ReleaseHostsInputBuilder,
}
impl ReleaseHostsFluentBuilder {
    /// Creates a new `ReleaseHosts`.
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
            crate::operation::release_hosts::ReleaseHosts,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::release_hosts::ReleaseHostsError>,
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
        crate::operation::release_hosts::ReleaseHostsOutput,
        aws_smithy_http::result::SdkError<crate::operation::release_hosts::ReleaseHostsError>,
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
    ///     let deserialized_parameters: crate::operation::release_hosts::builders::ReleaseHostsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.release_hosts().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::release_hosts::builders::ReleaseHostsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Appends an item to `HostIds`.
    ///
    /// To override the contents of this collection use [`set_host_ids`](Self::set_host_ids).
    ///
    /// <p>The IDs of the Dedicated Hosts to release.</p>
    pub fn host_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.host_ids(input.into());
        self
    }
    /// <p>The IDs of the Dedicated Hosts to release.</p>
    pub fn set_host_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_host_ids(input);
        self
    }
}