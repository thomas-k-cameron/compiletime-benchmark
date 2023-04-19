// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input for a GetChange request.</p>
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
pub struct GetChangeInput {
    /// <p>The ID of the change batch request. The value that you specify here is the value that <code>ChangeResourceRecordSets</code> returned in the <code>Id</code> element when you submitted the request.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
}
impl GetChangeInput {
    /// <p>The ID of the change batch request. The value that you specify here is the value that <code>ChangeResourceRecordSets</code> returned in the <code>Id</code> element when you submitted the request.</p>
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl GetChangeInput {
    /// Creates a new builder-style object to manufacture [`GetChangeInput`](crate::operation::get_change::GetChangeInput).
    pub fn builder() -> crate::operation::get_change::builders::GetChangeInputBuilder {
        crate::operation::get_change::builders::GetChangeInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_change::GetChangeInput;
/// A builder for [`GetChangeInput`](crate::operation::get_change::GetChangeInput).
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
pub struct GetChangeInputBuilder {
    pub(crate) id: std::option::Option<std::string::String>,
}
impl GetChangeInputBuilder {
    /// <p>The ID of the change batch request. The value that you specify here is the value that <code>ChangeResourceRecordSets</code> returned in the <code>Id</code> element when you submitted the request.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.id = Some(input.into());
        self
    }
    /// <p>The ID of the change batch request. The value that you specify here is the value that <code>ChangeResourceRecordSets</code> returned in the <code>Id</code> element when you submitted the request.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetChangeInput`](crate::operation::get_change::GetChangeInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_change::GetChangeInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_change::GetChangeInput { id: self.id })
    }
}
