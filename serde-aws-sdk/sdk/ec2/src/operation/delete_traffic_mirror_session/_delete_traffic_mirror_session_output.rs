// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct DeleteTrafficMirrorSessionOutput {
    /// <p>The ID of the deleted Traffic Mirror session.</p>
    #[doc(hidden)]
    pub traffic_mirror_session_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DeleteTrafficMirrorSessionOutput {
    /// <p>The ID of the deleted Traffic Mirror session.</p>
    pub fn traffic_mirror_session_id(&self) -> std::option::Option<&str> {
        self.traffic_mirror_session_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for DeleteTrafficMirrorSessionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteTrafficMirrorSessionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteTrafficMirrorSessionOutput`](crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput).
    pub fn builder() -> crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionOutputBuilder{
        crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput;
/// A builder for [`DeleteTrafficMirrorSessionOutput`](crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput).
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
pub struct DeleteTrafficMirrorSessionOutputBuilder {
    pub(crate) traffic_mirror_session_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DeleteTrafficMirrorSessionOutputBuilder {
    /// <p>The ID of the deleted Traffic Mirror session.</p>
    pub fn traffic_mirror_session_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.traffic_mirror_session_id = Some(input.into());
        self
    }
    /// <p>The ID of the deleted Traffic Mirror session.</p>
    pub fn set_traffic_mirror_session_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.traffic_mirror_session_id = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteTrafficMirrorSessionOutput`](crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput).
    pub fn build(
        self,
    ) -> crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput {
        crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput {
            traffic_mirror_session_id: self.traffic_mirror_session_id,
            _request_id: self._request_id,
        }
    }
}
