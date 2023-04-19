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
pub struct ListBucketIntelligentTieringConfigurationsInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The <code>ContinuationToken</code> that represents a placeholder from where this request should begin.</p>
    #[doc(hidden)]
    pub continuation_token: std::option::Option<std::string::String>,
}
impl ListBucketIntelligentTieringConfigurationsInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The <code>ContinuationToken</code> that represents a placeholder from where this request should begin.</p>
    pub fn continuation_token(&self) -> std::option::Option<&str> {
        self.continuation_token.as_deref()
    }
}
impl ListBucketIntelligentTieringConfigurationsInput {
    /// Creates a new builder-style object to manufacture [`ListBucketIntelligentTieringConfigurationsInput`](crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurationsInput).
    pub fn builder() -> crate::operation::list_bucket_intelligent_tiering_configurations::builders::ListBucketIntelligentTieringConfigurationsInputBuilder{
        crate::operation::list_bucket_intelligent_tiering_configurations::builders::ListBucketIntelligentTieringConfigurationsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurationsInput;
/// A builder for [`ListBucketIntelligentTieringConfigurationsInput`](crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurationsInput).
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
pub struct ListBucketIntelligentTieringConfigurationsInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) continuation_token: std::option::Option<std::string::String>,
}
impl ListBucketIntelligentTieringConfigurationsInputBuilder {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The <code>ContinuationToken</code> that represents a placeholder from where this request should begin.</p>
    pub fn continuation_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.continuation_token = Some(input.into());
        self
    }
    /// <p>The <code>ContinuationToken</code> that represents a placeholder from where this request should begin.</p>
    pub fn set_continuation_token(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.continuation_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListBucketIntelligentTieringConfigurationsInput`](crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurationsInput).
    pub fn build(self) -> Result<crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurationsInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::list_bucket_intelligent_tiering_configurations::ListBucketIntelligentTieringConfigurationsInput {
                bucket: self.bucket
                ,
                continuation_token: self.continuation_token
                ,
            }
        )
    }
}
