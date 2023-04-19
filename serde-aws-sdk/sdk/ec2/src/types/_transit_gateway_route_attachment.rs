// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a route attachment.</p>
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
pub struct TransitGatewayRouteAttachment {
    /// <p>The ID of the resource.</p>
    #[doc(hidden)]
    pub resource_id: std::option::Option<std::string::String>,
    /// <p>The ID of the attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_attachment_id: std::option::Option<std::string::String>,
    /// <p>The resource type. Note that the <code>tgw-peering</code> resource type has been deprecated. </p>
    #[doc(hidden)]
    pub resource_type: std::option::Option<crate::types::TransitGatewayAttachmentResourceType>,
}
impl TransitGatewayRouteAttachment {
    /// <p>The ID of the resource.</p>
    pub fn resource_id(&self) -> std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>The resource type. Note that the <code>tgw-peering</code> resource type has been deprecated. </p>
    pub fn resource_type(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayAttachmentResourceType> {
        self.resource_type.as_ref()
    }
}
impl TransitGatewayRouteAttachment {
    /// Creates a new builder-style object to manufacture [`TransitGatewayRouteAttachment`](crate::types::TransitGatewayRouteAttachment).
    pub fn builder() -> crate::types::builders::TransitGatewayRouteAttachmentBuilder {
        crate::types::builders::TransitGatewayRouteAttachmentBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TransitGatewayRouteAttachment;
/// A builder for [`TransitGatewayRouteAttachment`](crate::types::TransitGatewayRouteAttachment).
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
pub struct TransitGatewayRouteAttachmentBuilder {
    pub(crate) resource_id: std::option::Option<std::string::String>,
    pub(crate) transit_gateway_attachment_id: std::option::Option<std::string::String>,
    pub(crate) resource_type:
        std::option::Option<crate::types::TransitGatewayAttachmentResourceType>,
}
impl TransitGatewayRouteAttachmentBuilder {
    /// <p>The ID of the resource.</p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_id = Some(input.into());
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_attachment_id = Some(input.into());
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// <p>The resource type. Note that the <code>tgw-peering</code> resource type has been deprecated. </p>
    pub fn resource_type(
        mut self,
        input: crate::types::TransitGatewayAttachmentResourceType,
    ) -> Self {
        self.resource_type = Some(input);
        self
    }
    /// <p>The resource type. Note that the <code>tgw-peering</code> resource type has been deprecated. </p>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayAttachmentResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// Consumes the builder and constructs a [`TransitGatewayRouteAttachment`](crate::types::TransitGatewayRouteAttachment).
    pub fn build(self) -> crate::types::TransitGatewayRouteAttachment {
        crate::types::TransitGatewayRouteAttachment {
            resource_id: self.resource_id,
            transit_gateway_attachment_id: self.transit_gateway_attachment_id,
            resource_type: self.resource_type,
        }
    }
}
