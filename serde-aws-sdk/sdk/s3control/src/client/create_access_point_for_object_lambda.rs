// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateAccessPointForObjectLambda`](crate::operation::create_access_point_for_object_lambda::builders::CreateAccessPointForObjectLambdaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::create_access_point_for_object_lambda::builders::CreateAccessPointForObjectLambdaFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::create_access_point_for_object_lambda::builders::CreateAccessPointForObjectLambdaFluentBuilder::set_account_id): <p>The Amazon Web Services account ID for owner of the specified Object Lambda Access Point.</p>
    ///   - [`name(impl Into<String>)`](crate::operation::create_access_point_for_object_lambda::builders::CreateAccessPointForObjectLambdaFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_access_point_for_object_lambda::builders::CreateAccessPointForObjectLambdaFluentBuilder::set_name): <p>The name you want to assign to this Object Lambda Access Point.</p>
    ///   - [`configuration(ObjectLambdaConfiguration)`](crate::operation::create_access_point_for_object_lambda::builders::CreateAccessPointForObjectLambdaFluentBuilder::configuration) / [`set_configuration(Option<ObjectLambdaConfiguration>)`](crate::operation::create_access_point_for_object_lambda::builders::CreateAccessPointForObjectLambdaFluentBuilder::set_configuration): <p>Object Lambda Access Point configuration as a JSON document.</p>
    /// - On success, responds with [`CreateAccessPointForObjectLambdaOutput`](crate::operation::create_access_point_for_object_lambda::CreateAccessPointForObjectLambdaOutput) with field(s):
    ///   - [`object_lambda_access_point_arn(Option<String>)`](crate::operation::create_access_point_for_object_lambda::CreateAccessPointForObjectLambdaOutput::object_lambda_access_point_arn): <p>Specifies the ARN for the Object Lambda Access Point.</p>
    /// - On failure, responds with [`SdkError<CreateAccessPointForObjectLambdaError>`](crate::operation::create_access_point_for_object_lambda::CreateAccessPointForObjectLambdaError)
    pub fn create_access_point_for_object_lambda(&self) -> crate::operation::create_access_point_for_object_lambda::builders::CreateAccessPointForObjectLambdaFluentBuilder{
        crate::operation::create_access_point_for_object_lambda::builders::CreateAccessPointForObjectLambdaFluentBuilder::new(self.handle.clone())
    }
}