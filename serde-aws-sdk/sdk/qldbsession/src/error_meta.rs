// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequestException(crate::types::error::BadRequestException),
    /// <p>Returned when the request exceeds the processing capacity of the ledger.</p>
    CapacityExceededException(crate::types::error::CapacityExceededException),
    /// <p>Returned if the session doesn't exist anymore because it timed out or expired.</p>
    InvalidSessionException(crate::types::error::InvalidSessionException),
    /// <p>Returned if a resource limit such as number of active sessions is exceeded.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// <p>Returned when a transaction cannot be written to the journal due to a failure in the verification phase of <i>optimistic concurrency control</i> (OCC).</p>
    OccConflictException(crate::types::error::OccConflictException),
    /// <p>Returned when the rate of requests exceeds the allowed throughput.</p>
    RateExceededException(crate::types::error::RateExceededException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::CapacityExceededException(inner) => inner.fmt(f),
            Error::InvalidSessionException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::OccConflictException(inner) => inner.fmt(f),
            Error::RateExceededException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::send_command::SendCommandError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::send_command::SendCommandError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::send_command::SendCommandError> for Error {
    fn from(err: crate::operation::send_command::SendCommandError) -> Self {
        match err {
            crate::operation::send_command::SendCommandError::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::operation::send_command::SendCommandError::CapacityExceededException(inner) => {
                Error::CapacityExceededException(inner)
            }
            crate::operation::send_command::SendCommandError::InvalidSessionException(inner) => {
                Error::InvalidSessionException(inner)
            }
            crate::operation::send_command::SendCommandError::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::operation::send_command::SendCommandError::OccConflictException(inner) => {
                Error::OccConflictException(inner)
            }
            crate::operation::send_command::SendCommandError::RateExceededException(inner) => {
                Error::RateExceededException(inner)
            }
            crate::operation::send_command::SendCommandError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl std::error::Error for Error {}
impl aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::BadRequestException(e) => e.request_id(),
            Self::CapacityExceededException(e) => e.request_id(),
            Self::InvalidSessionException(e) => e.request_id(),
            Self::LimitExceededException(e) => e.request_id(),
            Self::OccConflictException(e) => e.request_id(),
            Self::RateExceededException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
