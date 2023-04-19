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
pub struct DeleteAliasInput {
    /// <p>The alias to be deleted. The alias name must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>.</p>
    #[doc(hidden)]
    pub alias_name: std::option::Option<std::string::String>,
}
impl DeleteAliasInput {
    /// <p>The alias to be deleted. The alias name must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>.</p>
    pub fn alias_name(&self) -> std::option::Option<&str> {
        self.alias_name.as_deref()
    }
}
impl DeleteAliasInput {
    /// Creates a new builder-style object to manufacture [`DeleteAliasInput`](crate::operation::delete_alias::DeleteAliasInput).
    pub fn builder() -> crate::operation::delete_alias::builders::DeleteAliasInputBuilder {
        crate::operation::delete_alias::builders::DeleteAliasInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_alias::DeleteAliasInput;
/// A builder for [`DeleteAliasInput`](crate::operation::delete_alias::DeleteAliasInput).
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
pub struct DeleteAliasInputBuilder {
    pub(crate) alias_name: std::option::Option<std::string::String>,
}
impl DeleteAliasInputBuilder {
    /// <p>The alias to be deleted. The alias name must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>.</p>
    pub fn alias_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.alias_name = Some(input.into());
        self
    }
    /// <p>The alias to be deleted. The alias name must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>.</p>
    pub fn set_alias_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.alias_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteAliasInput`](crate::operation::delete_alias::DeleteAliasInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_alias::DeleteAliasInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::delete_alias::DeleteAliasInput {
            alias_name: self.alias_name,
        })
    }
}
