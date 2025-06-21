use bluest::Adapter;
use futures::stream::StreamExt;


async fn connect() {
    let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found").unwrap();
    let _ = adapter.wait_available().await;

    println!("starting scan");
    let mut scan = adapter.scan(&[]).await.unwrap();
    println!("scan started");
    while let Some(discovered_device) = scan.next().await {
       println!(
           "{}{}: {:?}",
           discovered_device.device.name().as_deref().unwrap_or("(unknown)"),
           discovered_device
               .rssi
               .map(|x| format!(" ({}dBm)", x))
               .unwrap_or_default(),
           discovered_device.adv_data.services
       );
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    connect().await;
}
