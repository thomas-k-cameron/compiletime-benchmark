// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_resource_evaluations_input(
    input: &crate::operation::list_resource_evaluations::ListResourceEvaluationsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_resource_evaluations_input::ser_list_resource_evaluations_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_resource_evaluations_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_resource_evaluations::ListResourceEvaluationsOutput,
    crate::operation::list_resource_evaluations::ListResourceEvaluationsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::list_resource_evaluations::ListResourceEvaluationsError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::list_resource_evaluations::ListResourceEvaluationsError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidNextTokenException" => crate::operation::list_resource_evaluations::ListResourceEvaluationsError::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidNextTokenExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_resource_evaluations::ListResourceEvaluationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::list_resource_evaluations::ListResourceEvaluationsError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_resource_evaluations::ListResourceEvaluationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTimeRangeException" => crate::operation::list_resource_evaluations::ListResourceEvaluationsError::InvalidTimeRangeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidTimeRangeExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_time_range_exception::de_invalid_time_range_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::list_resource_evaluations::ListResourceEvaluationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::list_resource_evaluations::ListResourceEvaluationsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_resource_evaluations_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_resource_evaluations::ListResourceEvaluationsOutput,
    crate::operation::list_resource_evaluations::ListResourceEvaluationsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_resource_evaluations::builders::ListResourceEvaluationsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_resource_evaluations::de_list_resource_evaluations(response.body().as_ref(), output).map_err(crate::operation::list_resource_evaluations::ListResourceEvaluationsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_list_resource_evaluations(
    value: &[u8],
    mut builder: crate::operation::list_resource_evaluations::builders::ListResourceEvaluationsOutputBuilder,
) -> Result<
    crate::operation::list_resource_evaluations::builders::ListResourceEvaluationsOutputBuilder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
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
                    "ResourceEvaluations" => {
                        builder = builder.set_resource_evaluations(
                            crate::protocol_serde::shape_resource_evaluations::de_resource_evaluations(tokens)?
                        );
                    }
                    "NextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
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
