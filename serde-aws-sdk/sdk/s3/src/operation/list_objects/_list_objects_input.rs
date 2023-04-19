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
pub struct ListObjectsInput {
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>A delimiter is a character you use to group keys.</p>
    #[doc(hidden)]
    pub delimiter: std::option::Option<std::string::String>,
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key may contain any Unicode character; however, XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    #[doc(hidden)]
    pub encoding_type: std::option::Option<crate::types::EncodingType>,
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
    /// <p>Sets the maximum number of keys returned in the response. By default the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. </p>
    #[doc(hidden)]
    pub max_keys: std::option::Option<i32>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    #[doc(hidden)]
    pub prefix: std::option::Option<std::string::String>,
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    #[doc(hidden)]
    pub request_payer: std::option::Option<crate::types::RequestPayer>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: std::option::Option<std::string::String>,
}
impl ListObjectsInput {
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>A delimiter is a character you use to group keys.</p>
    pub fn delimiter(&self) -> std::option::Option<&str> {
        self.delimiter.as_deref()
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key may contain any Unicode character; however, XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn encoding_type(&self) -> std::option::Option<&crate::types::EncodingType> {
        self.encoding_type.as_ref()
    }
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket.</p>
    pub fn marker(&self) -> std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>Sets the maximum number of keys returned in the response. By default the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. </p>
    pub fn max_keys(&self) -> std::option::Option<i32> {
        self.max_keys
    }
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub fn request_payer(&self) -> std::option::Option<&crate::types::RequestPayer> {
        self.request_payer.as_ref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl ListObjectsInput {
    /// Creates a new builder-style object to manufacture [`ListObjectsInput`](crate::operation::list_objects::ListObjectsInput).
    pub fn builder() -> crate::operation::list_objects::builders::ListObjectsInputBuilder {
        crate::operation::list_objects::builders::ListObjectsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_objects::ListObjectsInput;
/// A builder for [`ListObjectsInput`](crate::operation::list_objects::ListObjectsInput).
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
pub struct ListObjectsInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) delimiter: std::option::Option<std::string::String>,
    pub(crate) encoding_type: std::option::Option<crate::types::EncodingType>,
    pub(crate) marker: std::option::Option<std::string::String>,
    pub(crate) max_keys: std::option::Option<i32>,
    pub(crate) prefix: std::option::Option<std::string::String>,
    pub(crate) request_payer: std::option::Option<crate::types::RequestPayer>,
    pub(crate) expected_bucket_owner: std::option::Option<std::string::String>,
}
impl ListObjectsInputBuilder {
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>A delimiter is a character you use to group keys.</p>
    pub fn delimiter(mut self, input: impl Into<std::string::String>) -> Self {
        self.delimiter = Some(input.into());
        self
    }
    /// <p>A delimiter is a character you use to group keys.</p>
    pub fn set_delimiter(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.delimiter = input;
        self
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key may contain any Unicode character; however, XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn encoding_type(mut self, input: crate::types::EncodingType) -> Self {
        self.encoding_type = Some(input);
        self
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key may contain any Unicode character; however, XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn set_encoding_type(
        mut self,
        input: std::option::Option<crate::types::EncodingType>,
    ) -> Self {
        self.encoding_type = input;
        self
    }
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.marker = Some(input.into());
        self
    }
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>Sets the maximum number of keys returned in the response. By default the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. </p>
    pub fn max_keys(mut self, input: i32) -> Self {
        self.max_keys = Some(input);
        self
    }
    /// <p>Sets the maximum number of keys returned in the response. By default the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. </p>
    pub fn set_max_keys(mut self, input: std::option::Option<i32>) -> Self {
        self.max_keys = input;
        self
    }
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix = Some(input.into());
        self
    }
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub fn request_payer(mut self, input: crate::types::RequestPayer) -> Self {
        self.request_payer = Some(input);
        self
    }
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub fn set_request_payer(
        mut self,
        input: std::option::Option<crate::types::RequestPayer>,
    ) -> Self {
        self.request_payer = input;
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl Into<std::string::String>) -> Self {
        self.expected_bucket_owner = Some(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.expected_bucket_owner = input;
        self
    }
    /// Consumes the builder and constructs a [`ListObjectsInput`](crate::operation::list_objects::ListObjectsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_objects::ListObjectsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::list_objects::ListObjectsInput {
            bucket: self.bucket,
            delimiter: self.delimiter,
            encoding_type: self.encoding_type,
            marker: self.marker,
            max_keys: self.max_keys,
            prefix: self.prefix,
            request_payer: self.request_payer,
            expected_bucket_owner: self.expected_bucket_owner,
        })
    }
}