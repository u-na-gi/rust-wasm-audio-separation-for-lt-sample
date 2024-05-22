import init, { separate_audio } from './pkg/audio_separator.js';

async function run() {
    await init();

    const playButton0 = document.getElementById('play-button0');
    const stopButton0 = document.getElementById('stop-button0');
    const separateButton = document.getElementById('separate-button');
    const conversionTimeSpan = document.getElementById('conversion-time');
    const playButton1 = document.getElementById('play-button1');
    const stopButton1 = document.getElementById('stop-button1');
    const playButton2 = document.getElementById('play-button2');
    const stopButton2 = document.getElementById('stop-button2');
    const playButton3 = document.getElementById('play-button3');
    const stopButton3 = document.getElementById('stop-button3');
    const waveformTitle = document.getElementById('waveform-title');
    const waveformContainer = document.getElementById('waveform');

    let originalAudio = null;
    let audio1 = null;
    let audio2 = null;
    let audio3 = null;
    const waveSurfer = WaveSurfer.create({
        container: waveformContainer,
        waveColor: 'black',
        progressColor: 'purple',
        cursorColor: 'blue',
        height: 100,
    });

    const response = await fetch('./audio/interleaved_stereo.wav');
    const arrayBuffer = await response.arrayBuffer();
    const data = new Uint8Array(arrayBuffer);
    originalAudio = new Audio(URL.createObjectURL(new Blob([data], { type: 'audio/wav' })));
    waveSurfer.load(URL.createObjectURL(new Blob([data], { type: 'audio/wav' })));

    function playAudio(audio, title) {
        if (audio) {
            stopAllAudios();
            audio.play();
            waveSurfer.load(audio.src);
            waveformTitle.textContent = title;
        }
    }

    function stopAudio(audio) {
        if (audio) {
            audio.pause();
            audio.currentTime = 0;
        }
    }

    function stopAllAudios() {
        stopAudio(originalAudio);
        stopAudio(audio1);
        stopAudio(audio2);
        stopAudio(audio3);
        waveformTitle.textContent = 'No audio playing';
    }

    playButton0.addEventListener('click', () => {
        playAudio(originalAudio, 'Original Audio');
        stopButton0.disabled = false;
    });

    stopButton0.addEventListener('click', () => {
        stopAudio(originalAudio);
        stopButton0.disabled = true;
        waveformTitle.textContent = 'No audio playing';
    });

    separateButton.addEventListener('click', async () => {
        const spinner = document.createElement('div');
        spinner.className = 'spinner';
        conversionTimeSpan.innerHTML = '';
        conversionTimeSpan.appendChild(spinner);
        const startTime = performance.now();

        try {
            const [separated1, separated2, separated3] = separate_audio(data);
            const endTime = performance.now();

            const blob1 = new Blob([separated1], { type: 'audio/wav' });
            const blob2 = new Blob([separated2], { type: 'audio/wav' });
            const blob3 = new Blob([separated3], { type: 'audio/wav' });

            audio1 = new Audio(URL.createObjectURL(blob1));
            audio2 = new Audio(URL.createObjectURL(blob2));
            audio3 = new Audio(URL.createObjectURL(blob3));

            playButton1.disabled = false;
            stopButton1.disabled = false;
            playButton2.disabled = false;
            stopButton2.disabled = false;
            playButton3.disabled = false;
            stopButton3.disabled = false;

            const timeTaken = (endTime - startTime).toFixed(0);
            conversionTimeSpan.innerHTML = `Time taken: ${timeTaken} ms`;
        } catch (e) {
            console.error("Error separating audio:", e);
            conversionTimeSpan.innerHTML = 'Error during conversion';
        } finally {
            spinner.remove();
        }
    });

    playButton1.addEventListener('click', () => {
        playAudio(audio1, 'Separated Audio 1');
        stopButton1.disabled = false;
    });

    stopButton1.addEventListener('click', () => {
        stopAudio(audio1);
        stopButton1.disabled = true;
        waveformTitle.textContent = 'No audio playing';
    });

    playButton2.addEventListener('click', () => {
        playAudio(audio2, 'Separated Audio 2');
        stopButton2.disabled = false;
    });

    stopButton2.addEventListener('click', () => {
        stopAudio(audio2);
        stopButton2.disabled = true;
        waveformTitle.textContent = 'No audio playing';
    });

    playButton3.addEventListener('click', () => {
        playAudio(audio3, 'Separated Audio 3');
        stopButton3.disabled = false;
    });

    stopButton3.addEventListener('click', () => {
        stopAudio(audio3);
        stopButton3.disabled = true;
        waveformTitle.textContent = 'No audio playing';
    });
}

run();
