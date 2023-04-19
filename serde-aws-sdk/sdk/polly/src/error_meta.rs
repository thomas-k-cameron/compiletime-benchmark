// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>This engine is not compatible with the voice that you have designated. Choose a new voice that is compatible with the engine or change the engine and restart the operation.</p>
    EngineNotSupportedException(crate::types::error::EngineNotSupportedException),
    /// <p>Amazon Polly can't find the specified lexicon. Verify that the lexicon's name is spelled correctly, and then try again.</p>
    InvalidLexiconException(crate::types::error::InvalidLexiconException),
    /// <p>The NextToken is invalid. Verify that it's spelled correctly, and then try again.</p>
    InvalidNextTokenException(crate::types::error::InvalidNextTokenException),
    /// <p>The provided Amazon S3 bucket name is invalid. Please check your input with S3 bucket naming requirements and try again.</p>
    InvalidS3BucketException(crate::types::error::InvalidS3BucketException),
    /// <p>The provided Amazon S3 key prefix is invalid. Please provide a valid S3 object key name.</p>
    InvalidS3KeyException(crate::types::error::InvalidS3KeyException),
    /// <p>The specified sample rate is not valid.</p>
    InvalidSampleRateException(crate::types::error::InvalidSampleRateException),
    /// <p>The provided SNS topic ARN is invalid. Please provide a valid SNS topic ARN and try again.</p>
    InvalidSnsTopicArnException(crate::types::error::InvalidSnsTopicArnException),
    /// <p>The SSML you provided is invalid. Verify the SSML syntax, spelling of tags and values, and then try again.</p>
    InvalidSsmlException(crate::types::error::InvalidSsmlException),
    /// <p>The provided Task ID is not valid. Please provide a valid Task ID and try again.</p>
    InvalidTaskIdException(crate::types::error::InvalidTaskIdException),
    /// <p>The language specified is not currently supported by Amazon Polly in this capacity.</p>
    LanguageNotSupportedException(crate::types::error::LanguageNotSupportedException),
    /// <p>Amazon Polly can't find the specified lexicon. This could be caused by a lexicon that is missing, its name is misspelled or specifying a lexicon that is in a different region.</p>
    /// <p>Verify that the lexicon exists, is in the region (see <code>ListLexicons</code>) and that you spelled its name is spelled correctly. Then try again.</p>
    LexiconNotFoundException(crate::types::error::LexiconNotFoundException),
    /// <p>The maximum size of the specified lexicon would be exceeded by this operation.</p>
    LexiconSizeExceededException(crate::types::error::LexiconSizeExceededException),
    /// <p>Speech marks are not supported for the <code>OutputFormat</code> selected. Speech marks are only available for content in <code>json</code> format.</p>
    MarksNotSupportedForFormatException(crate::types::error::MarksNotSupportedForFormatException),
    /// <p>The maximum size of the lexeme would be exceeded by this operation.</p>
    MaxLexemeLengthExceededException(crate::types::error::MaxLexemeLengthExceededException),
    /// <p>The maximum number of lexicons would be exceeded by this operation.</p>
    MaxLexiconsNumberExceededException(crate::types::error::MaxLexiconsNumberExceededException),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailureException(crate::types::error::ServiceFailureException),
    /// <p>SSML speech marks are not supported for plain text-type input.</p>
    SsmlMarksNotSupportedForTextTypeException(
        crate::types::error::SsmlMarksNotSupportedForTextTypeException,
    ),
    /// <p>The Speech Synthesis task with requested Task ID cannot be found.</p>
    SynthesisTaskNotFoundException(crate::types::error::SynthesisTaskNotFoundException),
    /// <p>The value of the "Text" parameter is longer than the accepted limits. For the <code>SynthesizeSpeech</code> API, the limit for input text is a maximum of 6000 characters total, of which no more than 3000 can be billed characters. For the <code>StartSpeechSynthesisTask</code> API, the maximum is 200,000 characters, of which no more than 100,000 can be billed characters. SSML tags are not counted as billed characters.</p>
    TextLengthExceededException(crate::types::error::TextLengthExceededException),
    /// <p>The alphabet specified by the lexicon is not a supported alphabet. Valid values are <code>x-sampa</code> and <code>ipa</code>.</p>
    UnsupportedPlsAlphabetException(crate::types::error::UnsupportedPlsAlphabetException),
    /// <p>The language specified in the lexicon is unsupported. For a list of supported languages, see <a href="https://docs.aws.amazon.com/polly/latest/dg/API_LexiconAttributes.html">Lexicon Attributes</a>.</p>
    UnsupportedPlsLanguageException(crate::types::error::UnsupportedPlsLanguageException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EngineNotSupportedException(inner) => inner.fmt(f),
            Error::InvalidLexiconException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::InvalidS3BucketException(inner) => inner.fmt(f),
            Error::InvalidS3KeyException(inner) => inner.fmt(f),
            Error::InvalidSampleRateException(inner) => inner.fmt(f),
            Error::InvalidSnsTopicArnException(inner) => inner.fmt(f),
            Error::InvalidSsmlException(inner) => inner.fmt(f),
            Error::InvalidTaskIdException(inner) => inner.fmt(f),
            Error::LanguageNotSupportedException(inner) => inner.fmt(f),
            Error::LexiconNotFoundException(inner) => inner.fmt(f),
            Error::LexiconSizeExceededException(inner) => inner.fmt(f),
            Error::MarksNotSupportedForFormatException(inner) => inner.fmt(f),
            Error::MaxLexemeLengthExceededException(inner) => inner.fmt(f),
            Error::MaxLexiconsNumberExceededException(inner) => inner.fmt(f),
            Error::ServiceFailureException(inner) => inner.fmt(f),
            Error::SsmlMarksNotSupportedForTextTypeException(inner) => inner.fmt(f),
            Error::SynthesisTaskNotFoundException(inner) => inner.fmt(f),
            Error::TextLengthExceededException(inner) => inner.fmt(f),
            Error::UnsupportedPlsAlphabetException(inner) => inner.fmt(f),
            Error::UnsupportedPlsLanguageException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::operation::delete_lexicon::DeleteLexiconError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::delete_lexicon::DeleteLexiconError,
            R,
        >,
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
impl From<crate::operation::delete_lexicon::DeleteLexiconError> for Error {
    fn from(err: crate::operation::delete_lexicon::DeleteLexiconError) -> Self {
        match err {
            crate::operation::delete_lexicon::DeleteLexiconError::LexiconNotFoundException(
                inner,
            ) => Error::LexiconNotFoundException(inner),
            crate::operation::delete_lexicon::DeleteLexiconError::ServiceFailureException(
                inner,
            ) => Error::ServiceFailureException(inner),
            crate::operation::delete_lexicon::DeleteLexiconError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::describe_voices::DescribeVoicesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::describe_voices::DescribeVoicesError,
            R,
        >,
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
impl From<crate::operation::describe_voices::DescribeVoicesError> for Error {
    fn from(err: crate::operation::describe_voices::DescribeVoicesError) -> Self {
        match err {
            crate::operation::describe_voices::DescribeVoicesError::InvalidNextTokenException(
                inner,
            ) => Error::InvalidNextTokenException(inner),
            crate::operation::describe_voices::DescribeVoicesError::ServiceFailureException(
                inner,
            ) => Error::ServiceFailureException(inner),
            crate::operation::describe_voices::DescribeVoicesError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::get_lexicon::GetLexiconError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::get_lexicon::GetLexiconError, R>,
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
impl From<crate::operation::get_lexicon::GetLexiconError> for Error {
    fn from(err: crate::operation::get_lexicon::GetLexiconError) -> Self {
        match err {
            crate::operation::get_lexicon::GetLexiconError::LexiconNotFoundException(inner) => {
                Error::LexiconNotFoundException(inner)
            }
            crate::operation::get_lexicon::GetLexiconError::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::operation::get_lexicon::GetLexiconError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError,
            R,
        >,
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
impl From<crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError> for Error {
    fn from(err: crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError) -> Self {
        match err {
            crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError::InvalidTaskIdException(inner) => Error::InvalidTaskIdException(inner),
            crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
            crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError::SynthesisTaskNotFoundException(inner) => Error::SynthesisTaskNotFoundException(inner),
            crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::operation::list_lexicons::ListLexiconsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::list_lexicons::ListLexiconsError,
            R,
        >,
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
impl From<crate::operation::list_lexicons::ListLexiconsError> for Error {
    fn from(err: crate::operation::list_lexicons::ListLexiconsError) -> Self {
        match err {
            crate::operation::list_lexicons::ListLexiconsError::InvalidNextTokenException(
                inner,
            ) => Error::InvalidNextTokenException(inner),
            crate::operation::list_lexicons::ListLexiconsError::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::operation::list_lexicons::ListLexiconsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksError,
            R,
        >,
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
impl From<crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksError> for Error {
    fn from(
        err: crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksError,
    ) -> Self {
        match err {
            crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksError::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksError::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
            crate::operation::list_speech_synthesis_tasks::ListSpeechSynthesisTasksError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::put_lexicon::PutLexiconError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::put_lexicon::PutLexiconError, R>,
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
impl From<crate::operation::put_lexicon::PutLexiconError> for Error {
    fn from(err: crate::operation::put_lexicon::PutLexiconError) -> Self {
        match err {
            crate::operation::put_lexicon::PutLexiconError::InvalidLexiconException(inner) => {
                Error::InvalidLexiconException(inner)
            }
            crate::operation::put_lexicon::PutLexiconError::LexiconSizeExceededException(inner) => {
                Error::LexiconSizeExceededException(inner)
            }
            crate::operation::put_lexicon::PutLexiconError::MaxLexemeLengthExceededException(
                inner,
            ) => Error::MaxLexemeLengthExceededException(inner),
            crate::operation::put_lexicon::PutLexiconError::MaxLexiconsNumberExceededException(
                inner,
            ) => Error::MaxLexiconsNumberExceededException(inner),
            crate::operation::put_lexicon::PutLexiconError::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::operation::put_lexicon::PutLexiconError::UnsupportedPlsAlphabetException(
                inner,
            ) => Error::UnsupportedPlsAlphabetException(inner),
            crate::operation::put_lexicon::PutLexiconError::UnsupportedPlsLanguageException(
                inner,
            ) => Error::UnsupportedPlsLanguageException(inner),
            crate::operation::put_lexicon::PutLexiconError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError,
            R,
        >,
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
impl From<crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError> for Error {
    fn from(
        err: crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError,
    ) -> Self {
        match err {
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::EngineNotSupportedException(inner) => Error::EngineNotSupportedException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::InvalidS3BucketException(inner) => Error::InvalidS3BucketException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::InvalidS3KeyException(inner) => Error::InvalidS3KeyException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::InvalidSampleRateException(inner) => Error::InvalidSampleRateException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::InvalidSnsTopicArnException(inner) => Error::InvalidSnsTopicArnException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::InvalidSsmlException(inner) => Error::InvalidSsmlException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::LanguageNotSupportedException(inner) => Error::LanguageNotSupportedException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::LexiconNotFoundException(inner) => Error::LexiconNotFoundException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::MarksNotSupportedForFormatException(inner) => Error::MarksNotSupportedForFormatException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::SsmlMarksNotSupportedForTextTypeException(inner) => Error::SsmlMarksNotSupportedForTextTypeException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::TextLengthExceededException(inner) => Error::TextLengthExceededException(inner),
            crate::operation::start_speech_synthesis_task::StartSpeechSynthesisTaskError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::synthesize_speech::SynthesizeSpeechError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::synthesize_speech::SynthesizeSpeechError,
            R,
        >,
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
impl From<crate::operation::synthesize_speech::SynthesizeSpeechError> for Error {
    fn from(err: crate::operation::synthesize_speech::SynthesizeSpeechError) -> Self {
        match err {
            crate::operation::synthesize_speech::SynthesizeSpeechError::EngineNotSupportedException(inner) => Error::EngineNotSupportedException(inner),
            crate::operation::synthesize_speech::SynthesizeSpeechError::InvalidSampleRateException(inner) => Error::InvalidSampleRateException(inner),
            crate::operation::synthesize_speech::SynthesizeSpeechError::InvalidSsmlException(inner) => Error::InvalidSsmlException(inner),
            crate::operation::synthesize_speech::SynthesizeSpeechError::LanguageNotSupportedException(inner) => Error::LanguageNotSupportedException(inner),
            crate::operation::synthesize_speech::SynthesizeSpeechError::LexiconNotFoundException(inner) => Error::LexiconNotFoundException(inner),
            crate::operation::synthesize_speech::SynthesizeSpeechError::MarksNotSupportedForFormatException(inner) => Error::MarksNotSupportedForFormatException(inner),
            crate::operation::synthesize_speech::SynthesizeSpeechError::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
            crate::operation::synthesize_speech::SynthesizeSpeechError::SsmlMarksNotSupportedForTextTypeException(inner) => Error::SsmlMarksNotSupportedForTextTypeException(inner),
            crate::operation::synthesize_speech::SynthesizeSpeechError::TextLengthExceededException(inner) => Error::TextLengthExceededException(inner),
            crate::operation::synthesize_speech::SynthesizeSpeechError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl std::error::Error for Error {}
impl aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::EngineNotSupportedException(e) => e.request_id(),
            Self::InvalidLexiconException(e) => e.request_id(),
            Self::InvalidNextTokenException(e) => e.request_id(),
            Self::InvalidS3BucketException(e) => e.request_id(),
            Self::InvalidS3KeyException(e) => e.request_id(),
            Self::InvalidSampleRateException(e) => e.request_id(),
            Self::InvalidSnsTopicArnException(e) => e.request_id(),
            Self::InvalidSsmlException(e) => e.request_id(),
            Self::InvalidTaskIdException(e) => e.request_id(),
            Self::LanguageNotSupportedException(e) => e.request_id(),
            Self::LexiconNotFoundException(e) => e.request_id(),
            Self::LexiconSizeExceededException(e) => e.request_id(),
            Self::MarksNotSupportedForFormatException(e) => e.request_id(),
            Self::MaxLexemeLengthExceededException(e) => e.request_id(),
            Self::MaxLexiconsNumberExceededException(e) => e.request_id(),
            Self::ServiceFailureException(e) => e.request_id(),
            Self::SsmlMarksNotSupportedForTextTypeException(e) => e.request_id(),
            Self::SynthesisTaskNotFoundException(e) => e.request_id(),
            Self::TextLengthExceededException(e) => e.request_id(),
            Self::UnsupportedPlsAlphabetException(e) => e.request_id(),
            Self::UnsupportedPlsLanguageException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}