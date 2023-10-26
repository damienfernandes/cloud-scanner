//! A module to allow inventory of cloud resources.
//!
//!  It define s a CloudInventory trait that you should use when implementing vendor specific inventory .

use crate::cloud_resource::*;
use anyhow::Result;
use async_trait::async_trait;
use aws_sdk_cloudwatch::operation::get_metric_statistics::GetMetricStatisticsOutput;
use aws_sdk_ec2::types::{Instance, Volume};

/// A that you should implement to support vendor-specific inventory of cloud resources.
///
/// For example, you may want to implement it to ensure that cloud-scanner is able to support an additional cloud provider.
#[async_trait]
pub trait CloudInventory {
    /// Returns a list list of cloud resources
    async fn list_resources(
        &self,
        tags: &[String],
        include_block_storage: bool,
        simulation: bool,
    ) -> Result<Vec<CloudResource>>;

    async fn new(aws_region: &str, filename: &str) -> Self;
    async fn get_data_from_file(filename: &str);
    async fn load_aws_config(aws_region: &str) -> aws_types::sdk_config::SdkConfig;
    fn cloud_resource_tags_from_aws_tags(aws_tags: Option<&[aws_sdk_ec2::types::Tag]>) -> Vec<CloudResourceTag>;
    async fn get_instances_with_usage_data(&self, tags: &[String], simulation: bool) -> Result<Vec<CloudResource>>;
    async fn list_instances(self, _tags: &[String]) -> Result<Vec<Instance>>;
    async fn get_average_cpu(self, instance_id: &str) -> Result<f64>;
    async fn get_average_cpu_usage_of_last_10_minutes(self, instance_id: &str) -> Result<GetMetricStatisticsOutput, aws_sdk_cloudwatch::Error>;
    async fn list_volumes(self, tags: &[String]) -> Result<Vec<Volume>>;
    async fn get_volumes_with_usage_data(&self, tags: &[String], simulation: bool) -> Result<Vec<CloudResource>>;
}
