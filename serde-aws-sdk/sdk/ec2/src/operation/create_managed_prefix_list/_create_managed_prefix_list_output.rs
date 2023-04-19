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
pub struct CreateManagedPrefixListOutput {
    /// <p>Information about the prefix list.</p>
    #[doc(hidden)]
    pub prefix_list: std::option::Option<crate::types::ManagedPrefixList>,
    _request_id: Option<String>,
}
impl CreateManagedPrefixListOutput {
    /// <p>Information about the prefix list.</p>
    pub fn prefix_list(&self) -> std::option::Option<&crate::types::ManagedPrefixList> {
        self.prefix_list.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateManagedPrefixListOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateManagedPrefixListOutput {
    /// Creates a new builder-style object to manufacture [`CreateManagedPrefixListOutput`](crate::operation::create_managed_prefix_list::CreateManagedPrefixListOutput).
    pub fn builder(
    ) -> crate::operation::create_managed_prefix_list::builders::CreateManagedPrefixListOutputBuilder
    {
        crate::operation::create_managed_prefix_list::builders::CreateManagedPrefixListOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_managed_prefix_list::CreateManagedPrefixListOutput;
/// A builder for [`CreateManagedPrefixListOutput`](crate::operation::create_managed_prefix_list::CreateManagedPrefixListOutput).
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
pub struct CreateManagedPrefixListOutputBuilder {
    pub(crate) prefix_list: std::option::Option<crate::types::ManagedPrefixList>,
    _request_id: Option<String>,
}
impl CreateManagedPrefixListOutputBuilder {
    /// <p>Information about the prefix list.</p>
    pub fn prefix_list(mut self, input: crate::types::ManagedPrefixList) -> Self {
        self.prefix_list = Some(input);
        self
    }
    /// <p>Information about the prefix list.</p>
    pub fn set_prefix_list(
        mut self,
        input: std::option::Option<crate::types::ManagedPrefixList>,
    ) -> Self {
        self.prefix_list = input;
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
    /// Consumes the builder and constructs a [`CreateManagedPrefixListOutput`](crate::operation::create_managed_prefix_list::CreateManagedPrefixListOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_managed_prefix_list::CreateManagedPrefixListOutput {
        crate::operation::create_managed_prefix_list::CreateManagedPrefixListOutput {
            prefix_list: self.prefix_list,
            _request_id: self._request_id,
        }
    }
}
