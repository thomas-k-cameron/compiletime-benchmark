// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_service_connect_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ServiceConnectConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("enabled").boolean(input.enabled);
    }
    if let Some(var_1) = &input.namespace {
        object.key("namespace").string(var_1.as_str());
    }
    if let Some(var_2) = &input.services {
        let mut array_3 = object.key("services").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_service_connect_service::ser_service_connect_service(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.log_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("logConfiguration").start_object();
        crate::protocol_serde::shape_log_configuration::ser_log_configuration(
            &mut object_7,
            var_6,
        )?;
        object_7.finish();
    }
    Ok(())
}

pub(crate) fn de_service_connect_configuration<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::ServiceConnectConfiguration>,
    aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            aws_smithy_json::deserialize::Token<'a>,
            aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ServiceConnectConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "enabled" => {
                                builder = builder.set_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "namespace" => {
                                builder = builder.set_namespace(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "services" => {
                                builder = builder.set_services(
                                    crate::protocol_serde::shape_service_connect_service_list::de_service_connect_service_list(tokens)?
                                );
                            }
                            "logConfiguration" => {
                                builder = builder.set_log_configuration(
                                    crate::protocol_serde::shape_log_configuration::de_log_configuration(tokens)?
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
            Ok(Some(builder.build()))
        }
        _ => Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}
