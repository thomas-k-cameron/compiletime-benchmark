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
pub struct CreateAccessPointInput {
    /// <p>The Amazon Web Services account ID for the account that owns the specified access point.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The name you want to assign to this access point.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The name of the bucket that you want to associate this access point with.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /bucket/
    /// <my-bucket-name></my-bucket-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded. </p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>If you include this field, Amazon S3 restricts access to this access point to requests from the specified virtual private cloud (VPC).</p> <note>
    /// <p>This is required for creating an access point for Amazon S3 on Outposts buckets.</p>
    /// </note>
    #[doc(hidden)]
    pub vpc_configuration: std::option::Option<crate::types::VpcConfiguration>,
    /// <p> The <code>PublicAccessBlock</code> configuration that you want to apply to the access point. </p>
    #[doc(hidden)]
    pub public_access_block_configuration:
        std::option::Option<crate::types::PublicAccessBlockConfiguration>,
    /// <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p>
    #[doc(hidden)]
    pub bucket_account_id: std::option::Option<std::string::String>,
}
impl CreateAccessPointInput {
    /// <p>The Amazon Web Services account ID for the account that owns the specified access point.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The name you want to assign to this access point.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The name of the bucket that you want to associate this access point with.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /bucket/
    /// <my-bucket-name></my-bucket-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded. </p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>If you include this field, Amazon S3 restricts access to this access point to requests from the specified virtual private cloud (VPC).</p> <note>
    /// <p>This is required for creating an access point for Amazon S3 on Outposts buckets.</p>
    /// </note>
    pub fn vpc_configuration(&self) -> std::option::Option<&crate::types::VpcConfiguration> {
        self.vpc_configuration.as_ref()
    }
    /// <p> The <code>PublicAccessBlock</code> configuration that you want to apply to the access point. </p>
    pub fn public_access_block_configuration(
        &self,
    ) -> std::option::Option<&crate::types::PublicAccessBlockConfiguration> {
        self.public_access_block_configuration.as_ref()
    }
    /// <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p>
    pub fn bucket_account_id(&self) -> std::option::Option<&str> {
        self.bucket_account_id.as_deref()
    }
}
impl CreateAccessPointInput {
    /// Creates a new builder-style object to manufacture [`CreateAccessPointInput`](crate::operation::create_access_point::CreateAccessPointInput).
    pub fn builder(
    ) -> crate::operation::create_access_point::builders::CreateAccessPointInputBuilder {
        crate::operation::create_access_point::builders::CreateAccessPointInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_access_point::CreateAccessPointInput;
/// A builder for [`CreateAccessPointInput`](crate::operation::create_access_point::CreateAccessPointInput).
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
pub struct CreateAccessPointInputBuilder {
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) vpc_configuration: std::option::Option<crate::types::VpcConfiguration>,
    pub(crate) public_access_block_configuration:
        std::option::Option<crate::types::PublicAccessBlockConfiguration>,
    pub(crate) bucket_account_id: std::option::Option<std::string::String>,
}
impl CreateAccessPointInputBuilder {
    /// <p>The Amazon Web Services account ID for the account that owns the specified access point.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the account that owns the specified access point.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The name you want to assign to this access point.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name you want to assign to this access point.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the bucket that you want to associate this access point with.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /bucket/
    /// <my-bucket-name></my-bucket-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded. </p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The name of the bucket that you want to associate this access point with.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /bucket/
    /// <my-bucket-name></my-bucket-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded. </p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>If you include this field, Amazon S3 restricts access to this access point to requests from the specified virtual private cloud (VPC).</p> <note>
    /// <p>This is required for creating an access point for Amazon S3 on Outposts buckets.</p>
    /// </note>
    pub fn vpc_configuration(mut self, input: crate::types::VpcConfiguration) -> Self {
        self.vpc_configuration = Some(input);
        self
    }
    /// <p>If you include this field, Amazon S3 restricts access to this access point to requests from the specified virtual private cloud (VPC).</p> <note>
    /// <p>This is required for creating an access point for Amazon S3 on Outposts buckets.</p>
    /// </note>
    pub fn set_vpc_configuration(
        mut self,
        input: std::option::Option<crate::types::VpcConfiguration>,
    ) -> Self {
        self.vpc_configuration = input;
        self
    }
    /// <p> The <code>PublicAccessBlock</code> configuration that you want to apply to the access point. </p>
    pub fn public_access_block_configuration(
        mut self,
        input: crate::types::PublicAccessBlockConfiguration,
    ) -> Self {
        self.public_access_block_configuration = Some(input);
        self
    }
    /// <p> The <code>PublicAccessBlock</code> configuration that you want to apply to the access point. </p>
    pub fn set_public_access_block_configuration(
        mut self,
        input: std::option::Option<crate::types::PublicAccessBlockConfiguration>,
    ) -> Self {
        self.public_access_block_configuration = input;
        self
    }
    /// <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p>
    pub fn bucket_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket_account_id = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p>
    pub fn set_bucket_account_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.bucket_account_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateAccessPointInput`](crate::operation::create_access_point::CreateAccessPointInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_access_point::CreateAccessPointInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_access_point::CreateAccessPointInput {
                account_id: self.account_id,
                name: self.name,
                bucket: self.bucket,
                vpc_configuration: self.vpc_configuration,
                public_access_block_configuration: self.public_access_block_configuration,
                bucket_account_id: self.bucket_account_id,
            },
        )
    }
}