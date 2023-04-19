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
pub struct DeleteVirtualMfaDeviceInput {
    /// <p>The serial number that uniquely identifies the MFA device. For virtual MFA devices, the serial number is the same as the ARN.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: =,.@:/-</p>
    #[doc(hidden)]
    pub serial_number: std::option::Option<std::string::String>,
}
impl DeleteVirtualMfaDeviceInput {
    /// <p>The serial number that uniquely identifies the MFA device. For virtual MFA devices, the serial number is the same as the ARN.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: =,.@:/-</p>
    pub fn serial_number(&self) -> std::option::Option<&str> {
        self.serial_number.as_deref()
    }
}
impl DeleteVirtualMfaDeviceInput {
    /// Creates a new builder-style object to manufacture [`DeleteVirtualMfaDeviceInput`](crate::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceInput).
    pub fn builder(
    ) -> crate::operation::delete_virtual_mfa_device::builders::DeleteVirtualMfaDeviceInputBuilder
    {
        crate::operation::delete_virtual_mfa_device::builders::DeleteVirtualMfaDeviceInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceInput;
/// A builder for [`DeleteVirtualMfaDeviceInput`](crate::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceInput).
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
pub struct DeleteVirtualMfaDeviceInputBuilder {
    pub(crate) serial_number: std::option::Option<std::string::String>,
}
impl DeleteVirtualMfaDeviceInputBuilder {
    /// <p>The serial number that uniquely identifies the MFA device. For virtual MFA devices, the serial number is the same as the ARN.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: =,.@:/-</p>
    pub fn serial_number(mut self, input: impl Into<std::string::String>) -> Self {
        self.serial_number = Some(input.into());
        self
    }
    /// <p>The serial number that uniquely identifies the MFA device. For virtual MFA devices, the serial number is the same as the ARN.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: =,.@:/-</p>
    pub fn set_serial_number(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.serial_number = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVirtualMfaDeviceInput`](crate::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceInput {
                serial_number: self.serial_number,
            },
        )
    }
}
