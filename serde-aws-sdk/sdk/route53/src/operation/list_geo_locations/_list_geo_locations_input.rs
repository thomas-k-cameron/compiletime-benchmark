// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A request to get a list of geographic locations that Amazon Route 53 supports for geolocation resource record sets. </p>
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
pub struct ListGeoLocationsInput {
    /// <p>The code for the continent with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is true, and if <code>NextContinentCode</code> from the previous response has a value, enter that value in <code>startcontinentcode</code> to return the next page of results.</p>
    /// <p>Include <code>startcontinentcode</code> only if you want to list continents. Don't include <code>startcontinentcode</code> when you're listing countries or countries with their subdivisions.</p>
    #[doc(hidden)]
    pub start_continent_code: std::option::Option<std::string::String>,
    /// <p>The code for the country with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextCountryCode</code> from the previous response has a value, enter that value in <code>startcountrycode</code> to return the next page of results.</p>
    #[doc(hidden)]
    pub start_country_code: std::option::Option<std::string::String>,
    /// <p>The code for the state of the United States with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextSubdivisionCode</code> from the previous response has a value, enter that value in <code>startsubdivisioncode</code> to return the next page of results.</p>
    /// <p>To list subdivisions (U.S. states), you must include both <code>startcountrycode</code> and <code>startsubdivisioncode</code>.</p>
    #[doc(hidden)]
    pub start_subdivision_code: std::option::Option<std::string::String>,
    /// <p>(Optional) The maximum number of geolocations to be included in the response body for this request. If more than <code>maxitems</code> geolocations remain to be listed, then the value of the <code>IsTruncated</code> element in the response is <code>true</code>.</p>
    #[doc(hidden)]
    pub max_items: std::option::Option<i32>,
}
impl ListGeoLocationsInput {
    /// <p>The code for the continent with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is true, and if <code>NextContinentCode</code> from the previous response has a value, enter that value in <code>startcontinentcode</code> to return the next page of results.</p>
    /// <p>Include <code>startcontinentcode</code> only if you want to list continents. Don't include <code>startcontinentcode</code> when you're listing countries or countries with their subdivisions.</p>
    pub fn start_continent_code(&self) -> std::option::Option<&str> {
        self.start_continent_code.as_deref()
    }
    /// <p>The code for the country with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextCountryCode</code> from the previous response has a value, enter that value in <code>startcountrycode</code> to return the next page of results.</p>
    pub fn start_country_code(&self) -> std::option::Option<&str> {
        self.start_country_code.as_deref()
    }
    /// <p>The code for the state of the United States with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextSubdivisionCode</code> from the previous response has a value, enter that value in <code>startsubdivisioncode</code> to return the next page of results.</p>
    /// <p>To list subdivisions (U.S. states), you must include both <code>startcountrycode</code> and <code>startsubdivisioncode</code>.</p>
    pub fn start_subdivision_code(&self) -> std::option::Option<&str> {
        self.start_subdivision_code.as_deref()
    }
    /// <p>(Optional) The maximum number of geolocations to be included in the response body for this request. If more than <code>maxitems</code> geolocations remain to be listed, then the value of the <code>IsTruncated</code> element in the response is <code>true</code>.</p>
    pub fn max_items(&self) -> std::option::Option<i32> {
        self.max_items
    }
}
impl ListGeoLocationsInput {
    /// Creates a new builder-style object to manufacture [`ListGeoLocationsInput`](crate::operation::list_geo_locations::ListGeoLocationsInput).
    pub fn builder() -> crate::operation::list_geo_locations::builders::ListGeoLocationsInputBuilder
    {
        crate::operation::list_geo_locations::builders::ListGeoLocationsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_geo_locations::ListGeoLocationsInput;
/// A builder for [`ListGeoLocationsInput`](crate::operation::list_geo_locations::ListGeoLocationsInput).
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
pub struct ListGeoLocationsInputBuilder {
    pub(crate) start_continent_code: std::option::Option<std::string::String>,
    pub(crate) start_country_code: std::option::Option<std::string::String>,
    pub(crate) start_subdivision_code: std::option::Option<std::string::String>,
    pub(crate) max_items: std::option::Option<i32>,
}
impl ListGeoLocationsInputBuilder {
    /// <p>The code for the continent with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is true, and if <code>NextContinentCode</code> from the previous response has a value, enter that value in <code>startcontinentcode</code> to return the next page of results.</p>
    /// <p>Include <code>startcontinentcode</code> only if you want to list continents. Don't include <code>startcontinentcode</code> when you're listing countries or countries with their subdivisions.</p>
    pub fn start_continent_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.start_continent_code = Some(input.into());
        self
    }
    /// <p>The code for the continent with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is true, and if <code>NextContinentCode</code> from the previous response has a value, enter that value in <code>startcontinentcode</code> to return the next page of results.</p>
    /// <p>Include <code>startcontinentcode</code> only if you want to list continents. Don't include <code>startcontinentcode</code> when you're listing countries or countries with their subdivisions.</p>
    pub fn set_start_continent_code(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.start_continent_code = input;
        self
    }
    /// <p>The code for the country with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextCountryCode</code> from the previous response has a value, enter that value in <code>startcountrycode</code> to return the next page of results.</p>
    pub fn start_country_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.start_country_code = Some(input.into());
        self
    }
    /// <p>The code for the country with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextCountryCode</code> from the previous response has a value, enter that value in <code>startcountrycode</code> to return the next page of results.</p>
    pub fn set_start_country_code(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.start_country_code = input;
        self
    }
    /// <p>The code for the state of the United States with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextSubdivisionCode</code> from the previous response has a value, enter that value in <code>startsubdivisioncode</code> to return the next page of results.</p>
    /// <p>To list subdivisions (U.S. states), you must include both <code>startcountrycode</code> and <code>startsubdivisioncode</code>.</p>
    pub fn start_subdivision_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.start_subdivision_code = Some(input.into());
        self
    }
    /// <p>The code for the state of the United States with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextSubdivisionCode</code> from the previous response has a value, enter that value in <code>startsubdivisioncode</code> to return the next page of results.</p>
    /// <p>To list subdivisions (U.S. states), you must include both <code>startcountrycode</code> and <code>startsubdivisioncode</code>.</p>
    pub fn set_start_subdivision_code(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.start_subdivision_code = input;
        self
    }
    /// <p>(Optional) The maximum number of geolocations to be included in the response body for this request. If more than <code>maxitems</code> geolocations remain to be listed, then the value of the <code>IsTruncated</code> element in the response is <code>true</code>.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = Some(input);
        self
    }
    /// <p>(Optional) The maximum number of geolocations to be included in the response body for this request. If more than <code>maxitems</code> geolocations remain to be listed, then the value of the <code>IsTruncated</code> element in the response is <code>true</code>.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// Consumes the builder and constructs a [`ListGeoLocationsInput`](crate::operation::list_geo_locations::ListGeoLocationsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_geo_locations::ListGeoLocationsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::list_geo_locations::ListGeoLocationsInput {
                start_continent_code: self.start_continent_code,
                start_country_code: self.start_country_code,
                start_subdivision_code: self.start_subdivision_code,
                max_items: self.max_items,
            },
        )
    }
}
