// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl StartCallAnalyticsStreamTranscriptionInput {
    /// Consumes the builder and constructs an Operation<[`StartCallAnalyticsStreamTranscription`](crate::operation::start_call_analytics_stream_transcription::StartCallAnalyticsStreamTranscription)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(self, _config: &crate::config::Config) -> std::result::Result<aws_smithy_http::operation::Operation<crate::operation::start_call_analytics_stream_transcription::StartCallAnalyticsStreamTranscription, aws_http::retry::AwsResponseRetryClassifier>, aws_smithy_http::operation::error::BuildError>{
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
                _input: &crate::operation::start_call_analytics_stream_transcription::StartCallAnalyticsStreamTranscriptionInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/call-analytics-stream-transcription")
                    .expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::start_call_analytics_stream_transcription::StartCallAnalyticsStreamTranscriptionInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                let builder = crate::protocol_serde::shape_start_call_analytics_stream_transcription::ser_start_call_analytics_stream_transcription_headers(input, builder)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/vnd.amazon.eventstream",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from({
            let error_marshaller = crate::event_stream_serde::AudioStreamErrorMarshaller::new();
            let marshaller = crate::event_stream_serde::AudioStreamMarshaller::new();
            let signer = _config.new_event_stream_signer(properties.clone());
            let adapter: aws_smithy_http::event_stream::MessageStreamAdapter<_, _> = self
                .audio_stream
                .into_body_stream(marshaller, error_marshaller, signer);
            let body: aws_smithy_http::body::SdkBody = hyper::Body::wrap_stream(adapter).into();
            body
        });
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
        request.properties_mut().insert(vec![http::Version::HTTP_2]);
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request
            .properties_mut()
            .insert(aws_sig_auth::signer::SignableBody::Bytes(&[]));
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
        let op = aws_smithy_http::operation::Operation::new(request, crate::operation::start_call_analytics_stream_transcription::StartCallAnalyticsStreamTranscription::new())
                            .with_metadata(aws_smithy_http::operation::Metadata::new("StartCallAnalyticsStreamTranscription", "transcribestreaming"));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `StartCallAnalyticsStreamTranscription`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct StartCallAnalyticsStreamTranscription;
impl StartCallAnalyticsStreamTranscription {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseHttpResponse for StartCallAnalyticsStreamTranscription {
    type Output = std::result::Result<crate::operation::start_call_analytics_stream_transcription::StartCallAnalyticsStreamTranscriptionOutput, crate::operation::start_call_analytics_stream_transcription::StartCallAnalyticsStreamTranscriptionError>;
    fn parse_unloaded(
        &self,
        response: &mut aws_smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::protocol_serde::shape_start_call_analytics_stream_transcription::de_start_call_analytics_stream_transcription_http_response(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::protocol_serde::shape_start_call_analytics_stream_transcription::de_start_call_analytics_stream_transcription_http_error(response)
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type StartCallAnalyticsStreamTranscriptionErrorKind =
    StartCallAnalyticsStreamTranscriptionError;
/// Error type for the `StartCallAnalyticsStreamTranscriptionError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum StartCallAnalyticsStreamTranscriptionError {
    /// <p>The service is currently unavailable. Try your request later.</p>
    ServiceUnavailableException(crate::types::error::ServiceUnavailableException),
    /// <p>One or more arguments to the <code>StartStreamTranscription</code>, <code>StartMedicalStreamTranscription</code>, or <code>StartCallAnalyticsStreamTranscription</code> operation was not valid. For example, <code>MediaEncoding</code> or <code>LanguageCode</code> used not valid values. Check the specified parameters and try your request again.</p>
    BadRequestException(crate::types::error::BadRequestException),
    /// <p>A problem occurred while processing the audio. Amazon Transcribe terminated processing.</p>
    InternalFailureException(crate::types::error::InternalFailureException),
    /// <p>A new stream started with the same session ID. The current stream has been terminated.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>Your client has exceeded one of the Amazon Transcribe limits. This is typically the audio length limit. Break your audio stream into smaller chunks and try your request again.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for StartCallAnalyticsStreamTranscriptionError {
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
impl std::fmt::Display for StartCallAnalyticsStreamTranscriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServiceUnavailableException(_inner) => _inner.fmt(f),
            Self::BadRequestException(_inner) => _inner.fmt(f),
            Self::InternalFailureException(_inner) => _inner.fmt(f),
            Self::ConflictException(_inner) => _inner.fmt(f),
            Self::LimitExceededException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata
    for StartCallAnalyticsStreamTranscriptionError
{
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ServiceUnavailableException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::BadRequestException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalFailureException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ConflictException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::LimitExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::operation::start_call_analytics_stream_transcription::StartCallAnalyticsStreamTranscriptionError {
                            fn request_id(&self) -> Option<&str> {
                                self.meta().request_id()
                            }
                        }
impl aws_smithy_types::retry::ProvideErrorKind for StartCallAnalyticsStreamTranscriptionError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl StartCallAnalyticsStreamTranscriptionError {
    /// Creates the `StartCallAnalyticsStreamTranscriptionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `StartCallAnalyticsStreamTranscriptionError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::ServiceUnavailableException(e) => e.meta(),
            Self::BadRequestException(e) => e.meta(),
            Self::InternalFailureException(e) => e.meta(),
            Self::ConflictException(e) => e.meta(),
            Self::LimitExceededException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `StartCallAnalyticsStreamTranscriptionError::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(self, Self::ServiceUnavailableException(_))
    }
    /// Returns `true` if the error kind is `StartCallAnalyticsStreamTranscriptionError::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(self, Self::BadRequestException(_))
    }
    /// Returns `true` if the error kind is `StartCallAnalyticsStreamTranscriptionError::InternalFailureException`.
    pub fn is_internal_failure_exception(&self) -> bool {
        matches!(self, Self::InternalFailureException(_))
    }
    /// Returns `true` if the error kind is `StartCallAnalyticsStreamTranscriptionError::ConflictException`.
    pub fn is_conflict_exception(&self) -> bool {
        matches!(self, Self::ConflictException(_))
    }
    /// Returns `true` if the error kind is `StartCallAnalyticsStreamTranscriptionError::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::LimitExceededException(_))
    }
}
impl std::error::Error for StartCallAnalyticsStreamTranscriptionError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ServiceUnavailableException(_inner) => Some(_inner),
            Self::BadRequestException(_inner) => Some(_inner),
            Self::InternalFailureException(_inner) => Some(_inner),
            Self::ConflictException(_inner) => Some(_inner),
            Self::LimitExceededException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::start_call_analytics_stream_transcription::_start_call_analytics_stream_transcription_output::StartCallAnalyticsStreamTranscriptionOutput;

pub use crate::operation::start_call_analytics_stream_transcription::_start_call_analytics_stream_transcription_input::StartCallAnalyticsStreamTranscriptionInput;

mod _start_call_analytics_stream_transcription_input;

mod _start_call_analytics_stream_transcription_output;

/// Builders
pub mod builders;
