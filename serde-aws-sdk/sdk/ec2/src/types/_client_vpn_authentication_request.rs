// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the authentication method to be used by a Client VPN endpoint. For more information, see <a href="https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/authentication-authrization.html#client-authentication">Authentication</a> in the <i>Client VPN Administrator Guide</i>.</p>
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
pub struct ClientVpnAuthenticationRequest {
    /// <p>The type of client authentication to be used.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::types::ClientVpnAuthenticationType>,
    /// <p>Information about the Active Directory to be used, if applicable. You must provide this information if <b>Type</b> is <code>directory-service-authentication</code>.</p>
    #[doc(hidden)]
    pub active_directory: std::option::Option<crate::types::DirectoryServiceAuthenticationRequest>,
    /// <p>Information about the authentication certificates to be used, if applicable. You must provide this information if <b>Type</b> is <code>certificate-authentication</code>.</p>
    #[doc(hidden)]
    pub mutual_authentication: std::option::Option<crate::types::CertificateAuthenticationRequest>,
    /// <p>Information about the IAM SAML identity provider to be used, if applicable. You must provide this information if <b>Type</b> is <code>federated-authentication</code>.</p>
    #[doc(hidden)]
    pub federated_authentication: std::option::Option<crate::types::FederatedAuthenticationRequest>,
}
impl ClientVpnAuthenticationRequest {
    /// <p>The type of client authentication to be used.</p>
    pub fn r#type(&self) -> std::option::Option<&crate::types::ClientVpnAuthenticationType> {
        self.r#type.as_ref()
    }
    /// <p>Information about the Active Directory to be used, if applicable. You must provide this information if <b>Type</b> is <code>directory-service-authentication</code>.</p>
    pub fn active_directory(
        &self,
    ) -> std::option::Option<&crate::types::DirectoryServiceAuthenticationRequest> {
        self.active_directory.as_ref()
    }
    /// <p>Information about the authentication certificates to be used, if applicable. You must provide this information if <b>Type</b> is <code>certificate-authentication</code>.</p>
    pub fn mutual_authentication(
        &self,
    ) -> std::option::Option<&crate::types::CertificateAuthenticationRequest> {
        self.mutual_authentication.as_ref()
    }
    /// <p>Information about the IAM SAML identity provider to be used, if applicable. You must provide this information if <b>Type</b> is <code>federated-authentication</code>.</p>
    pub fn federated_authentication(
        &self,
    ) -> std::option::Option<&crate::types::FederatedAuthenticationRequest> {
        self.federated_authentication.as_ref()
    }
}
impl ClientVpnAuthenticationRequest {
    /// Creates a new builder-style object to manufacture [`ClientVpnAuthenticationRequest`](crate::types::ClientVpnAuthenticationRequest).
    pub fn builder() -> crate::types::builders::ClientVpnAuthenticationRequestBuilder {
        crate::types::builders::ClientVpnAuthenticationRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ClientVpnAuthenticationRequest;
/// A builder for [`ClientVpnAuthenticationRequest`](crate::types::ClientVpnAuthenticationRequest).
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
pub struct ClientVpnAuthenticationRequestBuilder {
    pub(crate) r#type: std::option::Option<crate::types::ClientVpnAuthenticationType>,
    pub(crate) active_directory:
        std::option::Option<crate::types::DirectoryServiceAuthenticationRequest>,
    pub(crate) mutual_authentication:
        std::option::Option<crate::types::CertificateAuthenticationRequest>,
    pub(crate) federated_authentication:
        std::option::Option<crate::types::FederatedAuthenticationRequest>,
}
impl ClientVpnAuthenticationRequestBuilder {
    /// <p>The type of client authentication to be used.</p>
    pub fn r#type(mut self, input: crate::types::ClientVpnAuthenticationType) -> Self {
        self.r#type = Some(input);
        self
    }
    /// <p>The type of client authentication to be used.</p>
    pub fn set_type(
        mut self,
        input: std::option::Option<crate::types::ClientVpnAuthenticationType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>Information about the Active Directory to be used, if applicable. You must provide this information if <b>Type</b> is <code>directory-service-authentication</code>.</p>
    pub fn active_directory(
        mut self,
        input: crate::types::DirectoryServiceAuthenticationRequest,
    ) -> Self {
        self.active_directory = Some(input);
        self
    }
    /// <p>Information about the Active Directory to be used, if applicable. You must provide this information if <b>Type</b> is <code>directory-service-authentication</code>.</p>
    pub fn set_active_directory(
        mut self,
        input: std::option::Option<crate::types::DirectoryServiceAuthenticationRequest>,
    ) -> Self {
        self.active_directory = input;
        self
    }
    /// <p>Information about the authentication certificates to be used, if applicable. You must provide this information if <b>Type</b> is <code>certificate-authentication</code>.</p>
    pub fn mutual_authentication(
        mut self,
        input: crate::types::CertificateAuthenticationRequest,
    ) -> Self {
        self.mutual_authentication = Some(input);
        self
    }
    /// <p>Information about the authentication certificates to be used, if applicable. You must provide this information if <b>Type</b> is <code>certificate-authentication</code>.</p>
    pub fn set_mutual_authentication(
        mut self,
        input: std::option::Option<crate::types::CertificateAuthenticationRequest>,
    ) -> Self {
        self.mutual_authentication = input;
        self
    }
    /// <p>Information about the IAM SAML identity provider to be used, if applicable. You must provide this information if <b>Type</b> is <code>federated-authentication</code>.</p>
    pub fn federated_authentication(
        mut self,
        input: crate::types::FederatedAuthenticationRequest,
    ) -> Self {
        self.federated_authentication = Some(input);
        self
    }
    /// <p>Information about the IAM SAML identity provider to be used, if applicable. You must provide this information if <b>Type</b> is <code>federated-authentication</code>.</p>
    pub fn set_federated_authentication(
        mut self,
        input: std::option::Option<crate::types::FederatedAuthenticationRequest>,
    ) -> Self {
        self.federated_authentication = input;
        self
    }
    /// Consumes the builder and constructs a [`ClientVpnAuthenticationRequest`](crate::types::ClientVpnAuthenticationRequest).
    pub fn build(self) -> crate::types::ClientVpnAuthenticationRequest {
        crate::types::ClientVpnAuthenticationRequest {
            r#type: self.r#type,
            active_directory: self.active_directory,
            mutual_authentication: self.mutual_authentication,
            federated_authentication: self.federated_authentication,
        }
    }
}
