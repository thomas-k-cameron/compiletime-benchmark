// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_eks_container_security_context(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EksContainerSecurityContext,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.run_as_user {
        object.key("runAsUser").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.run_as_group {
        object.key("runAsGroup").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.privileged {
        object.key("privileged").boolean(*var_3);
    }
    if let Some(var_4) = &input.read_only_root_filesystem {
        object.key("readOnlyRootFilesystem").boolean(*var_4);
    }
    if let Some(var_5) = &input.run_as_non_root {
        object.key("runAsNonRoot").boolean(*var_5);
    }
    Ok(())
}

pub(crate) fn de_eks_container_security_context<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::EksContainerSecurityContext>,
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
            let mut builder = crate::types::builders::EksContainerSecurityContextBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "runAsUser" => {
                                builder = builder.set_run_as_user(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i64::try_from)
                                    .transpose()?,
                                );
                            }
                            "runAsGroup" => {
                                builder = builder.set_run_as_group(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i64::try_from)
                                    .transpose()?,
                                );
                            }
                            "privileged" => {
                                builder = builder.set_privileged(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "readOnlyRootFilesystem" => {
                                builder = builder.set_read_only_root_filesystem(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "runAsNonRoot" => {
                                builder = builder.set_run_as_non_root(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
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