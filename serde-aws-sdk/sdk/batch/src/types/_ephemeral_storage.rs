// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The amount of ephemeral storage to allocate for the task. This parameter is used to expand the total amount of ephemeral storage available, beyond the default amount, for tasks hosted on Fargate.</p>
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
pub struct EphemeralStorage {
    /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
    #[doc(hidden)]
    pub size_in_gi_b: std::option::Option<i32>,
}
impl EphemeralStorage {
    /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
    pub fn size_in_gi_b(&self) -> std::option::Option<i32> {
        self.size_in_gi_b
    }
}
impl EphemeralStorage {
    /// Creates a new builder-style object to manufacture [`EphemeralStorage`](crate::types::EphemeralStorage).
    pub fn builder() -> crate::types::builders::EphemeralStorageBuilder {
        crate::types::builders::EphemeralStorageBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EphemeralStorage;
/// A builder for [`EphemeralStorage`](crate::types::EphemeralStorage).
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
pub struct EphemeralStorageBuilder {
    pub(crate) size_in_gi_b: std::option::Option<i32>,
}
impl EphemeralStorageBuilder {
    /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
    pub fn size_in_gi_b(mut self, input: i32) -> Self {
        self.size_in_gi_b = Some(input);
        self
    }
    /// <p>The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is <code>21</code> GiB and the maximum supported value is <code>200</code> GiB.</p>
    pub fn set_size_in_gi_b(mut self, input: std::option::Option<i32>) -> Self {
        self.size_in_gi_b = input;
        self
    }
    /// Consumes the builder and constructs a [`EphemeralStorage`](crate::types::EphemeralStorage).
    pub fn build(self) -> crate::types::EphemeralStorage {
        crate::types::EphemeralStorage {
            size_in_gi_b: self.size_in_gi_b,
        }
    }
}
