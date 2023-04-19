// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides configuration information about a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-aliases.html">alias</a>.</p>
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
pub struct UpdateAliasOutput {
    /// <p>The Amazon Resource Name (ARN) of the alias.</p>
    #[doc(hidden)]
    pub alias_arn: std::option::Option<std::string::String>,
    /// <p>The name of the alias.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The function version that the alias invokes.</p>
    #[doc(hidden)]
    pub function_version: std::option::Option<std::string::String>,
    /// <p>A description of the alias.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">routing configuration</a> of the alias.</p>
    #[doc(hidden)]
    pub routing_config: std::option::Option<crate::types::AliasRoutingConfiguration>,
    /// <p>A unique identifier that changes when you update the alias.</p>
    #[doc(hidden)]
    pub revision_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl UpdateAliasOutput {
    /// <p>The Amazon Resource Name (ARN) of the alias.</p>
    pub fn alias_arn(&self) -> std::option::Option<&str> {
        self.alias_arn.as_deref()
    }
    /// <p>The name of the alias.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The function version that the alias invokes.</p>
    pub fn function_version(&self) -> std::option::Option<&str> {
        self.function_version.as_deref()
    }
    /// <p>A description of the alias.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">routing configuration</a> of the alias.</p>
    pub fn routing_config(&self) -> std::option::Option<&crate::types::AliasRoutingConfiguration> {
        self.routing_config.as_ref()
    }
    /// <p>A unique identifier that changes when you update the alias.</p>
    pub fn revision_id(&self) -> std::option::Option<&str> {
        self.revision_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for UpdateAliasOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateAliasOutput {
    /// Creates a new builder-style object to manufacture [`UpdateAliasOutput`](crate::operation::update_alias::UpdateAliasOutput).
    pub fn builder() -> crate::operation::update_alias::builders::UpdateAliasOutputBuilder {
        crate::operation::update_alias::builders::UpdateAliasOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::update_alias::UpdateAliasOutput;
/// A builder for [`UpdateAliasOutput`](crate::operation::update_alias::UpdateAliasOutput).
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
pub struct UpdateAliasOutputBuilder {
    pub(crate) alias_arn: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) function_version: std::option::Option<std::string::String>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) routing_config: std::option::Option<crate::types::AliasRoutingConfiguration>,
    pub(crate) revision_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl UpdateAliasOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the alias.</p>
    pub fn alias_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.alias_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the alias.</p>
    pub fn set_alias_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.alias_arn = input;
        self
    }
    /// <p>The name of the alias.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the alias.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The function version that the alias invokes.</p>
    pub fn function_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.function_version = Some(input.into());
        self
    }
    /// <p>The function version that the alias invokes.</p>
    pub fn set_function_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.function_version = input;
        self
    }
    /// <p>A description of the alias.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A description of the alias.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">routing configuration</a> of the alias.</p>
    pub fn routing_config(mut self, input: crate::types::AliasRoutingConfiguration) -> Self {
        self.routing_config = Some(input);
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">routing configuration</a> of the alias.</p>
    pub fn set_routing_config(
        mut self,
        input: std::option::Option<crate::types::AliasRoutingConfiguration>,
    ) -> Self {
        self.routing_config = input;
        self
    }
    /// <p>A unique identifier that changes when you update the alias.</p>
    pub fn revision_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.revision_id = Some(input.into());
        self
    }
    /// <p>A unique identifier that changes when you update the alias.</p>
    pub fn set_revision_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.revision_id = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAliasOutput`](crate::operation::update_alias::UpdateAliasOutput).
    pub fn build(self) -> crate::operation::update_alias::UpdateAliasOutput {
        crate::operation::update_alias::UpdateAliasOutput {
            alias_arn: self.alias_arn,
            name: self.name,
            function_version: self.function_version,
            description: self.description,
            routing_config: self.routing_config,
            revision_id: self.revision_id,
            _request_id: self._request_id,
        }
    }
}
