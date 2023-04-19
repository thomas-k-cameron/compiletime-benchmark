// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl ExecuteStatementInput {
    /// Consumes the builder and constructs an Operation<[`ExecuteStatement`](crate::operation::execute_statement::ExecuteStatement)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::execute_statement::ExecuteStatement,
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
                _input: &crate::operation::execute_statement::ExecuteStatementInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::execute_statement::ExecuteStatementInput,
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
                "application/x-amz-json-1.0",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "DynamoDB_20120810.ExecuteStatement",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_execute_statement::ser_execute_statement_input(&self)?,
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
            crate::operation::execute_statement::ExecuteStatement::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "ExecuteStatement",
            "dynamodb",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `ExecuteStatement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct ExecuteStatement;
impl ExecuteStatement {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExecuteStatement {
    type Output = std::result::Result<
        crate::operation::execute_statement::ExecuteStatementOutput,
        crate::operation::execute_statement::ExecuteStatementError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_execute_statement::de_execute_statement_http_error(
                response,
            )
        } else {
            crate::protocol_serde::shape_execute_statement::de_execute_statement_http_response(
                response,
            )
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type ExecuteStatementErrorKind = ExecuteStatementError;
/// Error type for the `ExecuteStatementError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ExecuteStatementError {
    /// <p>A condition specified in the operation could not be evaluated.</p>
    ConditionalCheckFailedException(crate::types::error::ConditionalCheckFailedException),
    /// <p> There was an attempt to insert an item with the same primary key as an item that already exists in the DynamoDB table.</p>
    DuplicateItemException(crate::types::error::DuplicateItemException),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::types::error::InternalServerError),
    /// <p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
    ItemCollectionSizeLimitExceededException(
        crate::types::error::ItemCollectionSizeLimitExceededException,
    ),
    /// <p>Your request rate is too high. The Amazon Web Services SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceededException(
        crate::types::error::ProvisionedThroughputExceededException,
    ),
    /// <p>Throughput exceeds the current throughput quota for your account. Please contact <a href="https://aws.amazon.com/support">Amazon Web Services Support</a> to request a quota increase.</p>
    RequestLimitExceeded(crate::types::error::RequestLimitExceeded),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>Operation was rejected because there is an ongoing transaction for the item.</p>
    TransactionConflictException(crate::types::error::TransactionConflictException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for ExecuteStatementError {
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
impl std::fmt::Display for ExecuteStatementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConditionalCheckFailedException(_inner) => _inner.fmt(f),
            Self::DuplicateItemException(_inner) => _inner.fmt(f),
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::ItemCollectionSizeLimitExceededException(_inner) => _inner.fmt(f),
            Self::ProvisionedThroughputExceededException(_inner) => _inner.fmt(f),
            Self::RequestLimitExceeded(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::TransactionConflictException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for ExecuteStatementError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ConditionalCheckFailedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DuplicateItemException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServerError(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ItemCollectionSizeLimitExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ProvisionedThroughputExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::RequestLimitExceeded(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TransactionConflictException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::execute_statement::ExecuteStatementError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ExecuteStatementError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ExecuteStatementError {
    /// Creates the `ExecuteStatementError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `ExecuteStatementError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::ConditionalCheckFailedException(e) => e.meta(),
            Self::DuplicateItemException(e) => e.meta(),
            Self::InternalServerError(e) => e.meta(),
            Self::ItemCollectionSizeLimitExceededException(e) => e.meta(),
            Self::ProvisionedThroughputExceededException(e) => e.meta(),
            Self::RequestLimitExceeded(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::TransactionConflictException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `ExecuteStatementError::ConditionalCheckFailedException`.
    pub fn is_conditional_check_failed_exception(&self) -> bool {
        matches!(self, Self::ConditionalCheckFailedException(_))
    }
    /// Returns `true` if the error kind is `ExecuteStatementError::DuplicateItemException`.
    pub fn is_duplicate_item_exception(&self) -> bool {
        matches!(self, Self::DuplicateItemException(_))
    }
    /// Returns `true` if the error kind is `ExecuteStatementError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `ExecuteStatementError::ItemCollectionSizeLimitExceededException`.
    pub fn is_item_collection_size_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::ItemCollectionSizeLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `ExecuteStatementError::ProvisionedThroughputExceededException`.
    pub fn is_provisioned_throughput_exceeded_exception(&self) -> bool {
        matches!(self, Self::ProvisionedThroughputExceededException(_))
    }
    /// Returns `true` if the error kind is `ExecuteStatementError::RequestLimitExceeded`.
    pub fn is_request_limit_exceeded(&self) -> bool {
        matches!(self, Self::RequestLimitExceeded(_))
    }
    /// Returns `true` if the error kind is `ExecuteStatementError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `ExecuteStatementError::TransactionConflictException`.
    pub fn is_transaction_conflict_exception(&self) -> bool {
        matches!(self, Self::TransactionConflictException(_))
    }
}
impl std::error::Error for ExecuteStatementError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConditionalCheckFailedException(_inner) => Some(_inner),
            Self::DuplicateItemException(_inner) => Some(_inner),
            Self::InternalServerError(_inner) => Some(_inner),
            Self::ItemCollectionSizeLimitExceededException(_inner) => Some(_inner),
            Self::ProvisionedThroughputExceededException(_inner) => Some(_inner),
            Self::RequestLimitExceeded(_inner) => Some(_inner),
            Self::ResourceNotFoundException(_inner) => Some(_inner),
            Self::TransactionConflictException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::execute_statement::_execute_statement_output::ExecuteStatementOutput;

pub use crate::operation::execute_statement::_execute_statement_input::ExecuteStatementInput;

mod _execute_statement_input;

mod _execute_statement_output;

/// Builders
pub mod builders;
