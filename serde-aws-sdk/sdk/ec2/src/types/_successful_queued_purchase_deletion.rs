// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Reserved Instance whose queued purchase was successfully deleted.</p>
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
pub struct SuccessfulQueuedPurchaseDeletion {
    /// <p>The ID of the Reserved Instance.</p>
    #[doc(hidden)]
    pub reserved_instances_id: std::option::Option<std::string::String>,
}
impl SuccessfulQueuedPurchaseDeletion {
    /// <p>The ID of the Reserved Instance.</p>
    pub fn reserved_instances_id(&self) -> std::option::Option<&str> {
        self.reserved_instances_id.as_deref()
    }
}
impl SuccessfulQueuedPurchaseDeletion {
    /// Creates a new builder-style object to manufacture [`SuccessfulQueuedPurchaseDeletion`](crate::types::SuccessfulQueuedPurchaseDeletion).
    pub fn builder() -> crate::types::builders::SuccessfulQueuedPurchaseDeletionBuilder {
        crate::types::builders::SuccessfulQueuedPurchaseDeletionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SuccessfulQueuedPurchaseDeletion;
/// A builder for [`SuccessfulQueuedPurchaseDeletion`](crate::types::SuccessfulQueuedPurchaseDeletion).
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
pub struct SuccessfulQueuedPurchaseDeletionBuilder {
    pub(crate) reserved_instances_id: std::option::Option<std::string::String>,
}
impl SuccessfulQueuedPurchaseDeletionBuilder {
    /// <p>The ID of the Reserved Instance.</p>
    pub fn reserved_instances_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.reserved_instances_id = Some(input.into());
        self
    }
    /// <p>The ID of the Reserved Instance.</p>
    pub fn set_reserved_instances_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.reserved_instances_id = input;
        self
    }
    /// Consumes the builder and constructs a [`SuccessfulQueuedPurchaseDeletion`](crate::types::SuccessfulQueuedPurchaseDeletion).
    pub fn build(self) -> crate::types::SuccessfulQueuedPurchaseDeletion {
        crate::types::SuccessfulQueuedPurchaseDeletion {
            reserved_instances_id: self.reserved_instances_id,
        }
    }
}
