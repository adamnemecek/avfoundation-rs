// func sampler_midi_scheduling() {
//     let engine = AVAudioEngine()
//     let sampler = AVAudioUnitSampler()

//     engine.attach(sampler)
//     let output = engine.outputNode
//     engine.connect(sampler, to: output, format: nil)
//     try! engine.start()
//     let path = "/Users/adamnemecek/Downloads/FatBoy-v0.790.sf2"
//     let url = URL(fileURLWithPath: path)
//     try! sampler.loadSoundBankInstrument(at: url, program: 100, bankMSB: 121, bankLSB: 0)

//     sampler.auAudioUnit.token(byAddingRenderObserver: {
//         (flags, timestamp, frameCount, outputBusNumber) in
//         let pitch: UInt8 = (0...127).randomElement()! as UInt8
//         sampler.startNote(pitch, withVelocity: 100, onChannel: 0)
//     })

//     RunLoop.main.run()

// }

fn main() {}
