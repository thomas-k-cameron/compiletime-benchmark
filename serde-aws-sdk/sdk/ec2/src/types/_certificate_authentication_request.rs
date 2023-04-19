// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the client certificate to be used for authentication.</p>
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
pub struct CertificateAuthenticationRequest {
    /// <p>The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in Certificate Manager (ACM).</p>
    #[doc(hidden)]
    pub client_root_certificate_chain_arn: std::option::Option<std::string::String>,
}
impl CertificateAuthenticationRequest {
    /// <p>The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in Certificate Manager (ACM).</p>
    pub fn client_root_certificate_chain_arn(&self) -> std::option::Option<&str> {
        self.client_root_certificate_chain_arn.as_deref()
    }
}
impl CertificateAuthenticationRequest {
    /// Creates a new builder-style object to manufacture [`CertificateAuthenticationRequest`](crate::types::CertificateAuthenticationRequest).
    pub fn builder() -> crate::types::builders::CertificateAuthenticationRequestBuilder {
        crate::types::builders::CertificateAuthenticationRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CertificateAuthenticationRequest;
/// A builder for [`CertificateAuthenticationRequest`](crate::types::CertificateAuthenticationRequest).
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
pub struct CertificateAuthenticationRequestBuilder {
    pub(crate) client_root_certificate_chain_arn: std::option::Option<std::string::String>,
}
impl CertificateAuthenticationRequestBuilder {
    /// <p>The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in Certificate Manager (ACM).</p>
    pub fn client_root_certificate_chain_arn(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.client_root_certificate_chain_arn = Some(input.into());
        self
    }
    /// <p>The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in Certificate Manager (ACM).</p>
    pub fn set_client_root_certificate_chain_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.client_root_certificate_chain_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`CertificateAuthenticationRequest`](crate::types::CertificateAuthenticationRequest).
    pub fn build(self) -> crate::types::CertificateAuthenticationRequest {
        crate::types::CertificateAuthenticationRequest {
            client_root_certificate_chain_arn: self.client_root_certificate_chain_arn,
        }
    }
}
