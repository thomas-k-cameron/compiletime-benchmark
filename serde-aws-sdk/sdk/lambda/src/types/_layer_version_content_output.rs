// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">Lambda layer</a>.</p>
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
pub struct LayerVersionContentOutput {
    /// <p>A link to the layer archive in Amazon S3 that is valid for 10 minutes.</p>
    #[doc(hidden)]
    pub location: std::option::Option<std::string::String>,
    /// <p>The SHA-256 hash of the layer archive.</p>
    #[doc(hidden)]
    pub code_sha256: std::option::Option<std::string::String>,
    /// <p>The size of the layer archive in bytes.</p>
    #[doc(hidden)]
    pub code_size: i64,
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    #[doc(hidden)]
    pub signing_profile_version_arn: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    #[doc(hidden)]
    pub signing_job_arn: std::option::Option<std::string::String>,
}
impl LayerVersionContentOutput {
    /// <p>A link to the layer archive in Amazon S3 that is valid for 10 minutes.</p>
    pub fn location(&self) -> std::option::Option<&str> {
        self.location.as_deref()
    }
    /// <p>The SHA-256 hash of the layer archive.</p>
    pub fn code_sha256(&self) -> std::option::Option<&str> {
        self.code_sha256.as_deref()
    }
    /// <p>The size of the layer archive in bytes.</p>
    pub fn code_size(&self) -> i64 {
        self.code_size
    }
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    pub fn signing_profile_version_arn(&self) -> std::option::Option<&str> {
        self.signing_profile_version_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    pub fn signing_job_arn(&self) -> std::option::Option<&str> {
        self.signing_job_arn.as_deref()
    }
}
impl LayerVersionContentOutput {
    /// Creates a new builder-style object to manufacture [`LayerVersionContentOutput`](crate::types::LayerVersionContentOutput).
    pub fn builder() -> crate::types::builders::LayerVersionContentOutputBuilder {
        crate::types::builders::LayerVersionContentOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LayerVersionContentOutput;
/// A builder for [`LayerVersionContentOutput`](crate::types::LayerVersionContentOutput).
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
pub struct LayerVersionContentOutputBuilder {
    pub(crate) location: std::option::Option<std::string::String>,
    pub(crate) code_sha256: std::option::Option<std::string::String>,
    pub(crate) code_size: std::option::Option<i64>,
    pub(crate) signing_profile_version_arn: std::option::Option<std::string::String>,
    pub(crate) signing_job_arn: std::option::Option<std::string::String>,
}
impl LayerVersionContentOutputBuilder {
    /// <p>A link to the layer archive in Amazon S3 that is valid for 10 minutes.</p>
    pub fn location(mut self, input: impl Into<std::string::String>) -> Self {
        self.location = Some(input.into());
        self
    }
    /// <p>A link to the layer archive in Amazon S3 that is valid for 10 minutes.</p>
    pub fn set_location(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.location = input;
        self
    }
    /// <p>The SHA-256 hash of the layer archive.</p>
    pub fn code_sha256(mut self, input: impl Into<std::string::String>) -> Self {
        self.code_sha256 = Some(input.into());
        self
    }
    /// <p>The SHA-256 hash of the layer archive.</p>
    pub fn set_code_sha256(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.code_sha256 = input;
        self
    }
    /// <p>The size of the layer archive in bytes.</p>
    pub fn code_size(mut self, input: i64) -> Self {
        self.code_size = Some(input);
        self
    }
    /// <p>The size of the layer archive in bytes.</p>
    pub fn set_code_size(mut self, input: std::option::Option<i64>) -> Self {
        self.code_size = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    pub fn signing_profile_version_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.signing_profile_version_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    pub fn set_signing_profile_version_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.signing_profile_version_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    pub fn signing_job_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.signing_job_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    pub fn set_signing_job_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.signing_job_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`LayerVersionContentOutput`](crate::types::LayerVersionContentOutput).
    pub fn build(self) -> crate::types::LayerVersionContentOutput {
        crate::types::LayerVersionContentOutput {
            location: self.location,
            code_sha256: self.code_sha256,
            code_size: self.code_size.unwrap_or_default(),
            signing_profile_version_arn: self.signing_profile_version_arn,
            signing_job_arn: self.signing_job_arn,
        }
    }
}
