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
pub struct GetLayerVersionInput {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[doc(hidden)]
    pub layer_name: std::option::Option<std::string::String>,
    /// <p>The version number.</p>
    #[doc(hidden)]
    pub version_number: std::option::Option<i64>,
}
impl GetLayerVersionInput {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn layer_name(&self) -> std::option::Option<&str> {
        self.layer_name.as_deref()
    }
    /// <p>The version number.</p>
    pub fn version_number(&self) -> std::option::Option<i64> {
        self.version_number
    }
}
impl GetLayerVersionInput {
    /// Creates a new builder-style object to manufacture [`GetLayerVersionInput`](crate::operation::get_layer_version::GetLayerVersionInput).
    pub fn builder() -> crate::operation::get_layer_version::builders::GetLayerVersionInputBuilder {
        crate::operation::get_layer_version::builders::GetLayerVersionInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_layer_version::GetLayerVersionInput;
/// A builder for [`GetLayerVersionInput`](crate::operation::get_layer_version::GetLayerVersionInput).
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
pub struct GetLayerVersionInputBuilder {
    pub(crate) layer_name: std::option::Option<std::string::String>,
    pub(crate) version_number: std::option::Option<i64>,
}
impl GetLayerVersionInputBuilder {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn layer_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.layer_name = Some(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn set_layer_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.layer_name = input;
        self
    }
    /// <p>The version number.</p>
    pub fn version_number(mut self, input: i64) -> Self {
        self.version_number = Some(input);
        self
    }
    /// <p>The version number.</p>
    pub fn set_version_number(mut self, input: std::option::Option<i64>) -> Self {
        self.version_number = input;
        self
    }
    /// Consumes the builder and constructs a [`GetLayerVersionInput`](crate::operation::get_layer_version::GetLayerVersionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_layer_version::GetLayerVersionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_layer_version::GetLayerVersionInput {
            layer_name: self.layer_name,
            version_number: self.version_number,
        })
    }
}
