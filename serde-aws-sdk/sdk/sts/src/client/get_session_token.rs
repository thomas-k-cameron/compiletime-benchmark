// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSessionToken`](crate::operation::get_session_token::builders::GetSessionTokenFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`duration_seconds(i32)`](crate::operation::get_session_token::builders::GetSessionTokenFluentBuilder::duration_seconds) / [`set_duration_seconds(Option<i32>)`](crate::operation::get_session_token::builders::GetSessionTokenFluentBuilder::set_duration_seconds): <p>The duration, in seconds, that the credentials should remain valid. Acceptable durations for IAM user sessions range from 900 seconds (15 minutes) to 129,600 seconds (36 hours), with 43,200 seconds (12 hours) as the default. Sessions for Amazon Web Services account owners are restricted to a maximum of 3,600 seconds (one hour). If the duration is longer than one hour, the session for Amazon Web Services account owners defaults to one hour.</p>
    ///   - [`serial_number(impl Into<String>)`](crate::operation::get_session_token::builders::GetSessionTokenFluentBuilder::serial_number) / [`set_serial_number(Option<String>)`](crate::operation::get_session_token::builders::GetSessionTokenFluentBuilder::set_serial_number): <p>The identification number of the MFA device that is associated with the IAM user who is making the <code>GetSessionToken</code> call. Specify this value if the IAM user has a policy that requires MFA authentication. The value is either the serial number for a hardware device (such as <code>GAHT12345678</code>) or an Amazon Resource Name (ARN) for a virtual device (such as <code>arn:aws:iam::123456789012:mfa/user</code>). You can find the device for an IAM user by going to the Amazon Web Services Management Console and viewing the user's security credentials. </p>  <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    ///   - [`token_code(impl Into<String>)`](crate::operation::get_session_token::builders::GetSessionTokenFluentBuilder::token_code) / [`set_token_code(Option<String>)`](crate::operation::get_session_token::builders::GetSessionTokenFluentBuilder::set_token_code): <p>The value provided by the MFA device, if MFA is required. If any policy requires the IAM user to submit an MFA code, specify this value. If MFA authentication is required, the user must provide a code when requesting a set of temporary security credentials. A user who fails to provide the code receives an "access denied" response when requesting resources that require MFA authentication.</p>  <p>The format for this parameter, as described by its regex pattern, is a sequence of six numeric digits.</p>
    /// - On success, responds with [`GetSessionTokenOutput`](crate::operation::get_session_token::GetSessionTokenOutput) with field(s):
    ///   - [`credentials(Option<Credentials>)`](crate::operation::get_session_token::GetSessionTokenOutput::credentials): <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p> <note>   <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>  </note>
    /// - On failure, responds with [`SdkError<GetSessionTokenError>`](crate::operation::get_session_token::GetSessionTokenError)
    pub fn get_session_token(
        &self,
    ) -> crate::operation::get_session_token::builders::GetSessionTokenFluentBuilder {
        crate::operation::get_session_token::builders::GetSessionTokenFluentBuilder::new(
            self.handle.clone(),
        )
    }
}