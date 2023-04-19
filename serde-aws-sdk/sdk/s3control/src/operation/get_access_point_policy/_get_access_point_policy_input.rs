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
pub struct GetAccessPointPolicyInput {
    /// <p>The account ID for the account that owns the specified access point.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The name of the access point whose policy you want to retrieve.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the access point accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /accesspoint/
    /// <my-accesspoint-name></my-accesspoint-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the access point <code>reports-ap</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/accesspoint/reports-ap</code>. The value must be URL encoded. </p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
}
impl GetAccessPointPolicyInput {
    /// <p>The account ID for the account that owns the specified access point.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The name of the access point whose policy you want to retrieve.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the access point accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /accesspoint/
    /// <my-accesspoint-name></my-accesspoint-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the access point <code>reports-ap</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/accesspoint/reports-ap</code>. The value must be URL encoded. </p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl GetAccessPointPolicyInput {
    /// Creates a new builder-style object to manufacture [`GetAccessPointPolicyInput`](crate::operation::get_access_point_policy::GetAccessPointPolicyInput).
    pub fn builder(
    ) -> crate::operation::get_access_point_policy::builders::GetAccessPointPolicyInputBuilder {
        crate::operation::get_access_point_policy::builders::GetAccessPointPolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_access_point_policy::GetAccessPointPolicyInput;
/// A builder for [`GetAccessPointPolicyInput`](crate::operation::get_access_point_policy::GetAccessPointPolicyInput).
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
pub struct GetAccessPointPolicyInputBuilder {
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
}
impl GetAccessPointPolicyInputBuilder {
    /// <p>The account ID for the account that owns the specified access point.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The account ID for the account that owns the specified access point.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The name of the access point whose policy you want to retrieve.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the access point accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /accesspoint/
    /// <my-accesspoint-name></my-accesspoint-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the access point <code>reports-ap</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/accesspoint/reports-ap</code>. The value must be URL encoded. </p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the access point whose policy you want to retrieve.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the access point accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /accesspoint/
    /// <my-accesspoint-name></my-accesspoint-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the access point <code>reports-ap</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/accesspoint/reports-ap</code>. The value must be URL encoded. </p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetAccessPointPolicyInput`](crate::operation::get_access_point_policy::GetAccessPointPolicyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_access_point_policy::GetAccessPointPolicyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_access_point_policy::GetAccessPointPolicyInput {
                account_id: self.account_id,
                name: self.name,
            },
        )
    }
}
