use bincode::{deserialize, serialize};
use bytes::Bytes;
use std::fs::{read, read_to_string};
use std::path::PathBuf;
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;

use forge_lib::structs::forgemod::{ForgeMod, IncludeData};
use forge_lib::structs::manifest::ForgeManifest;

pub fn pack(manifest: ForgeManifest, root_path: PathBuf) -> Result<Bytes, std::io::Error> {
    let root_path = root_path.canonicalize().unwrap().parent().unwrap().to_path_buf();
    let artifact_path = root_path.join(&manifest.artifact);

    if !artifact_path.exists() || artifact_path.extension().unwrap() != "dll" {
        panic!("Artifact does not exist or is invalid.");
    }

    let artifact_data = read(artifact_path)?;

    let includes_data = manifest
        .includes
        .iter()
        .map(|include| {
            let include_path = root_path.join(&include.source);
            let include_data = read(include_path).unwrap();
            IncludeData {
                dest: include.target.clone(),
                data: include_data,
            }
        })
        .collect::<Vec<IncludeData>>();

    let beatmod = ForgeMod {
        manifest,
        artifact_data,
        includes_data,
    };

    let buf = serialize(&beatmod).unwrap();
    let buf = XzEncoder::new(buf, 9).finish()?;

    Ok(Bytes::from(buf))
}

pub fn unpack(beatmod: PathBuf) -> Result<ForgeMod, std::io::Error> {
    let beatmod_data = read(beatmod)?;
    let contents = XzDecoder::new(beatmod_data.as_slice()).into_inner();
    Ok(deserialize(contents).unwrap())
}
