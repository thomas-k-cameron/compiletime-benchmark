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
pub struct DeleteEventSourceMappingInput {
    /// <p>The identifier of the event source mapping.</p>
    #[doc(hidden)]
    pub uuid: std::option::Option<std::string::String>,
}
impl DeleteEventSourceMappingInput {
    /// <p>The identifier of the event source mapping.</p>
    pub fn uuid(&self) -> std::option::Option<&str> {
        self.uuid.as_deref()
    }
}
impl DeleteEventSourceMappingInput {
    /// Creates a new builder-style object to manufacture [`DeleteEventSourceMappingInput`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingInput).
    pub fn builder(
    ) -> crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingInputBuilder
    {
        crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_event_source_mapping::DeleteEventSourceMappingInput;
/// A builder for [`DeleteEventSourceMappingInput`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingInput).
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
pub struct DeleteEventSourceMappingInputBuilder {
    pub(crate) uuid: std::option::Option<std::string::String>,
}
impl DeleteEventSourceMappingInputBuilder {
    /// <p>The identifier of the event source mapping.</p>
    pub fn uuid(mut self, input: impl Into<std::string::String>) -> Self {
        self.uuid = Some(input.into());
        self
    }
    /// <p>The identifier of the event source mapping.</p>
    pub fn set_uuid(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.uuid = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteEventSourceMappingInput`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_event_source_mapping::DeleteEventSourceMappingInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_event_source_mapping::DeleteEventSourceMappingInput {
                uuid: self.uuid,
            },
        )
    }
}
