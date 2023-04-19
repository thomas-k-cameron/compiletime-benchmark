// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl PutOrganizationConfigRuleInput {
    /// Consumes the builder and constructs an Operation<[`PutOrganizationConfigRule`](crate::operation::put_organization_config_rule::PutOrganizationConfigRule)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::put_organization_config_rule::PutOrganizationConfigRule,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::operation::put_organization_config_rule::PutOrganizationConfigRuleInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::put_organization_config_rule::PutOrganizationConfigRuleInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "StarlingDoveService.PutOrganizationConfigRule",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_put_organization_config_rule::ser_put_organization_config_rule_input(&self)?
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::put_organization_config_rule::PutOrganizationConfigRule::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "PutOrganizationConfigRule",
            "configservice",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `PutOrganizationConfigRule`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct PutOrganizationConfigRule;
impl PutOrganizationConfigRule {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutOrganizationConfigRule {
    type Output = std::result::Result<
        crate::operation::put_organization_config_rule::PutOrganizationConfigRuleOutput,
        crate::operation::put_organization_config_rule::PutOrganizationConfigRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_put_organization_config_rule::de_put_organization_config_rule_http_error(response)
        } else {
            crate::protocol_serde::shape_put_organization_config_rule::de_put_organization_config_rule_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type PutOrganizationConfigRuleErrorKind = PutOrganizationConfigRuleError;
/// Error type for the `PutOrganizationConfigRuleError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutOrganizationConfigRuleError {
    /// <p>Indicates one of the following errors:</p>
    /// <ul>
    /// <li> <p>For PutConfigRule, the rule cannot be created because the IAM role assigned to Config lacks permissions to perform the config:Put* action.</p> </li>
    /// <li> <p>For PutConfigRule, the Lambda function cannot be invoked. Check the function ARN, and check the function's permissions.</p> </li>
    /// <li> <p>For PutOrganizationConfigRule, organization Config rule cannot be created because you do not have permissions to call IAM <code>GetRole</code> action or create a service-linked role.</p> </li>
    /// <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack cannot be created because you do not have the following permissions: </p>
    /// <ul>
    /// <li> <p>You do not have permission to call IAM <code>GetRole</code> action or create a service-linked role.</p> </li>
    /// <li> <p>You do not have permission to read Amazon S3 bucket or call SSM:GetDocument.</p> </li>
    /// </ul> </li>
    /// </ul>
    InsufficientPermissionsException(crate::types::error::InsufficientPermissionsException),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValueException(crate::types::error::InvalidParameterValueException),
    /// <p>You have reached the limit of the number of organization Config rules you can create. For more information, see see <a href="https://docs.aws.amazon.com/config/latest/developerguide/configlimits.html"> <b>Service Limits</b> </a> in the Config Developer Guide.</p>
    MaxNumberOfOrganizationConfigRulesExceededException(
        crate::types::error::MaxNumberOfOrganizationConfigRulesExceededException,
    ),
    /// <p>Organization is no longer available.</p>
    NoAvailableOrganizationException(crate::types::error::NoAvailableOrganizationException),
    /// <p>For <code>PutConfigurationAggregator</code> API, you can see this exception for the following reasons:</p>
    /// <ul>
    /// <li> <p>No permission to call <code>EnableAWSServiceAccess</code> API</p> </li>
    /// <li> <p>The configuration aggregator cannot be updated because your Amazon Web Services Organization management account or the delegated administrator role changed. Delete this aggregator and create a new one with the current Amazon Web Services Organization.</p> </li>
    /// <li> <p>The configuration aggregator is associated with a previous Amazon Web Services Organization and Config cannot aggregate data with current Amazon Web Services Organization. Delete this aggregator and create a new one with the current Amazon Web Services Organization.</p> </li>
    /// <li> <p>You are not a registered delegated administrator for Config with permissions to call <code>ListDelegatedAdministrators</code> API. Ensure that the management account registers delagated administrator for Config service principle name before the delegated administrator creates an aggregator.</p> </li>
    /// </ul>
    /// <p>For all <code>OrganizationConfigRule</code> and <code>OrganizationConformancePack</code> APIs, Config throws an exception if APIs are called from member accounts. All APIs must be called from organization management account.</p>
    OrganizationAccessDeniedException(crate::types::error::OrganizationAccessDeniedException),
    /// <p>Config resource cannot be created because your organization does not have all features enabled.</p>
    OrganizationAllFeaturesNotEnabledException(
        crate::types::error::OrganizationAllFeaturesNotEnabledException,
    ),
    /// <p>You see this exception in the following cases: </p>
    /// <ul>
    /// <li> <p>For DeleteConfigRule, Config is deleting this rule. Try your request again later.</p> </li>
    /// <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li>
    /// <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li>
    /// <li> <p>For PutConfigOrganizationRule, organization Config rule deletion is in progress. Try your request again later.</p> </li>
    /// <li> <p>For DeleteOrganizationConfigRule, organization Config rule creation is in progress. Try your request again later.</p> </li>
    /// <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li>
    /// <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li>
    /// </ul>
    ResourceInUseException(crate::types::error::ResourceInUseException),
    /// <p>The requested action is invalid.</p>
    /// <p>For PutStoredQuery, you will see this exception if there are missing required fields or if the input value fails the validation, or if you are trying to create more than 300 queries.</p>
    /// <p>For GetStoredQuery, ListStoredQuery, and DeleteStoredQuery you will see this exception if there are missing required fields or if the input value fails the validation.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for PutOrganizationConfigRuleError {
    fn create_unhandled_error(
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl std::fmt::Display for PutOrganizationConfigRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientPermissionsException(_inner) => _inner.fmt(f),
            Self::InvalidParameterValueException(_inner) => _inner.fmt(f),
            Self::MaxNumberOfOrganizationConfigRulesExceededException(_inner) => _inner.fmt(f),
            Self::NoAvailableOrganizationException(_inner) => _inner.fmt(f),
            Self::OrganizationAccessDeniedException(_inner) => _inner.fmt(f),
            Self::OrganizationAllFeaturesNotEnabledException(_inner) => _inner.fmt(f),
            Self::ResourceInUseException(_inner) => _inner.fmt(f),
            Self::ValidationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for PutOrganizationConfigRuleError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InsufficientPermissionsException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidParameterValueException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::MaxNumberOfOrganizationConfigRulesExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NoAvailableOrganizationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::OrganizationAccessDeniedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::OrganizationAllFeaturesNotEnabledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceInUseException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ValidationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::put_organization_config_rule::PutOrganizationConfigRuleError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutOrganizationConfigRuleError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutOrganizationConfigRuleError {
    /// Creates the `PutOrganizationConfigRuleError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `PutOrganizationConfigRuleError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err.clone())
                .meta(err)
                .build(),
        )
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::InsufficientPermissionsException(e) => e.meta(),
            Self::InvalidParameterValueException(e) => e.meta(),
            Self::MaxNumberOfOrganizationConfigRulesExceededException(e) => e.meta(),
            Self::NoAvailableOrganizationException(e) => e.meta(),
            Self::OrganizationAccessDeniedException(e) => e.meta(),
            Self::OrganizationAllFeaturesNotEnabledException(e) => e.meta(),
            Self::ResourceInUseException(e) => e.meta(),
            Self::ValidationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `PutOrganizationConfigRuleError::InsufficientPermissionsException`.
    pub fn is_insufficient_permissions_exception(&self) -> bool {
        matches!(self, Self::InsufficientPermissionsException(_))
    }
    /// Returns `true` if the error kind is `PutOrganizationConfigRuleError::InvalidParameterValueException`.
    pub fn is_invalid_parameter_value_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterValueException(_))
    }
    /// Returns `true` if the error kind is `PutOrganizationConfigRuleError::MaxNumberOfOrganizationConfigRulesExceededException`.
    pub fn is_max_number_of_organization_config_rules_exceeded_exception(&self) -> bool {
        matches!(
            self,
            Self::MaxNumberOfOrganizationConfigRulesExceededException(_)
        )
    }
    /// Returns `true` if the error kind is `PutOrganizationConfigRuleError::NoAvailableOrganizationException`.
    pub fn is_no_available_organization_exception(&self) -> bool {
        matches!(self, Self::NoAvailableOrganizationException(_))
    }
    /// Returns `true` if the error kind is `PutOrganizationConfigRuleError::OrganizationAccessDeniedException`.
    pub fn is_organization_access_denied_exception(&self) -> bool {
        matches!(self, Self::OrganizationAccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `PutOrganizationConfigRuleError::OrganizationAllFeaturesNotEnabledException`.
    pub fn is_organization_all_features_not_enabled_exception(&self) -> bool {
        matches!(self, Self::OrganizationAllFeaturesNotEnabledException(_))
    }
    /// Returns `true` if the error kind is `PutOrganizationConfigRuleError::ResourceInUseException`.
    pub fn is_resource_in_use_exception(&self) -> bool {
        matches!(self, Self::ResourceInUseException(_))
    }
    /// Returns `true` if the error kind is `PutOrganizationConfigRuleError::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(self, Self::ValidationException(_))
    }
}
impl std::error::Error for PutOrganizationConfigRuleError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InsufficientPermissionsException(_inner) => Some(_inner),
            Self::InvalidParameterValueException(_inner) => Some(_inner),
            Self::MaxNumberOfOrganizationConfigRulesExceededException(_inner) => Some(_inner),
            Self::NoAvailableOrganizationException(_inner) => Some(_inner),
            Self::OrganizationAccessDeniedException(_inner) => Some(_inner),
            Self::OrganizationAllFeaturesNotEnabledException(_inner) => Some(_inner),
            Self::ResourceInUseException(_inner) => Some(_inner),
            Self::ValidationException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::put_organization_config_rule::_put_organization_config_rule_output::PutOrganizationConfigRuleOutput;

pub use crate::operation::put_organization_config_rule::_put_organization_config_rule_input::PutOrganizationConfigRuleInput;

mod _put_organization_config_rule_input;

mod _put_organization_config_rule_output;

/// Builders
pub mod builders;
