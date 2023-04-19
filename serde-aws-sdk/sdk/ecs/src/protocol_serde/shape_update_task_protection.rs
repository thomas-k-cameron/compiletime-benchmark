// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_task_protection_input(
    input: &crate::operation::update_task_protection::UpdateTaskProtectionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_task_protection_input::ser_update_task_protection_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_task_protection_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_task_protection::UpdateTaskProtectionOutput,
    crate::operation::update_task_protection::UpdateTaskProtectionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::update_task_protection::UpdateTaskProtectionError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ClientException" => crate::operation::update_task_protection::UpdateTaskProtectionError::ClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClientExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ClusterNotFoundException" => crate::operation::update_task_protection::UpdateTaskProtectionError::ClusterNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClusterNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cluster_not_found_exception::de_cluster_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::operation::update_task_protection::UpdateTaskProtectionError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::update_task_protection::UpdateTaskProtectionError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerException" => crate::operation::update_task_protection::UpdateTaskProtectionError::ServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedFeatureException" => crate::operation::update_task_protection::UpdateTaskProtectionError::UnsupportedFeatureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedFeatureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_feature_exception::de_unsupported_feature_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_task_protection::UpdateTaskProtectionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_task_protection_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_task_protection::UpdateTaskProtectionOutput,
    crate::operation::update_task_protection::UpdateTaskProtectionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_task_protection::builders::UpdateTaskProtectionOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_task_protection::de_update_task_protection(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::update_task_protection::UpdateTaskProtectionError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_update_task_protection(
    value: &[u8],
    mut builder: crate::operation::update_task_protection::builders::UpdateTaskProtectionOutputBuilder,
) -> Result<
    crate::operation::update_task_protection::builders::UpdateTaskProtectionOutputBuilder,
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
                    "protectedTasks" => {
                        builder = builder.set_protected_tasks(
                            crate::protocol_serde::shape_protected_tasks::de_protected_tasks(
                                tokens,
                            )?,
                        );
                    }
                    "failures" => {
                        builder = builder.set_failures(
                            crate::protocol_serde::shape_failures::de_failures(tokens)?,
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
