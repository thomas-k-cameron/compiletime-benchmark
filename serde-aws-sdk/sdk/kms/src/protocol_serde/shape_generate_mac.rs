// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_generate_mac_input(
    input: &crate::operation::generate_mac::GenerateMacInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_generate_mac_input::ser_generate_mac_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_generate_mac_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::generate_mac::GenerateMacOutput,
    crate::operation::generate_mac::GenerateMacError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::generate_mac::GenerateMacError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::generate_mac::GenerateMacError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DisabledException" => {
            crate::operation::generate_mac::GenerateMacError::DisabledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::DisabledExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_disabled_exception::de_disabled_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::generate_mac::GenerateMacError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidGrantTokenException" => {
            crate::operation::generate_mac::GenerateMacError::InvalidGrantTokenException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::generate_mac::GenerateMacError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidKeyUsageException" => {
            crate::operation::generate_mac::GenerateMacError::InvalidKeyUsageException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::generate_mac::GenerateMacError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KeyUnavailableException" => {
            crate::operation::generate_mac::GenerateMacError::KeyUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::KeyUnavailableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_key_unavailable_exception::de_key_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::generate_mac::GenerateMacError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSInternalException" => {
            crate::operation::generate_mac::GenerateMacError::KmsInternalException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::KmsInternalExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::generate_mac::GenerateMacError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSInvalidStateException" => {
            crate::operation::generate_mac::GenerateMacError::KmsInvalidStateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::KmsInvalidStateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::generate_mac::GenerateMacError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotFoundException" => {
            crate::operation::generate_mac::GenerateMacError::NotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::generate_mac::GenerateMacError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::generate_mac::GenerateMacError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_generate_mac_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::generate_mac::GenerateMacOutput,
    crate::operation::generate_mac::GenerateMacError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::generate_mac::builders::GenerateMacOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_generate_mac::de_generate_mac(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::generate_mac::GenerateMacError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_generate_mac(
    value: &[u8],
    mut builder: crate::operation::generate_mac::builders::GenerateMacOutputBuilder,
) -> Result<
    crate::operation::generate_mac::builders::GenerateMacOutputBuilder,
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
                    "Mac" => {
                        builder = builder.set_mac(
                            aws_smithy_json::deserialize::token::expect_blob_or_null(
                                tokens.next(),
                            )?,
                        );
                    }
                    "MacAlgorithm" => {
                        builder = builder.set_mac_algorithm(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::MacAlgorithmSpec::from(u.as_ref()))
                            })
                            .transpose()?,
                        );
                    }
                    "KeyId" => {
                        builder = builder.set_key_id(
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
