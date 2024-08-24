use std::io::{copy, Error};
use std::net::SocketAddr;
use clap::Parser;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};




#[derive(Parser, Debug)]
struct Args{
  #[arg(short, long)]
  bind: String,
  #[arg(short, long)]
  server: String,
}



async fn handle_client(mut stream: TcpStream, address: SocketAddr, url: String) -> Result<(), Error> {
  let mut outbound = TcpStream::connect(url).await?;

  println!("Handle: <{}:{}>", address.ip().to_string(), address.port().to_string());

  let res = copy_bidirectional(&mut stream, &mut outbound).await;

  match res {
      Ok(_)  => {},
      Err(e) => eprintln!("Error: {:?}", e)
  }

  Ok(())
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

  let args = Args::parse();

  let server = args.server;

  let listener = TcpListener::bind(args.bind.clone()).await?;

  println!("Listen: http://{}", args.bind.clone());

  loop {
      let ( stream, address) = listener.accept().await?;

      let link = server.clone();
      tokio::spawn(async move {
        if let Err(e) = handle_client(stream, address, link.clone()).await{
          eprintln!("Error: {e}");
        }
      });

  }


}