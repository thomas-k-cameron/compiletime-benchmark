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
pub struct VerifyMacOutput {
    /// <p>The HMAC KMS key used in the verification.</p>
    #[doc(hidden)]
    pub key_id: std::option::Option<std::string::String>,
    /// <p>A Boolean value that indicates whether the HMAC was verified. A value of <code>True</code> indicates that the HMAC (<code>Mac</code>) was generated with the specified <code>Message</code>, HMAC KMS key (<code>KeyID</code>) and <code>MacAlgorithm.</code>.</p>
    /// <p>If the HMAC is not verified, the <code>VerifyMac</code> operation fails with a <code>KMSInvalidMacException</code> exception. This exception indicates that one or more of the inputs changed since the HMAC was computed.</p>
    #[doc(hidden)]
    pub mac_valid: bool,
    /// <p>The MAC algorithm used in the verification.</p>
    #[doc(hidden)]
    pub mac_algorithm: std::option::Option<crate::types::MacAlgorithmSpec>,
    _request_id: Option<String>,
}
impl VerifyMacOutput {
    /// <p>The HMAC KMS key used in the verification.</p>
    pub fn key_id(&self) -> std::option::Option<&str> {
        self.key_id.as_deref()
    }
    /// <p>A Boolean value that indicates whether the HMAC was verified. A value of <code>True</code> indicates that the HMAC (<code>Mac</code>) was generated with the specified <code>Message</code>, HMAC KMS key (<code>KeyID</code>) and <code>MacAlgorithm.</code>.</p>
    /// <p>If the HMAC is not verified, the <code>VerifyMac</code> operation fails with a <code>KMSInvalidMacException</code> exception. This exception indicates that one or more of the inputs changed since the HMAC was computed.</p>
    pub fn mac_valid(&self) -> bool {
        self.mac_valid
    }
    /// <p>The MAC algorithm used in the verification.</p>
    pub fn mac_algorithm(&self) -> std::option::Option<&crate::types::MacAlgorithmSpec> {
        self.mac_algorithm.as_ref()
    }
}
impl aws_http::request_id::RequestId for VerifyMacOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl VerifyMacOutput {
    /// Creates a new builder-style object to manufacture [`VerifyMacOutput`](crate::operation::verify_mac::VerifyMacOutput).
    pub fn builder() -> crate::operation::verify_mac::builders::VerifyMacOutputBuilder {
        crate::operation::verify_mac::builders::VerifyMacOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::verify_mac::VerifyMacOutput;
/// A builder for [`VerifyMacOutput`](crate::operation::verify_mac::VerifyMacOutput).
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
pub struct VerifyMacOutputBuilder {
    pub(crate) key_id: std::option::Option<std::string::String>,
    pub(crate) mac_valid: std::option::Option<bool>,
    pub(crate) mac_algorithm: std::option::Option<crate::types::MacAlgorithmSpec>,
    _request_id: Option<String>,
}
impl VerifyMacOutputBuilder {
    /// <p>The HMAC KMS key used in the verification.</p>
    pub fn key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.key_id = Some(input.into());
        self
    }
    /// <p>The HMAC KMS key used in the verification.</p>
    pub fn set_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// <p>A Boolean value that indicates whether the HMAC was verified. A value of <code>True</code> indicates that the HMAC (<code>Mac</code>) was generated with the specified <code>Message</code>, HMAC KMS key (<code>KeyID</code>) and <code>MacAlgorithm.</code>.</p>
    /// <p>If the HMAC is not verified, the <code>VerifyMac</code> operation fails with a <code>KMSInvalidMacException</code> exception. This exception indicates that one or more of the inputs changed since the HMAC was computed.</p>
    pub fn mac_valid(mut self, input: bool) -> Self {
        self.mac_valid = Some(input);
        self
    }
    /// <p>A Boolean value that indicates whether the HMAC was verified. A value of <code>True</code> indicates that the HMAC (<code>Mac</code>) was generated with the specified <code>Message</code>, HMAC KMS key (<code>KeyID</code>) and <code>MacAlgorithm.</code>.</p>
    /// <p>If the HMAC is not verified, the <code>VerifyMac</code> operation fails with a <code>KMSInvalidMacException</code> exception. This exception indicates that one or more of the inputs changed since the HMAC was computed.</p>
    pub fn set_mac_valid(mut self, input: std::option::Option<bool>) -> Self {
        self.mac_valid = input;
        self
    }
    /// <p>The MAC algorithm used in the verification.</p>
    pub fn mac_algorithm(mut self, input: crate::types::MacAlgorithmSpec) -> Self {
        self.mac_algorithm = Some(input);
        self
    }
    /// <p>The MAC algorithm used in the verification.</p>
    pub fn set_mac_algorithm(
        mut self,
        input: std::option::Option<crate::types::MacAlgorithmSpec>,
    ) -> Self {
        self.mac_algorithm = input;
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
    /// Consumes the builder and constructs a [`VerifyMacOutput`](crate::operation::verify_mac::VerifyMacOutput).
    pub fn build(self) -> crate::operation::verify_mac::VerifyMacOutput {
        crate::operation::verify_mac::VerifyMacOutput {
            key_id: self.key_id,
            mac_valid: self.mac_valid.unwrap_or_default(),
            mac_algorithm: self.mac_algorithm,
            _request_id: self._request_id,
        }
    }
}