// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A Retention configuration for an object.</p>
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
pub struct ObjectLockRetention {
    /// <p>Indicates the Retention mode for the specified object.</p>
    #[doc(hidden)]
    pub mode: std::option::Option<crate::types::ObjectLockRetentionMode>,
    /// <p>The date on which this Object Lock Retention will expire.</p>
    #[doc(hidden)]
    pub retain_until_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl ObjectLockRetention {
    /// <p>Indicates the Retention mode for the specified object.</p>
    pub fn mode(&self) -> std::option::Option<&crate::types::ObjectLockRetentionMode> {
        self.mode.as_ref()
    }
    /// <p>The date on which this Object Lock Retention will expire.</p>
    pub fn retain_until_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.retain_until_date.as_ref()
    }
}
impl ObjectLockRetention {
    /// Creates a new builder-style object to manufacture [`ObjectLockRetention`](crate::types::ObjectLockRetention).
    pub fn builder() -> crate::types::builders::ObjectLockRetentionBuilder {
        crate::types::builders::ObjectLockRetentionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ObjectLockRetention;
/// A builder for [`ObjectLockRetention`](crate::types::ObjectLockRetention).
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
pub struct ObjectLockRetentionBuilder {
    pub(crate) mode: std::option::Option<crate::types::ObjectLockRetentionMode>,
    pub(crate) retain_until_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl ObjectLockRetentionBuilder {
    /// <p>Indicates the Retention mode for the specified object.</p>
    pub fn mode(mut self, input: crate::types::ObjectLockRetentionMode) -> Self {
        self.mode = Some(input);
        self
    }
    /// <p>Indicates the Retention mode for the specified object.</p>
    pub fn set_mode(
        mut self,
        input: std::option::Option<crate::types::ObjectLockRetentionMode>,
    ) -> Self {
        self.mode = input;
        self
    }
    /// <p>The date on which this Object Lock Retention will expire.</p>
    pub fn retain_until_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.retain_until_date = Some(input);
        self
    }
    /// <p>The date on which this Object Lock Retention will expire.</p>
    pub fn set_retain_until_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.retain_until_date = input;
        self
    }
    /// Consumes the builder and constructs a [`ObjectLockRetention`](crate::types::ObjectLockRetention).
    pub fn build(self) -> crate::types::ObjectLockRetention {
        crate::types::ObjectLockRetention {
            mode: self.mode,
            retain_until_date: self.retain_until_date,
        }
    }
}