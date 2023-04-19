// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl ListEntitiesForPolicyInput {
    /// Consumes the builder and constructs an Operation<[`ListEntitiesForPolicy`](crate::operation::list_entities_for_policy::ListEntitiesForPolicy)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::list_entities_for_policy::ListEntitiesForPolicy,
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
                _input: &crate::operation::list_entities_for_policy::ListEntitiesForPolicyInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::list_entities_for_policy::ListEntitiesForPolicyInput,
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
                "application/x-www-form-urlencoded",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_list_entities_for_policy_input::ser_list_entities_for_policy_input_input(&self)?
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
            crate::operation::list_entities_for_policy::ListEntitiesForPolicy::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "ListEntitiesForPolicy",
            "iam",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `ListEntitiesForPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct ListEntitiesForPolicy;
impl ListEntitiesForPolicy {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEntitiesForPolicy {
    type Output = std::result::Result<
        crate::operation::list_entities_for_policy::ListEntitiesForPolicyOutput,
        crate::operation::list_entities_for_policy::ListEntitiesForPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_list_entities_for_policy::de_list_entities_for_policy_http_error(response)
        } else {
            crate::protocol_serde::shape_list_entities_for_policy::de_list_entities_for_policy_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type ListEntitiesForPolicyErrorKind = ListEntitiesForPolicyError;
/// Error type for the `ListEntitiesForPolicyError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListEntitiesForPolicyError {
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInputException(crate::types::error::InvalidInputException),
    /// <p>The request was rejected because it referenced a resource entity that does not exist. The error message describes the resource.</p>
    NoSuchEntityException(crate::types::error::NoSuchEntityException),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    ServiceFailureException(crate::types::error::ServiceFailureException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for ListEntitiesForPolicyError {
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
impl std::fmt::Display for ListEntitiesForPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInputException(_inner) => _inner.fmt(f),
            Self::NoSuchEntityException(_inner) => _inner.fmt(f),
            Self::ServiceFailureException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for ListEntitiesForPolicyError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InvalidInputException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NoSuchEntityException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ServiceFailureException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::list_entities_for_policy::ListEntitiesForPolicyError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListEntitiesForPolicyError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListEntitiesForPolicyError {
    /// Creates the `ListEntitiesForPolicyError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `ListEntitiesForPolicyError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::InvalidInputException(e) => e.meta(),
            Self::NoSuchEntityException(e) => e.meta(),
            Self::ServiceFailureException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `ListEntitiesForPolicyError::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(self, Self::InvalidInputException(_))
    }
    /// Returns `true` if the error kind is `ListEntitiesForPolicyError::NoSuchEntityException`.
    pub fn is_no_such_entity_exception(&self) -> bool {
        matches!(self, Self::NoSuchEntityException(_))
    }
    /// Returns `true` if the error kind is `ListEntitiesForPolicyError::ServiceFailureException`.
    pub fn is_service_failure_exception(&self) -> bool {
        matches!(self, Self::ServiceFailureException(_))
    }
}
impl std::error::Error for ListEntitiesForPolicyError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidInputException(_inner) => Some(_inner),
            Self::NoSuchEntityException(_inner) => Some(_inner),
            Self::ServiceFailureException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::list_entities_for_policy::_list_entities_for_policy_output::ListEntitiesForPolicyOutput;

pub use crate::operation::list_entities_for_policy::_list_entities_for_policy_input::ListEntitiesForPolicyInput;

mod _list_entities_for_policy_input;

mod _list_entities_for_policy_output;

/// Builders
pub mod builders;

/// Paginator for this operation
pub mod paginator;
