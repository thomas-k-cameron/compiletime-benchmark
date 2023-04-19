// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_transit_gateway_connect::_create_transit_gateway_connect_output::CreateTransitGatewayConnectOutputBuilder;

pub use crate::operation::create_transit_gateway_connect::_create_transit_gateway_connect_input::CreateTransitGatewayConnectInputBuilder;

/// Fluent builder constructing a request to `CreateTransitGatewayConnect`.
///
/// <p>Creates a Connect attachment from a specified transit gateway attachment. A Connect attachment is a GRE-based tunnel attachment that you can use to establish a connection between a transit gateway and an appliance.</p>
/// <p>A Connect attachment uses an existing VPC or Amazon Web Services Direct Connect attachment as the underlying transport mechanism.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateTransitGatewayConnectFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectInputBuilder
            }
impl CreateTransitGatewayConnectFluentBuilder {
    /// Creates a new `CreateTransitGatewayConnect`.
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
            crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnect,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectError,
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
        crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectError,
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
    ///     let deserialized_parameters: crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_transit_gateway_connect().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the transit gateway attachment. You can specify a VPC attachment or Amazon Web Services Direct Connect attachment.</p>
    pub fn transport_transit_gateway_attachment_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self
            .inner
            .transport_transit_gateway_attachment_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway attachment. You can specify a VPC attachment or Amazon Web Services Direct Connect attachment.</p>
    pub fn set_transport_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self
            .inner
            .set_transport_transit_gateway_attachment_id(input);
        self
    }
    /// <p>The Connect attachment options.</p>
    pub fn options(
        mut self,
        input: crate::types::CreateTransitGatewayConnectRequestOptions,
    ) -> Self {
        self.inner = self.inner.options(input);
        self
    }
    /// <p>The Connect attachment options.</p>
    pub fn set_options(
        mut self,
        input: std::option::Option<crate::types::CreateTransitGatewayConnectRequestOptions>,
    ) -> Self {
        self.inner = self.inner.set_options(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the Connect attachment.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the Connect attachment.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
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
