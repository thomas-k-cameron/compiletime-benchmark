// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_log_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LogConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.log_driver {
        object.key("logDriver").string(var_1.as_str());
    }
    if let Some(var_2) = &input.options {
        #[allow(unused_mut)]
        let mut object_3 = object.key("options").start_object();
        for (key_4, value_5) in var_2 {
            {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.secret_options {
        let mut array_7 = object.key("secretOptions").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_secret::ser_secret(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

pub(crate) fn de_log_configuration<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::LogConfiguration>,
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
            let mut builder = crate::types::builders::LogConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "logDriver" => {
                                builder = builder.set_log_driver(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::LogDriver::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "options" => {
                                builder = builder.set_options(
                                    crate::protocol_serde::shape_log_configuration_options_map::de_log_configuration_options_map(tokens)?
                                );
                            }
                            "secretOptions" => {
                                builder = builder.set_secret_options(
                                    crate::protocol_serde::shape_secret_list::de_secret_list(
                                        tokens,
                                    )?,
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
