// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Movie decoder policy unit tests.
// - Must-Not:
//   - Own production behavior or execute external media tools.
// - Allows:
//   - Pure assertions over supported movie classifications.
// - Split-When:
//   - Split when decoder capabilities gain independent ownership.
// - Merge-When:
//   - Merge when another module owns the identical evidence.
// - Summary:
//   - Movie decoder policy unit tests.
// - Description:
//   - Proves FFmpeg-decodable Xbox XMV inputs are exported to HAP.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Unsupported or unknown movie kinds remain excluded.
//

//! Movie decoder policy unit tests.

use rmv::MovieKind;

use super::is_movie_decodable_by_ffmpeg;

#[test]
fn xbox_xmv_movies_are_exported_through_ffmpeg() {
    assert!(is_movie_decodable_by_ffmpeg(MovieKind::XboxXmvLike));
    assert!(is_movie_decodable_by_ffmpeg(MovieKind::BinkV1));
    assert!(is_movie_decodable_by_ffmpeg(MovieKind::BinkV2));
    assert!(!is_movie_decodable_by_ffmpeg(MovieKind::Unknown));
    assert!(!is_movie_decodable_by_ffmpeg(MovieKind::OggNamedRmv));
}
