// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an IAM role, including all of the role's policies.</p>
/// <p>This data type is used as a response element in the <code>GetAccountAuthorizationDetails</code> operation.</p>
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
pub struct RoleDetail {
    /// <p>The path to the role. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub path: std::option::Option<std::string::String>,
    /// <p>The friendly name that identifies the role.</p>
    #[doc(hidden)]
    pub role_name: std::option::Option<std::string::String>,
    /// <p>The stable and unique string identifying the role. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub role_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the role was created.</p>
    #[doc(hidden)]
    pub create_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The trust policy that grants permission to assume the role.</p>
    #[doc(hidden)]
    pub assume_role_policy_document: std::option::Option<std::string::String>,
    /// <p>A list of instance profiles that contain this role.</p>
    #[doc(hidden)]
    pub instance_profile_list: std::option::Option<std::vec::Vec<crate::types::InstanceProfile>>,
    /// <p>A list of inline policies embedded in the role. These policies are the role's access (permissions) policies.</p>
    #[doc(hidden)]
    pub role_policy_list: std::option::Option<std::vec::Vec<crate::types::PolicyDetail>>,
    /// <p>A list of managed policies attached to the role. These policies are the role's access (permissions) policies.</p>
    #[doc(hidden)]
    pub attached_managed_policies: std::option::Option<std::vec::Vec<crate::types::AttachedPolicy>>,
    /// <p>The ARN of the policy used to set the permissions boundary for the role.</p>
    /// <p>For more information about permissions boundaries, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries for IAM identities </a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub permissions_boundary: std::option::Option<crate::types::AttachedPermissionsBoundary>,
    /// <p>A list of tags that are attached to the role. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    /// <p>Contains information about the last time that an IAM role was used. This includes the date and time and the Region in which the role was last used. Activity is only reported for the trailing 400 days. This period can be shorter if your Region began supporting these features within the last year. The role might have been used more than 400 days ago. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period">Regions where data is tracked</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub role_last_used: std::option::Option<crate::types::RoleLastUsed>,
}
impl RoleDetail {
    /// <p>The path to the role. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn path(&self) -> std::option::Option<&str> {
        self.path.as_deref()
    }
    /// <p>The friendly name that identifies the role.</p>
    pub fn role_name(&self) -> std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>The stable and unique string identifying the role. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn role_id(&self) -> std::option::Option<&str> {
        self.role_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    pub fn arn(&self) -> std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the role was created.</p>
    pub fn create_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.create_date.as_ref()
    }
    /// <p>The trust policy that grants permission to assume the role.</p>
    pub fn assume_role_policy_document(&self) -> std::option::Option<&str> {
        self.assume_role_policy_document.as_deref()
    }
    /// <p>A list of instance profiles that contain this role.</p>
    pub fn instance_profile_list(&self) -> std::option::Option<&[crate::types::InstanceProfile]> {
        self.instance_profile_list.as_deref()
    }
    /// <p>A list of inline policies embedded in the role. These policies are the role's access (permissions) policies.</p>
    pub fn role_policy_list(&self) -> std::option::Option<&[crate::types::PolicyDetail]> {
        self.role_policy_list.as_deref()
    }
    /// <p>A list of managed policies attached to the role. These policies are the role's access (permissions) policies.</p>
    pub fn attached_managed_policies(
        &self,
    ) -> std::option::Option<&[crate::types::AttachedPolicy]> {
        self.attached_managed_policies.as_deref()
    }
    /// <p>The ARN of the policy used to set the permissions boundary for the role.</p>
    /// <p>For more information about permissions boundaries, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries for IAM identities </a> in the <i>IAM User Guide</i>.</p>
    pub fn permissions_boundary(
        &self,
    ) -> std::option::Option<&crate::types::AttachedPermissionsBoundary> {
        self.permissions_boundary.as_ref()
    }
    /// <p>A list of tags that are attached to the role. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>Contains information about the last time that an IAM role was used. This includes the date and time and the Region in which the role was last used. Activity is only reported for the trailing 400 days. This period can be shorter if your Region began supporting these features within the last year. The role might have been used more than 400 days ago. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period">Regions where data is tracked</a> in the <i>IAM User Guide</i>.</p>
    pub fn role_last_used(&self) -> std::option::Option<&crate::types::RoleLastUsed> {
        self.role_last_used.as_ref()
    }
}
impl RoleDetail {
    /// Creates a new builder-style object to manufacture [`RoleDetail`](crate::types::RoleDetail).
    pub fn builder() -> crate::types::builders::RoleDetailBuilder {
        crate::types::builders::RoleDetailBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::RoleDetail;
/// A builder for [`RoleDetail`](crate::types::RoleDetail).
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
pub struct RoleDetailBuilder {
    pub(crate) path: std::option::Option<std::string::String>,
    pub(crate) role_name: std::option::Option<std::string::String>,
    pub(crate) role_id: std::option::Option<std::string::String>,
    pub(crate) arn: std::option::Option<std::string::String>,
    pub(crate) create_date: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) assume_role_policy_document: std::option::Option<std::string::String>,
    pub(crate) instance_profile_list:
        std::option::Option<std::vec::Vec<crate::types::InstanceProfile>>,
    pub(crate) role_policy_list: std::option::Option<std::vec::Vec<crate::types::PolicyDetail>>,
    pub(crate) attached_managed_policies:
        std::option::Option<std::vec::Vec<crate::types::AttachedPolicy>>,
    pub(crate) permissions_boundary: std::option::Option<crate::types::AttachedPermissionsBoundary>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    pub(crate) role_last_used: std::option::Option<crate::types::RoleLastUsed>,
}
impl RoleDetailBuilder {
    /// <p>The path to the role. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn path(mut self, input: impl Into<std::string::String>) -> Self {
        self.path = Some(input.into());
        self
    }
    /// <p>The path to the role. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_path(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// <p>The friendly name that identifies the role.</p>
    pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.role_name = Some(input.into());
        self
    }
    /// <p>The friendly name that identifies the role.</p>
    pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// <p>The stable and unique string identifying the role. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn role_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.role_id = Some(input.into());
        self
    }
    /// <p>The stable and unique string identifying the role. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_role_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.role_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the role was created.</p>
    pub fn create_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.create_date = Some(input);
        self
    }
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the role was created.</p>
    pub fn set_create_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_date = input;
        self
    }
    /// <p>The trust policy that grants permission to assume the role.</p>
    pub fn assume_role_policy_document(mut self, input: impl Into<std::string::String>) -> Self {
        self.assume_role_policy_document = Some(input.into());
        self
    }
    /// <p>The trust policy that grants permission to assume the role.</p>
    pub fn set_assume_role_policy_document(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.assume_role_policy_document = input;
        self
    }
    /// Appends an item to `instance_profile_list`.
    ///
    /// To override the contents of this collection use [`set_instance_profile_list`](Self::set_instance_profile_list).
    ///
    /// <p>A list of instance profiles that contain this role.</p>
    pub fn instance_profile_list(mut self, input: crate::types::InstanceProfile) -> Self {
        let mut v = self.instance_profile_list.unwrap_or_default();
        v.push(input);
        self.instance_profile_list = Some(v);
        self
    }
    /// <p>A list of instance profiles that contain this role.</p>
    pub fn set_instance_profile_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::InstanceProfile>>,
    ) -> Self {
        self.instance_profile_list = input;
        self
    }
    /// Appends an item to `role_policy_list`.
    ///
    /// To override the contents of this collection use [`set_role_policy_list`](Self::set_role_policy_list).
    ///
    /// <p>A list of inline policies embedded in the role. These policies are the role's access (permissions) policies.</p>
    pub fn role_policy_list(mut self, input: crate::types::PolicyDetail) -> Self {
        let mut v = self.role_policy_list.unwrap_or_default();
        v.push(input);
        self.role_policy_list = Some(v);
        self
    }
    /// <p>A list of inline policies embedded in the role. These policies are the role's access (permissions) policies.</p>
    pub fn set_role_policy_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PolicyDetail>>,
    ) -> Self {
        self.role_policy_list = input;
        self
    }
    /// Appends an item to `attached_managed_policies`.
    ///
    /// To override the contents of this collection use [`set_attached_managed_policies`](Self::set_attached_managed_policies).
    ///
    /// <p>A list of managed policies attached to the role. These policies are the role's access (permissions) policies.</p>
    pub fn attached_managed_policies(mut self, input: crate::types::AttachedPolicy) -> Self {
        let mut v = self.attached_managed_policies.unwrap_or_default();
        v.push(input);
        self.attached_managed_policies = Some(v);
        self
    }
    /// <p>A list of managed policies attached to the role. These policies are the role's access (permissions) policies.</p>
    pub fn set_attached_managed_policies(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AttachedPolicy>>,
    ) -> Self {
        self.attached_managed_policies = input;
        self
    }
    /// <p>The ARN of the policy used to set the permissions boundary for the role.</p>
    /// <p>For more information about permissions boundaries, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries for IAM identities </a> in the <i>IAM User Guide</i>.</p>
    pub fn permissions_boundary(
        mut self,
        input: crate::types::AttachedPermissionsBoundary,
    ) -> Self {
        self.permissions_boundary = Some(input);
        self
    }
    /// <p>The ARN of the policy used to set the permissions boundary for the role.</p>
    /// <p>For more information about permissions boundaries, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries for IAM identities </a> in the <i>IAM User Guide</i>.</p>
    pub fn set_permissions_boundary(
        mut self,
        input: std::option::Option<crate::types::AttachedPermissionsBoundary>,
    ) -> Self {
        self.permissions_boundary = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags that are attached to the role. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>A list of tags that are attached to the role. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>Contains information about the last time that an IAM role was used. This includes the date and time and the Region in which the role was last used. Activity is only reported for the trailing 400 days. This period can be shorter if your Region began supporting these features within the last year. The role might have been used more than 400 days ago. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period">Regions where data is tracked</a> in the <i>IAM User Guide</i>.</p>
    pub fn role_last_used(mut self, input: crate::types::RoleLastUsed) -> Self {
        self.role_last_used = Some(input);
        self
    }
    /// <p>Contains information about the last time that an IAM role was used. This includes the date and time and the Region in which the role was last used. Activity is only reported for the trailing 400 days. This period can be shorter if your Region began supporting these features within the last year. The role might have been used more than 400 days ago. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period">Regions where data is tracked</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_role_last_used(
        mut self,
        input: std::option::Option<crate::types::RoleLastUsed>,
    ) -> Self {
        self.role_last_used = input;
        self
    }
    /// Consumes the builder and constructs a [`RoleDetail`](crate::types::RoleDetail).
    pub fn build(self) -> crate::types::RoleDetail {
        crate::types::RoleDetail {
            path: self.path,
            role_name: self.role_name,
            role_id: self.role_id,
            arn: self.arn,
            create_date: self.create_date,
            assume_role_policy_document: self.assume_role_policy_document,
            instance_profile_list: self.instance_profile_list,
            role_policy_list: self.role_policy_list,
            attached_managed_policies: self.attached_managed_policies,
            permissions_boundary: self.permissions_boundary,
            tags: self.tags,
            role_last_used: self.role_last_used,
        }
    }
}
