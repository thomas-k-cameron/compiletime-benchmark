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
pub struct CreateTransitGatewayVpcAttachmentOutput {
    /// <p>Information about the VPC attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_vpc_attachment:
        std::option::Option<crate::types::TransitGatewayVpcAttachment>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayVpcAttachmentOutput {
    /// <p>Information about the VPC attachment.</p>
    pub fn transit_gateway_vpc_attachment(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayVpcAttachment> {
        self.transit_gateway_vpc_attachment.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateTransitGatewayVpcAttachmentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateTransitGatewayVpcAttachmentOutput {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayVpcAttachmentOutput`](crate::operation::create_transit_gateway_vpc_attachment::CreateTransitGatewayVpcAttachmentOutput).
    pub fn builder() -> crate::operation::create_transit_gateway_vpc_attachment::builders::CreateTransitGatewayVpcAttachmentOutputBuilder{
        crate::operation::create_transit_gateway_vpc_attachment::builders::CreateTransitGatewayVpcAttachmentOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_transit_gateway_vpc_attachment::CreateTransitGatewayVpcAttachmentOutput;
/// A builder for [`CreateTransitGatewayVpcAttachmentOutput`](crate::operation::create_transit_gateway_vpc_attachment::CreateTransitGatewayVpcAttachmentOutput).
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
pub struct CreateTransitGatewayVpcAttachmentOutputBuilder {
    pub(crate) transit_gateway_vpc_attachment:
        std::option::Option<crate::types::TransitGatewayVpcAttachment>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayVpcAttachmentOutputBuilder {
    /// <p>Information about the VPC attachment.</p>
    pub fn transit_gateway_vpc_attachment(
        mut self,
        input: crate::types::TransitGatewayVpcAttachment,
    ) -> Self {
        self.transit_gateway_vpc_attachment = Some(input);
        self
    }
    /// <p>Information about the VPC attachment.</p>
    pub fn set_transit_gateway_vpc_attachment(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayVpcAttachment>,
    ) -> Self {
        self.transit_gateway_vpc_attachment = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateTransitGatewayVpcAttachmentOutput`](crate::operation::create_transit_gateway_vpc_attachment::CreateTransitGatewayVpcAttachmentOutput).
    pub fn build(self) -> crate::operation::create_transit_gateway_vpc_attachment::CreateTransitGatewayVpcAttachmentOutput{
        crate::operation::create_transit_gateway_vpc_attachment::CreateTransitGatewayVpcAttachmentOutput {
            transit_gateway_vpc_attachment: self.transit_gateway_vpc_attachment
            ,
            _request_id: self._request_id,
        }
    }
}
