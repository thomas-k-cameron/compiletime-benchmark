// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output of a <code>DescribeTable</code> operation.</p>
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
pub struct DescribeTableOutput {
    /// <p>The properties of the table.</p>
    #[doc(hidden)]
    pub table: std::option::Option<crate::types::TableDescription>,
    _request_id: Option<String>,
}
impl DescribeTableOutput {
    /// <p>The properties of the table.</p>
    pub fn table(&self) -> std::option::Option<&crate::types::TableDescription> {
        self.table.as_ref()
    }
}
impl aws_http::request_id::RequestId for DescribeTableOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeTableOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTableOutput`](crate::operation::describe_table::DescribeTableOutput).
    pub fn builder() -> crate::operation::describe_table::builders::DescribeTableOutputBuilder {
        crate::operation::describe_table::builders::DescribeTableOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_table::DescribeTableOutput;
/// A builder for [`DescribeTableOutput`](crate::operation::describe_table::DescribeTableOutput).
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
pub struct DescribeTableOutputBuilder {
    pub(crate) table: std::option::Option<crate::types::TableDescription>,
    _request_id: Option<String>,
}
impl DescribeTableOutputBuilder {
    /// <p>The properties of the table.</p>
    pub fn table(mut self, input: crate::types::TableDescription) -> Self {
        self.table = Some(input);
        self
    }
    /// <p>The properties of the table.</p>
    pub fn set_table(mut self, input: std::option::Option<crate::types::TableDescription>) -> Self {
        self.table = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeTableOutput`](crate::operation::describe_table::DescribeTableOutput).
    pub fn build(self) -> crate::operation::describe_table::DescribeTableOutput {
        crate::operation::describe_table::DescribeTableOutput {
            table: self.table,
            _request_id: self._request_id,
        }
    }
}
