// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The output when you delete the evaluation results for the specified Config rule.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteEvaluationResultsOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for DeleteEvaluationResultsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteEvaluationResultsOutput {
    /// Creates a new builder-style object to manufacture [`DeleteEvaluationResultsOutput`](crate::operation::delete_evaluation_results::DeleteEvaluationResultsOutput).
    pub fn builder(
    ) -> crate::operation::delete_evaluation_results::builders::DeleteEvaluationResultsOutputBuilder
    {
        crate::operation::delete_evaluation_results::builders::DeleteEvaluationResultsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_evaluation_results::DeleteEvaluationResultsOutput;
/// A builder for [`DeleteEvaluationResultsOutput`](crate::operation::delete_evaluation_results::DeleteEvaluationResultsOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct DeleteEvaluationResultsOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteEvaluationResultsOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteEvaluationResultsOutput`](crate::operation::delete_evaluation_results::DeleteEvaluationResultsOutput).
    pub fn build(
        self,
    ) -> crate::operation::delete_evaluation_results::DeleteEvaluationResultsOutput {
        crate::operation::delete_evaluation_results::DeleteEvaluationResultsOutput {
            _request_id: self._request_id,
        }
    }
}
