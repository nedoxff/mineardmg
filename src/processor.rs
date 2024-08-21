use bytes::{Buf, Bytes};
use vorbis_rs::{VorbisDecoder, VorbisEncoderBuilder};

pub fn process_audio(bytes: Bytes, increase_by: i32) -> Bytes {
    let mut reader = bytes.reader();
    let mut transcoded = vec![];

    let mut decoder = VorbisDecoder::new(&mut reader).expect("failed to decode ogg stream");
    let mut encoder = VorbisEncoderBuilder::new(
        decoder.sampling_frequency(),
        decoder.channels(),
        &mut transcoded,
    )
    .expect("failed to initialize an encoder")
    .build()
    .expect("failed to build an encoder");

    while let Some(block) = decoder
        .decode_audio_block()
        .expect("failed to decode an audio block")
    {
        let samples = block.samples()[0]
            .into_iter()
            .map(|f| f * (10f32).powf(increase_by as f32 / 20.0))
            .collect::<Vec<_>>();

        encoder
            .encode_audio_block(vec![samples])
            .expect("failed to encode audio block");
    }

    encoder.finish().expect("failed to finish the encoder");
    Bytes::from(transcoded)
}
