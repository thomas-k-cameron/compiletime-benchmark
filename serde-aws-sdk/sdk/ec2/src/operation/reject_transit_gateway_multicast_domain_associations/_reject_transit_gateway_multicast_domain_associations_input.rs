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
pub struct RejectTransitGatewayMulticastDomainAssociationsInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    /// <p>The ID of the transit gateway attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_attachment_id: std::option::Option<std::string::String>,
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub subnet_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl RejectTransitGatewayMulticastDomainAssociationsInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_multicast_domain_id.as_deref()
    }
    /// <p>The ID of the transit gateway attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub fn subnet_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.subnet_ids.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl RejectTransitGatewayMulticastDomainAssociationsInput {
    /// Creates a new builder-style object to manufacture [`RejectTransitGatewayMulticastDomainAssociationsInput`](crate::operation::reject_transit_gateway_multicast_domain_associations::RejectTransitGatewayMulticastDomainAssociationsInput).
    pub fn builder() -> crate::operation::reject_transit_gateway_multicast_domain_associations::builders::RejectTransitGatewayMulticastDomainAssociationsInputBuilder{
        crate::operation::reject_transit_gateway_multicast_domain_associations::builders::RejectTransitGatewayMulticastDomainAssociationsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::reject_transit_gateway_multicast_domain_associations::RejectTransitGatewayMulticastDomainAssociationsInput;
/// A builder for [`RejectTransitGatewayMulticastDomainAssociationsInput`](crate::operation::reject_transit_gateway_multicast_domain_associations::RejectTransitGatewayMulticastDomainAssociationsInput).
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
pub struct RejectTransitGatewayMulticastDomainAssociationsInputBuilder {
    pub(crate) transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    pub(crate) transit_gateway_attachment_id: std::option::Option<std::string::String>,
    pub(crate) subnet_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl RejectTransitGatewayMulticastDomainAssociationsInputBuilder {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_id = Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_id = input;
        self
    }
    /// <p>The ID of the transit gateway attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_attachment_id = Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub fn subnet_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = Some(v);
        self
    }
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub fn set_subnet_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.subnet_ids = input;
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
    /// Consumes the builder and constructs a [`RejectTransitGatewayMulticastDomainAssociationsInput`](crate::operation::reject_transit_gateway_multicast_domain_associations::RejectTransitGatewayMulticastDomainAssociationsInput).
    pub fn build(self) -> Result<crate::operation::reject_transit_gateway_multicast_domain_associations::RejectTransitGatewayMulticastDomainAssociationsInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::reject_transit_gateway_multicast_domain_associations::RejectTransitGatewayMulticastDomainAssociationsInput {
                transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id
                ,
                transit_gateway_attachment_id: self.transit_gateway_attachment_id
                ,
                subnet_ids: self.subnet_ids
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}
