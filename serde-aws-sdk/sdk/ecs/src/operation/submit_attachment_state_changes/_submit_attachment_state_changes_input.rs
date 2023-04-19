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
pub struct SubmitAttachmentStateChangesInput {
    /// <p>The short name or full ARN of the cluster that hosts the container instance the attachment belongs to.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<std::string::String>,
    /// <p>Any attachments associated with the state change request.</p>
    #[doc(hidden)]
    pub attachments: std::option::Option<std::vec::Vec<crate::types::AttachmentStateChange>>,
}
impl SubmitAttachmentStateChangesInput {
    /// <p>The short name or full ARN of the cluster that hosts the container instance the attachment belongs to.</p>
    pub fn cluster(&self) -> std::option::Option<&str> {
        self.cluster.as_deref()
    }
    /// <p>Any attachments associated with the state change request.</p>
    pub fn attachments(&self) -> std::option::Option<&[crate::types::AttachmentStateChange]> {
        self.attachments.as_deref()
    }
}
impl SubmitAttachmentStateChangesInput {
    /// Creates a new builder-style object to manufacture [`SubmitAttachmentStateChangesInput`](crate::operation::submit_attachment_state_changes::SubmitAttachmentStateChangesInput).
    pub fn builder() -> crate::operation::submit_attachment_state_changes::builders::SubmitAttachmentStateChangesInputBuilder{
        crate::operation::submit_attachment_state_changes::builders::SubmitAttachmentStateChangesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::submit_attachment_state_changes::SubmitAttachmentStateChangesInput;
/// A builder for [`SubmitAttachmentStateChangesInput`](crate::operation::submit_attachment_state_changes::SubmitAttachmentStateChangesInput).
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
pub struct SubmitAttachmentStateChangesInputBuilder {
    pub(crate) cluster: std::option::Option<std::string::String>,
    pub(crate) attachments: std::option::Option<std::vec::Vec<crate::types::AttachmentStateChange>>,
}
impl SubmitAttachmentStateChangesInputBuilder {
    /// <p>The short name or full ARN of the cluster that hosts the container instance the attachment belongs to.</p>
    pub fn cluster(mut self, input: impl Into<std::string::String>) -> Self {
        self.cluster = Some(input.into());
        self
    }
    /// <p>The short name or full ARN of the cluster that hosts the container instance the attachment belongs to.</p>
    pub fn set_cluster(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cluster = input;
        self
    }
    /// Appends an item to `attachments`.
    ///
    /// To override the contents of this collection use [`set_attachments`](Self::set_attachments).
    ///
    /// <p>Any attachments associated with the state change request.</p>
    pub fn attachments(mut self, input: crate::types::AttachmentStateChange) -> Self {
        let mut v = self.attachments.unwrap_or_default();
        v.push(input);
        self.attachments = Some(v);
        self
    }
    /// <p>Any attachments associated with the state change request.</p>
    pub fn set_attachments(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AttachmentStateChange>>,
    ) -> Self {
        self.attachments = input;
        self
    }
    /// Consumes the builder and constructs a [`SubmitAttachmentStateChangesInput`](crate::operation::submit_attachment_state_changes::SubmitAttachmentStateChangesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::submit_attachment_state_changes::SubmitAttachmentStateChangesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::submit_attachment_state_changes::SubmitAttachmentStateChangesInput {
                cluster: self.cluster,
                attachments: self.attachments,
            },
        )
    }
}
