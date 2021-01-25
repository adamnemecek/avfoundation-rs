# avfoundation

AVFoundation bindings for Rust.

# How to playback MIDI
```rust
let sequencer = AVAudioSequencer::with_engine(&engine);
sequencer.load_from_url(path, );
```