// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_export_table_to_point_in_time_input(
    input: &crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_export_table_to_point_in_time_input::ser_export_table_to_point_in_time_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_export_table_to_point_in_time_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput,
    crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ExportConflictException" => crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::ExportConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ExportConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_export_conflict_exception::de_export_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidExportTimeException" => crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::InvalidExportTimeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidExportTimeExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_export_time_exception::de_invalid_export_time_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PointInTimeRecoveryUnavailableException" => crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::PointInTimeRecoveryUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PointInTimeRecoveryUnavailableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_point_in_time_recovery_unavailable_exception::de_point_in_time_recovery_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TableNotFoundException" => crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::TableNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TableNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_table_not_found_exception::de_table_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_export_table_to_point_in_time_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput,
    crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_export_table_to_point_in_time::de_export_table_to_point_in_time(response.body().as_ref(), output).map_err(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_export_table_to_point_in_time(value: &[u8], mut builder: crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeOutputBuilder) -> Result<crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ExportDescription" => {
                        builder = builder.set_export_description(
                            crate::protocol_serde::shape_export_description::de_export_description(
                                tokens,
                            )?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}