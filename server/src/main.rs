use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    server_addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_tracing()?;
    tracing::info!("Starting up...");
    let args = Args::parse();
    tracing::info!("Running server with args: {:#?}", &args);
    let socket = tokio::net::UdpSocket::bind(args.server_addr).await?;
    tracing::info!("Listening on: {}", socket.local_addr()?);
    let mut buffer = vec![0u8; 1024 * 8];
    loop {
        let (len, addr) = match socket.recv_from(&mut buffer).await {
            Ok(value) => value,
            Err(err) => {
                tracing::error!("Failed to receive from socket: {}", err);
                continue;
            }
        };
        let packet = rmp_serde::from_slice::<udp_test::Packet>(&buffer[..len]);
        let packet = match packet {
            Ok(packet) => packet,
            Err(err) => {
                tracing::error!("Failed to deserialize packet: {}", err);
                continue;
            }
        };
        if packet.key != udp_test::KEY {
            tracing::error!("Key mismatch: {:#?}, nice try >:)", &packet.key);
            continue;
        }
        let packet_body = packet.body;
        tracing::info!("Received {:#?} from: {addr}", &packet_body);
        match packet_body {
            udp_test::PacketBody::Message(_) => {}
            udp_test::PacketBody::Ping => {
                tracing::info!("Sending pong");
                let pong = udp_test::PacketBody::Ping.new_packet();
                socket.send_to(&rmp_serde::to_vec(&pong)?, &addr).await?;
            }
        }
    }
}

fn init_tracing() -> Result<(), Box<dyn Error>> {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
