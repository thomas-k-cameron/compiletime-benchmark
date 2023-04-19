// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_capacity_id_header(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-capacity-id").iter();
    aws_smithy_http::header::one_or_none(headers)
}