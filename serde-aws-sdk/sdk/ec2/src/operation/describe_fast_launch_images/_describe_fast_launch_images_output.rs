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
pub struct DescribeFastLaunchImagesOutput {
    /// <p>A collection of details about the fast-launch enabled Windows images that meet the requested criteria.</p>
    #[doc(hidden)]
    pub fast_launch_images:
        std::option::Option<std::vec::Vec<crate::types::DescribeFastLaunchImagesSuccessItem>>,
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeFastLaunchImagesOutput {
    /// <p>A collection of details about the fast-launch enabled Windows images that meet the requested criteria.</p>
    pub fn fast_launch_images(
        &self,
    ) -> std::option::Option<&[crate::types::DescribeFastLaunchImagesSuccessItem]> {
        self.fast_launch_images.as_deref()
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeFastLaunchImagesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeFastLaunchImagesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeFastLaunchImagesOutput`](crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesOutput).
    pub fn builder() -> crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesOutputBuilder{
        crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesOutput;
/// A builder for [`DescribeFastLaunchImagesOutput`](crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesOutput).
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
pub struct DescribeFastLaunchImagesOutputBuilder {
    pub(crate) fast_launch_images:
        std::option::Option<std::vec::Vec<crate::types::DescribeFastLaunchImagesSuccessItem>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeFastLaunchImagesOutputBuilder {
    /// Appends an item to `fast_launch_images`.
    ///
    /// To override the contents of this collection use [`set_fast_launch_images`](Self::set_fast_launch_images).
    ///
    /// <p>A collection of details about the fast-launch enabled Windows images that meet the requested criteria.</p>
    pub fn fast_launch_images(
        mut self,
        input: crate::types::DescribeFastLaunchImagesSuccessItem,
    ) -> Self {
        let mut v = self.fast_launch_images.unwrap_or_default();
        v.push(input);
        self.fast_launch_images = Some(v);
        self
    }
    /// <p>A collection of details about the fast-launch enabled Windows images that meet the requested criteria.</p>
    pub fn set_fast_launch_images(
        mut self,
        input: std::option::Option<
            std::vec::Vec<crate::types::DescribeFastLaunchImagesSuccessItem>,
        >,
    ) -> Self {
        self.fast_launch_images = input;
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`DescribeFastLaunchImagesOutput`](crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesOutput {
        crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesOutput {
            fast_launch_images: self.fast_launch_images,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}