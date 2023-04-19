// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the location in Amazon S3 where the select job results are stored.</p>
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
pub struct S3Location {
    /// <p>The name of the Amazon S3 bucket where the job results are stored.</p>
    #[doc(hidden)]
    pub bucket_name: std::option::Option<std::string::String>,
    /// <p>The prefix that is prepended to the results for this request.</p>
    #[doc(hidden)]
    pub prefix: std::option::Option<std::string::String>,
    /// <p>Contains information about the encryption used to store the job results in Amazon S3.</p>
    #[doc(hidden)]
    pub encryption: std::option::Option<crate::types::Encryption>,
    /// <p>The canned access control list (ACL) to apply to the job results.</p>
    #[doc(hidden)]
    pub canned_acl: std::option::Option<crate::types::CannedAcl>,
    /// <p>A list of grants that control access to the staged results.</p>
    #[doc(hidden)]
    pub access_control_list: std::option::Option<std::vec::Vec<crate::types::Grant>>,
    /// <p>The tag-set that is applied to the job results.</p>
    #[doc(hidden)]
    pub tagging:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>A map of metadata to store with the job results in Amazon S3.</p>
    #[doc(hidden)]
    pub user_metadata:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>The storage class used to store the job results.</p>
    #[doc(hidden)]
    pub storage_class: std::option::Option<crate::types::StorageClass>,
}
impl S3Location {
    /// <p>The name of the Amazon S3 bucket where the job results are stored.</p>
    pub fn bucket_name(&self) -> std::option::Option<&str> {
        self.bucket_name.as_deref()
    }
    /// <p>The prefix that is prepended to the results for this request.</p>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>Contains information about the encryption used to store the job results in Amazon S3.</p>
    pub fn encryption(&self) -> std::option::Option<&crate::types::Encryption> {
        self.encryption.as_ref()
    }
    /// <p>The canned access control list (ACL) to apply to the job results.</p>
    pub fn canned_acl(&self) -> std::option::Option<&crate::types::CannedAcl> {
        self.canned_acl.as_ref()
    }
    /// <p>A list of grants that control access to the staged results.</p>
    pub fn access_control_list(&self) -> std::option::Option<&[crate::types::Grant]> {
        self.access_control_list.as_deref()
    }
    /// <p>The tag-set that is applied to the job results.</p>
    pub fn tagging(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tagging.as_ref()
    }
    /// <p>A map of metadata to store with the job results in Amazon S3.</p>
    pub fn user_metadata(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.user_metadata.as_ref()
    }
    /// <p>The storage class used to store the job results.</p>
    pub fn storage_class(&self) -> std::option::Option<&crate::types::StorageClass> {
        self.storage_class.as_ref()
    }
}
impl S3Location {
    /// Creates a new builder-style object to manufacture [`S3Location`](crate::types::S3Location).
    pub fn builder() -> crate::types::builders::S3LocationBuilder {
        crate::types::builders::S3LocationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::S3Location;
/// A builder for [`S3Location`](crate::types::S3Location).
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
pub struct S3LocationBuilder {
    pub(crate) bucket_name: std::option::Option<std::string::String>,
    pub(crate) prefix: std::option::Option<std::string::String>,
    pub(crate) encryption: std::option::Option<crate::types::Encryption>,
    pub(crate) canned_acl: std::option::Option<crate::types::CannedAcl>,
    pub(crate) access_control_list: std::option::Option<std::vec::Vec<crate::types::Grant>>,
    pub(crate) tagging:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    pub(crate) user_metadata:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    pub(crate) storage_class: std::option::Option<crate::types::StorageClass>,
}
impl S3LocationBuilder {
    /// <p>The name of the Amazon S3 bucket where the job results are stored.</p>
    pub fn bucket_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket_name = Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket where the job results are stored.</p>
    pub fn set_bucket_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket_name = input;
        self
    }
    /// <p>The prefix that is prepended to the results for this request.</p>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix = Some(input.into());
        self
    }
    /// <p>The prefix that is prepended to the results for this request.</p>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>Contains information about the encryption used to store the job results in Amazon S3.</p>
    pub fn encryption(mut self, input: crate::types::Encryption) -> Self {
        self.encryption = Some(input);
        self
    }
    /// <p>Contains information about the encryption used to store the job results in Amazon S3.</p>
    pub fn set_encryption(mut self, input: std::option::Option<crate::types::Encryption>) -> Self {
        self.encryption = input;
        self
    }
    /// <p>The canned access control list (ACL) to apply to the job results.</p>
    pub fn canned_acl(mut self, input: crate::types::CannedAcl) -> Self {
        self.canned_acl = Some(input);
        self
    }
    /// <p>The canned access control list (ACL) to apply to the job results.</p>
    pub fn set_canned_acl(mut self, input: std::option::Option<crate::types::CannedAcl>) -> Self {
        self.canned_acl = input;
        self
    }
    /// Appends an item to `access_control_list`.
    ///
    /// To override the contents of this collection use [`set_access_control_list`](Self::set_access_control_list).
    ///
    /// <p>A list of grants that control access to the staged results.</p>
    pub fn access_control_list(mut self, input: crate::types::Grant) -> Self {
        let mut v = self.access_control_list.unwrap_or_default();
        v.push(input);
        self.access_control_list = Some(v);
        self
    }
    /// <p>A list of grants that control access to the staged results.</p>
    pub fn set_access_control_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Grant>>,
    ) -> Self {
        self.access_control_list = input;
        self
    }
    /// Adds a key-value pair to `tagging`.
    ///
    /// To override the contents of this collection use [`set_tagging`](Self::set_tagging).
    ///
    /// <p>The tag-set that is applied to the job results.</p>
    pub fn tagging(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.tagging.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tagging = Some(hash_map);
        self
    }
    /// <p>The tag-set that is applied to the job results.</p>
    pub fn set_tagging(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.tagging = input;
        self
    }
    /// Adds a key-value pair to `user_metadata`.
    ///
    /// To override the contents of this collection use [`set_user_metadata`](Self::set_user_metadata).
    ///
    /// <p>A map of metadata to store with the job results in Amazon S3.</p>
    pub fn user_metadata(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.user_metadata.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.user_metadata = Some(hash_map);
        self
    }
    /// <p>A map of metadata to store with the job results in Amazon S3.</p>
    pub fn set_user_metadata(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.user_metadata = input;
        self
    }
    /// <p>The storage class used to store the job results.</p>
    pub fn storage_class(mut self, input: crate::types::StorageClass) -> Self {
        self.storage_class = Some(input);
        self
    }
    /// <p>The storage class used to store the job results.</p>
    pub fn set_storage_class(
        mut self,
        input: std::option::Option<crate::types::StorageClass>,
    ) -> Self {
        self.storage_class = input;
        self
    }
    /// Consumes the builder and constructs a [`S3Location`](crate::types::S3Location).
    pub fn build(self) -> crate::types::S3Location {
        crate::types::S3Location {
            bucket_name: self.bucket_name,
            prefix: self.prefix,
            encryption: self.encryption,
            canned_acl: self.canned_acl,
            access_control_list: self.access_control_list,
            tagging: self.tagging,
            user_metadata: self.user_metadata,
            storage_class: self.storage_class,
        }
    }
}