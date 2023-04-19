// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about an authorization rule.</p>
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
pub struct AuthorizationRule {
    /// <p>The ID of the Client VPN endpoint with which the authorization rule is associated.</p>
    #[doc(hidden)]
    pub client_vpn_endpoint_id: std::option::Option<std::string::String>,
    /// <p>A brief description of the authorization rule.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The ID of the Active Directory group to which the authorization rule grants access.</p>
    #[doc(hidden)]
    pub group_id: std::option::Option<std::string::String>,
    /// <p>Indicates whether the authorization rule grants access to all clients.</p>
    #[doc(hidden)]
    pub access_all: std::option::Option<bool>,
    /// <p>The IPv4 address range, in CIDR notation, of the network to which the authorization rule applies.</p>
    #[doc(hidden)]
    pub destination_cidr: std::option::Option<std::string::String>,
    /// <p>The current state of the authorization rule.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::ClientVpnAuthorizationRuleStatus>,
}
impl AuthorizationRule {
    /// <p>The ID of the Client VPN endpoint with which the authorization rule is associated.</p>
    pub fn client_vpn_endpoint_id(&self) -> std::option::Option<&str> {
        self.client_vpn_endpoint_id.as_deref()
    }
    /// <p>A brief description of the authorization rule.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The ID of the Active Directory group to which the authorization rule grants access.</p>
    pub fn group_id(&self) -> std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>Indicates whether the authorization rule grants access to all clients.</p>
    pub fn access_all(&self) -> std::option::Option<bool> {
        self.access_all
    }
    /// <p>The IPv4 address range, in CIDR notation, of the network to which the authorization rule applies.</p>
    pub fn destination_cidr(&self) -> std::option::Option<&str> {
        self.destination_cidr.as_deref()
    }
    /// <p>The current state of the authorization rule.</p>
    pub fn status(&self) -> std::option::Option<&crate::types::ClientVpnAuthorizationRuleStatus> {
        self.status.as_ref()
    }
}
impl AuthorizationRule {
    /// Creates a new builder-style object to manufacture [`AuthorizationRule`](crate::types::AuthorizationRule).
    pub fn builder() -> crate::types::builders::AuthorizationRuleBuilder {
        crate::types::builders::AuthorizationRuleBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AuthorizationRule;
/// A builder for [`AuthorizationRule`](crate::types::AuthorizationRule).
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
pub struct AuthorizationRuleBuilder {
    pub(crate) client_vpn_endpoint_id: std::option::Option<std::string::String>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) group_id: std::option::Option<std::string::String>,
    pub(crate) access_all: std::option::Option<bool>,
    pub(crate) destination_cidr: std::option::Option<std::string::String>,
    pub(crate) status: std::option::Option<crate::types::ClientVpnAuthorizationRuleStatus>,
}
impl AuthorizationRuleBuilder {
    /// <p>The ID of the Client VPN endpoint with which the authorization rule is associated.</p>
    pub fn client_vpn_endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_vpn_endpoint_id = Some(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint with which the authorization rule is associated.</p>
    pub fn set_client_vpn_endpoint_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.client_vpn_endpoint_id = input;
        self
    }
    /// <p>A brief description of the authorization rule.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A brief description of the authorization rule.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The ID of the Active Directory group to which the authorization rule grants access.</p>
    pub fn group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.group_id = Some(input.into());
        self
    }
    /// <p>The ID of the Active Directory group to which the authorization rule grants access.</p>
    pub fn set_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>Indicates whether the authorization rule grants access to all clients.</p>
    pub fn access_all(mut self, input: bool) -> Self {
        self.access_all = Some(input);
        self
    }
    /// <p>Indicates whether the authorization rule grants access to all clients.</p>
    pub fn set_access_all(mut self, input: std::option::Option<bool>) -> Self {
        self.access_all = input;
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, of the network to which the authorization rule applies.</p>
    pub fn destination_cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination_cidr = Some(input.into());
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, of the network to which the authorization rule applies.</p>
    pub fn set_destination_cidr(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.destination_cidr = input;
        self
    }
    /// <p>The current state of the authorization rule.</p>
    pub fn status(mut self, input: crate::types::ClientVpnAuthorizationRuleStatus) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The current state of the authorization rule.</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<crate::types::ClientVpnAuthorizationRuleStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`AuthorizationRule`](crate::types::AuthorizationRule).
    pub fn build(self) -> crate::types::AuthorizationRule {
        crate::types::AuthorizationRule {
            client_vpn_endpoint_id: self.client_vpn_endpoint_id,
            description: self.description,
            group_id: self.group_id,
            access_all: self.access_all,
            destination_cidr: self.destination_cidr,
            status: self.status,
        }
    }
}
