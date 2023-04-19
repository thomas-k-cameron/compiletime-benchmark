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
pub struct AbortVaultLockOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for AbortVaultLockOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AbortVaultLockOutput {
    /// Creates a new builder-style object to manufacture [`AbortVaultLockOutput`](crate::operation::abort_vault_lock::AbortVaultLockOutput).
    pub fn builder() -> crate::operation::abort_vault_lock::builders::AbortVaultLockOutputBuilder {
        crate::operation::abort_vault_lock::builders::AbortVaultLockOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::abort_vault_lock::AbortVaultLockOutput;
/// A builder for [`AbortVaultLockOutput`](crate::operation::abort_vault_lock::AbortVaultLockOutput).
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
pub struct AbortVaultLockOutputBuilder {
    _request_id: Option<String>,
}
impl AbortVaultLockOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`AbortVaultLockOutput`](crate::operation::abort_vault_lock::AbortVaultLockOutput).
    pub fn build(self) -> crate::operation::abort_vault_lock::AbortVaultLockOutput {
        crate::operation::abort_vault_lock::AbortVaultLockOutput {
            _request_id: self._request_id,
        }
    }
}
