use anyhow::{Context, Result};
use bytes::{Buf, Bytes};
use cliclack::{log, progress_bar, ProgressBar};
use concurrent_queue::ConcurrentQueue;
use dashmap::DashMap;
use std::thread;
use vorbis_rs::{VorbisDecoder, VorbisEncoderBuilder};

use crate::client::get_asset_bytes;

pub fn process_audio(bytes: Bytes, increase_by: u32) -> Result<Bytes> {
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
    increase_by: u32,
    hash: &String,
) -> Result<Bytes> {
    let bytes = get_asset_bytes(client, hash).context("failed to fetch asset bytes")?;
    process_audio(bytes, increase_by)
}

pub fn process_chunk(
    gain: u32,
    pb: &ProgressBar,
    output_map: &DashMap<String, Bytes>,
    queue: &ConcurrentQueue<String>,
) {
    let client = reqwest::blocking::Client::new();

    while let Ok(hash) = queue.pop() {
        let bytes_response = process_asset(&client, gain, &hash);
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

pub fn spawn_workers(
    gain: u32,
    output_map: &DashMap<String, Bytes>,
    sounds: &Vec<String>,
) -> Result<()> {
    let available_threads = thread::available_parallelism();
    let threads = if available_threads.is_ok() {
        available_threads.unwrap().get()
    } else {
        usize::from(1u8)
    };
    log::info(format!("using {} worker(s)", threads))?;

    let queue = ConcurrentQueue::bounded(sounds.len());
    for hash in sounds {
        let _ = queue.force_push(hash.clone());
    }

    let pb = progress_bar(queue.len() as u64);
    pb.start("processing sounds");

    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                process_chunk(gain, &pb, output_map, &queue);
            });
        }
    });

    pb.stop("processed sounds");
    Ok(())
}
