use rktk_rrp::{__reexports::futures::StreamExt as _, endpoint_client};
use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt as _},
    sync::Mutex,
};
use tokio_serial::{SerialPortBuilderExt as _, SerialStream};

#[tokio::main]
async fn main() {
    let stream = tokio_serial::new("COM8", 115200)
        .open_native_async()
        .unwrap();
    let client = Client {
        stream: Mutex::new(stream),
    };
    let start = std::time::Instant::now();

    let info = client.get_keyboard_info(()).await.unwrap();
    dbg!(info);
}

struct Client {
    stream: Mutex<SerialStream>,
}

impl Client {
    async fn send_all(&self, buf: &mut [u8]) -> Result<(), anyhow::Error> {
        self.stream.lock().await.write_all(buf).await?;
        Ok(())
    }
    async fn read_until_zero(&self, buf: &mut [u8]) -> Result<usize, anyhow::Error> {
        let mut stream = self.stream.lock().await;
        let mut read = 0;
        loop {
            let mut reader = [0u8];
            stream.read_exact(&mut reader).await.unwrap();
            buf[read] = reader[0];
            read += 1;
            if reader[0] == 0 {
                break;
            }
        }
        Ok(read)
    }

    endpoint_client!(
       get_keyboard_info normal normal
       get_keymaps normal stream
    );
}
