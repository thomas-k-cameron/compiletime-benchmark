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
pub struct DeleteTransitGatewayPrefixListReferenceOutput {
    /// <p>Information about the deleted prefix list reference.</p>
    #[doc(hidden)]
    pub transit_gateway_prefix_list_reference:
        std::option::Option<crate::types::TransitGatewayPrefixListReference>,
    _request_id: Option<String>,
}
impl DeleteTransitGatewayPrefixListReferenceOutput {
    /// <p>Information about the deleted prefix list reference.</p>
    pub fn transit_gateway_prefix_list_reference(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayPrefixListReference> {
        self.transit_gateway_prefix_list_reference.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteTransitGatewayPrefixListReferenceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteTransitGatewayPrefixListReferenceOutput {
    /// Creates a new builder-style object to manufacture [`DeleteTransitGatewayPrefixListReferenceOutput`](crate::operation::delete_transit_gateway_prefix_list_reference::DeleteTransitGatewayPrefixListReferenceOutput).
    pub fn builder() -> crate::operation::delete_transit_gateway_prefix_list_reference::builders::DeleteTransitGatewayPrefixListReferenceOutputBuilder{
        crate::operation::delete_transit_gateway_prefix_list_reference::builders::DeleteTransitGatewayPrefixListReferenceOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_transit_gateway_prefix_list_reference::DeleteTransitGatewayPrefixListReferenceOutput;
/// A builder for [`DeleteTransitGatewayPrefixListReferenceOutput`](crate::operation::delete_transit_gateway_prefix_list_reference::DeleteTransitGatewayPrefixListReferenceOutput).
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
pub struct DeleteTransitGatewayPrefixListReferenceOutputBuilder {
    pub(crate) transit_gateway_prefix_list_reference:
        std::option::Option<crate::types::TransitGatewayPrefixListReference>,
    _request_id: Option<String>,
}
impl DeleteTransitGatewayPrefixListReferenceOutputBuilder {
    /// <p>Information about the deleted prefix list reference.</p>
    pub fn transit_gateway_prefix_list_reference(
        mut self,
        input: crate::types::TransitGatewayPrefixListReference,
    ) -> Self {
        self.transit_gateway_prefix_list_reference = Some(input);
        self
    }
    /// <p>Information about the deleted prefix list reference.</p>
    pub fn set_transit_gateway_prefix_list_reference(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayPrefixListReference>,
    ) -> Self {
        self.transit_gateway_prefix_list_reference = input;
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
    /// Consumes the builder and constructs a [`DeleteTransitGatewayPrefixListReferenceOutput`](crate::operation::delete_transit_gateway_prefix_list_reference::DeleteTransitGatewayPrefixListReferenceOutput).
    pub fn build(self) -> crate::operation::delete_transit_gateway_prefix_list_reference::DeleteTransitGatewayPrefixListReferenceOutput{
        crate::operation::delete_transit_gateway_prefix_list_reference::DeleteTransitGatewayPrefixListReferenceOutput {
            transit_gateway_prefix_list_reference: self.transit_gateway_prefix_list_reference
            ,
            _request_id: self._request_id,
        }
    }
}
