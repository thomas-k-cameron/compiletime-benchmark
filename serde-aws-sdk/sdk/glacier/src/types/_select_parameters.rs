// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the parameters used for a select.</p>
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
pub struct SelectParameters {
    /// <p>Describes the serialization format of the object.</p>
    #[doc(hidden)]
    pub input_serialization: std::option::Option<crate::types::InputSerialization>,
    /// <p>The type of the provided expression, for example <code>SQL</code>.</p>
    #[doc(hidden)]
    pub expression_type: std::option::Option<crate::types::ExpressionType>,
    /// <p>The expression that is used to select the object.</p>
    #[doc(hidden)]
    pub expression: std::option::Option<std::string::String>,
    /// <p>Describes how the results of the select job are serialized.</p>
    #[doc(hidden)]
    pub output_serialization: std::option::Option<crate::types::OutputSerialization>,
}
impl SelectParameters {
    /// <p>Describes the serialization format of the object.</p>
    pub fn input_serialization(&self) -> std::option::Option<&crate::types::InputSerialization> {
        self.input_serialization.as_ref()
    }
    /// <p>The type of the provided expression, for example <code>SQL</code>.</p>
    pub fn expression_type(&self) -> std::option::Option<&crate::types::ExpressionType> {
        self.expression_type.as_ref()
    }
    /// <p>The expression that is used to select the object.</p>
    pub fn expression(&self) -> std::option::Option<&str> {
        self.expression.as_deref()
    }
    /// <p>Describes how the results of the select job are serialized.</p>
    pub fn output_serialization(&self) -> std::option::Option<&crate::types::OutputSerialization> {
        self.output_serialization.as_ref()
    }
}
impl SelectParameters {
    /// Creates a new builder-style object to manufacture [`SelectParameters`](crate::types::SelectParameters).
    pub fn builder() -> crate::types::builders::SelectParametersBuilder {
        crate::types::builders::SelectParametersBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SelectParameters;
/// A builder for [`SelectParameters`](crate::types::SelectParameters).
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
pub struct SelectParametersBuilder {
    pub(crate) input_serialization: std::option::Option<crate::types::InputSerialization>,
    pub(crate) expression_type: std::option::Option<crate::types::ExpressionType>,
    pub(crate) expression: std::option::Option<std::string::String>,
    pub(crate) output_serialization: std::option::Option<crate::types::OutputSerialization>,
}
impl SelectParametersBuilder {
    /// <p>Describes the serialization format of the object.</p>
    pub fn input_serialization(mut self, input: crate::types::InputSerialization) -> Self {
        self.input_serialization = Some(input);
        self
    }
    /// <p>Describes the serialization format of the object.</p>
    pub fn set_input_serialization(
        mut self,
        input: std::option::Option<crate::types::InputSerialization>,
    ) -> Self {
        self.input_serialization = input;
        self
    }
    /// <p>The type of the provided expression, for example <code>SQL</code>.</p>
    pub fn expression_type(mut self, input: crate::types::ExpressionType) -> Self {
        self.expression_type = Some(input);
        self
    }
    /// <p>The type of the provided expression, for example <code>SQL</code>.</p>
    pub fn set_expression_type(
        mut self,
        input: std::option::Option<crate::types::ExpressionType>,
    ) -> Self {
        self.expression_type = input;
        self
    }
    /// <p>The expression that is used to select the object.</p>
    pub fn expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.expression = Some(input.into());
        self
    }
    /// <p>The expression that is used to select the object.</p>
    pub fn set_expression(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.expression = input;
        self
    }
    /// <p>Describes how the results of the select job are serialized.</p>
    pub fn output_serialization(mut self, input: crate::types::OutputSerialization) -> Self {
        self.output_serialization = Some(input);
        self
    }
    /// <p>Describes how the results of the select job are serialized.</p>
    pub fn set_output_serialization(
        mut self,
        input: std::option::Option<crate::types::OutputSerialization>,
    ) -> Self {
        self.output_serialization = input;
        self
    }
    /// Consumes the builder and constructs a [`SelectParameters`](crate::types::SelectParameters).
    pub fn build(self) -> crate::types::SelectParameters {
        crate::types::SelectParameters {
            input_serialization: self.input_serialization,
            expression_type: self.expression_type,
            expression: self.expression,
            output_serialization: self.output_serialization,
        }
    }
}
