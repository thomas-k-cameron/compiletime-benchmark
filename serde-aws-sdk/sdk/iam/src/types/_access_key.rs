// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an Amazon Web Services access key.</p>
/// <p> This data type is used as a response element in the <code>CreateAccessKey</code> and <code>ListAccessKeys</code> operations. </p> <note>
/// <p>The <code>SecretAccessKey</code> value is returned only in response to <code>CreateAccessKey</code>. You can get a secret access key only when you first create an access key; you cannot recover the secret access key later. If you lose a secret access key, you must create a new access key.</p>
/// </note>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AccessKey {
    /// <p>The name of the IAM user that the access key is associated with.</p>
    #[doc(hidden)]
    pub user_name: std::option::Option<std::string::String>,
    /// <p>The ID for this access key.</p>
    #[doc(hidden)]
    pub access_key_id: std::option::Option<std::string::String>,
    /// <p>The status of the access key. <code>Active</code> means that the key is valid for API calls, while <code>Inactive</code> means it is not. </p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::StatusType>,
    /// <p>The secret key used to sign requests.</p>
    #[doc(hidden)]
    pub secret_access_key: std::option::Option<std::string::String>,
    /// <p>The date when the access key was created.</p>
    #[doc(hidden)]
    pub create_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl AccessKey {
    /// <p>The name of the IAM user that the access key is associated with.</p>
    pub fn user_name(&self) -> std::option::Option<&str> {
        self.user_name.as_deref()
    }
    /// <p>The ID for this access key.</p>
    pub fn access_key_id(&self) -> std::option::Option<&str> {
        self.access_key_id.as_deref()
    }
    /// <p>The status of the access key. <code>Active</code> means that the key is valid for API calls, while <code>Inactive</code> means it is not. </p>
    pub fn status(&self) -> std::option::Option<&crate::types::StatusType> {
        self.status.as_ref()
    }
    /// <p>The secret key used to sign requests.</p>
    pub fn secret_access_key(&self) -> std::option::Option<&str> {
        self.secret_access_key.as_deref()
    }
    /// <p>The date when the access key was created.</p>
    pub fn create_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.create_date.as_ref()
    }
}
impl std::fmt::Debug for AccessKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AccessKey");
        formatter.field("user_name", &self.user_name);
        formatter.field("access_key_id", &self.access_key_id);
        formatter.field("status", &self.status);
        formatter.field("secret_access_key", &"*** Sensitive Data Redacted ***");
        formatter.field("create_date", &self.create_date);
        formatter.finish()
    }
}
impl AccessKey {
    /// Creates a new builder-style object to manufacture [`AccessKey`](crate::types::AccessKey).
    pub fn builder() -> crate::types::builders::AccessKeyBuilder {
        crate::types::builders::AccessKeyBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AccessKey;
/// A builder for [`AccessKey`](crate::types::AccessKey).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct AccessKeyBuilder {
    pub(crate) user_name: std::option::Option<std::string::String>,
    pub(crate) access_key_id: std::option::Option<std::string::String>,
    pub(crate) status: std::option::Option<crate::types::StatusType>,
    pub(crate) secret_access_key: std::option::Option<std::string::String>,
    pub(crate) create_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl AccessKeyBuilder {
    /// <p>The name of the IAM user that the access key is associated with.</p>
    pub fn user_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.user_name = Some(input.into());
        self
    }
    /// <p>The name of the IAM user that the access key is associated with.</p>
    pub fn set_user_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.user_name = input;
        self
    }
    /// <p>The ID for this access key.</p>
    pub fn access_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.access_key_id = Some(input.into());
        self
    }
    /// <p>The ID for this access key.</p>
    pub fn set_access_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.access_key_id = input;
        self
    }
    /// <p>The status of the access key. <code>Active</code> means that the key is valid for API calls, while <code>Inactive</code> means it is not. </p>
    pub fn status(mut self, input: crate::types::StatusType) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The status of the access key. <code>Active</code> means that the key is valid for API calls, while <code>Inactive</code> means it is not. </p>
    pub fn set_status(mut self, input: std::option::Option<crate::types::StatusType>) -> Self {
        self.status = input;
        self
    }
    /// <p>The secret key used to sign requests.</p>
    pub fn secret_access_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.secret_access_key = Some(input.into());
        self
    }
    /// <p>The secret key used to sign requests.</p>
    pub fn set_secret_access_key(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.secret_access_key = input;
        self
    }
    /// <p>The date when the access key was created.</p>
    pub fn create_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.create_date = Some(input);
        self
    }
    /// <p>The date when the access key was created.</p>
    pub fn set_create_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_date = input;
        self
    }
    /// Consumes the builder and constructs a [`AccessKey`](crate::types::AccessKey).
    pub fn build(self) -> crate::types::AccessKey {
        crate::types::AccessKey {
            user_name: self.user_name,
            access_key_id: self.access_key_id,
            status: self.status,
            secret_access_key: self.secret_access_key,
            create_date: self.create_date,
        }
    }
}
impl std::fmt::Debug for AccessKeyBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AccessKeyBuilder");
        formatter.field("user_name", &self.user_name);
        formatter.field("access_key_id", &self.access_key_id);
        formatter.field("status", &self.status);
        formatter.field("secret_access_key", &"*** Sensitive Data Redacted ***");
        formatter.field("create_date", &self.create_date);
        formatter.finish()
    }
}
