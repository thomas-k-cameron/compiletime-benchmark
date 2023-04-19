// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the type of server-side encryption used.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Encryption {
    /// <p>The server-side encryption algorithm used when storing job results in Amazon S3 (for example, AES256, aws:kms).</p>
    #[doc(hidden)]
    pub encryption_type: std::option::Option<crate::types::ServerSideEncryption>,
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value specifies the ID of the symmetric customer managed key to use for encryption of job results. Amazon S3 only supports symmetric keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using symmetric and asymmetric keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    #[doc(hidden)]
    pub kms_key_id: std::option::Option<std::string::String>,
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value can be used to specify the encryption context for the restore results.</p>
    #[doc(hidden)]
    pub kms_context: std::option::Option<std::string::String>,
}
impl Encryption {
    /// <p>The server-side encryption algorithm used when storing job results in Amazon S3 (for example, AES256, aws:kms).</p>
    pub fn encryption_type(&self) -> std::option::Option<&crate::types::ServerSideEncryption> {
        self.encryption_type.as_ref()
    }
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value specifies the ID of the symmetric customer managed key to use for encryption of job results. Amazon S3 only supports symmetric keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using symmetric and asymmetric keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn kms_key_id(&self) -> std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value can be used to specify the encryption context for the restore results.</p>
    pub fn kms_context(&self) -> std::option::Option<&str> {
        self.kms_context.as_deref()
    }
}
impl std::fmt::Debug for Encryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Encryption");
        formatter.field("encryption_type", &self.encryption_type);
        formatter.field("kms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("kms_context", &self.kms_context);
        formatter.finish()
    }
}
impl Encryption {
    /// Creates a new builder-style object to manufacture [`Encryption`](crate::types::Encryption).
    pub fn builder() -> crate::types::builders::EncryptionBuilder {
        crate::types::builders::EncryptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Encryption;
/// A builder for [`Encryption`](crate::types::Encryption).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct EncryptionBuilder {
    pub(crate) encryption_type: std::option::Option<crate::types::ServerSideEncryption>,
    pub(crate) kms_key_id: std::option::Option<std::string::String>,
    pub(crate) kms_context: std::option::Option<std::string::String>,
}
impl EncryptionBuilder {
    /// <p>The server-side encryption algorithm used when storing job results in Amazon S3 (for example, AES256, aws:kms).</p>
    pub fn encryption_type(mut self, input: crate::types::ServerSideEncryption) -> Self {
        self.encryption_type = Some(input);
        self
    }
    /// <p>The server-side encryption algorithm used when storing job results in Amazon S3 (for example, AES256, aws:kms).</p>
    pub fn set_encryption_type(
        mut self,
        input: std::option::Option<crate::types::ServerSideEncryption>,
    ) -> Self {
        self.encryption_type = input;
        self
    }
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value specifies the ID of the symmetric customer managed key to use for encryption of job results. Amazon S3 only supports symmetric keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using symmetric and asymmetric keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.kms_key_id = Some(input.into());
        self
    }
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value specifies the ID of the symmetric customer managed key to use for encryption of job results. Amazon S3 only supports symmetric keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using symmetric and asymmetric keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn set_kms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value can be used to specify the encryption context for the restore results.</p>
    pub fn kms_context(mut self, input: impl Into<std::string::String>) -> Self {
        self.kms_context = Some(input.into());
        self
    }
    /// <p>If the encryption type is <code>aws:kms</code>, this optional value can be used to specify the encryption context for the restore results.</p>
    pub fn set_kms_context(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.kms_context = input;
        self
    }
    /// Consumes the builder and constructs a [`Encryption`](crate::types::Encryption).
    pub fn build(self) -> crate::types::Encryption {
        crate::types::Encryption {
            encryption_type: self.encryption_type,
            kms_key_id: self.kms_key_id,
            kms_context: self.kms_context,
        }
    }
}
impl std::fmt::Debug for EncryptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("EncryptionBuilder");
        formatter.field("encryption_type", &self.encryption_type);
        formatter.field("kms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("kms_context", &self.kms_context);
        formatter.finish()
    }
}
