use embedded_hal_async::i2c::I2c;

/// XW09DのI2Cスレーブアドレス (7-bit)
const I2C_ADDRESS: u8 = 0x40;

/// 9チャンネルのタッチ状態を保持する構造体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TouchState {
    /// pads[0] ~ pads[8] がそれぞれ PAD0 ~ PAD8 に対応。true ならタッチあり。
    pub pads: [bool; 9],
}

/// XW09D タッチセンサドライバ
pub struct Xw09d<I2C> {
    i2c: I2C,
}

impl<I2C, E> Xw09d<I2C>
where
    I2C: I2c<Error = E>,
{
    /// ドライバの新しいインスタンスを作成します
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// センサから現在のタッチ状態を読み取ります
    pub async fn read_touch(&mut self) -> Result<TouchState, E> {
        // レジスタ0x08から2バイト連続で読み取ります
        let mut buf = [0u8; 2];

        // I2Cの write_read を使用して、読み出し開始アドレス(0x08)を送信後にデータを受信
        self.i2c.write_read(I2C_ADDRESS, &[0x08], &mut buf).await?;

        let reg08 = buf[0];
        let reg09 = buf[1];

        let mut pads = [false; 9];

        // レジスタ0x08 の D4〜D0 を PAD0〜PAD4 にマッピング
        pads[0] = (reg08 & 0b0001_0000) != 0; // D4 = PAD0
        pads[1] = (reg08 & 0b0000_1000) != 0; // D3 = PAD1
        pads[2] = (reg08 & 0b0000_0100) != 0; // D2 = PAD2
        pads[3] = (reg08 & 0b0000_0010) != 0; // D1 = PAD3
        pads[4] = (reg08 & 0b0000_0001) != 0; // D0 = PAD4

        // レジスタ0x09 の D7〜D4 を PAD5〜PAD8 にマッピング
        pads[5] = (reg09 & 0b1000_0000) != 0; // D7 = PAD5
        pads[6] = (reg09 & 0b0100_0000) != 0; // D6 = PAD6
        pads[7] = (reg09 & 0b0010_0000) != 0; // D5 = PAD7
        pads[8] = (reg09 & 0b0001_0000) != 0; // D4 = PAD8

        Ok(TouchState { pads })
    }
}
