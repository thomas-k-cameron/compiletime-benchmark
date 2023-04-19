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
pub struct DeleteNatGatewayOutput {
    /// <p>The ID of the NAT gateway.</p>
    #[doc(hidden)]
    pub nat_gateway_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DeleteNatGatewayOutput {
    /// <p>The ID of the NAT gateway.</p>
    pub fn nat_gateway_id(&self) -> std::option::Option<&str> {
        self.nat_gateway_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for DeleteNatGatewayOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteNatGatewayOutput {
    /// Creates a new builder-style object to manufacture [`DeleteNatGatewayOutput`](crate::operation::delete_nat_gateway::DeleteNatGatewayOutput).
    pub fn builder() -> crate::operation::delete_nat_gateway::builders::DeleteNatGatewayOutputBuilder
    {
        crate::operation::delete_nat_gateway::builders::DeleteNatGatewayOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_nat_gateway::DeleteNatGatewayOutput;
/// A builder for [`DeleteNatGatewayOutput`](crate::operation::delete_nat_gateway::DeleteNatGatewayOutput).
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
pub struct DeleteNatGatewayOutputBuilder {
    pub(crate) nat_gateway_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DeleteNatGatewayOutputBuilder {
    /// <p>The ID of the NAT gateway.</p>
    pub fn nat_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.nat_gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of the NAT gateway.</p>
    pub fn set_nat_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.nat_gateway_id = input;
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
    /// Consumes the builder and constructs a [`DeleteNatGatewayOutput`](crate::operation::delete_nat_gateway::DeleteNatGatewayOutput).
    pub fn build(self) -> crate::operation::delete_nat_gateway::DeleteNatGatewayOutput {
        crate::operation::delete_nat_gateway::DeleteNatGatewayOutput {
            nat_gateway_id: self.nat_gateway_id,
            _request_id: self._request_id,
        }
    }
}
