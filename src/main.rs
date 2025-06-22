use std::time::Duration;

use bluest::{Adapter, AdvertisingDevice, Device, Service, Uuid};
use futures::stream::StreamExt;

const ESP_SERVICE: &str = "63fe1426-5be8-4956-8e4e-4a3f829e6681";

async fn discover() {
    let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found").unwrap();
    let _ = adapter.wait_available().await;

    println!("starting scan");
    let mut scan = adapter.scan(&[]).await.unwrap();
    println!("scan started");
    while let Some(discovered_device) = scan.next().await {
        println!("Found a Device.");
        println!(
           "{}{}: {:?}",
           discovered_device.device.name().as_deref().unwrap_or("(unknown)"),
           discovered_device
               .rssi
               .map(|x| format!(" ({}dBm)", x))
               .unwrap_or_default(),
           discovered_device.adv_data.services
       );
        if discovered_device.device.name().as_deref().unwrap() != "BlauxBuds" {
            connect(&adapter, discovered_device).await;
        }
    }
}

async fn find_service(device: &Device, uuid: Uuid) -> Result<Service, &str>{
    let service = if let Some(service) = device.discover_services_with_uuid(uuid)
            .await.unwrap()
            .first() {
        service.clone()
    } else {
        return Err("service not found")
    };
    println!("{:?}", service);
    Ok(service)
}

async fn connect(adapter: &Adapter, discovered_device: AdvertisingDevice) {
    println!("{:?} {:?}", discovered_device.rssi, discovered_device.adv_data);
    adapter.connect_device(&discovered_device.device).await.unwrap();
    println!("connected!");
    let services = discovered_device.device.services().await.unwrap();
    for service in services {
        println!("Services:");
        println!("  {:?}", service);
        let characteristics = service.characteristics().await.unwrap();
        for characteristic in characteristics {
            println!("Characteristics:");
            println!("{:?}", characteristic);
            let descriptors = characteristic.descriptors().await.unwrap();
            for descriptor in descriptors {
                println!("{:?}: {:?}", descriptor, descriptor.read().await);
            }
        }
    }
    tokio::time::sleep(Duration::from_secs(30)).await;
    adapter.disconnect_device(&discovered_device.device).await.unwrap();
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    discover().await;
}
