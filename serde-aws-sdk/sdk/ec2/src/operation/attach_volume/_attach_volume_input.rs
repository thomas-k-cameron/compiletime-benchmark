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
pub struct AttachVolumeInput {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    #[doc(hidden)]
    pub device: std::option::Option<std::string::String>,
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The ID of the EBS volume. The volume and instance must be within the same Availability Zone.</p>
    #[doc(hidden)]
    pub volume_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl AttachVolumeInput {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn device(&self) -> std::option::Option<&str> {
        self.device.as_deref()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The ID of the EBS volume. The volume and instance must be within the same Availability Zone.</p>
    pub fn volume_id(&self) -> std::option::Option<&str> {
        self.volume_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl AttachVolumeInput {
    /// Creates a new builder-style object to manufacture [`AttachVolumeInput`](crate::operation::attach_volume::AttachVolumeInput).
    pub fn builder() -> crate::operation::attach_volume::builders::AttachVolumeInputBuilder {
        crate::operation::attach_volume::builders::AttachVolumeInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::attach_volume::AttachVolumeInput;
/// A builder for [`AttachVolumeInput`](crate::operation::attach_volume::AttachVolumeInput).
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
pub struct AttachVolumeInputBuilder {
    pub(crate) device: std::option::Option<std::string::String>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) volume_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl AttachVolumeInputBuilder {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn device(mut self, input: impl Into<std::string::String>) -> Self {
        self.device = Some(input.into());
        self
    }
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn set_device(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.device = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The ID of the EBS volume. The volume and instance must be within the same Availability Zone.</p>
    pub fn volume_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.volume_id = Some(input.into());
        self
    }
    /// <p>The ID of the EBS volume. The volume and instance must be within the same Availability Zone.</p>
    pub fn set_volume_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.volume_id = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`AttachVolumeInput`](crate::operation::attach_volume::AttachVolumeInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::attach_volume::AttachVolumeInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::attach_volume::AttachVolumeInput {
            device: self.device,
            instance_id: self.instance_id,
            volume_id: self.volume_id,
            dry_run: self.dry_run,
        })
    }
}
