use etherparse::PacketBuilder;
use std::thread::sleep;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tunio::traits::{DriverT, InterfaceT};
use tunio::DefaultDriver;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut driver = DefaultDriver::new().unwrap();

    let mut interface_config = DefaultDriver::if_config_builder();
    interface_config.name("name".into());
    #[cfg(target_os = "windows")]
    interface_config
        .platform(|mut b| b.description("description".into()).build())
        .unwrap();
    let interface_config = interface_config.build().unwrap();

    let mut interface = driver.new_interface_up(interface_config).unwrap();
    let iff = interface.handle();

    iff.add_ip("18.3.5.6/24".parse().unwrap());
    iff.add_ip("20.3.5.6/24".parse().unwrap());
    iff.remove_ip("18.3.5.6/24".parse().unwrap());
    iff.add_ip("fd3c:dea:7f96:2b14::/64".parse().unwrap());

    for _ in 1..10 {
        let builder = PacketBuilder::ipv6(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            5,
        )
        .udp(8080, 8080);

        let mut packet = Vec::with_capacity(builder.size(0));
        builder.write(&mut packet, &[]).unwrap();

        interface.write(&*packet).await;

        sleep(Duration::from_secs(1));
    }

    let mut buf = vec![0u8; 4096];
    while let Ok(n) = interface.read(buf.as_mut_slice()).await {
        buf.truncate(n);
        println!("{buf:x?}");
        buf.resize(4096, 0u8);
    }

    tokio::signal::ctrl_c().await;
}
