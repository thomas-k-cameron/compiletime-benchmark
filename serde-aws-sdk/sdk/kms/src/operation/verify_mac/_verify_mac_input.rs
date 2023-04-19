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
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct VerifyMacInput {
    /// <p>The message that will be used in the verification. Enter the same message that was used to generate the HMAC.</p>
    /// <p> <code>GenerateMac</code> and <code>VerifyMac</code> do not provide special handling for message digests. If you generated an HMAC for a hash digest of a message, you must verify the HMAC for the same hash digest.</p>
    #[doc(hidden)]
    pub message: std::option::Option<aws_smithy_types::Blob>,
    /// <p>The KMS key that will be used in the verification.</p>
    /// <p>Enter a key ID of the KMS key that was used to generate the HMAC. If you identify a different KMS key, the <code>VerifyMac</code> operation fails.</p>
    #[doc(hidden)]
    pub key_id: std::option::Option<std::string::String>,
    /// <p>The MAC algorithm that will be used in the verification. Enter the same MAC algorithm that was used to compute the HMAC. This algorithm must be supported by the HMAC KMS key identified by the <code>KeyId</code> parameter.</p>
    #[doc(hidden)]
    pub mac_algorithm: std::option::Option<crate::types::MacAlgorithmSpec>,
    /// <p>The HMAC to verify. Enter the HMAC that was generated by the <code>GenerateMac</code> operation when you specified the same message, HMAC KMS key, and MAC algorithm as the values specified in this request.</p>
    #[doc(hidden)]
    pub mac: std::option::Option<aws_smithy_types::Blob>,
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    #[doc(hidden)]
    pub grant_tokens: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl VerifyMacInput {
    /// <p>The message that will be used in the verification. Enter the same message that was used to generate the HMAC.</p>
    /// <p> <code>GenerateMac</code> and <code>VerifyMac</code> do not provide special handling for message digests. If you generated an HMAC for a hash digest of a message, you must verify the HMAC for the same hash digest.</p>
    pub fn message(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.message.as_ref()
    }
    /// <p>The KMS key that will be used in the verification.</p>
    /// <p>Enter a key ID of the KMS key that was used to generate the HMAC. If you identify a different KMS key, the <code>VerifyMac</code> operation fails.</p>
    pub fn key_id(&self) -> std::option::Option<&str> {
        self.key_id.as_deref()
    }
    /// <p>The MAC algorithm that will be used in the verification. Enter the same MAC algorithm that was used to compute the HMAC. This algorithm must be supported by the HMAC KMS key identified by the <code>KeyId</code> parameter.</p>
    pub fn mac_algorithm(&self) -> std::option::Option<&crate::types::MacAlgorithmSpec> {
        self.mac_algorithm.as_ref()
    }
    /// <p>The HMAC to verify. Enter the HMAC that was generated by the <code>GenerateMac</code> operation when you specified the same message, HMAC KMS key, and MAC algorithm as the values specified in this request.</p>
    pub fn mac(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.mac.as_ref()
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_tokens(&self) -> std::option::Option<&[std::string::String]> {
        self.grant_tokens.as_deref()
    }
}
impl std::fmt::Debug for VerifyMacInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("VerifyMacInput");
        formatter.field("message", &"*** Sensitive Data Redacted ***");
        formatter.field("key_id", &self.key_id);
        formatter.field("mac_algorithm", &self.mac_algorithm);
        formatter.field("mac", &self.mac);
        formatter.field("grant_tokens", &self.grant_tokens);
        formatter.finish()
    }
}
impl VerifyMacInput {
    /// Creates a new builder-style object to manufacture [`VerifyMacInput`](crate::operation::verify_mac::VerifyMacInput).
    pub fn builder() -> crate::operation::verify_mac::builders::VerifyMacInputBuilder {
        crate::operation::verify_mac::builders::VerifyMacInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::verify_mac::VerifyMacInput;
/// A builder for [`VerifyMacInput`](crate::operation::verify_mac::VerifyMacInput).
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
pub struct VerifyMacInputBuilder {
    pub(crate) message: std::option::Option<aws_smithy_types::Blob>,
    pub(crate) key_id: std::option::Option<std::string::String>,
    pub(crate) mac_algorithm: std::option::Option<crate::types::MacAlgorithmSpec>,
    pub(crate) mac: std::option::Option<aws_smithy_types::Blob>,
    pub(crate) grant_tokens: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl VerifyMacInputBuilder {
    /// <p>The message that will be used in the verification. Enter the same message that was used to generate the HMAC.</p>
    /// <p> <code>GenerateMac</code> and <code>VerifyMac</code> do not provide special handling for message digests. If you generated an HMAC for a hash digest of a message, you must verify the HMAC for the same hash digest.</p>
    pub fn message(mut self, input: aws_smithy_types::Blob) -> Self {
        self.message = Some(input);
        self
    }
    /// <p>The message that will be used in the verification. Enter the same message that was used to generate the HMAC.</p>
    /// <p> <code>GenerateMac</code> and <code>VerifyMac</code> do not provide special handling for message digests. If you generated an HMAC for a hash digest of a message, you must verify the HMAC for the same hash digest.</p>
    pub fn set_message(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.message = input;
        self
    }
    /// <p>The KMS key that will be used in the verification.</p>
    /// <p>Enter a key ID of the KMS key that was used to generate the HMAC. If you identify a different KMS key, the <code>VerifyMac</code> operation fails.</p>
    pub fn key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.key_id = Some(input.into());
        self
    }
    /// <p>The KMS key that will be used in the verification.</p>
    /// <p>Enter a key ID of the KMS key that was used to generate the HMAC. If you identify a different KMS key, the <code>VerifyMac</code> operation fails.</p>
    pub fn set_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// <p>The MAC algorithm that will be used in the verification. Enter the same MAC algorithm that was used to compute the HMAC. This algorithm must be supported by the HMAC KMS key identified by the <code>KeyId</code> parameter.</p>
    pub fn mac_algorithm(mut self, input: crate::types::MacAlgorithmSpec) -> Self {
        self.mac_algorithm = Some(input);
        self
    }
    /// <p>The MAC algorithm that will be used in the verification. Enter the same MAC algorithm that was used to compute the HMAC. This algorithm must be supported by the HMAC KMS key identified by the <code>KeyId</code> parameter.</p>
    pub fn set_mac_algorithm(
        mut self,
        input: std::option::Option<crate::types::MacAlgorithmSpec>,
    ) -> Self {
        self.mac_algorithm = input;
        self
    }
    /// <p>The HMAC to verify. Enter the HMAC that was generated by the <code>GenerateMac</code> operation when you specified the same message, HMAC KMS key, and MAC algorithm as the values specified in this request.</p>
    pub fn mac(mut self, input: aws_smithy_types::Blob) -> Self {
        self.mac = Some(input);
        self
    }
    /// <p>The HMAC to verify. Enter the HMAC that was generated by the <code>GenerateMac</code> operation when you specified the same message, HMAC KMS key, and MAC algorithm as the values specified in this request.</p>
    pub fn set_mac(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.mac = input;
        self
    }
    /// Appends an item to `grant_tokens`.
    ///
    /// To override the contents of this collection use [`set_grant_tokens`](Self::set_grant_tokens).
    ///
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_tokens(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.grant_tokens.unwrap_or_default();
        v.push(input.into());
        self.grant_tokens = Some(v);
        self
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn set_grant_tokens(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.grant_tokens = input;
        self
    }
    /// Consumes the builder and constructs a [`VerifyMacInput`](crate::operation::verify_mac::VerifyMacInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::verify_mac::VerifyMacInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::verify_mac::VerifyMacInput {
            message: self.message,
            key_id: self.key_id,
            mac_algorithm: self.mac_algorithm,
            mac: self.mac,
            grant_tokens: self.grant_tokens,
        })
    }
}
impl std::fmt::Debug for VerifyMacInputBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("VerifyMacInputBuilder");
        formatter.field("message", &"*** Sensitive Data Redacted ***");
        formatter.field("key_id", &self.key_id);
        formatter.field("mac_algorithm", &self.mac_algorithm);
        formatter.field("mac", &self.mac);
        formatter.field("grant_tokens", &self.grant_tokens);
        formatter.finish()
    }
}