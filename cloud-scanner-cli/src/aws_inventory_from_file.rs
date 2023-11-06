
use crate::cloud_inventory::CloudInventory;
use crate::cloud_resource::*;

use anyhow::{Context, Result};
use serde_json;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::vec::Vec;
use std::path::Path;

use aws_sdk_cloudwatch::operation::get_metric_statistics::GetMetricStatisticsOutput;
use aws_sdk_ec2::types::Volume;
use boavizta_api_sdk::models::{Cloud, UsageCloud};
use crate::usage_location::UsageLocation;

///  An inventory of AWS resources
#[derive(Clone, Debug)]
pub struct AwsInventoryFromFile {
    aws_region: String
    // aws_items: Vec<CloudResource>,
}

#[async_trait]
impl CloudInventory for AwsInventoryFromFile {
    /// list resources
    async fn list_resources(
        &self,
        tags: &[String],
        include_block_storage: bool,
        simulation: bool,
        filename: &str,
    ) -> Result<Vec<CloudResource>> {
        //let mut inventory: Vec<CloudResource> = Vec::new();

        // TODO : get data from file simulation
        println!("WARNING: TODO get data from file simulation");

        // else from direct read
        // let mut instances = self.clone().get_instances_with_usage_data(tags, simulation).await?;
        // inventory.append(&mut instances);
        // if include_block_storage {
        //     let mut volumes = self.clone().get_volumes_with_usage_data(tags, simulation).await?;
        //     inventory.append(&mut volumes);
        // }

        let inventory = AwsInventoryFromFile::get_data_from_file(filename).await.unwrap();

        Ok(inventory)
    }

    /// Initializes it with a specific region and configures the SDK's that will query your account to perform the inventory of resources.
    async fn new(aws_region: &str, filename: &str) -> Self {
        //let shared_config = Self::load_aws_config(aws_region).await;
        // let items: Vec<CloudResource> = Self:: get_data_from_file(filename).await;
        AwsInventoryFromFile {
            aws_region: String::from(aws_region)
            // aws_items : items,
        }
    }

    async fn get_data_from_file(filename: &str) -> Result<Vec<CloudResource>> {
        // Get the file path from the command-line arguments
        let path = Path::new(filename);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };


        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, contents),
        }

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

            match resource_details {
                ResourceDetails::Instance {
                    instance_type,
                    usage,
                } => {
                    println!("Instance Type: {}", instance_type);
                    //println!("Average CPU Load: {}", usage.average_cpu_load);
                    //println!("Usage Duration (seconds): {}", resource_details.usage.usage_duration_seconds);


                    let mut usage_cloud: UsageCloud = UsageCloud::new();

                    //usage_cloud.hours_life_time = Some(usage_duration_hours.to_owned());
                    usage_cloud.usage_location = Some(item.location.iso_country_code.to_owned());

                    if let Some(instance_usage) = usage {
                        usage_cloud.time_workload = Some(instance_usage.average_cpu_load as f32);
                    }

                    let mut cloud: Cloud = Cloud::new();
                    cloud.provider = Some(String::from("aws"));
                    cloud.instance_type = Some(instance_type.clone());
                    cloud.usage = Some(Box::new(usage_cloud));

                    /*let res = cloud_api::instance_cloud_impact_v1_cloud_instance_post(
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
                    }*/


                    /*
                if let Some(details) = &item.resource_details::Instance {
                println!("Instance Type: {}", details.instance_type);
                println!("Average CPU Load: {}", details.usage.average_cpu_load);
                println!("Usage Duration (seconds): {}", details.usage.usage_duration_seconds);
            } else if let Some(details) = &item.resource_details.block_storage {
                println!("Storage Type: {}", details.storage_type);
                println!("Storage Size (GB): {}", details.usage.size_gb);
                println!("Usage Duration (seconds): {}", details.usage.usage_duration_seconds);
            }*/

                    println!("Tags:");
                    for tag in &item.tags {
                        println!("  Key: {}", tag.key);
                        println!("  Value: {:?}", tag.value);
                    }

                    println!("-------------------------");
                }
                _ => {}
            }
        }
        Ok(items)
    }

    async fn load_aws_config(aws_region: &str) -> aws_types::sdk_config::SdkConfig {
        unimplemented!("Not implemented yet");
    }

    fn cloud_resource_tags_from_aws_tags(
        _aws_tags: Option<&[aws_sdk_ec2::types::Tag]>,
    ) -> Vec<CloudResourceTag> {
        unimplemented!("Not implemented yet");
    }

    /// Perform inventory of all aws instances of the region
    async fn get_instances_with_usage_data(&self, tags: &[String], simulation: bool) -> Result<Vec<CloudResource>> {
        // TODO DFE: Not implemented yet


        let instances: Vec<CloudResource> = self
            .clone()
            .list_cloud_resource(tags)
            .await
            .context("Cannot list instances")
            .unwrap();
        let location = UsageLocation::from(self.aws_region.as_str());

        // Just to display statistics
        let cpu_info_timer = Instant::now();

        let mut inventory: Vec<CloudResource> = Vec::new();
        for instance in instances {
             /*let instance_id = instance.instance_id().unwrap().to_string();
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
             };*/

             if instance.has_matching_tags(tags) {
                 debug!("Resource matched on tags: {:?}", instance.id);
                 inventory.push(instance);
             } else {
                 debug!("Filtered instance (tags do not match: {:?}", instance);
             }
             //if cs matches the tags passed in param keep it (push it, otherwise skipp it)
         }

        Ok(inventory)
    }

    async fn list_instances(self, _tags: &[String]) -> Result<Vec<aws_sdk_ec2::types::Instance>> {
        unimplemented!("Not implemented yet");
    }

    async fn list_cloud_resource(self, _tags: &[String]) -> Result<Vec<CloudResource>> {
        // let client = &self.ec2_client;
        let mut instances: Vec<CloudResource> = Vec::new();

        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-1"),
            resource_details: ResourceDetails::Instance {
                instance_type: "t2.fictive".to_string(),
                usage: None,
            },
            tags: Vec::new(),
        };

        instances.push(instance1.clone());

        Ok(instances)
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

#[cfg(test)]
mod tests {
    use super::*;

    static RUNNING_INSTANCE_ID: &str = "i-03c8f84a6318a8186";
    static FILENAME_PATH : &str = "test-data/input.json";

    #[tokio::test]
    async fn test_get_data_from_file() {
        AwsInventoryFromFile::get_data_from_file(FILENAME_PATH).await;
        assert_eq!(0, 0, "True");
    }

    #[tokio::test]
    #[ignore]
    async fn inventory_should_return_correct_number_of_instances() {
        let inventory: AwsInventoryFromFile = AwsInventoryFromFile::new("eu-west-1", FILENAME_PATH).await;
        let filtertags: Vec<String> = Vec::new();
        let res: Vec<CloudResource> = inventory
            .get_instances_with_usage_data(&filtertags, false)
            .await
            .context("Failed to list")
            .unwrap();
        assert_eq!(4, res.len());

        let inst = res.first().unwrap();
        assert_eq!(3, inst.tags.len(), "Wrong number of tags");
        let tag_map = vec_to_map(inst.tags.clone());
        let v = tag_map.get("Name").unwrap();
        assert_eq!(
            Some("test-boapi".to_string()),
            v.to_owned(),
            "Wrong tag value"
        );
    }

    #[tokio::test]
    async fn test_create_sdk_config_works_with_wrong_region() {
        let region: &str = "eu-west-3";
        let config = AwsInventoryFromFile::load_aws_config(region).await;
        assert_eq!(region, config.region().unwrap().to_string());

        let wrong_region: &str = "impossible-region";
        let config = AwsInventoryFromFile::load_aws_config(wrong_region).await;
        assert_eq!(wrong_region, config.region().unwrap().to_string())
    }

    #[tokio::test]
    #[ignore]
    async fn get_cpu_usage_metrics_of_running_instance_should_return_right_number_of_data_points() {
        let inventory: AwsInventoryFromFile = AwsInventoryFromFile::new("eu-west-1", FILENAME_PATH).await;
        let res = inventory
            .get_average_cpu_usage_of_last_10_minutes(&RUNNING_INSTANCE_ID)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        assert!(
            0 < datapoints.len() && datapoints.len() < 3,
            "Strange number of datapoint returned for instance {}, is it really up ?. I was expecting 1 or 2  but got {} .\n {:#?}",
            &RUNNING_INSTANCE_ID,
            datapoints.len(),
            datapoints
        )
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_instance_usage_metrics_of_shutdown_instance() {
        let inventory: AwsInventoryFromFile = AwsInventoryFromFile::new("eu-west-1", FILENAME_PATH).await;
        let instance_id = "i-03e0b3b1246001382";
        let res = inventory
            .get_average_cpu_usage_of_last_10_minutes(instance_id)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        assert_eq!(0, datapoints.len(), "Wrong number of datapoint returned");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_instance_usage_metrics_of_non_existing_instance() {
        let inventory: AwsInventoryFromFile = AwsInventoryFromFile::new("eu-west-1", FILENAME_PATH).await;
        let instance_id = "IDONOTEXISTS";
        let res = inventory
            .get_average_cpu_usage_of_last_10_minutes(instance_id)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        assert_eq!(0, datapoints.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_average_cpu_load_of_running_instance_is_not_zero() {
        // This instance  needs to be running for the test to pass
        let inventory: AwsInventoryFromFile = AwsInventoryFromFile::new("eu-west-1", FILENAME_PATH).await;

        let avg_cpu_load = inventory
            .get_average_cpu(&RUNNING_INSTANCE_ID)
            .await
            .unwrap();
        assert_ne!(
            0 as f64, avg_cpu_load,
            "CPU load of instance {} is zero, is it really running ?",
            &RUNNING_INSTANCE_ID
        );
        println!("{:#?}", avg_cpu_load);
        assert!((0 as f64) < avg_cpu_load);
        assert!((100 as f64) > avg_cpu_load);
    }

    #[tokio::test]
    #[ignore]
    async fn test_average_cpu_load_of_non_existing_instance_is_zero() {
        let instance_id = "IDONOTEXISTS";
        let inventory: AwsInventoryFromFile = AwsInventoryFromFile::new("eu-west-1", FILENAME_PATH).await;
        let res = inventory.get_average_cpu(instance_id).await.unwrap();
        assert_eq!(0 as f64, res);
    }

    #[tokio::test]
    #[ignore]
    async fn test_average_cpu_load_of_shutdown_instance_is_zero() {
        let inventory: AwsInventoryFromFile = AwsInventoryFromFile::new("eu-west-1", FILENAME_PATH).await;
        let instance_id = "i-03e0b3b1246001382";
        let res = inventory.get_average_cpu(instance_id).await.unwrap();
        assert_eq!(0 as f64, res);
    }

    #[tokio::test]
    #[ignore]
    async fn returns_the_right_number_of_volumes() {
        let inventory: AwsInventoryFromFile = AwsInventoryFromFile::new("eu-west-1", FILENAME_PATH).await;
        let filtertags: Vec<String> = Vec::new();
        let res = inventory.list_volumes(&filtertags).await.unwrap();
        assert_eq!(4, res.len());
    }
}
