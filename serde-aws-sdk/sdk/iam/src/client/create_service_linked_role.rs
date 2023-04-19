// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateServiceLinkedRole`](crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_service_name(impl Into<String>)`](crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleFluentBuilder::aws_service_name) / [`set_aws_service_name(Option<String>)`](crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleFluentBuilder::set_aws_service_name): <p>The service principal for the Amazon Web Services service to which this role is attached. You use a string similar to a URL but without the http:// in front. For example: <code>elasticbeanstalk.amazonaws.com</code>. </p>  <p>Service principals are unique and case-sensitive. To find the exact service principal for your service-linked role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html">Amazon Web Services services that work with IAM</a> in the <i>IAM User Guide</i>. Look for the services that have <b>Yes </b>in the <b>Service-Linked Role</b> column. Choose the <b>Yes</b> link to view the service-linked role documentation for that service.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleFluentBuilder::set_description): <p>The description of the role.</p>
    ///   - [`custom_suffix(impl Into<String>)`](crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleFluentBuilder::custom_suffix) / [`set_custom_suffix(Option<String>)`](crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleFluentBuilder::set_custom_suffix): <p></p>  <p>A string that you provide, which is combined with the service-provided prefix to form the complete role name. If you make multiple requests for the same service, then you must supply a different <code>CustomSuffix</code> for each request. Otherwise the request fails with a duplicate role name error. For example, you could add <code>-1</code> or <code>-debug</code> to the suffix.</p>  <p>Some services do not support the <code>CustomSuffix</code> parameter. If you provide an optional suffix and the operation fails, try the operation again without the suffix.</p>
    /// - On success, responds with [`CreateServiceLinkedRoleOutput`](crate::operation::create_service_linked_role::CreateServiceLinkedRoleOutput) with field(s):
    ///   - [`role(Option<Role>)`](crate::operation::create_service_linked_role::CreateServiceLinkedRoleOutput::role): <p>A <code>Role</code> object that contains details about the newly created role.</p>
    /// - On failure, responds with [`SdkError<CreateServiceLinkedRoleError>`](crate::operation::create_service_linked_role::CreateServiceLinkedRoleError)
    pub fn create_service_linked_role(
        &self,
    ) -> crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleFluentBuilder
    {
        crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleFluentBuilder::new(self.handle.clone())
    }
}
