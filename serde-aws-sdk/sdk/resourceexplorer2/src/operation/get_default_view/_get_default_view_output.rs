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
pub struct GetDefaultViewOutput {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that is the current default for the Amazon Web Services Region in which you called this operation.</p>
    #[doc(hidden)]
    pub view_arn: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetDefaultViewOutput {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that is the current default for the Amazon Web Services Region in which you called this operation.</p>
    pub fn view_arn(&self) -> std::option::Option<&str> {
        self.view_arn.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetDefaultViewOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetDefaultViewOutput {
    /// Creates a new builder-style object to manufacture [`GetDefaultViewOutput`](crate::operation::get_default_view::GetDefaultViewOutput).
    pub fn builder() -> crate::operation::get_default_view::builders::GetDefaultViewOutputBuilder {
        crate::operation::get_default_view::builders::GetDefaultViewOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_default_view::GetDefaultViewOutput;
/// A builder for [`GetDefaultViewOutput`](crate::operation::get_default_view::GetDefaultViewOutput).
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
pub struct GetDefaultViewOutputBuilder {
    pub(crate) view_arn: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetDefaultViewOutputBuilder {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that is the current default for the Amazon Web Services Region in which you called this operation.</p>
    pub fn view_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.view_arn = Some(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that is the current default for the Amazon Web Services Region in which you called this operation.</p>
    pub fn set_view_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.view_arn = input;
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
    /// Consumes the builder and constructs a [`GetDefaultViewOutput`](crate::operation::get_default_view::GetDefaultViewOutput).
    pub fn build(self) -> crate::operation::get_default_view::GetDefaultViewOutput {
        crate::operation::get_default_view::GetDefaultViewOutput {
            view_arn: self.view_arn,
            _request_id: self._request_id,
        }
    }
}
