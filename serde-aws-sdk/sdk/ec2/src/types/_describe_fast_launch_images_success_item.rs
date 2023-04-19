// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describe details about a fast-launch enabled Windows image that meets the requested criteria. Criteria are defined by the <code>DescribeFastLaunchImages</code> action filters.</p>
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
pub struct DescribeFastLaunchImagesSuccessItem {
    /// <p>The image ID that identifies the fast-launch enabled Windows image.</p>
    #[doc(hidden)]
    pub image_id: std::option::Option<std::string::String>,
    /// <p>The resource type that is used for pre-provisioning the Windows AMI. Supported values include: <code>snapshot</code>.</p>
    #[doc(hidden)]
    pub resource_type: std::option::Option<crate::types::FastLaunchResourceType>,
    /// <p>A group of parameters that are used for pre-provisioning the associated Windows AMI using snapshots.</p>
    #[doc(hidden)]
    pub snapshot_configuration:
        std::option::Option<crate::types::FastLaunchSnapshotConfigurationResponse>,
    /// <p>The launch template that the fast-launch enabled Windows AMI uses when it launches Windows instances from pre-provisioned snapshots.</p>
    #[doc(hidden)]
    pub launch_template:
        std::option::Option<crate::types::FastLaunchLaunchTemplateSpecificationResponse>,
    /// <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows faster launching.</p>
    #[doc(hidden)]
    pub max_parallel_launches: std::option::Option<i32>,
    /// <p>The owner ID for the fast-launch enabled Windows AMI.</p>
    #[doc(hidden)]
    pub owner_id: std::option::Option<std::string::String>,
    /// <p>The current state of faster launching for the specified Windows AMI.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::FastLaunchStateCode>,
    /// <p>The reason that faster launching for the Windows AMI changed to the current state.</p>
    #[doc(hidden)]
    pub state_transition_reason: std::option::Option<std::string::String>,
    /// <p>The time that faster launching for the Windows AMI changed to the current state.</p>
    #[doc(hidden)]
    pub state_transition_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl DescribeFastLaunchImagesSuccessItem {
    /// <p>The image ID that identifies the fast-launch enabled Windows image.</p>
    pub fn image_id(&self) -> std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>The resource type that is used for pre-provisioning the Windows AMI. Supported values include: <code>snapshot</code>.</p>
    pub fn resource_type(&self) -> std::option::Option<&crate::types::FastLaunchResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>A group of parameters that are used for pre-provisioning the associated Windows AMI using snapshots.</p>
    pub fn snapshot_configuration(
        &self,
    ) -> std::option::Option<&crate::types::FastLaunchSnapshotConfigurationResponse> {
        self.snapshot_configuration.as_ref()
    }
    /// <p>The launch template that the fast-launch enabled Windows AMI uses when it launches Windows instances from pre-provisioned snapshots.</p>
    pub fn launch_template(
        &self,
    ) -> std::option::Option<&crate::types::FastLaunchLaunchTemplateSpecificationResponse> {
        self.launch_template.as_ref()
    }
    /// <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows faster launching.</p>
    pub fn max_parallel_launches(&self) -> std::option::Option<i32> {
        self.max_parallel_launches
    }
    /// <p>The owner ID for the fast-launch enabled Windows AMI.</p>
    pub fn owner_id(&self) -> std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The current state of faster launching for the specified Windows AMI.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::FastLaunchStateCode> {
        self.state.as_ref()
    }
    /// <p>The reason that faster launching for the Windows AMI changed to the current state.</p>
    pub fn state_transition_reason(&self) -> std::option::Option<&str> {
        self.state_transition_reason.as_deref()
    }
    /// <p>The time that faster launching for the Windows AMI changed to the current state.</p>
    pub fn state_transition_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.state_transition_time.as_ref()
    }
}
impl DescribeFastLaunchImagesSuccessItem {
    /// Creates a new builder-style object to manufacture [`DescribeFastLaunchImagesSuccessItem`](crate::types::DescribeFastLaunchImagesSuccessItem).
    pub fn builder() -> crate::types::builders::DescribeFastLaunchImagesSuccessItemBuilder {
        crate::types::builders::DescribeFastLaunchImagesSuccessItemBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::DescribeFastLaunchImagesSuccessItem;
/// A builder for [`DescribeFastLaunchImagesSuccessItem`](crate::types::DescribeFastLaunchImagesSuccessItem).
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
pub struct DescribeFastLaunchImagesSuccessItemBuilder {
    pub(crate) image_id: std::option::Option<std::string::String>,
    pub(crate) resource_type: std::option::Option<crate::types::FastLaunchResourceType>,
    pub(crate) snapshot_configuration:
        std::option::Option<crate::types::FastLaunchSnapshotConfigurationResponse>,
    pub(crate) launch_template:
        std::option::Option<crate::types::FastLaunchLaunchTemplateSpecificationResponse>,
    pub(crate) max_parallel_launches: std::option::Option<i32>,
    pub(crate) owner_id: std::option::Option<std::string::String>,
    pub(crate) state: std::option::Option<crate::types::FastLaunchStateCode>,
    pub(crate) state_transition_reason: std::option::Option<std::string::String>,
    pub(crate) state_transition_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl DescribeFastLaunchImagesSuccessItemBuilder {
    /// <p>The image ID that identifies the fast-launch enabled Windows image.</p>
    pub fn image_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.image_id = Some(input.into());
        self
    }
    /// <p>The image ID that identifies the fast-launch enabled Windows image.</p>
    pub fn set_image_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>The resource type that is used for pre-provisioning the Windows AMI. Supported values include: <code>snapshot</code>.</p>
    pub fn resource_type(mut self, input: crate::types::FastLaunchResourceType) -> Self {
        self.resource_type = Some(input);
        self
    }
    /// <p>The resource type that is used for pre-provisioning the Windows AMI. Supported values include: <code>snapshot</code>.</p>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::FastLaunchResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>A group of parameters that are used for pre-provisioning the associated Windows AMI using snapshots.</p>
    pub fn snapshot_configuration(
        mut self,
        input: crate::types::FastLaunchSnapshotConfigurationResponse,
    ) -> Self {
        self.snapshot_configuration = Some(input);
        self
    }
    /// <p>A group of parameters that are used for pre-provisioning the associated Windows AMI using snapshots.</p>
    pub fn set_snapshot_configuration(
        mut self,
        input: std::option::Option<crate::types::FastLaunchSnapshotConfigurationResponse>,
    ) -> Self {
        self.snapshot_configuration = input;
        self
    }
    /// <p>The launch template that the fast-launch enabled Windows AMI uses when it launches Windows instances from pre-provisioned snapshots.</p>
    pub fn launch_template(
        mut self,
        input: crate::types::FastLaunchLaunchTemplateSpecificationResponse,
    ) -> Self {
        self.launch_template = Some(input);
        self
    }
    /// <p>The launch template that the fast-launch enabled Windows AMI uses when it launches Windows instances from pre-provisioned snapshots.</p>
    pub fn set_launch_template(
        mut self,
        input: std::option::Option<crate::types::FastLaunchLaunchTemplateSpecificationResponse>,
    ) -> Self {
        self.launch_template = input;
        self
    }
    /// <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows faster launching.</p>
    pub fn max_parallel_launches(mut self, input: i32) -> Self {
        self.max_parallel_launches = Some(input);
        self
    }
    /// <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows faster launching.</p>
    pub fn set_max_parallel_launches(mut self, input: std::option::Option<i32>) -> Self {
        self.max_parallel_launches = input;
        self
    }
    /// <p>The owner ID for the fast-launch enabled Windows AMI.</p>
    pub fn owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.owner_id = Some(input.into());
        self
    }
    /// <p>The owner ID for the fast-launch enabled Windows AMI.</p>
    pub fn set_owner_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The current state of faster launching for the specified Windows AMI.</p>
    pub fn state(mut self, input: crate::types::FastLaunchStateCode) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The current state of faster launching for the specified Windows AMI.</p>
    pub fn set_state(
        mut self,
        input: std::option::Option<crate::types::FastLaunchStateCode>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The reason that faster launching for the Windows AMI changed to the current state.</p>
    pub fn state_transition_reason(mut self, input: impl Into<std::string::String>) -> Self {
        self.state_transition_reason = Some(input.into());
        self
    }
    /// <p>The reason that faster launching for the Windows AMI changed to the current state.</p>
    pub fn set_state_transition_reason(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.state_transition_reason = input;
        self
    }
    /// <p>The time that faster launching for the Windows AMI changed to the current state.</p>
    pub fn state_transition_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.state_transition_time = Some(input);
        self
    }
    /// <p>The time that faster launching for the Windows AMI changed to the current state.</p>
    pub fn set_state_transition_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.state_transition_time = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeFastLaunchImagesSuccessItem`](crate::types::DescribeFastLaunchImagesSuccessItem).
    pub fn build(self) -> crate::types::DescribeFastLaunchImagesSuccessItem {
        crate::types::DescribeFastLaunchImagesSuccessItem {
            image_id: self.image_id,
            resource_type: self.resource_type,
            snapshot_configuration: self.snapshot_configuration,
            launch_template: self.launch_template,
            max_parallel_launches: self.max_parallel_launches,
            owner_id: self.owner_id,
            state: self.state,
            state_transition_reason: self.state_transition_reason,
            state_transition_time: self.state_transition_time,
        }
    }
}
