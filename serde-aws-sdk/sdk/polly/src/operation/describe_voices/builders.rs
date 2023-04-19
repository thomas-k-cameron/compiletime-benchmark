// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_voices::_describe_voices_output::DescribeVoicesOutputBuilder;

pub use crate::operation::describe_voices::_describe_voices_input::DescribeVoicesInputBuilder;

/// Fluent builder constructing a request to `DescribeVoices`.
///
/// <p>Returns the list of voices that are available for use when requesting speech synthesis. Each voice speaks a specified language, is either male or female, and is identified by an ID, which is the ASCII version of the voice name. </p>
/// <p>When synthesizing speech ( <code>SynthesizeSpeech</code> ), you provide the voice ID for the voice you want from the list of voices returned by <code>DescribeVoices</code>.</p>
/// <p>For example, you want your news reader application to read news in a specific language, but giving a user the option to choose the voice. Using the <code>DescribeVoices</code> operation you can provide the user with a list of available voices to select from.</p>
/// <p> You can optionally specify a language code to filter the available voices. For example, if you specify <code>en-US</code>, the operation returns a list of all available US English voices. </p>
/// <p>This operation requires permissions to perform the <code>polly:DescribeVoices</code> action.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeVoicesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_voices::builders::DescribeVoicesInputBuilder,
}
impl DescribeVoicesFluentBuilder {
    /// Creates a new `DescribeVoices`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_voices::DescribeVoices,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::describe_voices::DescribeVoicesError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::describe_voices::DescribeVoicesOutput,
        aws_smithy_http::result::SdkError<crate::operation::describe_voices::DescribeVoicesError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::describe_voices::builders::DescribeVoicesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_voices().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_voices::builders::DescribeVoicesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Specifies the engine (<code>standard</code> or <code>neural</code>) used by Amazon Polly when processing input text for speech synthesis. </p>
    pub fn engine(mut self, input: crate::types::Engine) -> Self {
        self.inner = self.inner.engine(input);
        self
    }
    /// <p>Specifies the engine (<code>standard</code> or <code>neural</code>) used by Amazon Polly when processing input text for speech synthesis. </p>
    pub fn set_engine(mut self, input: std::option::Option<crate::types::Engine>) -> Self {
        self.inner = self.inner.set_engine(input);
        self
    }
    /// <p> The language identification tag (ISO 639 code for the language name-ISO 3166 country code) for filtering the list of voices returned. If you don't specify this optional parameter, all available voices are returned. </p>
    pub fn language_code(mut self, input: crate::types::LanguageCode) -> Self {
        self.inner = self.inner.language_code(input);
        self
    }
    /// <p> The language identification tag (ISO 639 code for the language name-ISO 3166 country code) for filtering the list of voices returned. If you don't specify this optional parameter, all available voices are returned. </p>
    pub fn set_language_code(
        mut self,
        input: std::option::Option<crate::types::LanguageCode>,
    ) -> Self {
        self.inner = self.inner.set_language_code(input);
        self
    }
    /// <p>Boolean value indicating whether to return any bilingual voices that use the specified language as an additional language. For instance, if you request all languages that use US English (es-US), and there is an Italian voice that speaks both Italian (it-IT) and US English, that voice will be included if you specify <code>yes</code> but not if you specify <code>no</code>.</p>
    pub fn include_additional_language_codes(mut self, input: bool) -> Self {
        self.inner = self.inner.include_additional_language_codes(input);
        self
    }
    /// <p>Boolean value indicating whether to return any bilingual voices that use the specified language as an additional language. For instance, if you request all languages that use US English (es-US), and there is an Italian voice that speaks both Italian (it-IT) and US English, that voice will be included if you specify <code>yes</code> but not if you specify <code>no</code>.</p>
    pub fn set_include_additional_language_codes(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.inner = self.inner.set_include_additional_language_codes(input);
        self
    }
    /// <p>An opaque pagination token returned from the previous <code>DescribeVoices</code> operation. If present, this indicates where to continue the listing.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An opaque pagination token returned from the previous <code>DescribeVoices</code> operation. If present, this indicates where to continue the listing.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
