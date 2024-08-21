use std::path::Path;

use anyhow::{Context, Result};
use bytes::{Buf, Bytes};
use vorbis_rs::{VorbisDecoder, VorbisEncoderBuilder};

use crate::client::get_asset_bytes;

pub fn process_audio(bytes: Bytes, increase_by: i32) -> Result<Bytes> {
    let mut reader = bytes.reader();
    let mut transcoded = vec![];

    let mut decoder = VorbisDecoder::new(&mut reader).expect("failed to decode ogg stream");
    let mut encoder = VorbisEncoderBuilder::new(
        decoder.sampling_frequency(),
        decoder.channels(),
        &mut transcoded,
    )
    .context("failed to initialize an encoder")?
    .build()
    .context("failed to build an encoder")?;

    while let Some(block) = decoder
        .decode_audio_block()
        .context("failed to decode an audio block")?
    {
        let samples = block.samples()[0]
            .into_iter()
            .map(|f| f * (10f32).powf(increase_by as f32 / 20.0))
            .collect::<Vec<_>>();

        encoder
            .encode_audio_block(vec![samples])
            .context("failed to encode audio block")?;
    }

    encoder.finish().context("failed to finish the encoder")?;
    Ok(Bytes::from(transcoded))
}

pub async fn process_asset(
    client: &reqwest::Client,
    output_path: &String,
    increase_by: i32,
    path: &String,
    hash: &String,
) {
    let bytes = get_asset_bytes(&client, hash)
        .await
        .expect("failed to fetch asset bytes");
    let processed = process_audio(bytes, increase_by);
}
