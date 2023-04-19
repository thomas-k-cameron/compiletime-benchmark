// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the associated IAM roles.</p>
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
pub struct AssociatedRole {
    /// <p>The ARN of the associated IAM role.</p>
    #[doc(hidden)]
    pub associated_role_arn: std::option::Option<std::string::String>,
    /// <p>The name of the Amazon S3 bucket in which the Amazon S3 object is stored.</p>
    #[doc(hidden)]
    pub certificate_s3_bucket_name: std::option::Option<std::string::String>,
    /// <p>The key of the Amazon S3 object ey where the certificate, certificate chain, and encrypted private key bundle is stored. The object key is formated as follows: <code>role_arn</code>/<code>certificate_arn</code>. </p>
    #[doc(hidden)]
    pub certificate_s3_object_key: std::option::Option<std::string::String>,
    /// <p>The ID of the KMS customer master key (CMK) used to encrypt the private key.</p>
    #[doc(hidden)]
    pub encryption_kms_key_id: std::option::Option<std::string::String>,
}
impl AssociatedRole {
    /// <p>The ARN of the associated IAM role.</p>
    pub fn associated_role_arn(&self) -> std::option::Option<&str> {
        self.associated_role_arn.as_deref()
    }
    /// <p>The name of the Amazon S3 bucket in which the Amazon S3 object is stored.</p>
    pub fn certificate_s3_bucket_name(&self) -> std::option::Option<&str> {
        self.certificate_s3_bucket_name.as_deref()
    }
    /// <p>The key of the Amazon S3 object ey where the certificate, certificate chain, and encrypted private key bundle is stored. The object key is formated as follows: <code>role_arn</code>/<code>certificate_arn</code>. </p>
    pub fn certificate_s3_object_key(&self) -> std::option::Option<&str> {
        self.certificate_s3_object_key.as_deref()
    }
    /// <p>The ID of the KMS customer master key (CMK) used to encrypt the private key.</p>
    pub fn encryption_kms_key_id(&self) -> std::option::Option<&str> {
        self.encryption_kms_key_id.as_deref()
    }
}
impl AssociatedRole {
    /// Creates a new builder-style object to manufacture [`AssociatedRole`](crate::types::AssociatedRole).
    pub fn builder() -> crate::types::builders::AssociatedRoleBuilder {
        crate::types::builders::AssociatedRoleBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AssociatedRole;
/// A builder for [`AssociatedRole`](crate::types::AssociatedRole).
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
pub struct AssociatedRoleBuilder {
    pub(crate) associated_role_arn: std::option::Option<std::string::String>,
    pub(crate) certificate_s3_bucket_name: std::option::Option<std::string::String>,
    pub(crate) certificate_s3_object_key: std::option::Option<std::string::String>,
    pub(crate) encryption_kms_key_id: std::option::Option<std::string::String>,
}
impl AssociatedRoleBuilder {
    /// <p>The ARN of the associated IAM role.</p>
    pub fn associated_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.associated_role_arn = Some(input.into());
        self
    }
    /// <p>The ARN of the associated IAM role.</p>
    pub fn set_associated_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.associated_role_arn = input;
        self
    }
    /// <p>The name of the Amazon S3 bucket in which the Amazon S3 object is stored.</p>
    pub fn certificate_s3_bucket_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.certificate_s3_bucket_name = Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket in which the Amazon S3 object is stored.</p>
    pub fn set_certificate_s3_bucket_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.certificate_s3_bucket_name = input;
        self
    }
    /// <p>The key of the Amazon S3 object ey where the certificate, certificate chain, and encrypted private key bundle is stored. The object key is formated as follows: <code>role_arn</code>/<code>certificate_arn</code>. </p>
    pub fn certificate_s3_object_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.certificate_s3_object_key = Some(input.into());
        self
    }
    /// <p>The key of the Amazon S3 object ey where the certificate, certificate chain, and encrypted private key bundle is stored. The object key is formated as follows: <code>role_arn</code>/<code>certificate_arn</code>. </p>
    pub fn set_certificate_s3_object_key(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.certificate_s3_object_key = input;
        self
    }
    /// <p>The ID of the KMS customer master key (CMK) used to encrypt the private key.</p>
    pub fn encryption_kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.encryption_kms_key_id = Some(input.into());
        self
    }
    /// <p>The ID of the KMS customer master key (CMK) used to encrypt the private key.</p>
    pub fn set_encryption_kms_key_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.encryption_kms_key_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociatedRole`](crate::types::AssociatedRole).
    pub fn build(self) -> crate::types::AssociatedRole {
        crate::types::AssociatedRole {
            associated_role_arn: self.associated_role_arn,
            certificate_s3_bucket_name: self.certificate_s3_bucket_name,
            certificate_s3_object_key: self.certificate_s3_object_key,
            encryption_kms_key_id: self.encryption_kms_key_id,
        }
    }
}
