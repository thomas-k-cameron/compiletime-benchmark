// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_run_task_input(
    input: &crate::operation::run_task::RunTaskInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_run_task_input::ser_run_task_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_run_task_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::run_task::RunTaskOutput,
    crate::operation::run_task::RunTaskError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::run_task::RunTaskError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::run_task::RunTaskError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::run_task::RunTaskError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::run_task::RunTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "BlockedException" => {
            crate::operation::run_task::RunTaskError::BlockedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::BlockedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_blocked_exception::de_blocked_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::run_task::RunTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ClientException" => {
            crate::operation::run_task::RunTaskError::ClientException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ClientExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::run_task::RunTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ClusterNotFoundException" => {
            crate::operation::run_task::RunTaskError::ClusterNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ClusterNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cluster_not_found_exception::de_cluster_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::run_task::RunTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterException" => {
            crate::operation::run_task::RunTaskError::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::run_task::RunTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "PlatformTaskDefinitionIncompatibilityException" => {
            crate::operation::run_task::RunTaskError::PlatformTaskDefinitionIncompatibilityException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PlatformTaskDefinitionIncompatibilityExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_platform_task_definition_incompatibility_exception::de_platform_task_definition_incompatibility_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::run_task::RunTaskError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "PlatformUnknownException" => {
            crate::operation::run_task::RunTaskError::PlatformUnknownException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::PlatformUnknownExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_platform_unknown_exception::de_platform_unknown_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::run_task::RunTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServerException" => {
            crate::operation::run_task::RunTaskError::ServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::run_task::RunTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnsupportedFeatureException" => {
            crate::operation::run_task::RunTaskError::UnsupportedFeatureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::UnsupportedFeatureExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_feature_exception::de_unsupported_feature_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::run_task::RunTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::run_task::RunTaskError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_run_task_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::run_task::RunTaskOutput,
    crate::operation::run_task::RunTaskError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::run_task::builders::RunTaskOutputBuilder::default();
        let _ = response;
        output =
            crate::protocol_serde::shape_run_task::de_run_task(response.body().as_ref(), output)
                .map_err(crate::operation::run_task::RunTaskError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_run_task(
    value: &[u8],
    mut builder: crate::operation::run_task::builders::RunTaskOutputBuilder,
) -> Result<
    crate::operation::run_task::builders::RunTaskOutputBuilder,
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
                    "tasks" => {
                        builder = builder
                            .set_tasks(crate::protocol_serde::shape_tasks::de_tasks(tokens)?);
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
