// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::{Language, export_language};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

fn temp_root(label: &str) -> PathBuf {
    let sequence = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "shar-language-{label}-{}-{sequence}",
        std::process::id()
    ))
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(path, bytes).map_err(|error| error.to_string())
}

fn fixture(root: &Path) -> Result<(PathBuf, PathBuf), String> {
    let game = root.join("game");
    let text = game.join("art/frontend/scrooby2/resource/txtbible/srr2.txt");
    let table = concat!(
        "Languages\tEFGIS\t\t\t\t\t\t\t\r\n",
        "\t\t\t\t\t\t\t\t\r\n",
        "Screen\tPHRASE TABLE\tSPACE\tE\tF\tG\tI\tS\tNOTES\r\n",
        "\tTERM\tCRITICAL\tENGLISH\tFRENCH\tGERMAN\tITALIAN\tSPANISH\t\r\n",
        "\t\t\t\t\t\t\t\t\r\n",
        "MENU\tHELLO\t\tHello\tBonjour\tHallo\tCiao\tHola\tnote\r\n"
    );
    write(&text, table.as_bytes())?;
    for code in ["F", "G", "I", "S"] {
        write(
            &text.with_file_name(format!("srr2.{code}")),
            code.as_bytes(),
        )?;
    }
    for (name, bytes) in [
        ("dialogf.rcf", b"french-dialogue".as_slice()),
        ("dialogg.rcf", b"german-dialogue".as_slice()),
        ("dialogi.rcf", b"italian-dialogue".as_slice()),
        ("dialogs.rcf", b"spanish-dialogue".as_slice()),
        ("Lisez-moi.rtf", b"french-readme".as_slice()),
        ("Liesmich.rtf", b"german-readme".as_slice()),
        ("Léeme.rtf", b"spanish-readme".as_slice()),
    ] {
        write(&game.join(name), bytes)?;
    }
    for language in ["french", "german", "spanish"] {
        write(
            &game.join(format!(
                "art/frontend/dynaload/images/loading/{language}/loading1.p3d"
            )),
            language.as_bytes(),
        )?;
        write(
            &game.join(format!(
                "art/frontend/dynaload/images/license/{language}/licenseG.p3d"
            )),
            language.as_bytes(),
        )?;
    }
    let movies = root.join("movies");
    for movie in ["fmv2", "fmv3"] {
        for track in 1..=4 {
            write(
                &movies
                    .join(movie)
                    .join(format!("audio_track_{track:02}.wav")),
                format!("{movie}-track-{track}").as_bytes(),
            )?;
        }
    }
    Ok((game, movies))
}

fn cleanup(root: &Path) {
    let _cleanup = fs::remove_dir_all(root);
}

#[test]
fn french_bundle_contains_dialogue_ui_and_cinematic_track_two() -> Result<(), String> {
    let root = temp_root("french");
    cleanup(&root);
    let (game, movies) = fixture(&root)?;
    let output = root.join("out/french");
    let manifest = export_language(&game, &movies, &output, Language::French)
        .map_err(|error| error.to_string())?;

    let result = if manifest.cinematic_audio.len() == 2
        && manifest
            .cinematic_audio
            .iter()
            .all(|entry| entry.track == "audio_track_02.wav")
        && output.join("source/dialogf.rcf").is_file()
        && output
            .join("source/art/frontend/dynaload/images/loading/french/loading1.p3d")
            .is_file()
        && output.join("cinematics/fmv2/audio_track_02.wav").is_file()
    {
        Ok(())
    } else {
        Err(format!("unexpected French manifest: {manifest:?}"))
    };
    cleanup(&root);
    result
}

#[test]
fn spanish_uses_normalized_track_four() -> Result<(), String> {
    let root = temp_root("spanish");
    cleanup(&root);
    let (game, movies) = fixture(&root)?;
    let output = root.join("out/spanish");
    let manifest = export_language(&game, &movies, &output, Language::Spanish)
        .map_err(|error| error.to_string())?;
    let result = if manifest
        .cinematic_audio
        .iter()
        .all(|entry| entry.track == "audio_track_04.wav")
    {
        Ok(())
    } else {
        Err(format!("unexpected Spanish manifest: {manifest:?}"))
    };
    cleanup(&root);
    result
}

#[test]
fn missing_localized_ui_fails_closed() -> Result<(), String> {
    let root = temp_root("missing-ui");
    cleanup(&root);
    let (game, movies) = fixture(&root)?;
    fs::remove_dir_all(game.join("art/frontend/dynaload/images/loading/german"))
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(game.join("art/frontend/dynaload/images/license/german"))
        .map_err(|error| error.to_string())?;
    let error = export_language(&game, &movies, &root.join("out/german"), Language::German)
        .err()
        .ok_or_else(|| "German export unexpectedly succeeded".to_owned())?;
    let result = if error
        .to_string()
        .contains("no localized loading/license UI")
    {
        Ok(())
    } else {
        Err(error.to_string())
    };
    cleanup(&root);
    result
}
