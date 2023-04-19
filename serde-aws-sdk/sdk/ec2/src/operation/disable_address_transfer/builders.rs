// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_address_transfer::_disable_address_transfer_output::DisableAddressTransferOutputBuilder;

pub use crate::operation::disable_address_transfer::_disable_address_transfer_input::DisableAddressTransferInputBuilder;

/// Fluent builder constructing a request to `DisableAddressTransfer`.
///
/// <p>Disables Elastic IP address transfer. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-eips.html#transfer-EIPs-intro">Transfer Elastic IP addresses</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisableAddressTransferFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_address_transfer::builders::DisableAddressTransferInputBuilder,
}
impl DisableAddressTransferFluentBuilder {
    /// Creates a new `DisableAddressTransfer`.
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
            crate::operation::disable_address_transfer::DisableAddressTransfer,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::disable_address_transfer::DisableAddressTransferError,
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
        crate::operation::disable_address_transfer::DisableAddressTransferOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::disable_address_transfer::DisableAddressTransferError,
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
    ///     let deserialized_parameters: crate::operation::disable_address_transfer::builders::DisableAddressTransferInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.disable_address_transfer().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::disable_address_transfer::builders::DisableAddressTransferInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The allocation ID of an Elastic IP address.</p>
    pub fn allocation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.allocation_id(input.into());
        self
    }
    /// <p>The allocation ID of an Elastic IP address.</p>
    pub fn set_allocation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_allocation_id(input);
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
}
