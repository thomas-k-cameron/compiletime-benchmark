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
pub struct DeleteBucketInventoryConfigurationInput {
    /// <p>The name of the bucket containing the inventory configuration to delete.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The ID used to identify the inventory configuration.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: std::option::Option<std::string::String>,
}
impl DeleteBucketInventoryConfigurationInput {
    /// <p>The name of the bucket containing the inventory configuration to delete.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The ID used to identify the inventory configuration.</p>
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl DeleteBucketInventoryConfigurationInput {
    /// Creates a new builder-style object to manufacture [`DeleteBucketInventoryConfigurationInput`](crate::operation::delete_bucket_inventory_configuration::DeleteBucketInventoryConfigurationInput).
    pub fn builder() -> crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationInputBuilder{
        crate::operation::delete_bucket_inventory_configuration::builders::DeleteBucketInventoryConfigurationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_bucket_inventory_configuration::DeleteBucketInventoryConfigurationInput;
/// A builder for [`DeleteBucketInventoryConfigurationInput`](crate::operation::delete_bucket_inventory_configuration::DeleteBucketInventoryConfigurationInput).
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
pub struct DeleteBucketInventoryConfigurationInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) id: std::option::Option<std::string::String>,
    pub(crate) expected_bucket_owner: std::option::Option<std::string::String>,
}
impl DeleteBucketInventoryConfigurationInputBuilder {
    /// <p>The name of the bucket containing the inventory configuration to delete.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The name of the bucket containing the inventory configuration to delete.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The ID used to identify the inventory configuration.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.id = Some(input.into());
        self
    }
    /// <p>The ID used to identify the inventory configuration.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl Into<std::string::String>) -> Self {
        self.expected_bucket_owner = Some(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.expected_bucket_owner = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteBucketInventoryConfigurationInput`](crate::operation::delete_bucket_inventory_configuration::DeleteBucketInventoryConfigurationInput).
    pub fn build(self) -> Result<crate::operation::delete_bucket_inventory_configuration::DeleteBucketInventoryConfigurationInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::delete_bucket_inventory_configuration::DeleteBucketInventoryConfigurationInput {
                bucket: self.bucket
                ,
                id: self.id
                ,
                expected_bucket_owner: self.expected_bucket_owner
                ,
            }
        )
    }
}
