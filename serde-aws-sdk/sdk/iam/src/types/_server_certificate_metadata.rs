// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a server certificate without its certificate body, certificate chain, and private key.</p>
/// <p> This data type is used as a response element in the <code>UploadServerCertificate</code> and <code>ListServerCertificates</code> operations. </p>
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
pub struct ServerCertificateMetadata {
    /// <p> The path to the server certificate. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    #[doc(hidden)]
    pub path: std::option::Option<std::string::String>,
    /// <p>The name that identifies the server certificate.</p>
    #[doc(hidden)]
    pub server_certificate_name: std::option::Option<std::string::String>,
    /// <p> The stable and unique string identifying the server certificate. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    #[doc(hidden)]
    pub server_certificate_id: std::option::Option<std::string::String>,
    /// <p> The Amazon Resource Name (ARN) specifying the server certificate. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>The date when the server certificate was uploaded.</p>
    #[doc(hidden)]
    pub upload_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The date on which the certificate is set to expire.</p>
    #[doc(hidden)]
    pub expiration: std::option::Option<aws_smithy_types::DateTime>,
}
impl ServerCertificateMetadata {
    /// <p> The path to the server certificate. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn path(&self) -> std::option::Option<&str> {
        self.path.as_deref()
    }
    /// <p>The name that identifies the server certificate.</p>
    pub fn server_certificate_name(&self) -> std::option::Option<&str> {
        self.server_certificate_name.as_deref()
    }
    /// <p> The stable and unique string identifying the server certificate. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn server_certificate_id(&self) -> std::option::Option<&str> {
        self.server_certificate_id.as_deref()
    }
    /// <p> The Amazon Resource Name (ARN) specifying the server certificate. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn arn(&self) -> std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The date when the server certificate was uploaded.</p>
    pub fn upload_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.upload_date.as_ref()
    }
    /// <p>The date on which the certificate is set to expire.</p>
    pub fn expiration(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.expiration.as_ref()
    }
}
impl ServerCertificateMetadata {
    /// Creates a new builder-style object to manufacture [`ServerCertificateMetadata`](crate::types::ServerCertificateMetadata).
    pub fn builder() -> crate::types::builders::ServerCertificateMetadataBuilder {
        crate::types::builders::ServerCertificateMetadataBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ServerCertificateMetadata;
/// A builder for [`ServerCertificateMetadata`](crate::types::ServerCertificateMetadata).
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
pub struct ServerCertificateMetadataBuilder {
    pub(crate) path: std::option::Option<std::string::String>,
    pub(crate) server_certificate_name: std::option::Option<std::string::String>,
    pub(crate) server_certificate_id: std::option::Option<std::string::String>,
    pub(crate) arn: std::option::Option<std::string::String>,
    pub(crate) upload_date: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) expiration: std::option::Option<aws_smithy_types::DateTime>,
}
impl ServerCertificateMetadataBuilder {
    /// <p> The path to the server certificate. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn path(mut self, input: impl Into<std::string::String>) -> Self {
        self.path = Some(input.into());
        self
    }
    /// <p> The path to the server certificate. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn set_path(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// <p>The name that identifies the server certificate.</p>
    pub fn server_certificate_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.server_certificate_name = Some(input.into());
        self
    }
    /// <p>The name that identifies the server certificate.</p>
    pub fn set_server_certificate_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.server_certificate_name = input;
        self
    }
    /// <p> The stable and unique string identifying the server certificate. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn server_certificate_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.server_certificate_id = Some(input.into());
        self
    }
    /// <p> The stable and unique string identifying the server certificate. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn set_server_certificate_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.server_certificate_id = input;
        self
    }
    /// <p> The Amazon Resource Name (ARN) specifying the server certificate. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.arn = Some(input.into());
        self
    }
    /// <p> The Amazon Resource Name (ARN) specifying the server certificate. For more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The date when the server certificate was uploaded.</p>
    pub fn upload_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.upload_date = Some(input);
        self
    }
    /// <p>The date when the server certificate was uploaded.</p>
    pub fn set_upload_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.upload_date = input;
        self
    }
    /// <p>The date on which the certificate is set to expire.</p>
    pub fn expiration(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.expiration = Some(input);
        self
    }
    /// <p>The date on which the certificate is set to expire.</p>
    pub fn set_expiration(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.expiration = input;
        self
    }
    /// Consumes the builder and constructs a [`ServerCertificateMetadata`](crate::types::ServerCertificateMetadata).
    pub fn build(self) -> crate::types::ServerCertificateMetadata {
        crate::types::ServerCertificateMetadata {
            path: self.path,
            server_certificate_name: self.server_certificate_name,
            server_certificate_id: self.server_certificate_id,
            arn: self.arn,
            upload_date: self.upload_date,
            expiration: self.expiration,
        }
    }
}