mod utils;
use rustfft::{num_complex::Complex, num_traits::Zero, FftPlanner};

use std::io::Cursor;

use hound::{WavReader, WavSpec, WavWriter};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn log(s: &str) {
    log_1(&JsValue::from_str(s));
}

#[wasm_bindgen]
pub fn separate_audio(data: &[u8]) -> js_sys::Array {
    set_panic_hook();

    let cursor = Cursor::new(data);
    log("separate_audio");
    let mut reader: WavReader<Cursor<&[u8]>> =
        WavReader::new(cursor).expect("Failed to read WAV file");

    log("reader created");

    let spec = reader.spec();
    log(&format!("Hello {:?}", spec));
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    // i16サンプルをf32に変換
    let float_samples: Vec<Complex<f32>> = samples
        .iter()
        .map(|&s| Complex::new(s as f32, 0.0))
        .collect();
    log(format!("float_samples {:?}", float_samples).as_str());

    // 音源分離を行う
    // FFTの準備

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(float_samples.len());
    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); float_samples.len()];

    // FFTの実行
    fft.process_with_scratch(&mut float_samples.clone(), &mut output);

    // バイナリマスキングの適用（ここで実際のマスキングを適用する）
    let mask1: Vec<bool> = output.iter().enumerate().map(|(i, _)| i % 3 == 0).collect();
    let mask2: Vec<bool> = output.iter().enumerate().map(|(i, _)| i % 3 == 1).collect();
    let mask3: Vec<bool> = output.iter().enumerate().map(|(i, _)| i % 3 == 2).collect();

    let mut output1 = output.clone();
    let mut output2 = output.clone();
    let mut output3 = output.clone();

    for i in 0..output.len() {
        if !mask1[i] {
            output1[i] = Complex::zero();
        }
        if !mask2[i] {
            output2[i] = Complex::zero();
        }
        if !mask3[i] {
            output3[i] = Complex::zero();
        }
    }

    // 逆FFTの準備
    let fft_inv = planner.plan_fft_inverse(output.len());
    let mut output_inv1: Vec<Complex<f32>> = vec![Complex::zero(); output.len()];
    let mut output_inv2: Vec<Complex<f32>> = vec![Complex::zero(); output.len()];
    let mut output_inv3: Vec<Complex<f32>> = vec![Complex::zero(); output.len()];

    // 逆FFTの実行
    fft_inv.process_with_scratch(&mut output1, &mut output_inv1);
    fft_inv.process_with_scratch(&mut output2, &mut output_inv2);
    fft_inv.process_with_scratch(&mut output3, &mut output_inv3);

    // スケーリングの適用
    let len = output.len() as f32;
    for val in &mut output_inv1 {
        *val /= len;
    }
    log(format!("output_inv1 {:?}", output_inv1).as_str());
    for val in &mut output_inv2 {
        *val /= len;
    }
    log(format!("output_inv2 {:?}", output_inv2).as_str());
    for val in &mut output_inv3 {
        *val /= len;
    }
    log(format!("output_inv3 {:?}", output_inv3).as_str());

    // 時間領域に変換
    let separated_samples1: Vec<i16> = output_inv1.iter().map(|c| c.re as i16).collect();
    let separated_samples2: Vec<i16> = output_inv2.iter().map(|c| c.re as i16).collect();
    let separated_samples3: Vec<i16> = output_inv3.iter().map(|c| c.re as i16).collect();

    // 分離された音声の書き出し
    let spec = WavSpec {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: spec.bits_per_sample,
        sample_format: spec.sample_format,
    };

    log("separate_audio end");

    let output1 = write_wav_to_vec(&spec, &separated_samples1);
    let output2 = write_wav_to_vec(&spec, &separated_samples2);
    let output3 = write_wav_to_vec(&spec, &separated_samples3);

    log("write_wav_to_vec end");

    let output1 = js_sys::Uint8Array::from(&output1[..]);
    let output2 = js_sys::Uint8Array::from(&output2[..]);
    let output3 = js_sys::Uint8Array::from(&output3[..]);

    let result = js_sys::Array::new();
    result.push(&output1);
    result.push(&output2);
    result.push(&output3);

    log("separate_audio end");

    result
}

fn write_wav_to_vec(spec: &WavSpec, samples: &[i16]) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut writer =
        WavWriter::new(Cursor::new(&mut buffer), *spec).expect("Failed to create WAV writer");

    for sample in samples {
        writer
            .write_sample(*sample)
            .expect("Failed to write sample");
    }
    writer.finalize().expect("Failed to finalize WAV writer");
    buffer
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use wasm_bindgen_test::*;

    use hound::{SampleFormat, WavSpec, WavWriter};
    use js_sys::Uint8Array;

    use crate::separate_audio;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_separate_audio() {
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let samples: Vec<i16> = (0..100).collect();
        println!("samples {:?}", samples);

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut writer = WavWriter::new(Cursor::new(&mut buffer), spec)
                .expect("Failed to create WAV writer");
            for sample in &samples {
                writer
                    .write_sample(*sample)
                    .expect("Failed to write sample");
            }
            writer.finalize().expect("Failed to finalize WAV writer");
        }

        let result = separate_audio(&buffer);

        // 結果の検証
        assert_eq!(result.length(), 3);

        let output1 = Uint8Array::new(&result.get(0));
        let output2 = Uint8Array::new(&result.get(1));
        let output3 = Uint8Array::new(&result.get(2));

        assert!(output1.length() > 0);
        assert!(output2.length() > 0);
        assert!(output3.length() > 0);
    }
}
