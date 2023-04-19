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
pub struct CreateInternetGatewayOutput {
    /// <p>Information about the internet gateway.</p>
    #[doc(hidden)]
    pub internet_gateway: std::option::Option<crate::types::InternetGateway>,
    _request_id: Option<String>,
}
impl CreateInternetGatewayOutput {
    /// <p>Information about the internet gateway.</p>
    pub fn internet_gateway(&self) -> std::option::Option<&crate::types::InternetGateway> {
        self.internet_gateway.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateInternetGatewayOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateInternetGatewayOutput {
    /// Creates a new builder-style object to manufacture [`CreateInternetGatewayOutput`](crate::operation::create_internet_gateway::CreateInternetGatewayOutput).
    pub fn builder(
    ) -> crate::operation::create_internet_gateway::builders::CreateInternetGatewayOutputBuilder
    {
        crate::operation::create_internet_gateway::builders::CreateInternetGatewayOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_internet_gateway::CreateInternetGatewayOutput;
/// A builder for [`CreateInternetGatewayOutput`](crate::operation::create_internet_gateway::CreateInternetGatewayOutput).
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
pub struct CreateInternetGatewayOutputBuilder {
    pub(crate) internet_gateway: std::option::Option<crate::types::InternetGateway>,
    _request_id: Option<String>,
}
impl CreateInternetGatewayOutputBuilder {
    /// <p>Information about the internet gateway.</p>
    pub fn internet_gateway(mut self, input: crate::types::InternetGateway) -> Self {
        self.internet_gateway = Some(input);
        self
    }
    /// <p>Information about the internet gateway.</p>
    pub fn set_internet_gateway(
        mut self,
        input: std::option::Option<crate::types::InternetGateway>,
    ) -> Self {
        self.internet_gateway = input;
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
    /// Consumes the builder and constructs a [`CreateInternetGatewayOutput`](crate::operation::create_internet_gateway::CreateInternetGatewayOutput).
    pub fn build(self) -> crate::operation::create_internet_gateway::CreateInternetGatewayOutput {
        crate::operation::create_internet_gateway::CreateInternetGatewayOutput {
            internet_gateway: self.internet_gateway,
            _request_id: self._request_id,
        }
    }
}