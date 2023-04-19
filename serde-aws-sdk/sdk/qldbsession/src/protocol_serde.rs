// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_send_command;

pub fn parse_http_error_metadata(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response.body(), response.headers())
}

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_capacity_exceeded_exception;

pub(crate) mod shape_invalid_session_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_occ_conflict_exception;

pub(crate) mod shape_rate_exceeded_exception;

pub(crate) mod shape_send_command_input;

pub(crate) mod shape_abort_transaction_request;

pub(crate) mod shape_abort_transaction_result;

pub(crate) mod shape_commit_transaction_request;

pub(crate) mod shape_commit_transaction_result;

pub(crate) mod shape_end_session_request;

pub(crate) mod shape_end_session_result;

pub(crate) mod shape_execute_statement_request;

pub(crate) mod shape_execute_statement_result;

pub(crate) mod shape_fetch_page_request;

pub(crate) mod shape_fetch_page_result;

pub(crate) mod shape_start_session_request;

pub(crate) mod shape_start_session_result;

pub(crate) mod shape_start_transaction_request;

pub(crate) mod shape_start_transaction_result;

pub(crate) mod shape_io_usage;

pub(crate) mod shape_page;

pub(crate) mod shape_timing_information;

pub(crate) mod shape_value_holder;

pub(crate) mod shape_value_holders;
