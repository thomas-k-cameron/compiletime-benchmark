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
pub struct ModifyVpnConnectionOptionsOutput {
    /// <p>Information about the VPN connection.</p>
    #[doc(hidden)]
    pub vpn_connection: std::option::Option<crate::types::VpnConnection>,
    _request_id: Option<String>,
}
impl ModifyVpnConnectionOptionsOutput {
    /// <p>Information about the VPN connection.</p>
    pub fn vpn_connection(&self) -> std::option::Option<&crate::types::VpnConnection> {
        self.vpn_connection.as_ref()
    }
}
impl aws_http::request_id::RequestId for ModifyVpnConnectionOptionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyVpnConnectionOptionsOutput {
    /// Creates a new builder-style object to manufacture [`ModifyVpnConnectionOptionsOutput`](crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput).
    pub fn builder() -> crate::operation::modify_vpn_connection_options::builders::ModifyVpnConnectionOptionsOutputBuilder{
        crate::operation::modify_vpn_connection_options::builders::ModifyVpnConnectionOptionsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput;
/// A builder for [`ModifyVpnConnectionOptionsOutput`](crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput).
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
pub struct ModifyVpnConnectionOptionsOutputBuilder {
    pub(crate) vpn_connection: std::option::Option<crate::types::VpnConnection>,
    _request_id: Option<String>,
}
impl ModifyVpnConnectionOptionsOutputBuilder {
    /// <p>Information about the VPN connection.</p>
    pub fn vpn_connection(mut self, input: crate::types::VpnConnection) -> Self {
        self.vpn_connection = Some(input);
        self
    }
    /// <p>Information about the VPN connection.</p>
    pub fn set_vpn_connection(
        mut self,
        input: std::option::Option<crate::types::VpnConnection>,
    ) -> Self {
        self.vpn_connection = input;
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
    /// Consumes the builder and constructs a [`ModifyVpnConnectionOptionsOutput`](crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput {
        crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput {
            vpn_connection: self.vpn_connection,
            _request_id: self._request_id,
        }
    }
}
