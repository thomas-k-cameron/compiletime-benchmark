// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the details of a container that's part of a job attempt.</p>
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
pub struct AttemptContainerDetail {
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS container instance that hosts the job attempt.</p>
    #[doc(hidden)]
    pub container_instance_arn: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that's associated with the job attempt. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    #[doc(hidden)]
    pub task_arn: std::option::Option<std::string::String>,
    /// <p>The exit code for the job attempt. A non-zero exit code is considered failed.</p>
    #[doc(hidden)]
    pub exit_code: std::option::Option<i32>,
    /// <p>A short (255 max characters) human-readable string to provide additional details for a running or stopped container.</p>
    #[doc(hidden)]
    pub reason: std::option::Option<std::string::String>,
    /// <p>The name of the CloudWatch Logs log stream that's associated with the container. The log group for Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    #[doc(hidden)]
    pub log_stream_name: std::option::Option<std::string::String>,
    /// <p>The network interfaces that are associated with the job attempt.</p>
    #[doc(hidden)]
    pub network_interfaces: std::option::Option<std::vec::Vec<crate::types::NetworkInterface>>,
}
impl AttemptContainerDetail {
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS container instance that hosts the job attempt.</p>
    pub fn container_instance_arn(&self) -> std::option::Option<&str> {
        self.container_instance_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that's associated with the job attempt. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    pub fn task_arn(&self) -> std::option::Option<&str> {
        self.task_arn.as_deref()
    }
    /// <p>The exit code for the job attempt. A non-zero exit code is considered failed.</p>
    pub fn exit_code(&self) -> std::option::Option<i32> {
        self.exit_code
    }
    /// <p>A short (255 max characters) human-readable string to provide additional details for a running or stopped container.</p>
    pub fn reason(&self) -> std::option::Option<&str> {
        self.reason.as_deref()
    }
    /// <p>The name of the CloudWatch Logs log stream that's associated with the container. The log group for Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    pub fn log_stream_name(&self) -> std::option::Option<&str> {
        self.log_stream_name.as_deref()
    }
    /// <p>The network interfaces that are associated with the job attempt.</p>
    pub fn network_interfaces(&self) -> std::option::Option<&[crate::types::NetworkInterface]> {
        self.network_interfaces.as_deref()
    }
}
impl AttemptContainerDetail {
    /// Creates a new builder-style object to manufacture [`AttemptContainerDetail`](crate::types::AttemptContainerDetail).
    pub fn builder() -> crate::types::builders::AttemptContainerDetailBuilder {
        crate::types::builders::AttemptContainerDetailBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AttemptContainerDetail;
/// A builder for [`AttemptContainerDetail`](crate::types::AttemptContainerDetail).
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
pub struct AttemptContainerDetailBuilder {
    pub(crate) container_instance_arn: std::option::Option<std::string::String>,
    pub(crate) task_arn: std::option::Option<std::string::String>,
    pub(crate) exit_code: std::option::Option<i32>,
    pub(crate) reason: std::option::Option<std::string::String>,
    pub(crate) log_stream_name: std::option::Option<std::string::String>,
    pub(crate) network_interfaces:
        std::option::Option<std::vec::Vec<crate::types::NetworkInterface>>,
}
impl AttemptContainerDetailBuilder {
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS container instance that hosts the job attempt.</p>
    pub fn container_instance_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.container_instance_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS container instance that hosts the job attempt.</p>
    pub fn set_container_instance_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.container_instance_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that's associated with the job attempt. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    pub fn task_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.task_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that's associated with the job attempt. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    pub fn set_task_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.task_arn = input;
        self
    }
    /// <p>The exit code for the job attempt. A non-zero exit code is considered failed.</p>
    pub fn exit_code(mut self, input: i32) -> Self {
        self.exit_code = Some(input);
        self
    }
    /// <p>The exit code for the job attempt. A non-zero exit code is considered failed.</p>
    pub fn set_exit_code(mut self, input: std::option::Option<i32>) -> Self {
        self.exit_code = input;
        self
    }
    /// <p>A short (255 max characters) human-readable string to provide additional details for a running or stopped container.</p>
    pub fn reason(mut self, input: impl Into<std::string::String>) -> Self {
        self.reason = Some(input.into());
        self
    }
    /// <p>A short (255 max characters) human-readable string to provide additional details for a running or stopped container.</p>
    pub fn set_reason(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// <p>The name of the CloudWatch Logs log stream that's associated with the container. The log group for Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    pub fn log_stream_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.log_stream_name = Some(input.into());
        self
    }
    /// <p>The name of the CloudWatch Logs log stream that's associated with the container. The log group for Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    pub fn set_log_stream_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.log_stream_name = input;
        self
    }
    /// Appends an item to `network_interfaces`.
    ///
    /// To override the contents of this collection use [`set_network_interfaces`](Self::set_network_interfaces).
    ///
    /// <p>The network interfaces that are associated with the job attempt.</p>
    pub fn network_interfaces(mut self, input: crate::types::NetworkInterface) -> Self {
        let mut v = self.network_interfaces.unwrap_or_default();
        v.push(input);
        self.network_interfaces = Some(v);
        self
    }
    /// <p>The network interfaces that are associated with the job attempt.</p>
    pub fn set_network_interfaces(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::NetworkInterface>>,
    ) -> Self {
        self.network_interfaces = input;
        self
    }
    /// Consumes the builder and constructs a [`AttemptContainerDetail`](crate::types::AttemptContainerDetail).
    pub fn build(self) -> crate::types::AttemptContainerDetail {
        crate::types::AttemptContainerDetail {
            container_instance_arn: self.container_instance_arn,
            task_arn: self.task_arn,
            exit_code: self.exit_code,
            reason: self.reason,
            log_stream_name: self.log_stream_name,
            network_interfaces: self.network_interfaces,
        }
    }
}
