// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterContainerInstance`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster to register your container instance with. If you do not specify a cluster, the default cluster is assumed.</p>
    ///   - [`instance_identity_document(impl Into<String>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::instance_identity_document) / [`set_instance_identity_document(Option<String>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::set_instance_identity_document): <p>The instance identity document for the EC2 instance to register. This document can be found by running the following command from the instance: <code>curl http://169.254.169.254/latest/dynamic/instance-identity/document/</code> </p>
    ///   - [`instance_identity_document_signature(impl Into<String>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::instance_identity_document_signature) / [`set_instance_identity_document_signature(Option<String>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::set_instance_identity_document_signature): <p>The instance identity document signature for the EC2 instance to register. This signature can be found by running the following command from the instance: <code>curl http://169.254.169.254/latest/dynamic/instance-identity/signature/</code> </p>
    ///   - [`total_resources(Vec<Resource>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::total_resources) / [`set_total_resources(Option<Vec<Resource>>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::set_total_resources): <p>The resources available on the instance.</p>
    ///   - [`version_info(VersionInfo)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::version_info) / [`set_version_info(Option<VersionInfo>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::set_version_info): <p>The version information for the Amazon ECS container agent and Docker daemon that runs on the container instance.</p>
    ///   - [`container_instance_arn(impl Into<String>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::container_instance_arn) / [`set_container_instance_arn(Option<String>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::set_container_instance_arn): <p>The ARN of the container instance (if it was previously registered).</p>
    ///   - [`attributes(Vec<Attribute>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::attributes) / [`set_attributes(Option<Vec<Attribute>>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::set_attributes): <p>The container instance attributes that this container instance supports.</p>
    ///   - [`platform_devices(Vec<PlatformDevice>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::platform_devices) / [`set_platform_devices(Option<Vec<PlatformDevice>>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::set_platform_devices): <p>The devices that are available on the container instance. The only supported device type is a GPU.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::set_tags): <p>The metadata that you apply to the container instance to help you categorize and organize them. Each tag consists of a key and an optional value. You define both.</p>  <p>The following basic restrictions apply to tags:</p>  <ul>   <li> <p>Maximum number of tags per resource - 50</p> </li>   <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>   <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li>   <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li>   <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>   <li> <p>Tag keys and values are case-sensitive.</p> </li>   <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li>  </ul>
    /// - On success, responds with [`RegisterContainerInstanceOutput`](crate::operation::register_container_instance::RegisterContainerInstanceOutput) with field(s):
    ///   - [`container_instance(Option<ContainerInstance>)`](crate::operation::register_container_instance::RegisterContainerInstanceOutput::container_instance): <p>The container instance that was registered.</p>
    /// - On failure, responds with [`SdkError<RegisterContainerInstanceError>`](crate::operation::register_container_instance::RegisterContainerInstanceError)
    pub fn register_container_instance(&self) -> crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder{
        crate::operation::register_container_instance::builders::RegisterContainerInstanceFluentBuilder::new(self.handle.clone())
    }
}
