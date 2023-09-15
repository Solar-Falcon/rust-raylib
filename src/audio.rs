use std::{ffi::CString, time::Duration};

use crate::ffi;

#[derive(Debug)]
pub struct AudioDevice(());

impl AudioDevice {
    /// Initialize audio device and context
    #[inline]
    pub fn init() -> Option<Self> {
        unsafe {
            ffi::InitAudioDevice();
        }

        if unsafe { ffi::IsAudioDeviceReady() } {
            Some(Self(()))
        } else {
            None
        }
    }

    /// Set master volume (listener)
    #[inline]
    pub fn set_master_volume(&mut self, volume: f32) {
        unsafe { ffi::SetMasterVolume(volume) }
    }
}

impl Drop for AudioDevice {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::CloseAudioDevice() }
    }
}

/// Wave, audio wave data
#[derive(Debug)]
pub struct Wave {
    raw: ffi::Wave,
}

impl Wave {
    /// Total number of frames (considering channels)
    #[inline]
    pub fn frame_count(&self) -> u32 {
        self.raw.frameCount
    }

    /// Frequency (samples per second)
    #[inline]
    pub fn sample_rate(&self) -> u32 {
        self.raw.sampleRate
    }

    /// Bit depth (bits per sample): 8, 16, 32 (24 not supported)
    #[inline]
    pub fn sample_size(&self) -> u32 {
        self.raw.sampleSize
    }

    /// Number of channels (1-mono, 2-stereo, ...)
    #[inline]
    pub fn channels(&self) -> u32 {
        self.raw.channels
    }

    /// Load wave data from file
    #[inline]
    pub fn from_file(file_name: &str) -> Option<Self> {
        let file_name = CString::new(file_name).unwrap();

        let raw = unsafe { ffi::LoadWave(file_name.as_ptr()) };

        if unsafe { ffi::IsWaveReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
    #[inline]
    pub fn from_memory(file_type: &str, file_data: &[u8]) -> Option<Self> {
        let file_type = CString::new(file_type).unwrap();

        let raw = unsafe {
            ffi::LoadWaveFromMemory(file_type.as_ptr(), file_data.as_ptr(), file_data.len() as _)
        };

        if unsafe { ffi::IsWaveReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Export wave data to file, returns true on success
    #[inline]
    pub fn export(&self, file_name: &str) -> bool {
        let file_name = CString::new(file_name).unwrap();

        unsafe { ffi::ExportWave(self.raw.clone(), file_name.as_ptr()) }
    }

    /// Export wave sample data to code (.h), returns true on success
    #[inline]
    pub fn export_as_code(&self, file_name: &str) -> bool {
        let file_name = CString::new(file_name).unwrap();

        unsafe { ffi::ExportWaveAsCode(self.raw.clone(), file_name.as_ptr()) }
    }

    /// Crop a wave to defined samples range
    #[inline]
    pub fn crop(&mut self, init_sample: u32, final_sample: u32) {
        unsafe { ffi::WaveCrop(&mut self.raw as *mut _, init_sample as _, final_sample as _) }
    }

    /// Convert wave data to desired format
    #[inline]
    pub fn convert_to_format(&mut self, sample_rate: u32, sample_size: u32, channels: u32) {
        unsafe {
            ffi::WaveFormat(
                &mut self.raw as *mut _,
                sample_rate as _,
                sample_size as _,
                channels as _,
            )
        }
    }

    /// Load samples data from wave as a 32bit float data array
    #[inline]
    pub fn load_samples(&self) -> Vec<f32> {
        let samples = unsafe { ffi::LoadWaveSamples(self.raw.clone()) };

        let mut vec = Vec::new();
        let len = (self.frame_count() * self.channels()) as usize;

        for i in 0..len {
            vec.push(unsafe { samples.add(i).read() });
        }

        unsafe {
            ffi::UnloadWaveSamples(samples);
        }

        vec
    }

    #[inline]
    pub fn as_raw(&self) -> &ffi::Wave {
        &self.raw
    }

    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Wave {
        &mut self.raw
    }
}

impl Clone for Wave {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { ffi::WaveCopy(self.raw.clone()) },
        }
    }
}

impl Drop for Wave {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadWave(self.raw.clone()) }
    }
}

/// AudioStream, custom audio stream
#[derive(Debug)]
pub struct AudioStream {
    raw: ffi::AudioStream,
}

impl AudioStream {
    /// Frequency (samples per second)
    #[inline]
    pub fn sample_rate(&self) -> u32 {
        self.raw.sampleRate
    }

    /// Bit depth (bits per sample): 8, 16, 32 (24 not supported)
    #[inline]
    pub fn sample_size(&self) -> u32 {
        self.raw.sampleSize
    }

    /// Number of channels (1-mono, 2-stereo, ...)
    #[inline]
    pub fn channels(&self) -> u32 {
        self.raw.channels
    }

    /// Load audio stream (to stream raw audio pcm data)
    #[inline]
    pub fn new(sample_rate: u32, sample_size: u32, channels: u32) -> Option<Self> {
        let raw = unsafe { ffi::LoadAudioStream(sample_rate, sample_size, channels) };

        if unsafe { ffi::IsAudioStreamReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Update audio stream buffers with data
    #[inline]
    pub fn update(&mut self, data: &[u8], frame_count: u32) {
        unsafe {
            ffi::UpdateAudioStream(
                self.raw.clone(),
                data.as_ptr() as *const _,
                frame_count as _,
            )
        }
    }

    /// Check if any audio stream buffers requires refill
    #[inline]
    pub fn is_processed(&self) -> bool {
        unsafe { ffi::IsAudioStreamProcessed(self.raw.clone()) }
    }

    /// Play audio stream
    #[inline]
    pub fn play(&self, _device: &mut AudioDevice) {
        unsafe { ffi::PlayAudioStream(self.raw.clone()) }
    }

    /// Pause audio stream
    #[inline]
    pub fn pause(&self, _device: &mut AudioDevice) {
        unsafe { ffi::PauseAudioStream(self.raw.clone()) }
    }

    /// Resume audio stream
    #[inline]
    pub fn resume(&self, _device: &mut AudioDevice) {
        unsafe { ffi::ResumeAudioStream(self.raw.clone()) }
    }

    /// Check if audio stream is playing
    #[inline]
    pub fn is_playing(&self, _device: &mut AudioDevice) -> bool {
        unsafe { ffi::IsAudioStreamPlaying(self.raw.clone()) }
    }

    /// Stop audio stream
    #[inline]
    pub fn stop(&self, _device: &mut AudioDevice) {
        unsafe { ffi::StopAudioStream(self.raw.clone()) }
    }

    /// Set volume for audio stream (1.0 is max level)
    #[inline]
    pub fn set_volume(&self, volume: f32, _device: &mut AudioDevice) {
        unsafe { ffi::SetAudioStreamVolume(self.raw.clone(), volume) }
    }

    /// Set pitch for audio stream (1.0 is base level)
    #[inline]
    pub fn set_pitch(&self, pitch: f32, _device: &mut AudioDevice) {
        unsafe { ffi::SetAudioStreamPitch(self.raw.clone(), pitch) }
    }

    /// Set pan for audio stream (0.5 is centered)
    #[inline]
    pub fn set_pan(&self, pan: f32, _device: &mut AudioDevice) {
        unsafe { ffi::SetAudioStreamPan(self.raw.clone(), pan) }
    }

    /// Default size for new audio streams
    #[inline]
    pub fn set_default_buffer_size(size: usize) {
        unsafe { ffi::SetAudioStreamBufferSizeDefault(size as _) }
    }

    #[inline]
    pub fn as_raw(&self) -> &ffi::AudioStream {
        &self.raw
    }

    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::AudioStream {
        &mut self.raw
    }
}

impl Drop for AudioStream {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadAudioStream(self.raw.clone()) }
    }
}

/// Sound
#[derive(Debug)]
pub struct Sound {
    raw: ffi::Sound,
}

impl Sound {
    /// Total number of frames (considering channels)
    #[inline]
    pub fn frame_count(&self) -> u32 {
        self.raw.frameCount
    }

    /// Load sound from file
    #[inline]
    pub fn from_file(filname: &str) -> Option<Self> {
        let file_name = CString::new(filname).unwrap();

        let raw = unsafe { ffi::LoadSound(file_name.as_ptr()) };

        if unsafe { ffi::IsSoundReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Load sound from wave data
    #[inline]
    pub fn from_wave(wave: &Wave) -> Option<Self> {
        let raw = unsafe { ffi::LoadSoundFromWave(wave.raw.clone()) };

        if unsafe { ffi::IsSoundReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Update sound buffer with new data
    #[inline]
    pub fn update(&mut self, data: &[u8], sample_count: u32) {
        unsafe {
            ffi::UpdateSound(
                self.raw.clone(),
                data.as_ptr() as *const _,
                sample_count as _,
            )
        }
    }

    /// Play a sound
    #[inline]
    pub fn play(&self, _device: &mut AudioDevice) {
        unsafe { ffi::PlaySound(self.raw.clone()) }
    }

    /// Stop playing a sound
    #[inline]
    pub fn stop(&self, _device: &mut AudioDevice) {
        unsafe { ffi::StopSound(self.raw.clone()) }
    }

    /// Pause a sound
    #[inline]
    pub fn pause(&self, _device: &mut AudioDevice) {
        unsafe { ffi::PauseSound(self.raw.clone()) }
    }

    /// Resume a paused sound
    #[inline]
    pub fn resume(&self, _device: &mut AudioDevice) {
        unsafe { ffi::ResumeSound(self.raw.clone()) }
    }

    /// Check if a sound is currently playing
    #[inline]
    pub fn is_playing(&self, _device: &mut AudioDevice) -> bool {
        unsafe { ffi::IsSoundPlaying(self.raw.clone()) }
    }

    /// Set volume for a sound (1.0 is max level)
    #[inline]
    pub fn set_volume(&self, volume: f32, _device: &mut AudioDevice) {
        unsafe { ffi::SetSoundVolume(self.raw.clone(), volume) }
    }

    /// Set pitch for a sound (1.0 is base level)
    #[inline]
    pub fn set_pitch(&self, pitch: f32, _device: &mut AudioDevice) {
        unsafe { ffi::SetSoundPitch(self.raw.clone(), pitch) }
    }

    /// Set pan for a sound (0.5 is center)
    #[inline]
    pub fn set_pan(&self, pan: f32, _device: &mut AudioDevice) {
        unsafe { ffi::SetSoundPan(self.raw.clone(), pan) }
    }

    #[inline]
    pub fn as_raw(&self) -> &ffi::Sound {
        &self.raw
    }

    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Sound {
        &mut self.raw
    }
}

impl Drop for Sound {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadSound(self.raw.clone()) }
    }
}

/// Music, audio stream, anything longer than ~10 seconds should be streamed
#[derive(Debug)]
pub struct Music {
    raw: ffi::Music,
}

impl Music {
    /// Total number of frames (considering channels)
    #[inline]
    pub fn frame_count(&self) -> u32 {
        self.raw.frameCount
    }

    /// Is music looping enabled
    #[inline]
    pub fn looping(&self) -> bool {
        self.raw.looping
    }

    /// Enable/disable looping
    #[inline]
    pub fn set_looping(&mut self, looping: bool) {
        self.raw.looping = looping;
    }

    /// Load music stream from file
    #[inline]
    pub fn from_file(file_name: &str) -> Option<Self> {
        let file_name = CString::new(file_name).unwrap();

        let raw = unsafe { ffi::LoadMusicStream(file_name.as_ptr()) };

        if unsafe { ffi::IsMusicReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }
    /// Load music stream from data
    #[inline]
    pub fn from_memory(file_type: &str, data: &[u8]) -> Option<Self> {
        let file_type = CString::new(file_type).unwrap();

        let raw = unsafe {
            ffi::LoadMusicStreamFromMemory(file_type.as_ptr(), data.as_ptr(), data.len() as _)
        };

        if unsafe { ffi::IsMusicReady(raw.clone()) } {
            Some(Self { raw })
        } else {
            None
        }
    }

    /// Start music playing
    #[inline]
    pub fn play(&self, _device: &mut AudioDevice) {
        unsafe { ffi::PlayMusicStream(self.raw.clone()) }
    }

    /// Check if music is playing
    #[inline]
    pub fn is_playing(&self, _device: &mut AudioDevice) -> bool {
        unsafe { ffi::IsMusicStreamPlaying(self.raw.clone()) }
    }

    /// Updates buffers for music streaming
    #[inline]
    pub fn update(&self, _device: &mut AudioDevice) {
        unsafe { ffi::UpdateMusicStream(self.raw.clone()) }
    }

    /// Stop music playing
    #[inline]
    pub fn stop(&self, _device: &mut AudioDevice) {
        unsafe { ffi::StopMusicStream(self.raw.clone()) }
    }

    /// Pause music playing
    #[inline]
    pub fn pause(&self, _device: &mut AudioDevice) {
        unsafe { ffi::PauseMusicStream(self.raw.clone()) }
    }

    /// Resume playing paused music
    #[inline]
    pub fn resume(&self, _device: &mut AudioDevice) {
        unsafe { ffi::ResumeMusicStream(self.raw.clone()) }
    }

    /// Seek music to a position
    #[inline]
    pub fn seek(&self, position: Duration, _device: &mut AudioDevice) {
        unsafe { ffi::SeekMusicStream(self.raw.clone(), position.as_secs_f32()) }
    }

    /// Set volume for music (1.0 is max level)
    #[inline]
    pub fn set_volume(&self, volume: f32, _device: &mut AudioDevice) {
        unsafe { ffi::SetMusicVolume(self.raw.clone(), volume) }
    }

    /// Set pitch for a music (1.0 is base level)
    #[inline]
    pub fn set_pitch(&self, pitch: f32, _device: &mut AudioDevice) {
        unsafe { ffi::SetMusicPitch(self.raw.clone(), pitch) }
    }

    /// Set pan for a music (0.5 is center)
    #[inline]
    pub fn set_pan(&self, pan: f32, _device: &mut AudioDevice) {
        unsafe { ffi::SetMusicPan(self.raw.clone(), pan) }
    }

    /// Get music time length
    #[inline]
    pub fn get_time_length(&self, _device: &mut AudioDevice) -> Duration {
        Duration::from_secs_f32(unsafe { ffi::GetMusicTimeLength(self.raw.clone()) })
    }

    /// Get current music time played
    #[inline]
    pub fn get_time_played(&self, _device: &mut AudioDevice) -> Duration {
        Duration::from_secs_f32(unsafe { ffi::GetMusicTimePlayed(self.raw.clone()) })
    }

    #[inline]
    pub fn as_raw(&self) -> &ffi::Music {
        &self.raw
    }

    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut ffi::Music {
        &mut self.raw
    }
}

impl Drop for Music {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::UnloadMusicStream(self.raw.clone()) }
    }
}

//pub type AudioCallback = Option<unsafe extern "C" fn(bufferData: *mut core::ffi::c_void, frames: u32, )>;

/*
    /// Audio thread callback to request new data
    #[inline]
    pub fn SetAudioStreamCallback(stream: AudioStream, callback: AudioCallback);
    /// Attach audio stream processor to stream
    #[inline]
    pub fn AttachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
    /// Detach audio stream processor from stream
    #[inline]
    pub fn DetachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
    /// Attach audio stream processor to the entire audio pipeline
    #[inline]
    pub fn AttachAudioMixedProcessor(processor: AudioCallback);
    /// Detach audio stream processor from the entire audio pipeline
    #[inline]
    pub fn DetachAudioMixedProcessor(processor: AudioCallback);
*/
