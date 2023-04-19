// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateLaunchTemplate`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>  <p>Constraint: Maximum 128 ASCII characters.</p>
    ///   - [`launch_template_name(impl Into<String>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::launch_template_name) / [`set_launch_template_name(Option<String>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::set_launch_template_name): <p>A name for the launch template.</p>
    ///   - [`version_description(impl Into<String>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::version_description) / [`set_version_description(Option<String>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::set_version_description): <p>A description for the first version of the launch template.</p>
    ///   - [`launch_template_data(RequestLaunchTemplateData)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::launch_template_data) / [`set_launch_template_data(Option<RequestLaunchTemplateData>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::set_launch_template_data): <p>The information for the launch template.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::set_tag_specifications): <p>The tags to apply to the launch template on creation. To tag the launch template, the resource type must be <code>launch-template</code>.</p> <note>   <p>To specify the tags for the resources that are created when an instance is launched, you must use the <code>TagSpecifications</code> parameter in the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestLaunchTemplateData.html">launch template data</a> structure.</p>  </note>
    /// - On success, responds with [`CreateLaunchTemplateOutput`](crate::operation::create_launch_template::CreateLaunchTemplateOutput) with field(s):
    ///   - [`launch_template(Option<LaunchTemplate>)`](crate::operation::create_launch_template::CreateLaunchTemplateOutput::launch_template): <p>Information about the launch template.</p>
    ///   - [`warning(Option<ValidationWarning>)`](crate::operation::create_launch_template::CreateLaunchTemplateOutput::warning): <p>If the launch template contains parameters or parameter combinations that are not valid, an error code and an error message are returned for each issue that's found.</p>
    /// - On failure, responds with [`SdkError<CreateLaunchTemplateError>`](crate::operation::create_launch_template::CreateLaunchTemplateError)
    pub fn create_launch_template(
        &self,
    ) -> crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder {
        crate::operation::create_launch_template::builders::CreateLaunchTemplateFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
