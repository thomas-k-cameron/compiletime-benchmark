// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides options for initiating an Amazon S3 Glacier job.</p>
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
pub struct InitiateJobInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The name of the vault.</p>
    #[doc(hidden)]
    pub vault_name: std::option::Option<std::string::String>,
    /// <p>Provides options for specifying job information.</p>
    #[doc(hidden)]
    pub job_parameters: std::option::Option<crate::types::JobParameters>,
}
impl InitiateJobInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(&self) -> std::option::Option<&str> {
        self.vault_name.as_deref()
    }
    /// <p>Provides options for specifying job information.</p>
    pub fn job_parameters(&self) -> std::option::Option<&crate::types::JobParameters> {
        self.job_parameters.as_ref()
    }
}
impl InitiateJobInput {
    /// Creates a new builder-style object to manufacture [`InitiateJobInput`](crate::operation::initiate_job::InitiateJobInput).
    pub fn builder() -> crate::operation::initiate_job::builders::InitiateJobInputBuilder {
        crate::operation::initiate_job::builders::InitiateJobInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::initiate_job::InitiateJobInput;
/// A builder for [`InitiateJobInput`](crate::operation::initiate_job::InitiateJobInput).
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
pub struct InitiateJobInputBuilder {
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) vault_name: std::option::Option<std::string::String>,
    pub(crate) job_parameters: std::option::Option<crate::types::JobParameters>,
}
impl InitiateJobInputBuilder {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.vault_name = Some(input.into());
        self
    }
    /// <p>The name of the vault.</p>
    pub fn set_vault_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vault_name = input;
        self
    }
    /// <p>Provides options for specifying job information.</p>
    pub fn job_parameters(mut self, input: crate::types::JobParameters) -> Self {
        self.job_parameters = Some(input);
        self
    }
    /// <p>Provides options for specifying job information.</p>
    pub fn set_job_parameters(
        mut self,
        input: std::option::Option<crate::types::JobParameters>,
    ) -> Self {
        self.job_parameters = input;
        self
    }
    /// Consumes the builder and constructs a [`InitiateJobInput`](crate::operation::initiate_job::InitiateJobInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::initiate_job::InitiateJobInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::initiate_job::InitiateJobInput {
            account_id: self.account_id,
            vault_name: self.vault_name,
            job_parameters: self.job_parameters,
        })
    }
}
