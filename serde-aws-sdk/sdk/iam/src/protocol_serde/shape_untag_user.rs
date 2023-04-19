// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_untag_user_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::untag_user::UntagUserOutput,
    crate::operation::untag_user::UntagUserError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::untag_user::UntagUserError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::untag_user::UntagUserError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModification" => {
            crate::operation::untag_user::UntagUserError::ConcurrentModificationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::untag_user::UntagUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NoSuchEntity" => crate::operation::untag_user::UntagUserError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::untag_user::UntagUserError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceFailure" => {
            crate::operation::untag_user::UntagUserError::ServiceFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::untag_user::UntagUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::untag_user::UntagUserError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_untag_user_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::untag_user::UntagUserOutput,
    crate::operation::untag_user::UntagUserError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::untag_user::builders::UntagUserOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
