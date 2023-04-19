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
pub struct CreateNatGatewayOutput {
    /// <p>Unique, case-sensitive identifier to ensure the idempotency of the request. Only returned if a client token was provided in the request.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>Information about the NAT gateway.</p>
    #[doc(hidden)]
    pub nat_gateway: std::option::Option<crate::types::NatGateway>,
    _request_id: Option<String>,
}
impl CreateNatGatewayOutput {
    /// <p>Unique, case-sensitive identifier to ensure the idempotency of the request. Only returned if a client token was provided in the request.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Information about the NAT gateway.</p>
    pub fn nat_gateway(&self) -> std::option::Option<&crate::types::NatGateway> {
        self.nat_gateway.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateNatGatewayOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateNatGatewayOutput {
    /// Creates a new builder-style object to manufacture [`CreateNatGatewayOutput`](crate::operation::create_nat_gateway::CreateNatGatewayOutput).
    pub fn builder() -> crate::operation::create_nat_gateway::builders::CreateNatGatewayOutputBuilder
    {
        crate::operation::create_nat_gateway::builders::CreateNatGatewayOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_nat_gateway::CreateNatGatewayOutput;
/// A builder for [`CreateNatGatewayOutput`](crate::operation::create_nat_gateway::CreateNatGatewayOutput).
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
pub struct CreateNatGatewayOutputBuilder {
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) nat_gateway: std::option::Option<crate::types::NatGateway>,
    _request_id: Option<String>,
}
impl CreateNatGatewayOutputBuilder {
    /// <p>Unique, case-sensitive identifier to ensure the idempotency of the request. Only returned if a client token was provided in the request.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier to ensure the idempotency of the request. Only returned if a client token was provided in the request.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Information about the NAT gateway.</p>
    pub fn nat_gateway(mut self, input: crate::types::NatGateway) -> Self {
        self.nat_gateway = Some(input);
        self
    }
    /// <p>Information about the NAT gateway.</p>
    pub fn set_nat_gateway(mut self, input: std::option::Option<crate::types::NatGateway>) -> Self {
        self.nat_gateway = input;
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
    /// Consumes the builder and constructs a [`CreateNatGatewayOutput`](crate::operation::create_nat_gateway::CreateNatGatewayOutput).
    pub fn build(self) -> crate::operation::create_nat_gateway::CreateNatGatewayOutput {
        crate::operation::create_nat_gateway::CreateNatGatewayOutput {
            client_token: self.client_token,
            nat_gateway: self.nat_gateway,
            _request_id: self._request_id,
        }
    }
}
