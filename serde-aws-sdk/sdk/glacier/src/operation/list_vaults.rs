// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl ListVaultsInput {
    /// Consumes the builder and constructs an Operation<[`ListVaults`](crate::operation::list_vaults::ListVaults)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::list_vaults::ListVaults,
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
        if self.account_id.as_deref().unwrap_or_default().is_empty() {
            self.account_id = Some("-".to_owned());
        }
        let mut request = {
            fn uri_base(
                _input: &crate::operation::list_vaults::ListVaultsInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                let input_1 = &_input.account_id;
                let input_1 = input_1.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "account_id",
                        "cannot be empty or unset",
                    )
                })?;
                let account_id = aws_smithy_http::label::fmt_string(
                    input_1,
                    aws_smithy_http::label::EncodingStrategy::Default,
                );
                if account_id.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "account_id",
                            "cannot be empty or unset",
                        ),
                    );
                }
                write!(output, "/{accountId}/vaults", accountId = account_id)
                    .expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::operation::list_vaults::ListVaultsInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                if let Some(inner_2) = &_input.marker {
                    {
                        query.push_kv("marker", &aws_smithy_http::query::fmt_string(&inner_2));
                    }
                }
                if let Some(inner_3) = &_input.limit {
                    if *inner_3 != 0 {
                        query.push_kv(
                            "limit",
                            aws_smithy_types::primitive::Encoder::from(*inner_3).encode(),
                        );
                    }
                }
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::list_vaults::ListVaultsInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
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
        request.http_mut().headers_mut().insert(
            "x-amz-glacier-version",
            http::HeaderValue::from_static("2012-06-01"),
        );
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
            crate::operation::list_vaults::ListVaults::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "ListVaults",
            "glacier",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `ListVaults`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct ListVaults;
impl ListVaults {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListVaults {
    type Output = std::result::Result<
        crate::operation::list_vaults::ListVaultsOutput,
        crate::operation::list_vaults::ListVaultsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_list_vaults::de_list_vaults_http_error(response)
        } else {
            crate::protocol_serde::shape_list_vaults::de_list_vaults_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type ListVaultsErrorKind = ListVaultsError;
/// Error type for the `ListVaultsError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListVaultsError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValueException(crate::types::error::InvalidParameterValueException),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValueException(crate::types::error::MissingParameterValueException),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailableException(crate::types::error::ServiceUnavailableException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for ListVaultsError {
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
impl std::fmt::Display for ListVaultsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameterValueException(_inner) => _inner.fmt(f),
            Self::MissingParameterValueException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::ServiceUnavailableException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for ListVaultsError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InvalidParameterValueException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::MissingParameterValueException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ServiceUnavailableException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::operation::list_vaults::ListVaultsError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListVaultsError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListVaultsError {
    /// Creates the `ListVaultsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `ListVaultsError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::InvalidParameterValueException(e) => e.meta(),
            Self::MissingParameterValueException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::ServiceUnavailableException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `ListVaultsError::InvalidParameterValueException`.
    pub fn is_invalid_parameter_value_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterValueException(_))
    }
    /// Returns `true` if the error kind is `ListVaultsError::MissingParameterValueException`.
    pub fn is_missing_parameter_value_exception(&self) -> bool {
        matches!(self, Self::MissingParameterValueException(_))
    }
    /// Returns `true` if the error kind is `ListVaultsError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `ListVaultsError::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(self, Self::ServiceUnavailableException(_))
    }
}
impl std::error::Error for ListVaultsError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidParameterValueException(_inner) => Some(_inner),
            Self::MissingParameterValueException(_inner) => Some(_inner),
            Self::ResourceNotFoundException(_inner) => Some(_inner),
            Self::ServiceUnavailableException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::list_vaults::_list_vaults_output::ListVaultsOutput;

pub use crate::operation::list_vaults::_list_vaults_input::ListVaultsInput;

mod _list_vaults_input;

mod _list_vaults_output;

/// Builders
pub mod builders;

/// Paginator for this operation
pub mod paginator;
