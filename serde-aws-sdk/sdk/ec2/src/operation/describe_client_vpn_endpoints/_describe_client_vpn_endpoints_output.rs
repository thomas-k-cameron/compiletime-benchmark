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
pub struct DescribeClientVpnEndpointsOutput {
    /// <p>Information about the Client VPN endpoints.</p>
    #[doc(hidden)]
    pub client_vpn_endpoints: std::option::Option<std::vec::Vec<crate::types::ClientVpnEndpoint>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeClientVpnEndpointsOutput {
    /// <p>Information about the Client VPN endpoints.</p>
    pub fn client_vpn_endpoints(&self) -> std::option::Option<&[crate::types::ClientVpnEndpoint]> {
        self.client_vpn_endpoints.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeClientVpnEndpointsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeClientVpnEndpointsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeClientVpnEndpointsOutput`](crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput).
    pub fn builder() -> crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsOutputBuilder{
        crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput;
/// A builder for [`DescribeClientVpnEndpointsOutput`](crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput).
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
pub struct DescribeClientVpnEndpointsOutputBuilder {
    pub(crate) client_vpn_endpoints:
        std::option::Option<std::vec::Vec<crate::types::ClientVpnEndpoint>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeClientVpnEndpointsOutputBuilder {
    /// Appends an item to `client_vpn_endpoints`.
    ///
    /// To override the contents of this collection use [`set_client_vpn_endpoints`](Self::set_client_vpn_endpoints).
    ///
    /// <p>Information about the Client VPN endpoints.</p>
    pub fn client_vpn_endpoints(mut self, input: crate::types::ClientVpnEndpoint) -> Self {
        let mut v = self.client_vpn_endpoints.unwrap_or_default();
        v.push(input);
        self.client_vpn_endpoints = Some(v);
        self
    }
    /// <p>Information about the Client VPN endpoints.</p>
    pub fn set_client_vpn_endpoints(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ClientVpnEndpoint>>,
    ) -> Self {
        self.client_vpn_endpoints = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`DescribeClientVpnEndpointsOutput`](crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput {
        crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput {
            client_vpn_endpoints: self.client_vpn_endpoints,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
