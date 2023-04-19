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
pub struct UntagMfaDeviceInput {
    /// <p>The unique identifier for the IAM virtual MFA device from which you want to remove tags. For virtual MFA devices, the serial number is the same as the ARN.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub serial_number: std::option::Option<std::string::String>,
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified instance profile.</p>
    #[doc(hidden)]
    pub tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UntagMfaDeviceInput {
    /// <p>The unique identifier for the IAM virtual MFA device from which you want to remove tags. For virtual MFA devices, the serial number is the same as the ARN.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn serial_number(&self) -> std::option::Option<&str> {
        self.serial_number.as_deref()
    }
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified instance profile.</p>
    pub fn tag_keys(&self) -> std::option::Option<&[std::string::String]> {
        self.tag_keys.as_deref()
    }
}
impl UntagMfaDeviceInput {
    /// Creates a new builder-style object to manufacture [`UntagMfaDeviceInput`](crate::operation::untag_mfa_device::UntagMfaDeviceInput).
    pub fn builder() -> crate::operation::untag_mfa_device::builders::UntagMfaDeviceInputBuilder {
        crate::operation::untag_mfa_device::builders::UntagMfaDeviceInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::untag_mfa_device::UntagMfaDeviceInput;
/// A builder for [`UntagMfaDeviceInput`](crate::operation::untag_mfa_device::UntagMfaDeviceInput).
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
pub struct UntagMfaDeviceInputBuilder {
    pub(crate) serial_number: std::option::Option<std::string::String>,
    pub(crate) tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UntagMfaDeviceInputBuilder {
    /// <p>The unique identifier for the IAM virtual MFA device from which you want to remove tags. For virtual MFA devices, the serial number is the same as the ARN.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn serial_number(mut self, input: impl Into<std::string::String>) -> Self {
        self.serial_number = Some(input.into());
        self
    }
    /// <p>The unique identifier for the IAM virtual MFA device from which you want to remove tags. For virtual MFA devices, the serial number is the same as the ARN.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_serial_number(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.serial_number = input;
        self
    }
    /// Appends an item to `tag_keys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified instance profile.</p>
    pub fn tag_keys(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.tag_keys.unwrap_or_default();
        v.push(input.into());
        self.tag_keys = Some(v);
        self
    }
    /// <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified instance profile.</p>
    pub fn set_tag_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.tag_keys = input;
        self
    }
    /// Consumes the builder and constructs a [`UntagMfaDeviceInput`](crate::operation::untag_mfa_device::UntagMfaDeviceInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::untag_mfa_device::UntagMfaDeviceInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::untag_mfa_device::UntagMfaDeviceInput {
            serial_number: self.serial_number,
            tag_keys: self.tag_keys,
        })
    }
}
