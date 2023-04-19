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
pub struct DescribeClientVpnTargetNetworksOutput {
    /// <p>Information about the associated target networks.</p>
    #[doc(hidden)]
    pub client_vpn_target_networks: std::option::Option<std::vec::Vec<crate::types::TargetNetwork>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeClientVpnTargetNetworksOutput {
    /// <p>Information about the associated target networks.</p>
    pub fn client_vpn_target_networks(
        &self,
    ) -> std::option::Option<&[crate::types::TargetNetwork]> {
        self.client_vpn_target_networks.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeClientVpnTargetNetworksOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeClientVpnTargetNetworksOutput {
    /// Creates a new builder-style object to manufacture [`DescribeClientVpnTargetNetworksOutput`](crate::operation::describe_client_vpn_target_networks::DescribeClientVpnTargetNetworksOutput).
    pub fn builder() -> crate::operation::describe_client_vpn_target_networks::builders::DescribeClientVpnTargetNetworksOutputBuilder{
        crate::operation::describe_client_vpn_target_networks::builders::DescribeClientVpnTargetNetworksOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_client_vpn_target_networks::DescribeClientVpnTargetNetworksOutput;
/// A builder for [`DescribeClientVpnTargetNetworksOutput`](crate::operation::describe_client_vpn_target_networks::DescribeClientVpnTargetNetworksOutput).
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
pub struct DescribeClientVpnTargetNetworksOutputBuilder {
    pub(crate) client_vpn_target_networks:
        std::option::Option<std::vec::Vec<crate::types::TargetNetwork>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeClientVpnTargetNetworksOutputBuilder {
    /// Appends an item to `client_vpn_target_networks`.
    ///
    /// To override the contents of this collection use [`set_client_vpn_target_networks`](Self::set_client_vpn_target_networks).
    ///
    /// <p>Information about the associated target networks.</p>
    pub fn client_vpn_target_networks(mut self, input: crate::types::TargetNetwork) -> Self {
        let mut v = self.client_vpn_target_networks.unwrap_or_default();
        v.push(input);
        self.client_vpn_target_networks = Some(v);
        self
    }
    /// <p>Information about the associated target networks.</p>
    pub fn set_client_vpn_target_networks(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TargetNetwork>>,
    ) -> Self {
        self.client_vpn_target_networks = input;
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
    /// Consumes the builder and constructs a [`DescribeClientVpnTargetNetworksOutput`](crate::operation::describe_client_vpn_target_networks::DescribeClientVpnTargetNetworksOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_client_vpn_target_networks::DescribeClientVpnTargetNetworksOutput
    {
        crate::operation::describe_client_vpn_target_networks::DescribeClientVpnTargetNetworksOutput {
            client_vpn_target_networks: self.client_vpn_target_networks
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
