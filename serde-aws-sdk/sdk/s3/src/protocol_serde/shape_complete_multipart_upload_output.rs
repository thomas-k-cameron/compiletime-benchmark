// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_bucket_key_enabled_header(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<bool>, aws_smithy_http::header::ParseError> {
    let headers = header_map
        .get_all("x-amz-server-side-encryption-bucket-key-enabled")
        .iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_1.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub(crate) fn de_expiration_header(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-expiration").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_request_charged_header(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::types::RequestCharged>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-request-charged").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_ssekms_key_id_header(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("x-amz-server-side-encryption-aws-kms-key-id")
        .iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_server_side_encryption_header(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::types::ServerSideEncryption>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-server-side-encryption").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_version_id_header(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-version-id").iter();
    aws_smithy_http::header::one_or_none(headers)
}
