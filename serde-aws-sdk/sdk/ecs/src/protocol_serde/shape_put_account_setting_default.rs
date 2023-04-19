// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_account_setting_default_input(
    input: &crate::operation::put_account_setting_default::PutAccountSettingDefaultInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_account_setting_default_input::ser_put_account_setting_default_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_account_setting_default_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_account_setting_default::PutAccountSettingDefaultOutput,
    crate::operation::put_account_setting_default::PutAccountSettingDefaultError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::put_account_setting_default::PutAccountSettingDefaultError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::put_account_setting_default::PutAccountSettingDefaultError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClientException" => crate::operation::put_account_setting_default::PutAccountSettingDefaultError::ClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClientExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_account_setting_default::PutAccountSettingDefaultError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::operation::put_account_setting_default::PutAccountSettingDefaultError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_account_setting_default::PutAccountSettingDefaultError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerException" => crate::operation::put_account_setting_default::PutAccountSettingDefaultError::ServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_account_setting_default::PutAccountSettingDefaultError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::put_account_setting_default::PutAccountSettingDefaultError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_account_setting_default_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_account_setting_default::PutAccountSettingDefaultOutput,
    crate::operation::put_account_setting_default::PutAccountSettingDefaultError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_account_setting_default::builders::PutAccountSettingDefaultOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_put_account_setting_default::de_put_account_setting_default(response.body().as_ref(), output).map_err(crate::operation::put_account_setting_default::PutAccountSettingDefaultError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_put_account_setting_default(
    value: &[u8],
    mut builder: crate::operation::put_account_setting_default::builders::PutAccountSettingDefaultOutputBuilder,
) -> Result<
    crate::operation::put_account_setting_default::builders::PutAccountSettingDefaultOutputBuilder,
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
                    "setting" => {
                        builder = builder
                            .set_setting(crate::protocol_serde::shape_setting::de_setting(tokens)?);
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
