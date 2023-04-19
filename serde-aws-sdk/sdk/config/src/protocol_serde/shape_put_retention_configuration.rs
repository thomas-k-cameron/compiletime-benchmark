// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_retention_configuration_input(
    input: &crate::operation::put_retention_configuration::PutRetentionConfigurationInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_retention_configuration_input::ser_put_retention_configuration_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_retention_configuration_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_retention_configuration::PutRetentionConfigurationOutput,
    crate::operation::put_retention_configuration::PutRetentionConfigurationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::put_retention_configuration::PutRetentionConfigurationError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::put_retention_configuration::PutRetentionConfigurationError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidParameterValueException" => crate::operation::put_retention_configuration::PutRetentionConfigurationError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_retention_configuration::PutRetentionConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MaxNumberOfRetentionConfigurationsExceededException" => crate::operation::put_retention_configuration::PutRetentionConfigurationError::MaxNumberOfRetentionConfigurationsExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::MaxNumberOfRetentionConfigurationsExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_max_number_of_retention_configurations_exceeded_exception::de_max_number_of_retention_configurations_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_retention_configuration::PutRetentionConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::put_retention_configuration::PutRetentionConfigurationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_retention_configuration_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_retention_configuration::PutRetentionConfigurationOutput,
    crate::operation::put_retention_configuration::PutRetentionConfigurationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_retention_configuration::builders::PutRetentionConfigurationOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_put_retention_configuration::de_put_retention_configuration(response.body().as_ref(), output).map_err(crate::operation::put_retention_configuration::PutRetentionConfigurationError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_put_retention_configuration(
    value: &[u8],
    mut builder: crate::operation::put_retention_configuration::builders::PutRetentionConfigurationOutputBuilder,
) -> Result<
    crate::operation::put_retention_configuration::builders::PutRetentionConfigurationOutputBuilder,
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
                    "RetentionConfiguration" => {
                        builder = builder.set_retention_configuration(
                            crate::protocol_serde::shape_retention_configuration::de_retention_configuration(tokens)?
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