// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The container element for an ownership control rule.</p>
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
pub struct OwnershipControlsRule {
    /// <p>The container element for object ownership for a bucket's ownership controls.</p>
    /// <p>BucketOwnerPreferred - Objects uploaded to the bucket change ownership to the bucket owner if the objects are uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p>ObjectWriter - The uploading account will own the object if the object is uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p>BucketOwnerEnforced - Access control lists (ACLs) are disabled and no longer affect permissions. The bucket owner automatically owns and has full control over every object in the bucket. The bucket only accepts PUT requests that don't specify an ACL or bucket owner full control ACLs, such as the <code>bucket-owner-full-control</code> canned ACL or an equivalent form of this ACL expressed in the XML format.</p>
    #[doc(hidden)]
    pub object_ownership: std::option::Option<crate::types::ObjectOwnership>,
}
impl OwnershipControlsRule {
    /// <p>The container element for object ownership for a bucket's ownership controls.</p>
    /// <p>BucketOwnerPreferred - Objects uploaded to the bucket change ownership to the bucket owner if the objects are uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p>ObjectWriter - The uploading account will own the object if the object is uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p>BucketOwnerEnforced - Access control lists (ACLs) are disabled and no longer affect permissions. The bucket owner automatically owns and has full control over every object in the bucket. The bucket only accepts PUT requests that don't specify an ACL or bucket owner full control ACLs, such as the <code>bucket-owner-full-control</code> canned ACL or an equivalent form of this ACL expressed in the XML format.</p>
    pub fn object_ownership(&self) -> std::option::Option<&crate::types::ObjectOwnership> {
        self.object_ownership.as_ref()
    }
}
impl OwnershipControlsRule {
    /// Creates a new builder-style object to manufacture [`OwnershipControlsRule`](crate::types::OwnershipControlsRule).
    pub fn builder() -> crate::types::builders::OwnershipControlsRuleBuilder {
        crate::types::builders::OwnershipControlsRuleBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::OwnershipControlsRule;
/// A builder for [`OwnershipControlsRule`](crate::types::OwnershipControlsRule).
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
pub struct OwnershipControlsRuleBuilder {
    pub(crate) object_ownership: std::option::Option<crate::types::ObjectOwnership>,
}
impl OwnershipControlsRuleBuilder {
    /// <p>The container element for object ownership for a bucket's ownership controls.</p>
    /// <p>BucketOwnerPreferred - Objects uploaded to the bucket change ownership to the bucket owner if the objects are uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p>ObjectWriter - The uploading account will own the object if the object is uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p>BucketOwnerEnforced - Access control lists (ACLs) are disabled and no longer affect permissions. The bucket owner automatically owns and has full control over every object in the bucket. The bucket only accepts PUT requests that don't specify an ACL or bucket owner full control ACLs, such as the <code>bucket-owner-full-control</code> canned ACL or an equivalent form of this ACL expressed in the XML format.</p>
    pub fn object_ownership(mut self, input: crate::types::ObjectOwnership) -> Self {
        self.object_ownership = Some(input);
        self
    }
    /// <p>The container element for object ownership for a bucket's ownership controls.</p>
    /// <p>BucketOwnerPreferred - Objects uploaded to the bucket change ownership to the bucket owner if the objects are uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p>ObjectWriter - The uploading account will own the object if the object is uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p>BucketOwnerEnforced - Access control lists (ACLs) are disabled and no longer affect permissions. The bucket owner automatically owns and has full control over every object in the bucket. The bucket only accepts PUT requests that don't specify an ACL or bucket owner full control ACLs, such as the <code>bucket-owner-full-control</code> canned ACL or an equivalent form of this ACL expressed in the XML format.</p>
    pub fn set_object_ownership(
        mut self,
        input: std::option::Option<crate::types::ObjectOwnership>,
    ) -> Self {
        self.object_ownership = input;
        self
    }
    /// Consumes the builder and constructs a [`OwnershipControlsRule`](crate::types::OwnershipControlsRule).
    pub fn build(self) -> crate::types::OwnershipControlsRule {
        crate::types::OwnershipControlsRule {
            object_ownership: self.object_ownership,
        }
    }
}
