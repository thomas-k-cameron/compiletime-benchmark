// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the information required to locate a manifest object.</p>
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
pub struct JobManifestLocation {
    /// <p>The Amazon Resource Name (ARN) for a manifest object.</p> <important>
    /// <p>When you're using XML requests, you must replace special characters (such as carriage returns) in object keys with their equivalent XML entity codes. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML-related object key constraints</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    #[doc(hidden)]
    pub object_arn: std::option::Option<std::string::String>,
    /// <p>The optional version ID to identify a specific version of the manifest object.</p>
    #[doc(hidden)]
    pub object_version_id: std::option::Option<std::string::String>,
    /// <p>The ETag for the specified manifest object.</p>
    #[doc(hidden)]
    pub e_tag: std::option::Option<std::string::String>,
}
impl JobManifestLocation {
    /// <p>The Amazon Resource Name (ARN) for a manifest object.</p> <important>
    /// <p>When you're using XML requests, you must replace special characters (such as carriage returns) in object keys with their equivalent XML entity codes. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML-related object key constraints</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    pub fn object_arn(&self) -> std::option::Option<&str> {
        self.object_arn.as_deref()
    }
    /// <p>The optional version ID to identify a specific version of the manifest object.</p>
    pub fn object_version_id(&self) -> std::option::Option<&str> {
        self.object_version_id.as_deref()
    }
    /// <p>The ETag for the specified manifest object.</p>
    pub fn e_tag(&self) -> std::option::Option<&str> {
        self.e_tag.as_deref()
    }
}
impl JobManifestLocation {
    /// Creates a new builder-style object to manufacture [`JobManifestLocation`](crate::types::JobManifestLocation).
    pub fn builder() -> crate::types::builders::JobManifestLocationBuilder {
        crate::types::builders::JobManifestLocationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::JobManifestLocation;
/// A builder for [`JobManifestLocation`](crate::types::JobManifestLocation).
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
pub struct JobManifestLocationBuilder {
    pub(crate) object_arn: std::option::Option<std::string::String>,
    pub(crate) object_version_id: std::option::Option<std::string::String>,
    pub(crate) e_tag: std::option::Option<std::string::String>,
}
impl JobManifestLocationBuilder {
    /// <p>The Amazon Resource Name (ARN) for a manifest object.</p> <important>
    /// <p>When you're using XML requests, you must replace special characters (such as carriage returns) in object keys with their equivalent XML entity codes. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML-related object key constraints</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    pub fn object_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.object_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for a manifest object.</p> <important>
    /// <p>When you're using XML requests, you must replace special characters (such as carriage returns) in object keys with their equivalent XML entity codes. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML-related object key constraints</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// </important>
    pub fn set_object_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.object_arn = input;
        self
    }
    /// <p>The optional version ID to identify a specific version of the manifest object.</p>
    pub fn object_version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.object_version_id = Some(input.into());
        self
    }
    /// <p>The optional version ID to identify a specific version of the manifest object.</p>
    pub fn set_object_version_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.object_version_id = input;
        self
    }
    /// <p>The ETag for the specified manifest object.</p>
    pub fn e_tag(mut self, input: impl Into<std::string::String>) -> Self {
        self.e_tag = Some(input.into());
        self
    }
    /// <p>The ETag for the specified manifest object.</p>
    pub fn set_e_tag(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.e_tag = input;
        self
    }
    /// Consumes the builder and constructs a [`JobManifestLocation`](crate::types::JobManifestLocation).
    pub fn build(self) -> crate::types::JobManifestLocation {
        crate::types::JobManifestLocation {
            object_arn: self.object_arn,
            object_version_id: self.object_version_id,
            e_tag: self.e_tag,
        }
    }
}