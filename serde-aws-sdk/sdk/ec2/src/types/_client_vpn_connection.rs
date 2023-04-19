// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a client connection.</p>
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
pub struct ClientVpnConnection {
    /// <p>The ID of the Client VPN endpoint to which the client is connected.</p>
    #[doc(hidden)]
    pub client_vpn_endpoint_id: std::option::Option<std::string::String>,
    /// <p>The current date and time.</p>
    #[doc(hidden)]
    pub timestamp: std::option::Option<std::string::String>,
    /// <p>The ID of the client connection.</p>
    #[doc(hidden)]
    pub connection_id: std::option::Option<std::string::String>,
    /// <p>The username of the client who established the client connection. This information is only provided if Active Directory client authentication is used.</p>
    #[doc(hidden)]
    pub username: std::option::Option<std::string::String>,
    /// <p>The date and time the client connection was established.</p>
    #[doc(hidden)]
    pub connection_established_time: std::option::Option<std::string::String>,
    /// <p>The number of bytes sent by the client.</p>
    #[doc(hidden)]
    pub ingress_bytes: std::option::Option<std::string::String>,
    /// <p>The number of bytes received by the client.</p>
    #[doc(hidden)]
    pub egress_bytes: std::option::Option<std::string::String>,
    /// <p>The number of packets sent by the client.</p>
    #[doc(hidden)]
    pub ingress_packets: std::option::Option<std::string::String>,
    /// <p>The number of packets received by the client.</p>
    #[doc(hidden)]
    pub egress_packets: std::option::Option<std::string::String>,
    /// <p>The IP address of the client.</p>
    #[doc(hidden)]
    pub client_ip: std::option::Option<std::string::String>,
    /// <p>The common name associated with the client. This is either the name of the client certificate, or the Active Directory user name.</p>
    #[doc(hidden)]
    pub common_name: std::option::Option<std::string::String>,
    /// <p>The current state of the client connection.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::ClientVpnConnectionStatus>,
    /// <p>The date and time the client connection was terminated.</p>
    #[doc(hidden)]
    pub connection_end_time: std::option::Option<std::string::String>,
    /// <p>The statuses returned by the client connect handler for posture compliance, if applicable.</p>
    #[doc(hidden)]
    pub posture_compliance_statuses: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl ClientVpnConnection {
    /// <p>The ID of the Client VPN endpoint to which the client is connected.</p>
    pub fn client_vpn_endpoint_id(&self) -> std::option::Option<&str> {
        self.client_vpn_endpoint_id.as_deref()
    }
    /// <p>The current date and time.</p>
    pub fn timestamp(&self) -> std::option::Option<&str> {
        self.timestamp.as_deref()
    }
    /// <p>The ID of the client connection.</p>
    pub fn connection_id(&self) -> std::option::Option<&str> {
        self.connection_id.as_deref()
    }
    /// <p>The username of the client who established the client connection. This information is only provided if Active Directory client authentication is used.</p>
    pub fn username(&self) -> std::option::Option<&str> {
        self.username.as_deref()
    }
    /// <p>The date and time the client connection was established.</p>
    pub fn connection_established_time(&self) -> std::option::Option<&str> {
        self.connection_established_time.as_deref()
    }
    /// <p>The number of bytes sent by the client.</p>
    pub fn ingress_bytes(&self) -> std::option::Option<&str> {
        self.ingress_bytes.as_deref()
    }
    /// <p>The number of bytes received by the client.</p>
    pub fn egress_bytes(&self) -> std::option::Option<&str> {
        self.egress_bytes.as_deref()
    }
    /// <p>The number of packets sent by the client.</p>
    pub fn ingress_packets(&self) -> std::option::Option<&str> {
        self.ingress_packets.as_deref()
    }
    /// <p>The number of packets received by the client.</p>
    pub fn egress_packets(&self) -> std::option::Option<&str> {
        self.egress_packets.as_deref()
    }
    /// <p>The IP address of the client.</p>
    pub fn client_ip(&self) -> std::option::Option<&str> {
        self.client_ip.as_deref()
    }
    /// <p>The common name associated with the client. This is either the name of the client certificate, or the Active Directory user name.</p>
    pub fn common_name(&self) -> std::option::Option<&str> {
        self.common_name.as_deref()
    }
    /// <p>The current state of the client connection.</p>
    pub fn status(&self) -> std::option::Option<&crate::types::ClientVpnConnectionStatus> {
        self.status.as_ref()
    }
    /// <p>The date and time the client connection was terminated.</p>
    pub fn connection_end_time(&self) -> std::option::Option<&str> {
        self.connection_end_time.as_deref()
    }
    /// <p>The statuses returned by the client connect handler for posture compliance, if applicable.</p>
    pub fn posture_compliance_statuses(&self) -> std::option::Option<&[std::string::String]> {
        self.posture_compliance_statuses.as_deref()
    }
}
impl ClientVpnConnection {
    /// Creates a new builder-style object to manufacture [`ClientVpnConnection`](crate::types::ClientVpnConnection).
    pub fn builder() -> crate::types::builders::ClientVpnConnectionBuilder {
        crate::types::builders::ClientVpnConnectionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ClientVpnConnection;
/// A builder for [`ClientVpnConnection`](crate::types::ClientVpnConnection).
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
pub struct ClientVpnConnectionBuilder {
    pub(crate) client_vpn_endpoint_id: std::option::Option<std::string::String>,
    pub(crate) timestamp: std::option::Option<std::string::String>,
    pub(crate) connection_id: std::option::Option<std::string::String>,
    pub(crate) username: std::option::Option<std::string::String>,
    pub(crate) connection_established_time: std::option::Option<std::string::String>,
    pub(crate) ingress_bytes: std::option::Option<std::string::String>,
    pub(crate) egress_bytes: std::option::Option<std::string::String>,
    pub(crate) ingress_packets: std::option::Option<std::string::String>,
    pub(crate) egress_packets: std::option::Option<std::string::String>,
    pub(crate) client_ip: std::option::Option<std::string::String>,
    pub(crate) common_name: std::option::Option<std::string::String>,
    pub(crate) status: std::option::Option<crate::types::ClientVpnConnectionStatus>,
    pub(crate) connection_end_time: std::option::Option<std::string::String>,
    pub(crate) posture_compliance_statuses: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl ClientVpnConnectionBuilder {
    /// <p>The ID of the Client VPN endpoint to which the client is connected.</p>
    pub fn client_vpn_endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_vpn_endpoint_id = Some(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint to which the client is connected.</p>
    pub fn set_client_vpn_endpoint_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.client_vpn_endpoint_id = input;
        self
    }
    /// <p>The current date and time.</p>
    pub fn timestamp(mut self, input: impl Into<std::string::String>) -> Self {
        self.timestamp = Some(input.into());
        self
    }
    /// <p>The current date and time.</p>
    pub fn set_timestamp(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>The ID of the client connection.</p>
    pub fn connection_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.connection_id = Some(input.into());
        self
    }
    /// <p>The ID of the client connection.</p>
    pub fn set_connection_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.connection_id = input;
        self
    }
    /// <p>The username of the client who established the client connection. This information is only provided if Active Directory client authentication is used.</p>
    pub fn username(mut self, input: impl Into<std::string::String>) -> Self {
        self.username = Some(input.into());
        self
    }
    /// <p>The username of the client who established the client connection. This information is only provided if Active Directory client authentication is used.</p>
    pub fn set_username(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.username = input;
        self
    }
    /// <p>The date and time the client connection was established.</p>
    pub fn connection_established_time(mut self, input: impl Into<std::string::String>) -> Self {
        self.connection_established_time = Some(input.into());
        self
    }
    /// <p>The date and time the client connection was established.</p>
    pub fn set_connection_established_time(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.connection_established_time = input;
        self
    }
    /// <p>The number of bytes sent by the client.</p>
    pub fn ingress_bytes(mut self, input: impl Into<std::string::String>) -> Self {
        self.ingress_bytes = Some(input.into());
        self
    }
    /// <p>The number of bytes sent by the client.</p>
    pub fn set_ingress_bytes(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ingress_bytes = input;
        self
    }
    /// <p>The number of bytes received by the client.</p>
    pub fn egress_bytes(mut self, input: impl Into<std::string::String>) -> Self {
        self.egress_bytes = Some(input.into());
        self
    }
    /// <p>The number of bytes received by the client.</p>
    pub fn set_egress_bytes(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.egress_bytes = input;
        self
    }
    /// <p>The number of packets sent by the client.</p>
    pub fn ingress_packets(mut self, input: impl Into<std::string::String>) -> Self {
        self.ingress_packets = Some(input.into());
        self
    }
    /// <p>The number of packets sent by the client.</p>
    pub fn set_ingress_packets(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ingress_packets = input;
        self
    }
    /// <p>The number of packets received by the client.</p>
    pub fn egress_packets(mut self, input: impl Into<std::string::String>) -> Self {
        self.egress_packets = Some(input.into());
        self
    }
    /// <p>The number of packets received by the client.</p>
    pub fn set_egress_packets(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.egress_packets = input;
        self
    }
    /// <p>The IP address of the client.</p>
    pub fn client_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_ip = Some(input.into());
        self
    }
    /// <p>The IP address of the client.</p>
    pub fn set_client_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_ip = input;
        self
    }
    /// <p>The common name associated with the client. This is either the name of the client certificate, or the Active Directory user name.</p>
    pub fn common_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.common_name = Some(input.into());
        self
    }
    /// <p>The common name associated with the client. This is either the name of the client certificate, or the Active Directory user name.</p>
    pub fn set_common_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.common_name = input;
        self
    }
    /// <p>The current state of the client connection.</p>
    pub fn status(mut self, input: crate::types::ClientVpnConnectionStatus) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The current state of the client connection.</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<crate::types::ClientVpnConnectionStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The date and time the client connection was terminated.</p>
    pub fn connection_end_time(mut self, input: impl Into<std::string::String>) -> Self {
        self.connection_end_time = Some(input.into());
        self
    }
    /// <p>The date and time the client connection was terminated.</p>
    pub fn set_connection_end_time(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.connection_end_time = input;
        self
    }
    /// Appends an item to `posture_compliance_statuses`.
    ///
    /// To override the contents of this collection use [`set_posture_compliance_statuses`](Self::set_posture_compliance_statuses).
    ///
    /// <p>The statuses returned by the client connect handler for posture compliance, if applicable.</p>
    pub fn posture_compliance_statuses(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.posture_compliance_statuses.unwrap_or_default();
        v.push(input.into());
        self.posture_compliance_statuses = Some(v);
        self
    }
    /// <p>The statuses returned by the client connect handler for posture compliance, if applicable.</p>
    pub fn set_posture_compliance_statuses(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.posture_compliance_statuses = input;
        self
    }
    /// Consumes the builder and constructs a [`ClientVpnConnection`](crate::types::ClientVpnConnection).
    pub fn build(self) -> crate::types::ClientVpnConnection {
        crate::types::ClientVpnConnection {
            client_vpn_endpoint_id: self.client_vpn_endpoint_id,
            timestamp: self.timestamp,
            connection_id: self.connection_id,
            username: self.username,
            connection_established_time: self.connection_established_time,
            ingress_bytes: self.ingress_bytes,
            egress_bytes: self.egress_bytes,
            ingress_packets: self.ingress_packets,
            egress_packets: self.egress_packets,
            client_ip: self.client_ip,
            common_name: self.common_name,
            status: self.status,
            connection_end_time: self.connection_end_time,
            posture_compliance_statuses: self.posture_compliance_statuses,
        }
    }
}
