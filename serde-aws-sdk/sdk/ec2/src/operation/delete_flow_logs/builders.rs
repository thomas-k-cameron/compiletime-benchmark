// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_flow_logs::_delete_flow_logs_output::DeleteFlowLogsOutputBuilder;

pub use crate::operation::delete_flow_logs::_delete_flow_logs_input::DeleteFlowLogsInputBuilder;

/// Fluent builder constructing a request to `DeleteFlowLogs`.
///
/// <p>Deletes one or more flow logs.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteFlowLogsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_flow_logs::builders::DeleteFlowLogsInputBuilder,
}
impl DeleteFlowLogsFluentBuilder {
    /// Creates a new `DeleteFlowLogs`.
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
            crate::operation::delete_flow_logs::DeleteFlowLogs,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::delete_flow_logs::DeleteFlowLogsError>,
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
        crate::operation::delete_flow_logs::DeleteFlowLogsOutput,
        aws_smithy_http::result::SdkError<crate::operation::delete_flow_logs::DeleteFlowLogsError>,
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
    ///     let deserialized_parameters: crate::operation::delete_flow_logs::builders::DeleteFlowLogsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_flow_logs().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_flow_logs::builders::DeleteFlowLogsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `FlowLogIds`.
    ///
    /// To override the contents of this collection use [`set_flow_log_ids`](Self::set_flow_log_ids).
    ///
    /// <p>One or more flow log IDs.</p>
    /// <p>Constraint: Maximum of 1000 flow log IDs.</p>
    pub fn flow_log_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.flow_log_ids(input.into());
        self
    }
    /// <p>One or more flow log IDs.</p>
    /// <p>Constraint: Maximum of 1000 flow log IDs.</p>
    pub fn set_flow_log_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_flow_log_ids(input);
        self
    }
}
