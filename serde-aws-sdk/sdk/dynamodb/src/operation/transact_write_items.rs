// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl TransactWriteItemsInput {
    /// Consumes the builder and constructs an Operation<[`TransactWriteItems`](crate::operation::transact_write_items::TransactWriteItems)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::transact_write_items::TransactWriteItems,
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
        if self.client_request_token.is_none() {
            self.client_request_token = Some(_config.make_token.make_idempotency_token());
        }
        let mut request = {
            fn uri_base(
                _input: &crate::operation::transact_write_items::TransactWriteItemsInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::transact_write_items::TransactWriteItemsInput,
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
                "DynamoDB_20120810.TransactWriteItems",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_transact_write_items::ser_transact_write_items_input(
                &self,
            )?,
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
            crate::operation::transact_write_items::TransactWriteItems::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "TransactWriteItems",
            "dynamodb",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `TransactWriteItems`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct TransactWriteItems;
impl TransactWriteItems {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TransactWriteItems {
    type Output = std::result::Result<
        crate::operation::transact_write_items::TransactWriteItemsOutput,
        crate::operation::transact_write_items::TransactWriteItemsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_transact_write_items::de_transact_write_items_http_error(
                response,
            )
        } else {
            crate::protocol_serde::shape_transact_write_items::de_transact_write_items_http_response(
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
pub type TransactWriteItemsErrorKind = TransactWriteItemsError;
/// Error type for the `TransactWriteItemsError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum TransactWriteItemsError {
    /// <p>DynamoDB rejected the request because you retried a request with a different payload but with an idempotent token that was already used.</p>
    IdempotentParameterMismatchException(crate::types::error::IdempotentParameterMismatchException),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::types::error::InternalServerError),
    #[allow(missing_docs)] // documentation missing in model
    InvalidEndpointException(crate::types::error::InvalidEndpointException),
    /// <p>Your request rate is too high. The Amazon Web Services SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceededException(
        crate::types::error::ProvisionedThroughputExceededException,
    ),
    /// <p>Throughput exceeds the current throughput quota for your account. Please contact <a href="https://aws.amazon.com/support">Amazon Web Services Support</a> to request a quota increase.</p>
    RequestLimitExceeded(crate::types::error::RequestLimitExceeded),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The entire transaction request was canceled.</p>
    /// <p>DynamoDB cancels a <code>TransactWriteItems</code> request under the following circumstances:</p>
    /// <ul>
    /// <li> <p>A condition in one of the condition expressions is not met.</p> </li>
    /// <li> <p>A table in the <code>TransactWriteItems</code> request is in a different account or region.</p> </li>
    /// <li> <p>More than one action in the <code>TransactWriteItems</code> operation targets the same item.</p> </li>
    /// <li> <p>There is insufficient provisioned capacity for the transaction to be completed.</p> </li>
    /// <li> <p>An item size becomes too large (larger than 400 KB), or a local secondary index (LSI) becomes too large, or a similar validation error occurs because of changes made by the transaction.</p> </li>
    /// <li> <p>There is a user error, such as an invalid data format.</p> </li>
    /// </ul>
    /// <p>DynamoDB cancels a <code>TransactGetItems</code> request under the following circumstances:</p>
    /// <ul>
    /// <li> <p>There is an ongoing <code>TransactGetItems</code> operation that conflicts with a concurrent <code>PutItem</code>, <code>UpdateItem</code>, <code>DeleteItem</code> or <code>TransactWriteItems</code> request. In this case the <code>TransactGetItems</code> operation fails with a <code>TransactionCanceledException</code>.</p> </li>
    /// <li> <p>A table in the <code>TransactGetItems</code> request is in a different account or region.</p> </li>
    /// <li> <p>There is insufficient provisioned capacity for the transaction to be completed.</p> </li>
    /// <li> <p>There is a user error, such as an invalid data format.</p> </li>
    /// </ul> <note>
    /// <p>If using Java, DynamoDB lists the cancellation reasons on the <code>CancellationReasons</code> property. This property is not set for other languages. Transaction cancellation reasons are ordered in the order of requested items, if an item has no error it will have <code>None</code> code and <code>Null</code> message.</p>
    /// </note>
    /// <p>Cancellation reason codes and possible error messages:</p>
    /// <ul>
    /// <li> <p>No Errors:</p>
    /// <ul>
    /// <li> <p>Code: <code>None</code> </p> </li>
    /// <li> <p>Message: <code>null</code> </p> </li>
    /// </ul> </li>
    /// <li> <p>Conditional Check Failed:</p>
    /// <ul>
    /// <li> <p>Code: <code>ConditionalCheckFailed</code> </p> </li>
    /// <li> <p>Message: The conditional request failed. </p> </li>
    /// </ul> </li>
    /// <li> <p>Item Collection Size Limit Exceeded:</p>
    /// <ul>
    /// <li> <p>Code: <code>ItemCollectionSizeLimitExceeded</code> </p> </li>
    /// <li> <p>Message: Collection size exceeded.</p> </li>
    /// </ul> </li>
    /// <li> <p>Transaction Conflict:</p>
    /// <ul>
    /// <li> <p>Code: <code>TransactionConflict</code> </p> </li>
    /// <li> <p>Message: Transaction is ongoing for the item.</p> </li>
    /// </ul> </li>
    /// <li> <p>Provisioned Throughput Exceeded:</p>
    /// <ul>
    /// <li> <p>Code: <code>ProvisionedThroughputExceeded</code> </p> </li>
    /// <li> <p>Messages:</p>
    /// <ul>
    /// <li> <p>The level of configured provisioned throughput for the table was exceeded. Consider increasing your provisioning level with the UpdateTable API.</p> <note>
    /// <p>This Message is received when provisioned throughput is exceeded is on a provisioned DynamoDB table.</p>
    /// </note> </li>
    /// <li> <p>The level of configured provisioned throughput for one or more global secondary indexes of the table was exceeded. Consider increasing your provisioning level for the under-provisioned global secondary indexes with the UpdateTable API.</p> <note>
    /// <p>This message is returned when provisioned throughput is exceeded is on a provisioned GSI.</p>
    /// </note> </li>
    /// </ul> </li>
    /// </ul> </li>
    /// <li> <p>Throttling Error:</p>
    /// <ul>
    /// <li> <p>Code: <code>ThrottlingError</code> </p> </li>
    /// <li> <p>Messages: </p>
    /// <ul>
    /// <li> <p>Throughput exceeds the current capacity of your table or index. DynamoDB is automatically scaling your table or index so please try again shortly. If exceptions persist, check if you have a hot key: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/bp-partition-key-design.html.</p> <note>
    /// <p>This message is returned when writes get throttled on an On-Demand table as DynamoDB is automatically scaling the table.</p>
    /// </note> </li>
    /// <li> <p>Throughput exceeds the current capacity for one or more global secondary indexes. DynamoDB is automatically scaling your index so please try again shortly.</p> <note>
    /// <p>This message is returned when writes get throttled on an On-Demand GSI as DynamoDB is automatically scaling the GSI.</p>
    /// </note> </li>
    /// </ul> </li>
    /// </ul> </li>
    /// <li> <p>Validation Error:</p>
    /// <ul>
    /// <li> <p>Code: <code>ValidationError</code> </p> </li>
    /// <li> <p>Messages: </p>
    /// <ul>
    /// <li> <p>One or more parameter values were invalid.</p> </li>
    /// <li> <p>The update expression attempted to update the secondary index key beyond allowed size limits.</p> </li>
    /// <li> <p>The update expression attempted to update the secondary index key to unsupported type.</p> </li>
    /// <li> <p>An operand in the update expression has an incorrect data type.</p> </li>
    /// <li> <p>Item size to update has exceeded the maximum allowed size.</p> </li>
    /// <li> <p>Number overflow. Attempting to store a number with magnitude larger than supported range.</p> </li>
    /// <li> <p>Type mismatch for attribute to update.</p> </li>
    /// <li> <p>Nesting Levels have exceeded supported limits.</p> </li>
    /// <li> <p>The document path provided in the update expression is invalid for update.</p> </li>
    /// <li> <p>The provided expression refers to an attribute that does not exist in the item.</p> </li>
    /// </ul> </li>
    /// </ul> </li>
    /// </ul>
    TransactionCanceledException(crate::types::error::TransactionCanceledException),
    /// <p>The transaction with the given request token is already in progress.</p>
    /// <p> Recommended Settings </p> <note>
    /// <p> This is a general recommendation for handling the <code>TransactionInProgressException</code>. These settings help ensure that the client retries will trigger completion of the ongoing <code>TransactWriteItems</code> request. </p>
    /// </note>
    /// <ul>
    /// <li> <p> Set <code>clientExecutionTimeout</code> to a value that allows at least one retry to be processed after 5 seconds have elapsed since the first attempt for the <code>TransactWriteItems</code> operation. </p> </li>
    /// <li> <p> Set <code>socketTimeout</code> to a value a little lower than the <code>requestTimeout</code> setting. </p> </li>
    /// <li> <p> <code>requestTimeout</code> should be set based on the time taken for the individual retries of a single HTTP request for your use case, but setting it to 1 second or higher should work well to reduce chances of retries and <code>TransactionInProgressException</code> errors. </p> </li>
    /// <li> <p> Use exponential backoff when retrying and tune backoff if needed. </p> </li>
    /// </ul>
    /// <p> Assuming <a href="https://github.com/aws/aws-sdk-java/blob/fd409dee8ae23fb8953e0bb4dbde65536a7e0514/aws-java-sdk-core/src/main/java/com/amazonaws/retry/PredefinedRetryPolicies.java#L97">default retry policy</a>, example timeout settings based on the guidelines above are as follows: </p>
    /// <p>Example timeline:</p>
    /// <ul>
    /// <li> <p>0-1000 first attempt</p> </li>
    /// <li> <p>1000-1500 first sleep/delay (default retry policy uses 500 ms as base delay for 4xx errors)</p> </li>
    /// <li> <p>1500-2500 second attempt</p> </li>
    /// <li> <p>2500-3500 second sleep/delay (500 * 2, exponential backoff)</p> </li>
    /// <li> <p>3500-4500 third attempt</p> </li>
    /// <li> <p>4500-6500 third sleep/delay (500 * 2^2)</p> </li>
    /// <li> <p>6500-7500 fourth attempt (this can trigger inline recovery since 5 seconds have elapsed since the first attempt reached TC)</p> </li>
    /// </ul>
    TransactionInProgressException(crate::types::error::TransactionInProgressException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for TransactWriteItemsError {
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
impl std::fmt::Display for TransactWriteItemsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdempotentParameterMismatchException(_inner) => _inner.fmt(f),
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::InvalidEndpointException(_inner) => _inner.fmt(f),
            Self::ProvisionedThroughputExceededException(_inner) => _inner.fmt(f),
            Self::RequestLimitExceeded(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::TransactionCanceledException(_inner) => _inner.fmt(f),
            Self::TransactionInProgressException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for TransactWriteItemsError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::IdempotentParameterMismatchException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServerError(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidEndpointException(_inner) => {
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
            Self::TransactionCanceledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TransactionInProgressException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::transact_write_items::TransactWriteItemsError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for TransactWriteItemsError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl TransactWriteItemsError {
    /// Creates the `TransactWriteItemsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `TransactWriteItemsError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::IdempotentParameterMismatchException(e) => e.meta(),
            Self::InternalServerError(e) => e.meta(),
            Self::InvalidEndpointException(e) => e.meta(),
            Self::ProvisionedThroughputExceededException(e) => e.meta(),
            Self::RequestLimitExceeded(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::TransactionCanceledException(e) => e.meta(),
            Self::TransactionInProgressException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `TransactWriteItemsError::IdempotentParameterMismatchException`.
    pub fn is_idempotent_parameter_mismatch_exception(&self) -> bool {
        matches!(self, Self::IdempotentParameterMismatchException(_))
    }
    /// Returns `true` if the error kind is `TransactWriteItemsError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `TransactWriteItemsError::InvalidEndpointException`.
    pub fn is_invalid_endpoint_exception(&self) -> bool {
        matches!(self, Self::InvalidEndpointException(_))
    }
    /// Returns `true` if the error kind is `TransactWriteItemsError::ProvisionedThroughputExceededException`.
    pub fn is_provisioned_throughput_exceeded_exception(&self) -> bool {
        matches!(self, Self::ProvisionedThroughputExceededException(_))
    }
    /// Returns `true` if the error kind is `TransactWriteItemsError::RequestLimitExceeded`.
    pub fn is_request_limit_exceeded(&self) -> bool {
        matches!(self, Self::RequestLimitExceeded(_))
    }
    /// Returns `true` if the error kind is `TransactWriteItemsError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `TransactWriteItemsError::TransactionCanceledException`.
    pub fn is_transaction_canceled_exception(&self) -> bool {
        matches!(self, Self::TransactionCanceledException(_))
    }
    /// Returns `true` if the error kind is `TransactWriteItemsError::TransactionInProgressException`.
    pub fn is_transaction_in_progress_exception(&self) -> bool {
        matches!(self, Self::TransactionInProgressException(_))
    }
}
impl std::error::Error for TransactWriteItemsError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IdempotentParameterMismatchException(_inner) => Some(_inner),
            Self::InternalServerError(_inner) => Some(_inner),
            Self::InvalidEndpointException(_inner) => Some(_inner),
            Self::ProvisionedThroughputExceededException(_inner) => Some(_inner),
            Self::RequestLimitExceeded(_inner) => Some(_inner),
            Self::ResourceNotFoundException(_inner) => Some(_inner),
            Self::TransactionCanceledException(_inner) => Some(_inner),
            Self::TransactionInProgressException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::transact_write_items::_transact_write_items_output::TransactWriteItemsOutput;

pub use crate::operation::transact_write_items::_transact_write_items_input::TransactWriteItemsInput;

mod _transact_write_items_input;

mod _transact_write_items_output;

/// Builders
pub mod builders;
