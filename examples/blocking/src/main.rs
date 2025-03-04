use embedded_io::adapters::FromStd;
use embedded_tls::blocking::*;
use embedded_tls::webpki::CertVerifier;
use rand::rngs::OsRng;
use std::net::TcpStream;
use std::time::SystemTime;

fn main() {
    env_logger::init();
    let stream = TcpStream::connect("127.0.0.1:12345").expect("error connecting to server");

    log::info!("Connected");
    let mut read_record_buffer = [0; 16384];
    let mut write_record_buffer = [0; 16384];
    let config = TlsConfig::new().with_server_name("localhost");
    let mut tls: TlsConnection<FromStd<TcpStream>, Aes128GcmSha256> =
        TlsConnection::new(FromStd::new(stream), &mut read_record_buffer, &mut write_record_buffer);
    let mut rng = OsRng;

    tls.open::<OsRng, CertVerifier<Aes128GcmSha256, SystemTime, 4096>>(TlsContext::new(
        &config, &mut rng,
    ))
    .expect("error establishing TLS connection");

    tls.write(b"ping").expect("error writing data");

    let mut rx_buf = [0; 4096];
    let sz = tls.read(&mut rx_buf).expect("error reading data");
    log::info!("Read {} bytes: {:?}", sz, &rx_buf[..sz]);
}
