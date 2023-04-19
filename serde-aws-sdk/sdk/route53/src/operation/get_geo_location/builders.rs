// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_geo_location::_get_geo_location_output::GetGeoLocationOutputBuilder;

pub use crate::operation::get_geo_location::_get_geo_location_input::GetGeoLocationInputBuilder;

/// Fluent builder constructing a request to `GetGeoLocation`.
///
/// <p>Gets information about whether a specified geographic location is supported for Amazon Route 53 geolocation resource record sets.</p>
/// <p>Route 53 does not perform authorization for this API because it retrieves information that is already available to the public.</p>
/// <p>Use the following syntax to determine whether a continent is supported for geolocation:</p>
/// <p> <code>GET /2013-04-01/geolocation?continentcode=<i>two-letter abbreviation for a continent</i> </code> </p>
/// <p>Use the following syntax to determine whether a country is supported for geolocation:</p>
/// <p> <code>GET /2013-04-01/geolocation?countrycode=<i>two-character country code</i> </code> </p>
/// <p>Use the following syntax to determine whether a subdivision of a country is supported for geolocation:</p>
/// <p> <code>GET /2013-04-01/geolocation?countrycode=<i>two-character country code</i>&amp;subdivisioncode=<i>subdivision code</i> </code> </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetGeoLocationFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_geo_location::builders::GetGeoLocationInputBuilder,
}
impl GetGeoLocationFluentBuilder {
    /// Creates a new `GetGeoLocation`.
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
            crate::operation::get_geo_location::GetGeoLocation,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_geo_location::GetGeoLocationError>,
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
        crate::operation::get_geo_location::GetGeoLocationOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_geo_location::GetGeoLocationError>,
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
    ///     let deserialized_parameters: crate::operation::get_geo_location::builders::GetGeoLocationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_geo_location().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_geo_location::builders::GetGeoLocationInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>For geolocation resource record sets, a two-letter abbreviation that identifies a continent. Amazon Route 53 supports the following continent codes:</p>
    /// <ul>
    /// <li> <p> <b>AF</b>: Africa</p> </li>
    /// <li> <p> <b>AN</b>: Antarctica</p> </li>
    /// <li> <p> <b>AS</b>: Asia</p> </li>
    /// <li> <p> <b>EU</b>: Europe</p> </li>
    /// <li> <p> <b>OC</b>: Oceania</p> </li>
    /// <li> <p> <b>NA</b>: North America</p> </li>
    /// <li> <p> <b>SA</b>: South America</p> </li>
    /// </ul>
    pub fn continent_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.continent_code(input.into());
        self
    }
    /// <p>For geolocation resource record sets, a two-letter abbreviation that identifies a continent. Amazon Route 53 supports the following continent codes:</p>
    /// <ul>
    /// <li> <p> <b>AF</b>: Africa</p> </li>
    /// <li> <p> <b>AN</b>: Antarctica</p> </li>
    /// <li> <p> <b>AS</b>: Asia</p> </li>
    /// <li> <p> <b>EU</b>: Europe</p> </li>
    /// <li> <p> <b>OC</b>: Oceania</p> </li>
    /// <li> <p> <b>NA</b>: North America</p> </li>
    /// <li> <p> <b>SA</b>: South America</p> </li>
    /// </ul>
    pub fn set_continent_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_continent_code(input);
        self
    }
    /// <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    pub fn country_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.country_code(input.into());
        self
    }
    /// <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    pub fn set_country_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_country_code(input);
        self
    }
    /// <p>The code for the subdivision, such as a particular state within the United States. For a list of US state abbreviations, see <a href="https://pe.usps.com/text/pub28/28apb.htm">Appendix B: Two–Letter State and Possession Abbreviations</a> on the United States Postal Service website. For a list of all supported subdivision codes, use the <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListGeoLocations.html">ListGeoLocations</a> API.</p>
    pub fn subdivision_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.subdivision_code(input.into());
        self
    }
    /// <p>The code for the subdivision, such as a particular state within the United States. For a list of US state abbreviations, see <a href="https://pe.usps.com/text/pub28/28apb.htm">Appendix B: Two–Letter State and Possession Abbreviations</a> on the United States Postal Service website. For a list of all supported subdivision codes, use the <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListGeoLocations.html">ListGeoLocations</a> API.</p>
    pub fn set_subdivision_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_subdivision_code(input);
        self
    }
}
