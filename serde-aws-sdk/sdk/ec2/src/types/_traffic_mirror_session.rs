// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Traffic Mirror session.</p>
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
pub struct TrafficMirrorSession {
    /// <p>The ID for the Traffic Mirror session.</p>
    #[doc(hidden)]
    pub traffic_mirror_session_id: std::option::Option<std::string::String>,
    /// <p>The ID of the Traffic Mirror target.</p>
    #[doc(hidden)]
    pub traffic_mirror_target_id: std::option::Option<std::string::String>,
    /// <p>The ID of the Traffic Mirror filter.</p>
    #[doc(hidden)]
    pub traffic_mirror_filter_id: std::option::Option<std::string::String>,
    /// <p>The ID of the Traffic Mirror session's network interface.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>The ID of the account that owns the Traffic Mirror session.</p>
    #[doc(hidden)]
    pub owner_id: std::option::Option<std::string::String>,
    /// <p>The number of bytes in each packet to mirror. These are the bytes after the VXLAN header. To mirror a subset, set this to the length (in bytes) to mirror. For example, if you set this value to 100, then the first 100 bytes that meet the filter criteria are copied to the target. Do not specify this parameter when you want to mirror the entire packet</p>
    #[doc(hidden)]
    pub packet_length: std::option::Option<i32>,
    /// <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
    /// <p>Valid values are 1-32766.</p>
    #[doc(hidden)]
    pub session_number: std::option::Option<i32>,
    /// <p>The virtual network ID associated with the Traffic Mirror session.</p>
    #[doc(hidden)]
    pub virtual_network_id: std::option::Option<i32>,
    /// <p>The description of the Traffic Mirror session.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The tags assigned to the Traffic Mirror session.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TrafficMirrorSession {
    /// <p>The ID for the Traffic Mirror session.</p>
    pub fn traffic_mirror_session_id(&self) -> std::option::Option<&str> {
        self.traffic_mirror_session_id.as_deref()
    }
    /// <p>The ID of the Traffic Mirror target.</p>
    pub fn traffic_mirror_target_id(&self) -> std::option::Option<&str> {
        self.traffic_mirror_target_id.as_deref()
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn traffic_mirror_filter_id(&self) -> std::option::Option<&str> {
        self.traffic_mirror_filter_id.as_deref()
    }
    /// <p>The ID of the Traffic Mirror session's network interface.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The ID of the account that owns the Traffic Mirror session.</p>
    pub fn owner_id(&self) -> std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The number of bytes in each packet to mirror. These are the bytes after the VXLAN header. To mirror a subset, set this to the length (in bytes) to mirror. For example, if you set this value to 100, then the first 100 bytes that meet the filter criteria are copied to the target. Do not specify this parameter when you want to mirror the entire packet</p>
    pub fn packet_length(&self) -> std::option::Option<i32> {
        self.packet_length
    }
    /// <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
    /// <p>Valid values are 1-32766.</p>
    pub fn session_number(&self) -> std::option::Option<i32> {
        self.session_number
    }
    /// <p>The virtual network ID associated with the Traffic Mirror session.</p>
    pub fn virtual_network_id(&self) -> std::option::Option<i32> {
        self.virtual_network_id
    }
    /// <p>The description of the Traffic Mirror session.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The tags assigned to the Traffic Mirror session.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl TrafficMirrorSession {
    /// Creates a new builder-style object to manufacture [`TrafficMirrorSession`](crate::types::TrafficMirrorSession).
    pub fn builder() -> crate::types::builders::TrafficMirrorSessionBuilder {
        crate::types::builders::TrafficMirrorSessionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TrafficMirrorSession;
/// A builder for [`TrafficMirrorSession`](crate::types::TrafficMirrorSession).
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
pub struct TrafficMirrorSessionBuilder {
    pub(crate) traffic_mirror_session_id: std::option::Option<std::string::String>,
    pub(crate) traffic_mirror_target_id: std::option::Option<std::string::String>,
    pub(crate) traffic_mirror_filter_id: std::option::Option<std::string::String>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) owner_id: std::option::Option<std::string::String>,
    pub(crate) packet_length: std::option::Option<i32>,
    pub(crate) session_number: std::option::Option<i32>,
    pub(crate) virtual_network_id: std::option::Option<i32>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TrafficMirrorSessionBuilder {
    /// <p>The ID for the Traffic Mirror session.</p>
    pub fn traffic_mirror_session_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.traffic_mirror_session_id = Some(input.into());
        self
    }
    /// <p>The ID for the Traffic Mirror session.</p>
    pub fn set_traffic_mirror_session_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.traffic_mirror_session_id = input;
        self
    }
    /// <p>The ID of the Traffic Mirror target.</p>
    pub fn traffic_mirror_target_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.traffic_mirror_target_id = Some(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror target.</p>
    pub fn set_traffic_mirror_target_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.traffic_mirror_target_id = input;
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn traffic_mirror_filter_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.traffic_mirror_filter_id = Some(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn set_traffic_mirror_filter_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.traffic_mirror_filter_id = input;
        self
    }
    /// <p>The ID of the Traffic Mirror session's network interface.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_id = Some(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror session's network interface.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>The ID of the account that owns the Traffic Mirror session.</p>
    pub fn owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.owner_id = Some(input.into());
        self
    }
    /// <p>The ID of the account that owns the Traffic Mirror session.</p>
    pub fn set_owner_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The number of bytes in each packet to mirror. These are the bytes after the VXLAN header. To mirror a subset, set this to the length (in bytes) to mirror. For example, if you set this value to 100, then the first 100 bytes that meet the filter criteria are copied to the target. Do not specify this parameter when you want to mirror the entire packet</p>
    pub fn packet_length(mut self, input: i32) -> Self {
        self.packet_length = Some(input);
        self
    }
    /// <p>The number of bytes in each packet to mirror. These are the bytes after the VXLAN header. To mirror a subset, set this to the length (in bytes) to mirror. For example, if you set this value to 100, then the first 100 bytes that meet the filter criteria are copied to the target. Do not specify this parameter when you want to mirror the entire packet</p>
    pub fn set_packet_length(mut self, input: std::option::Option<i32>) -> Self {
        self.packet_length = input;
        self
    }
    /// <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
    /// <p>Valid values are 1-32766.</p>
    pub fn session_number(mut self, input: i32) -> Self {
        self.session_number = Some(input);
        self
    }
    /// <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
    /// <p>Valid values are 1-32766.</p>
    pub fn set_session_number(mut self, input: std::option::Option<i32>) -> Self {
        self.session_number = input;
        self
    }
    /// <p>The virtual network ID associated with the Traffic Mirror session.</p>
    pub fn virtual_network_id(mut self, input: i32) -> Self {
        self.virtual_network_id = Some(input);
        self
    }
    /// <p>The virtual network ID associated with the Traffic Mirror session.</p>
    pub fn set_virtual_network_id(mut self, input: std::option::Option<i32>) -> Self {
        self.virtual_network_id = input;
        self
    }
    /// <p>The description of the Traffic Mirror session.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>The description of the Traffic Mirror session.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags assigned to the Traffic Mirror session.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags assigned to the Traffic Mirror session.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TrafficMirrorSession`](crate::types::TrafficMirrorSession).
    pub fn build(self) -> crate::types::TrafficMirrorSession {
        crate::types::TrafficMirrorSession {
            traffic_mirror_session_id: self.traffic_mirror_session_id,
            traffic_mirror_target_id: self.traffic_mirror_target_id,
            traffic_mirror_filter_id: self.traffic_mirror_filter_id,
            network_interface_id: self.network_interface_id,
            owner_id: self.owner_id,
            packet_length: self.packet_length,
            session_number: self.session_number,
            virtual_network_id: self.virtual_network_id,
            description: self.description,
            tags: self.tags,
        }
    }
}
