// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the transit gateway peering attachment.</p>
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
pub struct TransitGatewayPeeringAttachment {
    /// <p>The ID of the transit gateway peering attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_attachment_id: std::option::Option<std::string::String>,
    /// <p>The ID of the accepter transit gateway attachment.</p>
    #[doc(hidden)]
    pub accepter_transit_gateway_attachment_id: std::option::Option<std::string::String>,
    /// <p>Information about the requester transit gateway.</p>
    #[doc(hidden)]
    pub requester_tgw_info: std::option::Option<crate::types::PeeringTgwInfo>,
    /// <p>Information about the accepter transit gateway.</p>
    #[doc(hidden)]
    pub accepter_tgw_info: std::option::Option<crate::types::PeeringTgwInfo>,
    /// <p>Details about the transit gateway peering attachment.</p>
    #[doc(hidden)]
    pub options: std::option::Option<crate::types::TransitGatewayPeeringAttachmentOptions>,
    /// <p>The status of the transit gateway peering attachment.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::PeeringAttachmentStatus>,
    /// <p>The state of the transit gateway peering attachment. Note that the <code>initiating</code> state has been deprecated.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::TransitGatewayAttachmentState>,
    /// <p>The time the transit gateway peering attachment was created.</p>
    #[doc(hidden)]
    pub creation_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The tags for the transit gateway peering attachment.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TransitGatewayPeeringAttachment {
    /// <p>The ID of the transit gateway peering attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>The ID of the accepter transit gateway attachment.</p>
    pub fn accepter_transit_gateway_attachment_id(&self) -> std::option::Option<&str> {
        self.accepter_transit_gateway_attachment_id.as_deref()
    }
    /// <p>Information about the requester transit gateway.</p>
    pub fn requester_tgw_info(&self) -> std::option::Option<&crate::types::PeeringTgwInfo> {
        self.requester_tgw_info.as_ref()
    }
    /// <p>Information about the accepter transit gateway.</p>
    pub fn accepter_tgw_info(&self) -> std::option::Option<&crate::types::PeeringTgwInfo> {
        self.accepter_tgw_info.as_ref()
    }
    /// <p>Details about the transit gateway peering attachment.</p>
    pub fn options(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayPeeringAttachmentOptions> {
        self.options.as_ref()
    }
    /// <p>The status of the transit gateway peering attachment.</p>
    pub fn status(&self) -> std::option::Option<&crate::types::PeeringAttachmentStatus> {
        self.status.as_ref()
    }
    /// <p>The state of the transit gateway peering attachment. Note that the <code>initiating</code> state has been deprecated.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::TransitGatewayAttachmentState> {
        self.state.as_ref()
    }
    /// <p>The time the transit gateway peering attachment was created.</p>
    pub fn creation_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The tags for the transit gateway peering attachment.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl TransitGatewayPeeringAttachment {
    /// Creates a new builder-style object to manufacture [`TransitGatewayPeeringAttachment`](crate::types::TransitGatewayPeeringAttachment).
    pub fn builder() -> crate::types::builders::TransitGatewayPeeringAttachmentBuilder {
        crate::types::builders::TransitGatewayPeeringAttachmentBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TransitGatewayPeeringAttachment;
/// A builder for [`TransitGatewayPeeringAttachment`](crate::types::TransitGatewayPeeringAttachment).
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
pub struct TransitGatewayPeeringAttachmentBuilder {
    pub(crate) transit_gateway_attachment_id: std::option::Option<std::string::String>,
    pub(crate) accepter_transit_gateway_attachment_id: std::option::Option<std::string::String>,
    pub(crate) requester_tgw_info: std::option::Option<crate::types::PeeringTgwInfo>,
    pub(crate) accepter_tgw_info: std::option::Option<crate::types::PeeringTgwInfo>,
    pub(crate) options: std::option::Option<crate::types::TransitGatewayPeeringAttachmentOptions>,
    pub(crate) status: std::option::Option<crate::types::PeeringAttachmentStatus>,
    pub(crate) state: std::option::Option<crate::types::TransitGatewayAttachmentState>,
    pub(crate) creation_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TransitGatewayPeeringAttachmentBuilder {
    /// <p>The ID of the transit gateway peering attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_attachment_id = Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway peering attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// <p>The ID of the accepter transit gateway attachment.</p>
    pub fn accepter_transit_gateway_attachment_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.accepter_transit_gateway_attachment_id = Some(input.into());
        self
    }
    /// <p>The ID of the accepter transit gateway attachment.</p>
    pub fn set_accepter_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.accepter_transit_gateway_attachment_id = input;
        self
    }
    /// <p>Information about the requester transit gateway.</p>
    pub fn requester_tgw_info(mut self, input: crate::types::PeeringTgwInfo) -> Self {
        self.requester_tgw_info = Some(input);
        self
    }
    /// <p>Information about the requester transit gateway.</p>
    pub fn set_requester_tgw_info(
        mut self,
        input: std::option::Option<crate::types::PeeringTgwInfo>,
    ) -> Self {
        self.requester_tgw_info = input;
        self
    }
    /// <p>Information about the accepter transit gateway.</p>
    pub fn accepter_tgw_info(mut self, input: crate::types::PeeringTgwInfo) -> Self {
        self.accepter_tgw_info = Some(input);
        self
    }
    /// <p>Information about the accepter transit gateway.</p>
    pub fn set_accepter_tgw_info(
        mut self,
        input: std::option::Option<crate::types::PeeringTgwInfo>,
    ) -> Self {
        self.accepter_tgw_info = input;
        self
    }
    /// <p>Details about the transit gateway peering attachment.</p>
    pub fn options(mut self, input: crate::types::TransitGatewayPeeringAttachmentOptions) -> Self {
        self.options = Some(input);
        self
    }
    /// <p>Details about the transit gateway peering attachment.</p>
    pub fn set_options(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayPeeringAttachmentOptions>,
    ) -> Self {
        self.options = input;
        self
    }
    /// <p>The status of the transit gateway peering attachment.</p>
    pub fn status(mut self, input: crate::types::PeeringAttachmentStatus) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The status of the transit gateway peering attachment.</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<crate::types::PeeringAttachmentStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The state of the transit gateway peering attachment. Note that the <code>initiating</code> state has been deprecated.</p>
    pub fn state(mut self, input: crate::types::TransitGatewayAttachmentState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The state of the transit gateway peering attachment. Note that the <code>initiating</code> state has been deprecated.</p>
    pub fn set_state(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayAttachmentState>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The time the transit gateway peering attachment was created.</p>
    pub fn creation_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.creation_time = Some(input);
        self
    }
    /// <p>The time the transit gateway peering attachment was created.</p>
    pub fn set_creation_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the transit gateway peering attachment.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags for the transit gateway peering attachment.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TransitGatewayPeeringAttachment`](crate::types::TransitGatewayPeeringAttachment).
    pub fn build(self) -> crate::types::TransitGatewayPeeringAttachment {
        crate::types::TransitGatewayPeeringAttachment {
            transit_gateway_attachment_id: self.transit_gateway_attachment_id,
            accepter_transit_gateway_attachment_id: self.accepter_transit_gateway_attachment_id,
            requester_tgw_info: self.requester_tgw_info,
            accepter_tgw_info: self.accepter_tgw_info,
            options: self.options,
            status: self.status,
            state: self.state,
            creation_time: self.creation_time,
            tags: self.tags,
        }
    }
}
