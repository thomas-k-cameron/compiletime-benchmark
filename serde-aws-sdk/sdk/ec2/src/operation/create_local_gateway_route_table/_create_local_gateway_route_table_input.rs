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
pub struct CreateLocalGatewayRouteTableInput {
    /// <p> The ID of the local gateway. </p>
    #[doc(hidden)]
    pub local_gateway_id: std::option::Option<std::string::String>,
    /// <p> The mode of the local gateway route table. </p>
    #[doc(hidden)]
    pub mode: std::option::Option<crate::types::LocalGatewayRouteTableMode>,
    /// <p> The tags assigned to the local gateway route table. </p>
    #[doc(hidden)]
    pub tag_specifications: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl CreateLocalGatewayRouteTableInput {
    /// <p> The ID of the local gateway. </p>
    pub fn local_gateway_id(&self) -> std::option::Option<&str> {
        self.local_gateway_id.as_deref()
    }
    /// <p> The mode of the local gateway route table. </p>
    pub fn mode(&self) -> std::option::Option<&crate::types::LocalGatewayRouteTableMode> {
        self.mode.as_ref()
    }
    /// <p> The tags assigned to the local gateway route table. </p>
    pub fn tag_specifications(&self) -> std::option::Option<&[crate::types::TagSpecification]> {
        self.tag_specifications.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl CreateLocalGatewayRouteTableInput {
    /// Creates a new builder-style object to manufacture [`CreateLocalGatewayRouteTableInput`](crate::operation::create_local_gateway_route_table::CreateLocalGatewayRouteTableInput).
    pub fn builder() -> crate::operation::create_local_gateway_route_table::builders::CreateLocalGatewayRouteTableInputBuilder{
        crate::operation::create_local_gateway_route_table::builders::CreateLocalGatewayRouteTableInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_local_gateway_route_table::CreateLocalGatewayRouteTableInput;
/// A builder for [`CreateLocalGatewayRouteTableInput`](crate::operation::create_local_gateway_route_table::CreateLocalGatewayRouteTableInput).
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
pub struct CreateLocalGatewayRouteTableInputBuilder {
    pub(crate) local_gateway_id: std::option::Option<std::string::String>,
    pub(crate) mode: std::option::Option<crate::types::LocalGatewayRouteTableMode>,
    pub(crate) tag_specifications:
        std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl CreateLocalGatewayRouteTableInputBuilder {
    /// <p> The ID of the local gateway. </p>
    pub fn local_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_gateway_id = Some(input.into());
        self
    }
    /// <p> The ID of the local gateway. </p>
    pub fn set_local_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.local_gateway_id = input;
        self
    }
    /// <p> The mode of the local gateway route table. </p>
    pub fn mode(mut self, input: crate::types::LocalGatewayRouteTableMode) -> Self {
        self.mode = Some(input);
        self
    }
    /// <p> The mode of the local gateway route table. </p>
    pub fn set_mode(
        mut self,
        input: std::option::Option<crate::types::LocalGatewayRouteTableMode>,
    ) -> Self {
        self.mode = input;
        self
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p> The tags assigned to the local gateway route table. </p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = Some(v);
        self
    }
    /// <p> The tags assigned to the local gateway route table. </p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateLocalGatewayRouteTableInput`](crate::operation::create_local_gateway_route_table::CreateLocalGatewayRouteTableInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_local_gateway_route_table::CreateLocalGatewayRouteTableInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_local_gateway_route_table::CreateLocalGatewayRouteTableInput {
                local_gateway_id: self.local_gateway_id,
                mode: self.mode,
                tag_specifications: self.tag_specifications,
                dry_run: self.dry_run,
            },
        )
    }
}
