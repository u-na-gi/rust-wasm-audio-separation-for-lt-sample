import numpy as np
from scipy.io.wavfile import write
from pydub import AudioSegment
import os

# スクリプトのディレクトリを取得
script_dir = os.path.dirname(os.path.abspath(__file__))

# スクリプトのディレクトリをカレントディレクトリに変更
os.chdir(script_dir)

# パラメータ設定
sample_rate = 44100  # サンプリングレート（Hz）
duration = 1  # 音声の長さ（秒）

# 複雑な波形を生成
frequency1 = 120.0  # 周波数1
frequency2 = 320.0  # 周波数2
frequency3 = 510.0  # 周波数3

# 時間軸を生成
t = np.linspace(0, duration, int(sample_rate * duration), endpoint=False)


# 各チャンネルの波形を生成
wave1 = 0.5 * np.sin(2 * np.pi * frequency1 * t) + 0.3 * np.sin(2 * np.pi * frequency2 * t + np.pi / 4)
wave2 = 0.5 * np.sin(2 * np.pi * frequency2 * t + np.pi / 2) + 0.3 * np.sin(2 * np.pi * frequency3 * t + np.pi / 3)
wave3 = 0.5 * np.sin(2 * np.pi * frequency3 * t + np.pi / 6) + 0.3 * np.sin(2 * np.pi * frequency1 * t + np.pi / 2)

# 3チャンネルを交互に配置
stereo_signal = np.empty((wave1.size + wave2.size + wave3.size,), dtype=wave1.dtype)
stereo_signal[0::3] = wave1
stereo_signal[1::3] = wave2
stereo_signal[2::3] = wave3

# 16ビット整数形式に変換
stereo_signal_int16 = np.int16(stereo_signal * 32767)

# 一時ファイルとして書き出し
temp_output_path = os.path.join(script_dir, "interleaved_stereo_temp.wav")
write(temp_output_path, sample_rate, stereo_signal_int16)

# pydubで音声を読み込み、速度を変更
audio = AudioSegment.from_wav(temp_output_path)
slowed_audio = audio._spawn(audio.raw_data, overrides={
    "frame_rate": int(audio.frame_rate * 1)
}).set_frame_rate(audio.frame_rate)

# 出力パスの設定
output_wav = os.path.join("..", 'static', 'audio', "interleaved_stereo.wav")

# 遅くした音声を保存
slowed_audio.export(output_wav, format="wav")

# 一時ファイルの削除
os.remove(temp_output_path)

print(f"WAVファイルが生成されました: {output_wav}")
