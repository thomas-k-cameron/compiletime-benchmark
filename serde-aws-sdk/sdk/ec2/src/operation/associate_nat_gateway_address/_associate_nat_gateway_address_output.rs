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
pub struct AssociateNatGatewayAddressOutput {
    /// <p>The NAT gateway ID.</p>
    #[doc(hidden)]
    pub nat_gateway_id: std::option::Option<std::string::String>,
    /// <p>The IP addresses.</p>
    #[doc(hidden)]
    pub nat_gateway_addresses: std::option::Option<std::vec::Vec<crate::types::NatGatewayAddress>>,
    _request_id: Option<String>,
}
impl AssociateNatGatewayAddressOutput {
    /// <p>The NAT gateway ID.</p>
    pub fn nat_gateway_id(&self) -> std::option::Option<&str> {
        self.nat_gateway_id.as_deref()
    }
    /// <p>The IP addresses.</p>
    pub fn nat_gateway_addresses(&self) -> std::option::Option<&[crate::types::NatGatewayAddress]> {
        self.nat_gateway_addresses.as_deref()
    }
}
impl aws_http::request_id::RequestId for AssociateNatGatewayAddressOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssociateNatGatewayAddressOutput {
    /// Creates a new builder-style object to manufacture [`AssociateNatGatewayAddressOutput`](crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput).
    pub fn builder() -> crate::operation::associate_nat_gateway_address::builders::AssociateNatGatewayAddressOutputBuilder{
        crate::operation::associate_nat_gateway_address::builders::AssociateNatGatewayAddressOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput;
/// A builder for [`AssociateNatGatewayAddressOutput`](crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput).
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
pub struct AssociateNatGatewayAddressOutputBuilder {
    pub(crate) nat_gateway_id: std::option::Option<std::string::String>,
    pub(crate) nat_gateway_addresses:
        std::option::Option<std::vec::Vec<crate::types::NatGatewayAddress>>,
    _request_id: Option<String>,
}
impl AssociateNatGatewayAddressOutputBuilder {
    /// <p>The NAT gateway ID.</p>
    pub fn nat_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.nat_gateway_id = Some(input.into());
        self
    }
    /// <p>The NAT gateway ID.</p>
    pub fn set_nat_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.nat_gateway_id = input;
        self
    }
    /// Appends an item to `nat_gateway_addresses`.
    ///
    /// To override the contents of this collection use [`set_nat_gateway_addresses`](Self::set_nat_gateway_addresses).
    ///
    /// <p>The IP addresses.</p>
    pub fn nat_gateway_addresses(mut self, input: crate::types::NatGatewayAddress) -> Self {
        let mut v = self.nat_gateway_addresses.unwrap_or_default();
        v.push(input);
        self.nat_gateway_addresses = Some(v);
        self
    }
    /// <p>The IP addresses.</p>
    pub fn set_nat_gateway_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::NatGatewayAddress>>,
    ) -> Self {
        self.nat_gateway_addresses = input;
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
    /// Consumes the builder and constructs a [`AssociateNatGatewayAddressOutput`](crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput).
    pub fn build(
        self,
    ) -> crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput {
        crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput {
            nat_gateway_id: self.nat_gateway_id,
            nat_gateway_addresses: self.nat_gateway_addresses,
            _request_id: self._request_id,
        }
    }
}
