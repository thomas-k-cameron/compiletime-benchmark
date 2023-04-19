// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_storage_lens_configuration_headers(
    input: &crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.account_id {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "account_id",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("x-amz-account-id", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_storage_lens_configuration_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput,
    crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(
        crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationError::generic(
            generic,
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_storage_lens_configuration_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationOutput,
    crate::operation::get_storage_lens_configuration::GetStorageLensConfigurationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_storage_lens_configuration::builders::GetStorageLensConfigurationOutputBuilder::default();
        let _ = response;
        output = output.set_storage_lens_configuration(
            crate::protocol_serde::shape_get_storage_lens_configuration_output::de_storage_lens_configuration_payload(response.body().as_ref())?
        );
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
