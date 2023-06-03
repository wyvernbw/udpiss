use inquire::{Select, Text};
use std::{error::Error, time::Duration};
use strum::IntoEnumIterator;
use udp_test::Message;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "0.0.0.0:0")]
    client_addr: String,
    #[clap(short, long)]
    server_addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_tracing()?;
    let args = Args::parse();
    let socket = tokio::net::UdpSocket::bind(&args.client_addr).await?;
    tracing::info!("Client binded to: {}", socket.local_addr()?);
    tracing::info!("Sending to: {}", args.server_addr);
    server_loop(socket, args).await?;
    Ok(())
}

async fn server_loop(socket: tokio::net::UdpSocket, args: Args) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0u8; 1024 * 8];
    loop {
        let opt = Select::new("Send: ", udp_test::PacketBody::iter().collect()).prompt()?;
        match opt {
            udp_test::PacketBody::Message(_) => {
                let msg = Text::new("Message: ").prompt()?;
                tracing::info!("Sending message: {}", &msg);
                let msg = udp_test::PacketBody::Message(Message(msg)).new_packet();
                let msg = rmp_serde::to_vec(&msg)?;
                socket.send_to(&msg, &args.server_addr).await?;
            }
            udp_test::PacketBody::Ping => {
                tracing::info!("Sending ping");
                let ping = udp_test::PacketBody::Ping.new_packet();
                socket
                    .send_to(&rmp_serde::to_vec(&ping)?, &args.server_addr)
                    .await?;
                let req = socket.recv_from(&mut buffer);
                let time = tokio::time::Instant::now();
                let (len, addr) = match tokio::time::timeout(Duration::from_millis(200), req).await
                {
                    Ok(value) => value?,
                    Err(_) => {
                        tracing::error!("Pong timed out");
                        continue;
                    }
                };
                let packet = rmp_serde::from_slice::<udp_test::Packet>(&buffer[..len])?;
                if packet.key != udp_test::KEY {
                    tracing::error!("Key mismatch: {:#?}, nice try >:)", &packet.key);
                    continue;
                }
                let packet_body = packet.body;
                tracing::info!(
                    "{addr}: received {}, {}ms",
                    &packet_body,
                    time.elapsed().as_millis()
                );
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
