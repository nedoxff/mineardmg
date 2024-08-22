use std::{path::Path, thread};

use anyhow::{Context, Result};
use bytes::{Buf, Bytes};
use cliclack::{log, progress_bar, MultiProgress};
use dashmap::DashMap;
use vorbis_rs::{VorbisDecoder, VorbisEncoderBuilder};
use wg::WaitGroup;

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
        let mut new_samples = vec![];
        for channel in block.samples() {
            new_samples.push(
                channel
                    .into_iter()
                    .map(|f| f * (10f32).powf(increase_by as f32 / 20.0))
                    .collect::<Vec<_>>(),
            );
        }

        encoder
            .encode_audio_block(new_samples)
            .context("failed to encode audio block")?;
    }

    encoder.finish().context("failed to finish the encoder")?;
    Ok(Bytes::from(transcoded))
}

pub fn process_asset(
    client: &reqwest::blocking::Client,
    increase_by: i32,
    hash: &String,
) -> Result<Bytes> {
    let bytes = get_asset_bytes(client, hash).context("failed to fetch asset bytes")?;
    process_audio(bytes, increase_by)
}

pub fn process_chunk(
    id: i32,
    gain: i32,
    multi_progress: &MultiProgress,
    output_map: &DashMap<String, Bytes>,
    chunk: &[String],
) {
    let client = reqwest::blocking::Client::new();
    let pb = multi_progress.add(progress_bar(chunk.len() as u64));
    pb.start(format!("worker #{}", id));

    for hash in chunk {
        let bytes_response = process_asset(&client, gain, hash);
        match bytes_response {
            Ok(bytes) => {
                output_map.insert(hash.clone(), bytes);
                pb.inc(1);
            }
            Err(err) => {
                pb.stop(err);
            }
        }
    }
}

pub fn spawn_processors(
    gain: i32,
    multi_progress: &MultiProgress,
    thread_count: usize,
    output_map: &DashMap<String, Bytes>,
    sounds: &Vec<String>,
) {
    let chunks = sounds.chunks(sounds.len() / thread_count);
    let wg = WaitGroup::new();
    wg.add(chunks.len());
    let mut counter = 1;

    thread::scope(|s| {
        for chunk in chunks {
            let t_wg = wg.clone();
            s.spawn(move || {
                process_chunk(counter, gain, multi_progress, output_map, chunk);
                t_wg.done();
            });
            counter += 1;
        }
    });

    wg.wait();
}
