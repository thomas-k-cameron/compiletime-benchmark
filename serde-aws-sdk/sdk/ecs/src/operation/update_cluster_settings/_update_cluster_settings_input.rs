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
pub struct UpdateClusterSettingsInput {
    /// <p>The name of the cluster to modify the settings for.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<std::string::String>,
    /// <p>The setting to use by default for a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <code>PutAccountSetting</code> or <code>PutAccountSettingDefault</code>.</p> <important>
    /// <p>Currently, if you delete an existing cluster that does not have Container Insights turned on, and then create a new cluster with the same name with Container Insights tuned on, Container Insights will not actually be turned on. If you want to preserve the same name for your existing cluster and turn on Container Insights, you must wait 7 days before you can re-create it.</p>
    /// </important>
    #[doc(hidden)]
    pub settings: std::option::Option<std::vec::Vec<crate::types::ClusterSetting>>,
}
impl UpdateClusterSettingsInput {
    /// <p>The name of the cluster to modify the settings for.</p>
    pub fn cluster(&self) -> std::option::Option<&str> {
        self.cluster.as_deref()
    }
    /// <p>The setting to use by default for a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <code>PutAccountSetting</code> or <code>PutAccountSettingDefault</code>.</p> <important>
    /// <p>Currently, if you delete an existing cluster that does not have Container Insights turned on, and then create a new cluster with the same name with Container Insights tuned on, Container Insights will not actually be turned on. If you want to preserve the same name for your existing cluster and turn on Container Insights, you must wait 7 days before you can re-create it.</p>
    /// </important>
    pub fn settings(&self) -> std::option::Option<&[crate::types::ClusterSetting]> {
        self.settings.as_deref()
    }
}
impl UpdateClusterSettingsInput {
    /// Creates a new builder-style object to manufacture [`UpdateClusterSettingsInput`](crate::operation::update_cluster_settings::UpdateClusterSettingsInput).
    pub fn builder(
    ) -> crate::operation::update_cluster_settings::builders::UpdateClusterSettingsInputBuilder
    {
        crate::operation::update_cluster_settings::builders::UpdateClusterSettingsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::update_cluster_settings::UpdateClusterSettingsInput;
/// A builder for [`UpdateClusterSettingsInput`](crate::operation::update_cluster_settings::UpdateClusterSettingsInput).
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
pub struct UpdateClusterSettingsInputBuilder {
    pub(crate) cluster: std::option::Option<std::string::String>,
    pub(crate) settings: std::option::Option<std::vec::Vec<crate::types::ClusterSetting>>,
}
impl UpdateClusterSettingsInputBuilder {
    /// <p>The name of the cluster to modify the settings for.</p>
    pub fn cluster(mut self, input: impl Into<std::string::String>) -> Self {
        self.cluster = Some(input.into());
        self
    }
    /// <p>The name of the cluster to modify the settings for.</p>
    pub fn set_cluster(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cluster = input;
        self
    }
    /// Appends an item to `settings`.
    ///
    /// To override the contents of this collection use [`set_settings`](Self::set_settings).
    ///
    /// <p>The setting to use by default for a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <code>PutAccountSetting</code> or <code>PutAccountSettingDefault</code>.</p> <important>
    /// <p>Currently, if you delete an existing cluster that does not have Container Insights turned on, and then create a new cluster with the same name with Container Insights tuned on, Container Insights will not actually be turned on. If you want to preserve the same name for your existing cluster and turn on Container Insights, you must wait 7 days before you can re-create it.</p>
    /// </important>
    pub fn settings(mut self, input: crate::types::ClusterSetting) -> Self {
        let mut v = self.settings.unwrap_or_default();
        v.push(input);
        self.settings = Some(v);
        self
    }
    /// <p>The setting to use by default for a cluster. This parameter is used to turn on CloudWatch Container Insights for a cluster. If this value is specified, it overrides the <code>containerInsights</code> value set with <code>PutAccountSetting</code> or <code>PutAccountSettingDefault</code>.</p> <important>
    /// <p>Currently, if you delete an existing cluster that does not have Container Insights turned on, and then create a new cluster with the same name with Container Insights tuned on, Container Insights will not actually be turned on. If you want to preserve the same name for your existing cluster and turn on Container Insights, you must wait 7 days before you can re-create it.</p>
    /// </important>
    pub fn set_settings(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ClusterSetting>>,
    ) -> Self {
        self.settings = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateClusterSettingsInput`](crate::operation::update_cluster_settings::UpdateClusterSettingsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::update_cluster_settings::UpdateClusterSettingsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::update_cluster_settings::UpdateClusterSettingsInput {
                cluster: self.cluster,
                settings: self.settings,
            },
        )
    }
}
