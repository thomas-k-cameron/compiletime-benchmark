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
pub struct DeleteCapacityProviderInput {
    /// <p>The short name or full Amazon Resource Name (ARN) of the capacity provider to delete.</p>
    #[doc(hidden)]
    pub capacity_provider: std::option::Option<std::string::String>,
}
impl DeleteCapacityProviderInput {
    /// <p>The short name or full Amazon Resource Name (ARN) of the capacity provider to delete.</p>
    pub fn capacity_provider(&self) -> std::option::Option<&str> {
        self.capacity_provider.as_deref()
    }
}
impl DeleteCapacityProviderInput {
    /// Creates a new builder-style object to manufacture [`DeleteCapacityProviderInput`](crate::operation::delete_capacity_provider::DeleteCapacityProviderInput).
    pub fn builder(
    ) -> crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderInputBuilder
    {
        crate::operation::delete_capacity_provider::builders::DeleteCapacityProviderInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_capacity_provider::DeleteCapacityProviderInput;
/// A builder for [`DeleteCapacityProviderInput`](crate::operation::delete_capacity_provider::DeleteCapacityProviderInput).
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
pub struct DeleteCapacityProviderInputBuilder {
    pub(crate) capacity_provider: std::option::Option<std::string::String>,
}
impl DeleteCapacityProviderInputBuilder {
    /// <p>The short name or full Amazon Resource Name (ARN) of the capacity provider to delete.</p>
    pub fn capacity_provider(mut self, input: impl Into<std::string::String>) -> Self {
        self.capacity_provider = Some(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the capacity provider to delete.</p>
    pub fn set_capacity_provider(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.capacity_provider = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteCapacityProviderInput`](crate::operation::delete_capacity_provider::DeleteCapacityProviderInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_capacity_provider::DeleteCapacityProviderInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_capacity_provider::DeleteCapacityProviderInput {
                capacity_provider: self.capacity_provider,
            },
        )
    }
}
