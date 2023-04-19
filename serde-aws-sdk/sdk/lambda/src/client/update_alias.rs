// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAlias`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_function_name): <p>The name of the Lambda function.</p>  <p class="title"> <b>Name formats</b> </p>  <ul>   <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li>   <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>   <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li>  </ul>  <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    ///   - [`name(impl Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_name): <p>The name of the alias.</p>
    ///   - [`function_version(impl Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::function_version) / [`set_function_version(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_function_version): <p>The function version that the alias invokes.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_description): <p>A description of the alias.</p>
    ///   - [`routing_config(AliasRoutingConfiguration)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::routing_config) / [`set_routing_config(Option<AliasRoutingConfiguration>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_routing_config): <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-aliases.html#configuring-alias-routing">routing configuration</a> of the alias.</p>
    ///   - [`revision_id(impl Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_revision_id): <p>Only update the alias if the revision ID matches the ID that's specified. Use this option to avoid modifying an alias that has changed since you last read it.</p>
    /// - On success, responds with [`UpdateAliasOutput`](crate::operation::update_alias::UpdateAliasOutput) with field(s):
    ///   - [`alias_arn(Option<String>)`](crate::operation::update_alias::UpdateAliasOutput::alias_arn): <p>The Amazon Resource Name (ARN) of the alias.</p>
    ///   - [`name(Option<String>)`](crate::operation::update_alias::UpdateAliasOutput::name): <p>The name of the alias.</p>
    ///   - [`function_version(Option<String>)`](crate::operation::update_alias::UpdateAliasOutput::function_version): <p>The function version that the alias invokes.</p>
    ///   - [`description(Option<String>)`](crate::operation::update_alias::UpdateAliasOutput::description): <p>A description of the alias.</p>
    ///   - [`routing_config(Option<AliasRoutingConfiguration>)`](crate::operation::update_alias::UpdateAliasOutput::routing_config): <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">routing configuration</a> of the alias.</p>
    ///   - [`revision_id(Option<String>)`](crate::operation::update_alias::UpdateAliasOutput::revision_id): <p>A unique identifier that changes when you update the alias.</p>
    /// - On failure, responds with [`SdkError<UpdateAliasError>`](crate::operation::update_alias::UpdateAliasError)
    pub fn update_alias(
        &self,
    ) -> crate::operation::update_alias::builders::UpdateAliasFluentBuilder {
        crate::operation::update_alias::builders::UpdateAliasFluentBuilder::new(self.handle.clone())
    }
}
