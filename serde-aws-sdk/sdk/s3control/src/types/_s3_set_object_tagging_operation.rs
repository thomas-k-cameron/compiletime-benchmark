// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the configuration parameters for a PUT Object Tagging operation. S3 Batch Operations passes every object to the underlying <code>PutObjectTagging</code> API operation. For more information about the parameters for this operation, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectPUTtagging.html">PutObjectTagging</a>.</p>
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
pub struct S3SetObjectTaggingOperation {
    /// <p></p>
    #[doc(hidden)]
    pub tag_set: std::option::Option<std::vec::Vec<crate::types::S3Tag>>,
}
impl S3SetObjectTaggingOperation {
    /// <p></p>
    pub fn tag_set(&self) -> std::option::Option<&[crate::types::S3Tag]> {
        self.tag_set.as_deref()
    }
}
impl S3SetObjectTaggingOperation {
    /// Creates a new builder-style object to manufacture [`S3SetObjectTaggingOperation`](crate::types::S3SetObjectTaggingOperation).
    pub fn builder() -> crate::types::builders::S3SetObjectTaggingOperationBuilder {
        crate::types::builders::S3SetObjectTaggingOperationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::S3SetObjectTaggingOperation;
/// A builder for [`S3SetObjectTaggingOperation`](crate::types::S3SetObjectTaggingOperation).
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
pub struct S3SetObjectTaggingOperationBuilder {
    pub(crate) tag_set: std::option::Option<std::vec::Vec<crate::types::S3Tag>>,
}
impl S3SetObjectTaggingOperationBuilder {
    /// Appends an item to `tag_set`.
    ///
    /// To override the contents of this collection use [`set_tag_set`](Self::set_tag_set).
    ///
    /// <p></p>
    pub fn tag_set(mut self, input: crate::types::S3Tag) -> Self {
        let mut v = self.tag_set.unwrap_or_default();
        v.push(input);
        self.tag_set = Some(v);
        self
    }
    /// <p></p>
    pub fn set_tag_set(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::S3Tag>>,
    ) -> Self {
        self.tag_set = input;
        self
    }
    /// Consumes the builder and constructs a [`S3SetObjectTaggingOperation`](crate::types::S3SetObjectTaggingOperation).
    pub fn build(self) -> crate::types::S3SetObjectTaggingOperation {
        crate::types::S3SetObjectTaggingOperation {
            tag_set: self.tag_set,
        }
    }
}
