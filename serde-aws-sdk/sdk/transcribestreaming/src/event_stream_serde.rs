// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(Debug)]
pub struct AudioStreamErrorMarshaller;

impl AudioStreamErrorMarshaller {
    pub fn new() -> Self {
        AudioStreamErrorMarshaller
    }
}
impl aws_smithy_eventstream::frame::MarshallMessage for AudioStreamErrorMarshaller {
    type Input = crate::types::error::AudioStreamError;
    fn marshall(
        &self,
        _input: Self::Input,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::Message,
        aws_smithy_eventstream::error::Error,
    > {
        let mut headers = Vec::new();
        headers.push(aws_smithy_eventstream::frame::Header::new(
            ":message-type",
            aws_smithy_eventstream::frame::HeaderValue::String("exception".into()),
        ));
        let payload = Vec::new();
        Ok(aws_smithy_eventstream::frame::Message::new_from_parts(
            headers, payload,
        ))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct AudioStreamMarshaller;

impl AudioStreamMarshaller {
    pub fn new() -> Self {
        AudioStreamMarshaller
    }
}
impl aws_smithy_eventstream::frame::MarshallMessage for AudioStreamMarshaller {
    type Input = crate::types::AudioStream;
    fn marshall(
        &self,
        input: Self::Input,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::Message,
        aws_smithy_eventstream::error::Error,
    > {
        let mut headers = Vec::new();
        headers.push(aws_smithy_eventstream::frame::Header::new(
            ":message-type",
            aws_smithy_eventstream::frame::HeaderValue::String("event".into()),
        ));
        let payload = match input {
            Self::Input::AudioEvent(inner) =>  {
                headers.push(aws_smithy_eventstream::frame::Header::new(":event-type", aws_smithy_eventstream::frame::HeaderValue::String("AudioEvent".into())));
                headers.push(aws_smithy_eventstream::frame::Header::new(":content-type", aws_smithy_eventstream::frame::HeaderValue::String("application/octet-stream".into())));
                if let Some(inner_payload) = inner.audio_chunk {
                    inner_payload.into_inner()
                }
                 else  {
                    Vec::new()
                }
            }
            Self::Input::ConfigurationEvent(inner) =>  {
                headers.push(aws_smithy_eventstream::frame::Header::new(":event-type", aws_smithy_eventstream::frame::HeaderValue::String("ConfigurationEvent".into())));
                headers.push(aws_smithy_eventstream::frame::Header::new(":content-type", aws_smithy_eventstream::frame::HeaderValue::String("application/vnd.amazon.eventstream".into())));
                crate::protocol_serde::shape_audio_stream::ser_configuration_event_payload(&inner)
                                            .map_err(|err| aws_smithy_eventstream::error::Error::marshalling(format!("{}", err)))?
            }
            Self::Input::Unknown => return Err(
                                            aws_smithy_eventstream::error::Error::marshalling("Cannot serialize `AudioStream::Unknown` for the request. The `Unknown` variant is intended for responses only. It occurs when an outdated client is used after a new enum variant was added on the server side.".to_owned())
                                        )
        }
        ;
        Ok(aws_smithy_eventstream::frame::Message::new_from_parts(
            headers, payload,
        ))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct CallAnalyticsTranscriptResultStreamUnmarshaller;

impl CallAnalyticsTranscriptResultStreamUnmarshaller {
    pub fn new() -> Self {
        CallAnalyticsTranscriptResultStreamUnmarshaller
    }
}
impl aws_smithy_eventstream::frame::UnmarshallMessage
    for CallAnalyticsTranscriptResultStreamUnmarshaller
{
    type Output = crate::types::CallAnalyticsTranscriptResultStream;
    type Error = crate::types::error::CallAnalyticsTranscriptResultStreamError;
    fn unmarshall(
        &self,
        message: &aws_smithy_eventstream::frame::Message,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        aws_smithy_eventstream::error::Error,
    > {
        let response_headers = aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => match response_headers.smithy_type.as_str() {
                "UtteranceEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_utterance_event::de_utterance_event_payload(
                            &message.payload()[..],
                        )
                        .map_err(|err| {
                            aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                "failed to unmarshall UtteranceEvent: {}",
                                err
                            ))
                        })?;
                    Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::CallAnalyticsTranscriptResultStream::UtteranceEvent(parsed),
                    ))
                }
                "CategoryEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_category_event::de_category_event_payload(
                            &message.payload()[..],
                        )
                        .map_err(|err| {
                            aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                "failed to unmarshall CategoryEvent: {}",
                                err
                            ))
                        })?;
                    Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::CallAnalyticsTranscriptResultStream::CategoryEvent(parsed),
                    ))
                }
                _unknown_variant => Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::types::CallAnalyticsTranscriptResultStream::Unknown,
                )),
            },
            "exception" => {
                let generic = match crate::protocol_serde::parse_event_stream_error_metadata(message.payload()) {
                                        Ok(builder) => builder.build(),
                                        Err(err) => return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(crate::types::error::CallAnalyticsTranscriptResultStreamError::unhandled(err))),
                                    };
                match response_headers.smithy_type.as_str() {
                    "BadRequestException" => {
                        let mut builder =
                            crate::types::error::builders::BadRequestExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall BadRequestException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::CallAnalyticsTranscriptResultStreamError::BadRequestException(builder.build())
                                                        ));
                    }
                    "LimitExceededException" => {
                        let mut builder =
                            crate::types::error::builders::LimitExceededExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall LimitExceededException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::CallAnalyticsTranscriptResultStreamError::LimitExceededException(builder.build())
                                                        ));
                    }
                    "InternalFailureException" => {
                        let mut builder =
                            crate::types::error::builders::InternalFailureExceptionBuilder::default(
                            );
                        builder = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InternalFailureException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::CallAnalyticsTranscriptResultStreamError::InternalFailureException(builder.build())
                                                        ));
                    }
                    "ConflictException" => {
                        let mut builder =
                            crate::types::error::builders::ConflictExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ConflictException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::CallAnalyticsTranscriptResultStreamError::ConflictException(builder.build())
                                                        ));
                    }
                    "ServiceUnavailableException" => {
                        let mut builder = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ServiceUnavailableException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::CallAnalyticsTranscriptResultStreamError::ServiceUnavailableException(builder.build())
                                                        ));
                    }
                    _ => {}
                }
                Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::types::error::CallAnalyticsTranscriptResultStreamError::generic(generic),
                ))
            }
            value => {
                return Err(aws_smithy_eventstream::error::Error::unmarshalling(
                    format!("unrecognized :message-type: {}", value),
                ));
            }
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct MedicalTranscriptResultStreamUnmarshaller;

impl MedicalTranscriptResultStreamUnmarshaller {
    pub fn new() -> Self {
        MedicalTranscriptResultStreamUnmarshaller
    }
}
impl aws_smithy_eventstream::frame::UnmarshallMessage
    for MedicalTranscriptResultStreamUnmarshaller
{
    type Output = crate::types::MedicalTranscriptResultStream;
    type Error = crate::types::error::MedicalTranscriptResultStreamError;
    fn unmarshall(
        &self,
        message: &aws_smithy_eventstream::frame::Message,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        aws_smithy_eventstream::error::Error,
    > {
        let response_headers = aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => match response_headers.smithy_type.as_str() {
                "TranscriptEvent" => {
                    let parsed =
                            crate::protocol_serde::shape_medical_transcript_event::de_medical_transcript_event_payload(&message.payload()[..])
                                            .map_err(|err| {
                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall TranscriptEvent: {}", err))
                                            })?
                        ;
                    Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::MedicalTranscriptResultStream::TranscriptEvent(parsed),
                    ))
                }
                _unknown_variant => Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::types::MedicalTranscriptResultStream::Unknown,
                )),
            },
            "exception" => {
                let generic = match crate::protocol_serde::parse_event_stream_error_metadata(
                    message.payload(),
                ) {
                    Ok(builder) => builder.build(),
                    Err(err) => {
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::MedicalTranscriptResultStreamError::unhandled(err),
                        ))
                    }
                };
                match response_headers.smithy_type.as_str() {
                    "BadRequestException" => {
                        let mut builder =
                            crate::types::error::builders::BadRequestExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall BadRequestException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::MedicalTranscriptResultStreamError::BadRequestException(builder.build())
                                                        ));
                    }
                    "LimitExceededException" => {
                        let mut builder =
                            crate::types::error::builders::LimitExceededExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall LimitExceededException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::MedicalTranscriptResultStreamError::LimitExceededException(builder.build())
                                                        ));
                    }
                    "InternalFailureException" => {
                        let mut builder =
                            crate::types::error::builders::InternalFailureExceptionBuilder::default(
                            );
                        builder = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InternalFailureException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::MedicalTranscriptResultStreamError::InternalFailureException(builder.build())
                                                        ));
                    }
                    "ConflictException" => {
                        let mut builder =
                            crate::types::error::builders::ConflictExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ConflictException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::MedicalTranscriptResultStreamError::ConflictException(builder.build())
                                                        ));
                    }
                    "ServiceUnavailableException" => {
                        let mut builder = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ServiceUnavailableException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::MedicalTranscriptResultStreamError::ServiceUnavailableException(builder.build())
                                                        ));
                    }
                    _ => {}
                }
                Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::types::error::MedicalTranscriptResultStreamError::generic(generic),
                ))
            }
            value => {
                return Err(aws_smithy_eventstream::error::Error::unmarshalling(
                    format!("unrecognized :message-type: {}", value),
                ));
            }
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct TranscriptResultStreamUnmarshaller;

impl TranscriptResultStreamUnmarshaller {
    pub fn new() -> Self {
        TranscriptResultStreamUnmarshaller
    }
}
impl aws_smithy_eventstream::frame::UnmarshallMessage for TranscriptResultStreamUnmarshaller {
    type Output = crate::types::TranscriptResultStream;
    type Error = crate::types::error::TranscriptResultStreamError;
    fn unmarshall(
        &self,
        message: &aws_smithy_eventstream::frame::Message,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        aws_smithy_eventstream::error::Error,
    > {
        let response_headers = aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => {
                match response_headers.smithy_type.as_str() {
                    "TranscriptEvent" => {
                        let parsed =
                            crate::protocol_serde::shape_transcript_event::de_transcript_event_payload(&message.payload()[..])
                                            .map_err(|err| {
                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall TranscriptEvent: {}", err))
                                            })?
                        ;
                        Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                            crate::types::TranscriptResultStream::TranscriptEvent(parsed),
                        ))
                    }
                    _unknown_variant => {
                        Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                            crate::types::TranscriptResultStream::Unknown,
                        ))
                    }
                }
            }
            "exception" => {
                let generic = match crate::protocol_serde::parse_event_stream_error_metadata(
                    message.payload(),
                ) {
                    Ok(builder) => builder.build(),
                    Err(err) => {
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::TranscriptResultStreamError::unhandled(err),
                        ))
                    }
                };
                match response_headers.smithy_type.as_str() {
                    "BadRequestException" => {
                        let mut builder =
                            crate::types::error::builders::BadRequestExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall BadRequestException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::TranscriptResultStreamError::BadRequestException(
                                builder.build(),
                            ),
                        ));
                    }
                    "LimitExceededException" => {
                        let mut builder =
                            crate::types::error::builders::LimitExceededExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall LimitExceededException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::TranscriptResultStreamError::LimitExceededException(builder.build())
                                                        ));
                    }
                    "InternalFailureException" => {
                        let mut builder =
                            crate::types::error::builders::InternalFailureExceptionBuilder::default(
                            );
                        builder = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InternalFailureException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::TranscriptResultStreamError::InternalFailureException(builder.build())
                                                        ));
                    }
                    "ConflictException" => {
                        let mut builder =
                            crate::types::error::builders::ConflictExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ConflictException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::TranscriptResultStreamError::ConflictException(
                                builder.build(),
                            ),
                        ));
                    }
                    "ServiceUnavailableException" => {
                        let mut builder = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ServiceUnavailableException: {}", err))
                                                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::types::error::TranscriptResultStreamError::ServiceUnavailableException(builder.build())
                                                        ));
                    }
                    _ => {}
                }
                Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::types::error::TranscriptResultStreamError::generic(generic),
                ))
            }
            value => {
                return Err(aws_smithy_eventstream::error::Error::unmarshalling(
                    format!("unrecognized :message-type: {}", value),
                ));
            }
        }
    }
}
