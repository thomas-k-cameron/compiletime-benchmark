// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_access_point_configuration_for_object_lambda_headers(
    input: &crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaInput,
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

pub fn ser_put_access_point_configuration_for_object_lambda_op_input(
    input: &crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("PutAccessPointConfigurationForObjectLambdaRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::protocol_serde::shape_put_access_point_configuration_for_object_lambda_input::ser_put_access_point_configuration_for_object_lambda_input_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_access_point_configuration_for_object_lambda_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaOutput, crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_access_point_configuration_for_object_lambda_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaOutput, crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
