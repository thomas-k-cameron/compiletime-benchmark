// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_remediation_execution_input(
    input: &crate::operation::start_remediation_execution::StartRemediationExecutionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_remediation_execution_input::ser_start_remediation_execution_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_remediation_execution_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::start_remediation_execution::StartRemediationExecutionOutput,
    crate::operation::start_remediation_execution::StartRemediationExecutionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::start_remediation_execution::StartRemediationExecutionError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::start_remediation_execution::StartRemediationExecutionError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InsufficientPermissionsException" => crate::operation::start_remediation_execution::StartRemediationExecutionError::InsufficientPermissionsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InsufficientPermissionsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_permissions_exception::de_insufficient_permissions_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_remediation_execution::StartRemediationExecutionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::start_remediation_execution::StartRemediationExecutionError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_remediation_execution::StartRemediationExecutionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchRemediationConfigurationException" => crate::operation::start_remediation_execution::StartRemediationExecutionError::NoSuchRemediationConfigurationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchRemediationConfigurationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_remediation_configuration_exception::de_no_such_remediation_configuration_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_remediation_execution::StartRemediationExecutionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::start_remediation_execution::StartRemediationExecutionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_remediation_execution_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::start_remediation_execution::StartRemediationExecutionOutput,
    crate::operation::start_remediation_execution::StartRemediationExecutionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::start_remediation_execution::builders::StartRemediationExecutionOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_start_remediation_execution::de_start_remediation_execution(response.body().as_ref(), output).map_err(crate::operation::start_remediation_execution::StartRemediationExecutionError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_start_remediation_execution(
    value: &[u8],
    mut builder: crate::operation::start_remediation_execution::builders::StartRemediationExecutionOutputBuilder,
) -> Result<
    crate::operation::start_remediation_execution::builders::StartRemediationExecutionOutputBuilder,
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
                    "FailureMessage" => {
                        builder = builder.set_failure_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "FailedItems" => {
                        builder = builder.set_failed_items(
                            crate::protocol_serde::shape_resource_keys::de_resource_keys(tokens)?,
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
