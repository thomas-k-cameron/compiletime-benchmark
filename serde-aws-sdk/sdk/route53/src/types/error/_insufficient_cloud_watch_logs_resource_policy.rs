// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Amazon Route 53 doesn't have the permissions required to create log streams and send query logs to log streams. Possible causes include the following:</p>
/// <ul>
/// <li> <p>There is no resource policy that specifies the log group ARN in the value for <code>Resource</code>.</p> </li>
/// <li> <p>The resource policy that includes the log group ARN in the value for <code>Resource</code> doesn't have the necessary permissions.</p> </li>
/// <li> <p>The resource policy hasn't finished propagating yet.</p> </li>
/// <li> <p>The Key management service (KMS) key you specified doesn’t exist or it can’t be used with the log group associated with query log. Update or provide a resource policy to grant permissions for the KMS key.</p> </li>
/// <li> <p>The Key management service (KMS) key you specified is marked as disabled for the log group associated with query log. Update or provide a resource policy to grant permissions for the KMS key.</p> </li>
/// </ul>
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
pub struct InsufficientCloudWatchLogsResourcePolicy {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl InsufficientCloudWatchLogsResourcePolicy {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InsufficientCloudWatchLogsResourcePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InsufficientCloudWatchLogsResourcePolicy")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InsufficientCloudWatchLogsResourcePolicy {}
impl aws_http::request_id::RequestId
    for crate::types::error::InsufficientCloudWatchLogsResourcePolicy
{
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata
    for InsufficientCloudWatchLogsResourcePolicy
{
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl InsufficientCloudWatchLogsResourcePolicy {
    /// Creates a new builder-style object to manufacture [`InsufficientCloudWatchLogsResourcePolicy`](crate::types::error::InsufficientCloudWatchLogsResourcePolicy).
    pub fn builder(
    ) -> crate::types::error::builders::InsufficientCloudWatchLogsResourcePolicyBuilder {
        crate::types::error::builders::InsufficientCloudWatchLogsResourcePolicyBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::InsufficientCloudWatchLogsResourcePolicy;
/// A builder for [`InsufficientCloudWatchLogsResourcePolicy`](crate::types::error::InsufficientCloudWatchLogsResourcePolicy).
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
pub struct InsufficientCloudWatchLogsResourcePolicyBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl InsufficientCloudWatchLogsResourcePolicyBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(
        &mut self,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`InsufficientCloudWatchLogsResourcePolicy`](crate::types::error::InsufficientCloudWatchLogsResourcePolicy).
    pub fn build(self) -> crate::types::error::InsufficientCloudWatchLogsResourcePolicy {
        crate::types::error::InsufficientCloudWatchLogsResourcePolicy {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
