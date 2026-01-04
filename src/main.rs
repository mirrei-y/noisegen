use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

struct Rng {
    state: u32,
}
impl Rng {
    fn new(seed: u32) -> Self {
        Rng { state: seed }
    }

    fn next(&mut self) -> f32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        (x as f32) / (u32::MAX as f32)
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("エラー: 出力デバイスが見つかりません");
    let config = device.default_output_config().expect("エラー: デバイスの設定を取得できません");

    let channels = config.channels() as usize;
    println!(
        "{} ({}ch / {}Hz) で再生します。\nCtrl-C で停止します。",
        device.name().map_err(|e| e.to_string()).unwrap_or("Unknown Device".to_string()),
        channels,
        config.sample_rate().0
    );

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut rng = Rng::new(20250329);
            for frame in data.chunks_mut(channels) {
                let value: f32 = (rng.next() * 0.0002) - 0.0001;
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        move |err| {
            eprintln!("an error occurred on stream: {}", err);
        },
        None,
    ).unwrap();

    stream.play().unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
