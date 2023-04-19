// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_stored_query<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::types::StoredQuery>, aws_smithy_json::deserialize::error::DeserializeError>
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
            let mut builder = crate::types::builders::StoredQueryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "QueryId" => {
                                builder = builder.set_query_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "QueryArn" => {
                                builder = builder.set_query_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "QueryName" => {
                                builder = builder.set_query_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Expression" => {
                                builder = builder.set_expression(
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
            Ok(Some(builder.build()))
        }
        _ => Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}

pub fn ser_stored_query(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::StoredQuery,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.query_id {
        object.key("QueryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.query_arn {
        object.key("QueryArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.query_name {
        object.key("QueryName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.expression {
        object.key("Expression").string(var_5.as_str());
    }
    Ok(())
}
