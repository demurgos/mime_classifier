/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

/// A loading context, for context-specific sniffing, as defined in
/// <https://mimesniff.spec.whatwg.org/#context-specific-sniffing>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LoadContext {
  Browsing,
  Image,
  AudioVideo,
  Plugin,
  Style,
  Script,
  Font,
  TextTrack,
  CacheManifest,
}
