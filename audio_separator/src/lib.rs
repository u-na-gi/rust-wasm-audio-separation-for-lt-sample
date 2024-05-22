mod utils;

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
pub fn separate_audio(data: &[u8]) -> js_sys::Array{
    set_panic_hook();
    
    let cursor = Cursor::new(data);
    log("separate_audio");
    let mut reader: WavReader<Cursor<&[u8]>> = WavReader::new(cursor).expect("Failed to read WAV file");

    log("reader created");

    let spec = reader.spec();
    log(&format!("Hello {:?}", spec));
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    let mut samples1: Vec<i16> = Vec::new();
    let mut samples2: Vec<i16> = Vec::new();
    let mut samples3: Vec<i16> = Vec::new();

    for (i, sample) in samples.iter().enumerate() {
        match i % 3 {
            0 => samples1.push(*sample),
            1 => samples2.push(*sample),
            2 => samples3.push(*sample),
            _ => panic!("unexpected"),
        }
    }

    log("separate_audio end");

    let output1 = write_wav_to_vec(&spec, &samples1);
    let output2 = write_wav_to_vec(&spec, &samples2);
    let output3 = write_wav_to_vec(&spec, &samples3);

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
    let mut writer = WavWriter::new(Cursor::new(&mut buffer), *spec).expect("Failed to create WAV writer");

    for sample in samples {
        writer.write_sample(*sample).expect("Failed to write sample");
    }
    writer.finalize().expect("Failed to finalize WAV writer");
    buffer
}


#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use std::io::Cursor;

    use hound::{WavSpec, WavWriter, SampleFormat};
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
            let mut writer = WavWriter::new(Cursor::new(&mut buffer), spec).expect("Failed to create WAV writer");
            for sample in &samples {
                writer.write_sample(*sample).expect("Failed to write sample");
            }
            writer.finalize().expect("Failed to finalize WAV writer");
        }

        let result = separate_audio(&buffer);
        
        // 結果の検証
        assert_eq!(result.length(), 2);

        let output1 = Uint8Array::new(&result.get(0));
        let output2 = Uint8Array::new(&result.get(1));

        assert!(output1.length() > 0);
        assert!(output2.length() > 0);
    }
}