use rktk_rrp::endpoint_client;
use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt as _},
    sync::Mutex,
};
use tokio_serial::{SerialPortBuilderExt as _, SerialStream};

pub struct Client {
    stream: Mutex<SerialStream>,
}

impl Client {
    pub fn new(name: &str, baud: u32) -> anyhow::Result<Self> {
        let stream = tokio_serial::new(name, baud).open_native_async()?;
        Ok(Client {
            stream: Mutex::new(stream),
        })
    }
}

impl Client {
    async fn send_all(&self, buf: &[u8]) -> Result<(), anyhow::Error> {
        self.stream.lock().await.write_all(buf).await?;
        Ok(())
    }
    async fn read_until_zero(&self, buf: &mut Vec<u8>) -> Result<usize, anyhow::Error> {
        let mut stream = self.stream.lock().await;
        let mut read = 0;
        loop {
            let mut reader = [0u8];
            stream.read_exact(&mut reader).await.unwrap();
            buf.push(reader[0]);
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
       get_layout_json normal stream
    );
}
