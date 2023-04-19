// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl DisassociateDefaultViewInput {
    /// Consumes the builder and constructs an Operation<[`DisassociateDefaultView`](crate::operation::disassociate_default_view::DisassociateDefaultView)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::disassociate_default_view::DisassociateDefaultView,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
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
                _input: &crate::operation::disassociate_default_view::DisassociateDefaultViewInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/DisassociateDefaultView").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::disassociate_default_view::DisassociateDefaultViewInput,
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
            crate::operation::disassociate_default_view::DisassociateDefaultView::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DisassociateDefaultView",
            "resourceexplorer2",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `DisassociateDefaultView`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct DisassociateDefaultView;
impl DisassociateDefaultView {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateDefaultView {
    type Output = std::result::Result<
        crate::operation::disassociate_default_view::DisassociateDefaultViewOutput,
        crate::operation::disassociate_default_view::DisassociateDefaultViewError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_disassociate_default_view::de_disassociate_default_view_http_error(response)
        } else {
            crate::protocol_serde::shape_disassociate_default_view::de_disassociate_default_view_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type DisassociateDefaultViewErrorKind = DisassociateDefaultViewError;
/// Error type for the `DisassociateDefaultViewError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DisassociateDefaultViewError {
    /// <p>The credentials that you used to call this operation don't have the minimum required permissions.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>The request failed because of internal service error. Try your request again later.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The request failed because you exceeded a rate limit for this operation. For more information, see <a href="https://docs.aws.amazon.com/arexug/mainline/quotas.html">Quotas for Resource Explorer</a>.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>You provided an invalid value for one of the operation's parameters. Check the syntax for the operation, and try again.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for DisassociateDefaultViewError {
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
impl std::fmt::Display for DisassociateDefaultViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccessDeniedException(_inner) => _inner.fmt(f),
            Self::InternalServerException(_inner) => _inner.fmt(f),
            Self::ThrottlingException(_inner) => _inner.fmt(f),
            Self::ValidationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for DisassociateDefaultViewError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AccessDeniedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServerException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ThrottlingException(_inner) => {
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
    for crate::operation::disassociate_default_view::DisassociateDefaultViewError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DisassociateDefaultViewError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DisassociateDefaultViewError {
    /// Creates the `DisassociateDefaultViewError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `DisassociateDefaultViewError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::AccessDeniedException(e) => e.meta(),
            Self::InternalServerException(e) => e.meta(),
            Self::ThrottlingException(e) => e.meta(),
            Self::ValidationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `DisassociateDefaultViewError::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(self, Self::AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `DisassociateDefaultViewError::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(self, Self::InternalServerException(_))
    }
    /// Returns `true` if the error kind is `DisassociateDefaultViewError::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(self, Self::ThrottlingException(_))
    }
    /// Returns `true` if the error kind is `DisassociateDefaultViewError::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(self, Self::ValidationException(_))
    }
}
impl std::error::Error for DisassociateDefaultViewError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AccessDeniedException(_inner) => Some(_inner),
            Self::InternalServerException(_inner) => Some(_inner),
            Self::ThrottlingException(_inner) => Some(_inner),
            Self::ValidationException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::disassociate_default_view::_disassociate_default_view_output::DisassociateDefaultViewOutput;

pub use crate::operation::disassociate_default_view::_disassociate_default_view_input::DisassociateDefaultViewInput;

mod _disassociate_default_view_input;

mod _disassociate_default_view_output;

/// Builders
pub mod builders;
