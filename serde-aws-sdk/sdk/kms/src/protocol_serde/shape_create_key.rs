// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_key_input(
    input: &crate::operation::create_key::CreateKeyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_key_input::ser_create_key_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_key_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_key::CreateKeyOutput,
    crate::operation::create_key::CreateKeyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::create_key::CreateKeyError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CloudHsmClusterInvalidConfigurationException" => crate::operation::create_key::CreateKeyError::CloudHsmClusterInvalidConfigurationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CloudHsmClusterInvalidConfigurationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_cluster_invalid_configuration_exception::de_cloud_hsm_cluster_invalid_configuration_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CustomKeyStoreInvalidStateException" => crate::operation::create_key::CreateKeyError::CustomKeyStoreInvalidStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CustomKeyStoreInvalidStateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CustomKeyStoreNotFoundException" => crate::operation::create_key::CreateKeyError::CustomKeyStoreNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DependencyTimeoutException" => crate::operation::create_key::CreateKeyError::DependencyTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DependencyTimeoutExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArnException" => crate::operation::create_key::CreateKeyError::InvalidArnException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidArnExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_arn_exception::de_invalid_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSInternalException" => crate::operation::create_key::CreateKeyError::KmsInternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::KmsInternalExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::operation::create_key::CreateKeyError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MalformedPolicyDocumentException" => crate::operation::create_key::CreateKeyError::MalformedPolicyDocumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TagException" => crate::operation::create_key::CreateKeyError::TagException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TagExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tag_exception::de_tag_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedOperationException" => crate::operation::create_key::CreateKeyError::UnsupportedOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedOperationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksKeyAlreadyInUseException" => crate::operation::create_key::CreateKeyError::XksKeyAlreadyInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::XksKeyAlreadyInUseExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_key_already_in_use_exception::de_xks_key_already_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksKeyInvalidConfigurationException" => crate::operation::create_key::CreateKeyError::XksKeyInvalidConfigurationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::XksKeyInvalidConfigurationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_key_invalid_configuration_exception::de_xks_key_invalid_configuration_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksKeyNotFoundException" => crate::operation::create_key::CreateKeyError::XksKeyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::XksKeyNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_key_not_found_exception::de_xks_key_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::create_key::CreateKeyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_key_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_key::CreateKeyOutput,
    crate::operation::create_key::CreateKeyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_key::builders::CreateKeyOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_key::de_create_key(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::create_key::CreateKeyError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_create_key(
    value: &[u8],
    mut builder: crate::operation::create_key::builders::CreateKeyOutputBuilder,
) -> Result<
    crate::operation::create_key::builders::CreateKeyOutputBuilder,
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
                    "KeyMetadata" => {
                        builder = builder.set_key_metadata(
                            crate::protocol_serde::shape_key_metadata::de_key_metadata(tokens)?,
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
