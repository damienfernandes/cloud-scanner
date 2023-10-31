use crate::cloud_inventory::CloudInventory;
use crate::cloud_resource::*;

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::vec::Vec;
use aws_sdk_cloudwatch::operation::get_metric_statistics::GetMetricStatisticsOutput;
use aws_sdk_ec2::types::Volume;
use crate::cloud_resource::ResourceDetails::Instance;

#[derive(Debug, Deserialize)]
struct Tag {
    key: String,
    value: String,
}

///  An inventory of AWS resources
#[derive(Clone, Debug)]
pub struct AwsInventoryFromFile {
    aws_region: String,
}

#[async_trait]
impl CloudInventory for AwsInventoryFromFile {
    /// list resources
    async fn list_resources(
        &self,
        tags: &[String],
        include_block_storage: bool,
        simulation: bool,
    ) -> Result<Vec<CloudResource>> {
        let mut inventory: Vec<CloudResource> = Vec::new();

        /// TODO : get data from file simulation
        println!("WARNING: TODO get data from file simulation");

        /// else from direct read
        let mut instances = self.clone().get_instances_with_usage_data(tags, simulation).await?;
        inventory.append(&mut instances);
        if include_block_storage {
            let mut volumes = self.clone().get_volumes_with_usage_data(tags, simulation).await?;
            inventory.append(&mut volumes);
        }
        Ok(inventory)
    }

    /// Initializes it with a specific region and configures the SDK's that will query your account to perform the inventory of resources.
    async fn new(aws_region: &str, filename: &str) -> Self {
        //let shared_config = Self::load_aws_config(aws_region).await;
        Self:: get_data_from_file(filename).await;
        AwsInventoryFromFile {
            aws_region: String::from(aws_region),
        }
    }

    async fn get_data_from_file(filename: &str) {
        // Get the file path from the command-line arguments
        let file_path: String = String::from(filename);

        // Read the JSON file into a string
        let mut file = File::open(file_path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");

        // Deserialize the JSON data into a Vec<Item>
        let items: Vec<CloudResource> = serde_json::from_str(&contents).expect("Unable to parse JSON");

        // Print the data
        for item in &items {
            println!("Provider: {:?}", item.provider);
            println!("ID: {}", item.id);
            println!("AWS Region: {}", item.location.aws_region);
            println!("ISO Country Code: {}", item.location.iso_country_code);


            let resource_details = &item.resource_details;
            println!("Resource details: {:?}", resource_details);

            /*match resource_details {
                ResourceDetails::Instance {
                    instance_type,
                    usage,
                } => {
                    println!("Instance Type: {}", resource_details.instance_type);
                    println!("Average CPU Load: {}", resource_details.usage.average_cpu_load);
                    println!("Usage Duration (seconds): {}", resource_details.usage.usage_duration_seconds);

                }
                    let mut usage_cloud: UsageCloud = UsageCloud::new();

                    //usage_cloud.hours_life_time = Some(usage_duration_hours.to_owned());
                    usage_cloud.usage_location = Some(cr.location.iso_country_code.to_owned());

                    if let Some(instance_usage) = usage {
                        usage_cloud.time_workload = Some(instance_usage.average_cpu_load as f32);
                    }

                    let mut cloud: Cloud = Cloud::new();
                    cloud.provider = Some(String::from("aws"));
                    cloud.instance_type = Some(instance_type.clone());
                    cloud.usage = Some(Box::new(usage_cloud));

                    let res = cloud_api::instance_cloud_impact_v1_cloud_instance_post(
                        &self.configuration,
                        Some(verbose),
                        Some(usage_duration_hours.to_owned()),
                        Some(criteria),
                        Some(cloud),
                    )
                        .await;

                    match res {
                        Ok(res) => Some(res),
                        Err(e) => {
                            warn!(
                            "Warning: Cannot get impacts from API for instance type {}: {}",
                            instance_type, e
                        );
                            None
                        }
                    }
                }


                if let Some(details) = &item.resource_details::Instance {
                println!("Instance Type: {}", details.instance_type);
                println!("Average CPU Load: {}", details.usage.average_cpu_load);
                println!("Usage Duration (seconds): {}", details.usage.usage_duration_seconds);
            } else if let Some(details) = &item.resource_details.block_storage {
                println!("Storage Type: {}", details.storage_type);
                println!("Storage Size (GB): {}", details.usage.size_gb);
                println!("Usage Duration (seconds): {}", details.usage.usage_duration_seconds);
            }

            println!("Tags:");
            for tag in &item.tags {
                println!("  Key: {}", tag.key);
                println!("  Value: {}", tag.value);
            }*/

            println!("-------------------------");
        }
    }

    async fn load_aws_config(aws_region: &str) -> aws_types::sdk_config::SdkConfig {
        unimplemented!("Not implemented yet");
    }

    fn cloud_resource_tags_from_aws_tags(
        aws_tags: Option<&[aws_sdk_ec2::types::Tag]>,
    ) -> Vec<CloudResourceTag> {
        unimplemented!("Not implemented yet");
    }

    /// Perform inventory of all aws instances of the region
    async fn get_instances_with_usage_data(&self, tags: &[String], simulation: bool) -> Result<Vec<CloudResource>> {
        // TODO DFE: Not implemented yet



        /*let instances: Vec<Instance> = self
            .clone()
            .list_instances(tags)
            .await
            .context("Cannot list instances")
            .unwrap();
        let location = UsageLocation::from(self.aws_region.as_str());*/

        // Just to display statistics
        let cpu_info_timer = Instant::now();

        let mut inventory: Vec<CloudResource> = Vec::new();
        /* for instance in instances {
             let instance_id = instance.instance_id().unwrap().to_string();
             let cpuload: f64 = self
                 .clone()
                 .get_average_cpu(&instance_id)
                 .await
                 .context("Cannot get CPU load of instance")
                 .unwrap();

             let usage: InstanceUsage = InstanceUsage {
                 average_cpu_load: cpuload,
                 usage_duration_seconds: 300,
             };

             let cloud_resource_tags = Self::cloud_resource_tags_from_aws_tags(instance.tags());

             info!(
                 "Total time spend querying CPU load of instances: {:?}",
                 cpu_info_timer.elapsed()
             );

             let inst = CloudResource {
                 provider: CloudProvider::AWS,
                 id: instance_id,
                 location: location.clone(),
                 resource_details: ResourceDetails::Instance {
                     instance_type: instance.instance_type().unwrap().as_str().to_owned(),
                     usage: Some(usage),
                 },

                 tags: cloud_resource_tags,
             };

             if inst.has_matching_tags(tags) {
                 debug!("Resource matched on tags: {:?}", inst.id);
                 inventory.push(inst);
             } else {
                 debug!("Filtered instance (tags do not match: {:?}", inst);
             }
             //if cs matches the tags passed in param keep it (push it, otherwise skipp it)
         }*/

        Ok(inventory)
    }

    async fn list_instances(self, _tags: &[String]) -> Result<Vec<aws_sdk_ec2::types::Instance>> {
        unimplemented!("Not implemented yet");
    }

    async fn get_average_cpu(self, instance_id: &str) -> Result<f64> {
        unimplemented!("Not implemented yet");
    }

    async fn get_average_cpu_usage_of_last_10_minutes(
        self,
        instance_id: &str,
    ) -> Result<GetMetricStatisticsOutput, aws_sdk_cloudwatch::Error> {
        unimplemented!("Not implemented yet");
    }

    async fn list_volumes(self, tags: &[String]) -> Result<Vec<Volume>> {
        unimplemented!("Not implemented yet");
    }

    /// Perform inventory of all aws volumes of the region
    async fn get_volumes_with_usage_data(&self, tags: &[String], simulation: bool) -> Result<Vec<CloudResource>> {
        // TODO DFE: Not implemented yet
        /*let location = UsageLocation::from(self.aws_region.as_str());
        let volumes = self.clone().list_volumes(tags).await.unwrap();*/
        let mut resources: Vec<CloudResource> = Vec::new();

        /*for volume in volumes {
            let volume_id = volume.volume_id().unwrap();

            let usage: StorageUsage = StorageUsage {
                size_gb: volume.size().unwrap(),
                usage_duration_seconds: 3600,
            };

            let volume_type: String = volume.volume_type().unwrap().as_str().to_string();
            let mut attached_instances: Option<Vec<StorageAttachment>> = None;

            if let Some(all_volume_attachments) = volume.attachments.clone() {
                for single_attachment in all_volume_attachments {
                    let mut attachment_list: Vec<StorageAttachment> = Vec::new();

                    if let Some(instance_id) = single_attachment.instance_id {
                        attachment_list.push(StorageAttachment { instance_id });
                    }
                    attached_instances = Some(attachment_list);
                }
            }

            let disk = CloudResource {
                provider: CloudProvider::AWS,
                id: volume_id.into(),
                location: location.clone(),
                resource_details: ResourceDetails::BlockStorage {
                    storage_type: volume_type,
                    usage: Some(usage),
                    attached_instances,
                },
                tags: Self::cloud_resource_tags_from_aws_tags(volume.tags()),
            };
            resources.push(disk);
        }*/

        Ok(resources)
    }
}
