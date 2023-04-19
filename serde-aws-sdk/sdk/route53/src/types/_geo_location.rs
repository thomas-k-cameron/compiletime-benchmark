// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains information about a geographic location.</p>
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
pub struct GeoLocation {
    /// <p>The two-letter code for the continent.</p>
    /// <p>Amazon Route 53 supports the following continent codes:</p>
    /// <ul>
    /// <li> <p> <b>AF</b>: Africa</p> </li>
    /// <li> <p> <b>AN</b>: Antarctica</p> </li>
    /// <li> <p> <b>AS</b>: Asia</p> </li>
    /// <li> <p> <b>EU</b>: Europe</p> </li>
    /// <li> <p> <b>OC</b>: Oceania</p> </li>
    /// <li> <p> <b>NA</b>: North America</p> </li>
    /// <li> <p> <b>SA</b>: South America</p> </li>
    /// </ul>
    /// <p>Constraint: Specifying <code>ContinentCode</code> with either <code>CountryCode</code> or <code>SubdivisionCode</code> returns an <code>InvalidInput</code> error.</p>
    #[doc(hidden)]
    pub continent_code: std::option::Option<std::string::String>,
    /// <p>For geolocation resource record sets, the two-letter code for a country.</p>
    /// <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    #[doc(hidden)]
    pub country_code: std::option::Option<std::string::String>,
    /// <p>For geolocation resource record sets, the two-letter code for a state of the United States. Route 53 doesn't support any other values for <code>SubdivisionCode</code>. For a list of state abbreviations, see <a href="https://pe.usps.com/text/pub28/28apb.htm">Appendix B: Two–Letter State and Possession Abbreviations</a> on the United States Postal Service website. </p>
    /// <p>If you specify <code>subdivisioncode</code>, you must also specify <code>US</code> for <code>CountryCode</code>. </p>
    #[doc(hidden)]
    pub subdivision_code: std::option::Option<std::string::String>,
}
impl GeoLocation {
    /// <p>The two-letter code for the continent.</p>
    /// <p>Amazon Route 53 supports the following continent codes:</p>
    /// <ul>
    /// <li> <p> <b>AF</b>: Africa</p> </li>
    /// <li> <p> <b>AN</b>: Antarctica</p> </li>
    /// <li> <p> <b>AS</b>: Asia</p> </li>
    /// <li> <p> <b>EU</b>: Europe</p> </li>
    /// <li> <p> <b>OC</b>: Oceania</p> </li>
    /// <li> <p> <b>NA</b>: North America</p> </li>
    /// <li> <p> <b>SA</b>: South America</p> </li>
    /// </ul>
    /// <p>Constraint: Specifying <code>ContinentCode</code> with either <code>CountryCode</code> or <code>SubdivisionCode</code> returns an <code>InvalidInput</code> error.</p>
    pub fn continent_code(&self) -> std::option::Option<&str> {
        self.continent_code.as_deref()
    }
    /// <p>For geolocation resource record sets, the two-letter code for a country.</p>
    /// <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    pub fn country_code(&self) -> std::option::Option<&str> {
        self.country_code.as_deref()
    }
    /// <p>For geolocation resource record sets, the two-letter code for a state of the United States. Route 53 doesn't support any other values for <code>SubdivisionCode</code>. For a list of state abbreviations, see <a href="https://pe.usps.com/text/pub28/28apb.htm">Appendix B: Two–Letter State and Possession Abbreviations</a> on the United States Postal Service website. </p>
    /// <p>If you specify <code>subdivisioncode</code>, you must also specify <code>US</code> for <code>CountryCode</code>. </p>
    pub fn subdivision_code(&self) -> std::option::Option<&str> {
        self.subdivision_code.as_deref()
    }
}
impl GeoLocation {
    /// Creates a new builder-style object to manufacture [`GeoLocation`](crate::types::GeoLocation).
    pub fn builder() -> crate::types::builders::GeoLocationBuilder {
        crate::types::builders::GeoLocationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::GeoLocation;
/// A builder for [`GeoLocation`](crate::types::GeoLocation).
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
pub struct GeoLocationBuilder {
    pub(crate) continent_code: std::option::Option<std::string::String>,
    pub(crate) country_code: std::option::Option<std::string::String>,
    pub(crate) subdivision_code: std::option::Option<std::string::String>,
}
impl GeoLocationBuilder {
    /// <p>The two-letter code for the continent.</p>
    /// <p>Amazon Route 53 supports the following continent codes:</p>
    /// <ul>
    /// <li> <p> <b>AF</b>: Africa</p> </li>
    /// <li> <p> <b>AN</b>: Antarctica</p> </li>
    /// <li> <p> <b>AS</b>: Asia</p> </li>
    /// <li> <p> <b>EU</b>: Europe</p> </li>
    /// <li> <p> <b>OC</b>: Oceania</p> </li>
    /// <li> <p> <b>NA</b>: North America</p> </li>
    /// <li> <p> <b>SA</b>: South America</p> </li>
    /// </ul>
    /// <p>Constraint: Specifying <code>ContinentCode</code> with either <code>CountryCode</code> or <code>SubdivisionCode</code> returns an <code>InvalidInput</code> error.</p>
    pub fn continent_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.continent_code = Some(input.into());
        self
    }
    /// <p>The two-letter code for the continent.</p>
    /// <p>Amazon Route 53 supports the following continent codes:</p>
    /// <ul>
    /// <li> <p> <b>AF</b>: Africa</p> </li>
    /// <li> <p> <b>AN</b>: Antarctica</p> </li>
    /// <li> <p> <b>AS</b>: Asia</p> </li>
    /// <li> <p> <b>EU</b>: Europe</p> </li>
    /// <li> <p> <b>OC</b>: Oceania</p> </li>
    /// <li> <p> <b>NA</b>: North America</p> </li>
    /// <li> <p> <b>SA</b>: South America</p> </li>
    /// </ul>
    /// <p>Constraint: Specifying <code>ContinentCode</code> with either <code>CountryCode</code> or <code>SubdivisionCode</code> returns an <code>InvalidInput</code> error.</p>
    pub fn set_continent_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.continent_code = input;
        self
    }
    /// <p>For geolocation resource record sets, the two-letter code for a country.</p>
    /// <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    pub fn country_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.country_code = Some(input.into());
        self
    }
    /// <p>For geolocation resource record sets, the two-letter code for a country.</p>
    /// <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    pub fn set_country_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.country_code = input;
        self
    }
    /// <p>For geolocation resource record sets, the two-letter code for a state of the United States. Route 53 doesn't support any other values for <code>SubdivisionCode</code>. For a list of state abbreviations, see <a href="https://pe.usps.com/text/pub28/28apb.htm">Appendix B: Two–Letter State and Possession Abbreviations</a> on the United States Postal Service website. </p>
    /// <p>If you specify <code>subdivisioncode</code>, you must also specify <code>US</code> for <code>CountryCode</code>. </p>
    pub fn subdivision_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.subdivision_code = Some(input.into());
        self
    }
    /// <p>For geolocation resource record sets, the two-letter code for a state of the United States. Route 53 doesn't support any other values for <code>SubdivisionCode</code>. For a list of state abbreviations, see <a href="https://pe.usps.com/text/pub28/28apb.htm">Appendix B: Two–Letter State and Possession Abbreviations</a> on the United States Postal Service website. </p>
    /// <p>If you specify <code>subdivisioncode</code>, you must also specify <code>US</code> for <code>CountryCode</code>. </p>
    pub fn set_subdivision_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.subdivision_code = input;
        self
    }
    /// Consumes the builder and constructs a [`GeoLocation`](crate::types::GeoLocation).
    pub fn build(self) -> crate::types::GeoLocation {
        crate::types::GeoLocation {
            continent_code: self.continent_code,
            country_code: self.country_code,
            subdivision_code: self.subdivision_code,
        }
    }
}
