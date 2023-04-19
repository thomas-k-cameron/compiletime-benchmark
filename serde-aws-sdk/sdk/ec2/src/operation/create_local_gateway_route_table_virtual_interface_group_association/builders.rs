// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_local_gateway_route_table_virtual_interface_group_association::_create_local_gateway_route_table_virtual_interface_group_association_output::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutputBuilder;

pub use crate::operation::create_local_gateway_route_table_virtual_interface_group_association::_create_local_gateway_route_table_virtual_interface_group_association_input::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder;

/// Fluent builder constructing a request to `CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation`.
///
/// <p> Creates a local gateway route table virtual interface group association. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_local_gateway_route_table_virtual_interface_group_association::builders::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder
            }
impl CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder {
    /// Creates a new `CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, aws_smithy_http::result::SdkError<crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::create_local_gateway_route_table_virtual_interface_group_association::builders::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_local_gateway_route_table_virtual_interface_group_association().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_local_gateway_route_table_virtual_interface_group_association::builders::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p> The ID of the local gateway route table. </p>
    pub fn local_gateway_route_table_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.local_gateway_route_table_id(input.into());
        self
    }
    /// <p> The ID of the local gateway route table. </p>
    pub fn set_local_gateway_route_table_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_local_gateway_route_table_id(input);
        self
    }
    /// <p> The ID of the local gateway route table virtual interface group association. </p>
    pub fn local_gateway_virtual_interface_group_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self
            .inner
            .local_gateway_virtual_interface_group_id(input.into());
        self
    }
    /// <p> The ID of the local gateway route table virtual interface group association. </p>
    pub fn set_local_gateway_virtual_interface_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self
            .inner
            .set_local_gateway_virtual_interface_group_id(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p> The tags assigned to the local gateway route table virtual interface group association. </p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p> The tags assigned to the local gateway route table virtual interface group association. </p>
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
