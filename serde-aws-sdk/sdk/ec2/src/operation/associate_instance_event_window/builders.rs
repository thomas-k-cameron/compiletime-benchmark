// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_instance_event_window::_associate_instance_event_window_output::AssociateInstanceEventWindowOutputBuilder;

pub use crate::operation::associate_instance_event_window::_associate_instance_event_window_input::AssociateInstanceEventWindowInputBuilder;

/// Fluent builder constructing a request to `AssociateInstanceEventWindow`.
///
/// <p>Associates one or more targets with an event window. Only one type of target (instance IDs, Dedicated Host IDs, or tags) can be specified with an event window.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/event-windows.html">Define event windows for scheduled events</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AssociateInstanceEventWindowFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::associate_instance_event_window::builders::AssociateInstanceEventWindowInputBuilder
            }
impl AssociateInstanceEventWindowFluentBuilder {
    /// Creates a new `AssociateInstanceEventWindow`.
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
            crate::operation::associate_instance_event_window::AssociateInstanceEventWindow,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_instance_event_window::AssociateInstanceEventWindowError,
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
        crate::operation::associate_instance_event_window::AssociateInstanceEventWindowOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_instance_event_window::AssociateInstanceEventWindowError,
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
    ///     let deserialized_parameters: crate::operation::associate_instance_event_window::builders::AssociateInstanceEventWindowInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.associate_instance_event_window().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::associate_instance_event_window::builders::AssociateInstanceEventWindowInputBuilder,
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
    /// <p>The ID of the event window.</p>
    pub fn instance_event_window_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_event_window_id(input.into());
        self
    }
    /// <p>The ID of the event window.</p>
    pub fn set_instance_event_window_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_instance_event_window_id(input);
        self
    }
    /// <p>One or more targets associated with the specified event window.</p>
    pub fn association_target(
        mut self,
        input: crate::types::InstanceEventWindowAssociationRequest,
    ) -> Self {
        self.inner = self.inner.association_target(input);
        self
    }
    /// <p>One or more targets associated with the specified event window.</p>
    pub fn set_association_target(
        mut self,
        input: std::option::Option<crate::types::InstanceEventWindowAssociationRequest>,
    ) -> Self {
        self.inner = self.inner.set_association_target(input);
        self
    }
}
