// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_remediation_configurations_input(
    input: &crate::operation::put_remediation_configurations::PutRemediationConfigurationsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_remediation_configurations_input::ser_put_remediation_configurations_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_remediation_configurations_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_remediation_configurations::PutRemediationConfigurationsOutput,
    crate::operation::put_remediation_configurations::PutRemediationConfigurationsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::put_remediation_configurations::PutRemediationConfigurationsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::put_remediation_configurations::PutRemediationConfigurationsError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InsufficientPermissionsException" => crate::operation::put_remediation_configurations::PutRemediationConfigurationsError::InsufficientPermissionsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InsufficientPermissionsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_permissions_exception::de_insufficient_permissions_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_remediation_configurations::PutRemediationConfigurationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::put_remediation_configurations::PutRemediationConfigurationsError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_remediation_configurations::PutRemediationConfigurationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::put_remediation_configurations::PutRemediationConfigurationsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_remediation_configurations_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_remediation_configurations::PutRemediationConfigurationsOutput,
    crate::operation::put_remediation_configurations::PutRemediationConfigurationsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_remediation_configurations::builders::PutRemediationConfigurationsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_put_remediation_configurations::de_put_remediation_configurations(response.body().as_ref(), output).map_err(crate::operation::put_remediation_configurations::PutRemediationConfigurationsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_put_remediation_configurations(value: &[u8], mut builder: crate::operation::put_remediation_configurations::builders::PutRemediationConfigurationsOutputBuilder) -> Result<crate::operation::put_remediation_configurations::builders::PutRemediationConfigurationsOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "FailedBatches" => {
                        builder = builder.set_failed_batches(
                            crate::protocol_serde::shape_failed_remediation_batches::de_failed_remediation_batches(tokens)?
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