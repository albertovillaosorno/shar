// File:
//   - domain.rs
// Path:
//   - src/rtf/src/domain/domain.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Pure RTF parsing, decoding, normalization, and date conversion rules.
// - Must-Not:
//   - Read files, write outputs, parse CLI arguments, or select adapters.
// - Allows:
//   - Deterministic byte decoding and Markdown normalization.
// - Split-When:
//   - Split when parser and date conversion become independent domains.
// - Merge-When:
//   - Another domain module owns the same conversion invariants.
// - Summary:
//   - Dependency-free RTF-to-Markdown conversion domain.
// - Description:
//   - Defines pure format behavior without filesystem or process policy.
// - Usage:
//   - Used by application commands and direct conversion callers.
// - Defaults:
//   - No filesystem or process IO is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Parser state and normalization helpers form one format domain.
//

//! A small, dependency-free RTF-to-Markdown converter.
//!
//! This is not a complete RTF implementation. It handles what the original
//! game's plain-text README needs: paragraph breaks, tabs, `\'hh` code-page
//! escapes, `\u` Unicode escapes, and the skipping of metadata destinations
//! (font, color, and style tables, document info, list tables, and any
//! ignorable `\*` destination). Inline formatting is intentionally dropped in
//! favor of clean, readable Markdown text.
//!
//! Input is treated as Windows-1252 (the `\ansicpg1252` code page declared by
//! the source document); bytes are decoded accordingly.

/// Destinations whose entire group content is metadata and must be skipped.
const SKIP_DESTINATIONS: &[&str] = &[
    "fonttbl",
    "filetbl",
    "bkmkstart",
    "bkmkend",
    "xe",
    "tc",
    "rxe",
    "txe",
    "revtbl",
    "protusertbl",
    "userprops",
    "fldinst",
    "datafield",
    "colortbl",
    "stylesheet",
    "info",
    "annotation",
    "listtable",
    "listoverridetable",
    "listpicture",
    "pntext",
    "pntxta",
    "pntxtb",
    "header",
    "footer",
    "headerf",
    "footerf",
    "headerl",
    "headerr",
    "footerl",
    "footerr",
    "pict",
    "shppict",
    "nonshppict",
    "shp",
    "shpinst",
    "shprslt",
    "object",
    "themedata",
    "colorschememapping",
    "latentstyles",
    "datastore",
    "generator",
    "xmlnstbl",
    "rsidtbl",
    "mmathPr",
    "ftncn",
    "ftnsep",
    "ftnsepc",
    "aftncn",
    "aftnsep",
    "aftnsepc",
    "xmlopen",
    "xmlclose",
    "xmlattrname",
    "xmlattrvalue",
    "xmlname",
    "smarttag",
    "smarttagtype",
    "factoidname",
    "factoidtype",
    "mailmerge",
    "mmconnectstr",
    "mmconnectstrdata",
    "mmquery",
    "mmsource",
    "mmodso",
    "mmodsoudl",
    "mmodsoudldata",
    "mmodsorecipdata",
    "mmodsofilter",
    "mmodsofldmpdata",
    "atnauthor",
    "atndate",
    "atnicn",
    "atnid",
    "atnparent",
    "atnref",
    "atntime",
    "atrfstart",
    "atrfend",
    "listname",
    "liststylename",
    "leveltext",
    "levelnumbers",
    "listlevel",
    "listoverride",
    "lfolevel",
    "lsdlockedexcept",
    "falt",
    "fname",
    "fontemb",
    "fontfile",
    "fchars",
    "lchars",
    "keycode",
    "gridtbl",
    "pgptbl",
    "blipuid",
    "bliptag",
    "blipupi",
    "picprop",
    "sp",
    "sn",
    "sv",
    "do",
    "htmltag",
    "mhtmltag",
    "hl",
    "hlloc",
    "hlsrc",
    "hlfr",
    "hsv",
    "linkval",
    "file",
    "oldcprops",
    "oldpprops",
    "oldsprops",
    "oldtprops",
    "author",
    "background",
    "buptim",
    "category",
    "comment",
    "company",
    "creatim",
    "doccomm",
    "docvar",
    "fldtype",
    "keywords",
    "manager",
    "operator",
    "printim",
    "revtim",
    "subject",
    "title",
    "template",
    "private",
    "panose",
    "nextfile",
    "ffdeftext",
    "ffentrymcr",
    "ffexitmcr",
    "ffformat",
    "ffhelptext",
    "ffl",
    "ffname",
    "ffstattext",
    "formfield",
    "nesttableprops",
    "defchp",
    "defpap",
    "dptxbxtext",
    "ebcend",
    "ebcstart",
    "footnote",
    "g",
    "hlinkbase",
    "list",
    "listtext",
    "macc",
    "maccPr",
    "maln",
    "malnScr",
    "margPr",
    "mbar",
    "mbarPr",
    "mbaseJc",
    "mbegChr",
    "mborderBox",
    "mborderBoxPr",
    "mbox",
    "mboxPr",
    "mchr",
    "mcount",
    "mctrlPr",
    "md",
    "mdeg",
    "mdegHide",
    "mden",
    "mdiff",
    "mdPr",
    "me",
    "mendChr",
    "meqArr",
    "meqArrPr",
    "mf",
    "mfName",
    "mfPr",
    "mfunc",
    "mfuncPr",
    "mgroupChr",
    "mgroupChrPr",
    "mgrow",
    "mhideBot",
    "mhideLeft",
    "mhideRight",
    "mhideTop",
    "mlim",
    "mlimloc",
    "mlimlow",
    "mlimlowPr",
    "mlimupp",
    "mlimuppPr",
    "mm",
    "mmaddfieldname",
    "mmath",
    "mmathPict",
    "mmaxdist",
    "mmc",
    "mmcJc",
    "mmcPr",
    "mmcs",
    "mmdatasource",
    "mmheadersource",
    "mmmailsubject",
    "mmodsomappedname",
    "mmodsoname",
    "mmodsosort",
    "mmodsosrc",
    "mmodsotable",
    "mmodsouniquetag",
    "mmPr",
    "mmr",
    "mnary",
    "mnaryPr",
    "mnoBreak",
    "mnum",
    "mobjDist",
    "moMath",
    "moMathPara",
    "moMathParaPr",
    "mopEmu",
    "mphant",
    "mphantPr",
    "mplcHide",
    "mpos",
    "mr",
    "mrad",
    "mradPr",
    "mrPr",
    "msepChr",
    "mshow",
    "mshp",
    "msPre",
    "msPrePr",
    "msSub",
    "msSubPr",
    "msSubSup",
    "msSubSupPr",
    "msSup",
    "msSupPr",
    "mstrikeBLTR",
    "mstrikeH",
    "mstrikeTLBR",
    "mstrikeV",
    "msub",
    "msubHide",
    "msup",
    "msupHide",
    "mtransp",
    "mtype",
    "mvertJc",
    "mvfmf",
    "mvfml",
    "mvtof",
    "mvtol",
    "mzeroAsc",
    "mzeroDesc",
    "mzeroWid",
    "nonesttables",
    "objalias",
    "objclass",
    "objdata",
    "objname",
    "objsect",
    "objtime",
    "oleclsid",
    "password",
    "passwordhash",
    "pgp",
    "pn",
    "pnseclvl",
    "propname",
    "protend",
    "protstart",
    "pxe",
    "result",
    "shpgrp",
    "shptxt",
    "staticval",
    "svb",
    "ud",
    "upr",
    "wgrffmtfilter",
    "windowcaption",
    "writereservation",
    "writereservhash",
    "xform",
];

/// Converts RTF bytes to Markdown text.
#[must_use]
pub fn rtf_to_markdown(bytes: &[u8]) -> String {
    Parser::new(bytes).run()
}

/// Formats a Unix timestamp (seconds since the epoch, UTC) as `YYYY-MM-DD`.
#[must_use]
pub fn format_unix_date(seconds: i64) -> String {
    let days = seconds.div_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}")
}

/// Converts a day count since the Unix epoch to a civil `(year, month, day)`,
/// using Howard Hinnant's `civil_from_days` algorithm.
const fn civil_from_days(
    days: i64
) -> (
    i64,
    i64,
    i64,
) {
    let shifted = days.saturating_add(719_468);
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.saturating_sub(era.saturating_mul(146_097));
    let year_numerator = day_of_era
        .saturating_sub(day_of_era.div_euclid(1_460))
        .saturating_add(day_of_era.div_euclid(36_524))
        .saturating_sub(day_of_era.div_euclid(146_096));
    let year_of_era = year_numerator.div_euclid(365);
    let base_year = year_of_era.saturating_add(era.saturating_mul(400));
    let leap_adjustment = year_of_era
        .div_euclid(4)
        .saturating_sub(year_of_era.div_euclid(100));
    let elapsed_days = year_of_era
        .saturating_mul(365)
        .saturating_add(leap_adjustment);
    let day_of_year = day_of_era.saturating_sub(elapsed_days);
    let month_position = day_of_year
        .saturating_mul(5)
        .saturating_add(2)
        .div_euclid(153);
    let prior_month_days = month_position
        .saturating_mul(153)
        .saturating_add(2)
        .div_euclid(5);
    let day = day_of_year
        .saturating_sub(prior_month_days)
        .saturating_add(1);
    let month = if month_position < 10 {
        month_position.saturating_add(3)
    } else {
        month_position.saturating_sub(9)
    };
    let final_year = if month <= 2 {
        base_year.saturating_add(1)
    } else {
        base_year
    };
    (
        final_year, month, day,
    )
}

/// Returns `true` if a control word names a destination to skip wholesale.
fn is_skip_destination(word: &str) -> bool {
    SKIP_DESTINATIONS.contains(&word)
}

/// Decodes a single Windows-1252 byte to a Unicode character.
fn decode_cp1252(byte: u8) -> char {
    if (0x80..0xA0).contains(&byte) {
        cp1252_high(byte)
    } else {
        char::from(byte)
    }
}

/// Maps the Windows-1252 0x80..=0x9F range, which differs from Latin-1.
fn cp1252_high(byte: u8) -> char {
    let code = match byte {
        0x80 => 0x20AC,
        0x82 => 0x201A,
        0x83 => 0x0192,
        0x84 => 0x201E,
        0x85 => 0x2026,
        0x86 => 0x2020,
        0x87 => 0x2021,
        0x88 => 0x02C6,
        0x89 => 0x2030,
        0x8A => 0x0160,
        0x8B => 0x2039,
        0x8C => 0x0152,
        0x8E => 0x017D,
        0x91 => 0x2018,
        0x92 => 0x2019,
        0x93 => 0x201C,
        0x94 => 0x201D,
        0x95 => 0x2022,
        0x96 => 0x2013,
        0x97 => 0x2014,
        0x98 => 0x02DC,
        0x99 => 0x2122,
        0x9A => 0x0161,
        0x9B => 0x203A,
        0x9C => 0x0153,
        0x9E => 0x017E,
        0x9F => 0x0178,
        _ => 0xFFFD,
    };
    char::from_u32(code).unwrap_or('\u{FFFD}')
}

/// Collapses breakable ASCII whitespace without destroying semantic Unicode
/// spacing such as the nonbreaking space emitted by RTF.
fn collapse_breakable_whitespace(line: &str) -> String {
    let mut collapsed = String::new();
    let mut pending_space = false;
    for character in line.chars() {
        if character.is_ascii_whitespace() {
            pending_space = !collapsed.is_empty();
            continue;
        }
        if pending_space {
            collapsed.push(' ');
            pending_space = false;
        }
        collapsed.push(character);
    }
    collapsed
}

/// Collapses runs of whitespace within each line and runs of blank lines, then
/// trims surrounding blank lines and ensures a single trailing newline.
fn normalize(raw: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    for line in raw.split('\n') {
        lines.push(collapse_breakable_whitespace(line));
    }

    let mut result = String::new();
    let mut previous_blank = true;
    for line in lines {
        let blank = line.is_empty();
        if blank && previous_blank {
            continue;
        }
        result.push_str(&line);
        result.push('\n');
        previous_blank = blank;
    }
    while result.ends_with('\n') {
        result.truncate(
            result
                .len()
                .saturating_sub(1),
        );
    }
    if !result.is_empty() {
        result.push('\n');
    }
    result
}

/// Stateful byte cursor over the RTF document.
struct Parser<'a> {
    /// Borrowed bytes stay raw because RTF control words are byte-oriented.
    bytes: &'a [u8],
    /// The cursor is explicit so malformed input can be bounds-checked.
    pos: usize,
    /// Text is accumulated after metadata groups have been filtered out.
    out: String,
    /// One ignore flag per open group; the top entry applies to current text.
    ignore: Vec<bool>,
    /// One Unicode fallback count per group, inherited according to RTF scope.
    unicode_fallback: Vec<usize>,
    /// One selected font id per group so charset decoding follows scope.
    font: Vec<Option<i32>>,
    /// Document-level font restored by the RTF plain-formatting control.
    default_font: Option<i32>,
    /// Font ids declared with the Symbol charset in the font table.
    symbol_fonts: std::collections::BTreeSet<i32>,
    /// Pending high surrogate for UTF-16 pairs encoded by RTF unicode
    /// controls.
    pending_high_surrogate: Option<u16>,
}

impl<'a> Parser<'a> {
    /// Starts in a root visible group because bare text outside groups is valid
    /// enough to preserve.
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            pos: 0,
            out: String::new(),
            ignore: vec![false],
            unicode_fallback: vec![1],
            font: vec![None],
            default_font: None,
            symbol_fonts: std::collections::BTreeSet::new(),
            pending_high_surrogate: None,
        }
    }

    /// Reads the current group policy so nested skipped metadata stays silent.
    fn current_ignore(&self) -> bool {
        *self
            .ignore
            .last()
            .unwrap_or(&false)
    }

    /// Reads the current group's selected font id.
    fn current_font(&self) -> Option<i32> {
        self.font
            .last()
            .copied()
            .flatten()
    }

    /// Updates the current group's selected font id.
    fn set_font(
        &mut self,
        font_id: i32,
    ) {
        if let Some(top) = self
            .font
            .last_mut()
        {
            *top = Some(font_id);
        }
    }

    /// Records and selects the document's default font.
    fn set_default_font(
        &mut self,
        font_id: i32,
    ) {
        self.default_font = Some(font_id);
        self.set_font(font_id);
    }

    /// Restores the current group to the document's default font.
    fn reset_font(&mut self) {
        if let Some(top) = self
            .font
            .last_mut()
        {
            *top = self.default_font;
        }
    }

    /// Decodes one ANSI byte according to the selected font charset.
    fn decode_ansi_byte(
        &self,
        byte: u8,
    ) -> char {
        let uses_symbol_charset = self
            .current_font()
            .is_some_and(
                |font_id| {
                    self.symbol_fonts
                        .contains(&font_id)
                },
            );
        if byte == 0xD4 && uses_symbol_charset {
            '™'
        } else {
            decode_cp1252(byte)
        }
    }

    /// Reads the current group's Unicode fallback width.
    fn current_unicode_fallback(&self) -> usize {
        self.unicode_fallback
            .last()
            .copied()
            .unwrap_or(1)
    }

    /// Updates the current group's Unicode fallback width.
    fn set_unicode_fallback(
        &mut self,
        count: usize,
    ) {
        if let Some(top) = self
            .unicode_fallback
            .last_mut()
        {
            *top = count;
        }
    }

    /// Flushes an unmatched high surrogate before unrelated output.
    fn flush_pending_surrogate(&mut self) {
        if self
            .pending_high_surrogate
            .take()
            .is_some()
        {
            self.out
                .push('�');
        }
    }

    /// Emits one signed UTF-16 code unit from an RTF unicode control.
    fn emit_unicode(
        &mut self,
        value: i32,
    ) {
        if self.current_ignore() {
            return;
        }
        if !(i32::from(i16::MIN)..=i32::from(i16::MAX)).contains(&value) {
            self.flush_pending_surrogate();
            self.out
                .push('�');
            return;
        }
        let adjusted_value = if value.is_negative() {
            value.checked_add(65_536_i32)
        } else {
            Some(value)
        };
        let Some(adjusted) = adjusted_value else {
            self.flush_pending_surrogate();
            self.out
                .push('�');
            return;
        };
        let Ok(code_unit) = u16::try_from(adjusted) else {
            self.flush_pending_surrogate();
            self.out
                .push('�');
            return;
        };
        match code_unit {
            0xD800_u16..=0xDBFF_u16 => {
                self.flush_pending_surrogate();
                self.pending_high_surrogate = Some(code_unit);
            }
            0xDC00_u16..=0xDFFF_u16 => {
                let Some(high) = self
                    .pending_high_surrogate
                    .take()
                else {
                    self.out
                        .push('�');
                    return;
                };
                let Some(high_offset) = u32::from(high).checked_sub(0xD800_u32)
                else {
                    self.out
                        .push('�');
                    return;
                };
                let Some(low_offset) =
                    u32::from(code_unit).checked_sub(0xDC00_u32)
                else {
                    self.out
                        .push('�');
                    return;
                };
                let scalar = high_offset
                    .checked_mul(0x400_u32)
                    .and_then(
                        |high_component| {
                            0x1_0000_u32.checked_add(high_component)
                        },
                    )
                    .and_then(|base| base.checked_add(low_offset));
                if let Some(character) = scalar.and_then(char::from_u32) {
                    self.out
                        .push(character);
                } else {
                    self.out
                        .push('�');
                }
            }
            _ => {
                self.flush_pending_surrogate();
                if let Some(character) = char::from_u32(u32::from(code_unit)) {
                    self.out
                        .push(character);
                }
            }
        }
    }

    /// Marks the current group ignored when a destination is metadata-only.
    fn set_ignore(&mut self) {
        self.flush_pending_surrogate();
        if let Some(top) = self
            .ignore
            .last_mut()
        {
            *top = true;
        }
    }

    /// Advances the cursor without permitting malformed input to overflow it.
    fn advance(
        &mut self,
        amount: usize,
    ) {
        self.pos = self
            .pos
            .saturating_add(amount)
            .min(
                self.bytes
                    .len(),
            );
    }

    /// Reads a byte relative to the current cursor with checked offset math.
    fn byte_at_offset(
        &self,
        offset: usize,
    ) -> Option<u8> {
        self.pos
            .checked_add(offset)
            .and_then(
                |index| {
                    self.bytes
                        .get(index)
                },
            )
            .copied()
    }

    /// Runs the byte cursor before normalization so malformed input cannot
    /// escape bounds checks.
    fn run(mut self) -> String {
        while let Some(current) = self
            .bytes
            .get(self.pos)
            .copied()
        {
            match current {
                b'{' => {
                    let inherited_ignore = self.current_ignore();
                    let inherited_fallback = self.current_unicode_fallback();
                    let inherited_font = self.current_font();
                    self.ignore
                        .push(inherited_ignore);
                    self.unicode_fallback
                        .push(inherited_fallback);
                    self.font
                        .push(inherited_font);
                    self.advance(1);
                }
                b'}' => {
                    if self
                        .ignore
                        .pop()
                        .is_none()
                    {
                        self.ignore
                            .push(false);
                    }
                    if self
                        .ignore
                        .is_empty()
                    {
                        self.ignore
                            .push(false);
                    }
                    if self
                        .unicode_fallback
                        .pop()
                        .is_none()
                    {
                        self.unicode_fallback
                            .push(1);
                    }
                    if self
                        .unicode_fallback
                        .is_empty()
                    {
                        self.unicode_fallback
                            .push(1);
                    }
                    if self
                        .font
                        .pop()
                        .is_none()
                    {
                        self.font
                            .push(None);
                    }
                    if self
                        .font
                        .is_empty()
                    {
                        self.font
                            .push(None);
                    }
                    self.advance(1);
                }
                b'\\' => self.control(),
                b'\t' => {
                    self.emit(' ');
                    self.advance(1);
                }
                0x00..=0x1F | 0x7F => self.advance(1),
                other => {
                    let character = self.decode_ansi_byte(other);
                    self.emit(character);
                    self.advance(1);
                }
            }
        }
        self.flush_pending_surrogate();
        normalize(&self.out)
    }

    /// Dispatches after `\` because RTF uses separate word and symbol control
    /// forms.
    fn control(&mut self) {
        self.advance(1);
        let Some(symbol) = self
            .bytes
            .get(self.pos)
            .copied()
        else {
            return;
        };
        if symbol.is_ascii_alphabetic() {
            self.control_word();
        } else {
            self.control_symbol(symbol);
        }
    }

    /// Parses alphabetic controls separately so optional numeric parameters
    /// remain attached.
    fn control_word(&mut self) {
        let start = self.pos;
        while self
            .bytes
            .get(self.pos)
            .is_some_and(u8::is_ascii_alphabetic)
        {
            self.advance(1);
        }
        let word = self
            .bytes
            .get(start..self.pos)
            .and_then(|slice| std::str::from_utf8(slice).ok())
            .unwrap_or_default()
            .to_owned();

        let param = self.read_param();
        if self
            .bytes
            .get(self.pos)
            == Some(&b' ')
        {
            self.advance(1);
        }
        self.apply_word(
            &word, param,
        );
    }

    /// Reads signed RTF parameters without panicking when a control has no
    /// value.
    fn read_param(&mut self) -> Option<i32> {
        let start = self.pos;
        if self
            .bytes
            .get(self.pos)
            == Some(&b'-')
        {
            self.advance(1);
        }
        while self
            .bytes
            .get(self.pos)
            .is_some_and(u8::is_ascii_digit)
        {
            self.advance(1);
        }
        if self.pos == start {
            return None;
        }
        self.bytes
            .get(start..self.pos)
            .and_then(|slice| std::str::from_utf8(slice).ok())
            .and_then(
                |value| {
                    value
                        .parse::<i32>()
                        .ok()
                },
            )
    }

    /// Applies only text-affecting controls because layout fidelity is outside
    /// this converter.
    fn apply_word(
        &mut self,
        word: &str,
        param: Option<i32>,
    ) {
        match word {
            "par" | "sect" => self.emit_text("\n\n"),
            "line" => self.emit_text("<br>\n"),
            "page" | "row" => self.emit('\n'),
            "tab" | "cell" => self.emit(' '),
            "emdash" => self.emit('—'),
            "endash" => self.emit('–'),
            "bullet" => self.emit('•'),
            "emspace" => self.emit(' '),
            "enspace" => self.emit(' '),
            "qmspace" => self.emit(' '),
            "zwj" => self.emit('\u{200D}'),
            "zwnj" => self.emit('\u{200C}'),
            "zwbo" => self.emit('\u{200B}'),
            "zwnbo" => self.emit('\u{FEFF}'),
            "lquote" => self.emit('‘'),
            "rquote" => self.emit('’'),
            "ldblquote" => self.emit('“'),
            "rdblquote" => self.emit('”'),
            "bin" => {
                let parsed_byte_count =
                    param.and_then(|value| usize::try_from(value).ok());
                if let Some(byte_count) = parsed_byte_count {
                    self.flush_pending_surrogate();
                    self.advance(byte_count);
                } else {
                    self.set_ignore();
                }
            }
            "u" => {
                if let Some(value) = param {
                    self.emit_unicode(value);
                    self.skip_unicode_fallback();
                }
            }
            "uc" => {
                if let Some(value) = param
                    && let Ok(count) = usize::try_from(value)
                {
                    self.set_unicode_fallback(count);
                }
            }
            "f" => {
                if let Some(font_id) = param {
                    self.set_font(font_id);
                }
            }
            "deff" => {
                if let Some(font_id) = param {
                    self.set_default_font(font_id);
                }
            }
            "plain" => self.reset_font(),
            "fcharset" => {
                if param == Some(2_i32)
                    && let Some(font_id) = self.current_font()
                {
                    let _inserted = self
                        .symbol_fonts
                        .insert(font_id);
                }
            }
            other if is_skip_destination(other) => self.set_ignore(),
            _ => {}
        }
    }

    /// Appends text atomically unless the current group is ignored.
    fn emit_text(
        &mut self,
        text: &str,
    ) {
        if !self.current_ignore() {
            self.flush_pending_surrogate();
            self.out
                .push_str(text);
        }
    }

    /// Appends a character to the output unless the current group is ignored.
    fn emit(
        &mut self,
        character: char,
    ) {
        if !self.current_ignore() {
            self.flush_pending_surrogate();
            self.out
                .push(character);
        }
    }

    /// Handles one-byte controls that can emit literal characters or metadata
    /// markers.
    fn control_symbol(
        &mut self,
        symbol: u8,
    ) {
        self.advance(1);
        match symbol {
            b'\'' => self.emit_hex_byte(),
            b'*' => self.set_ignore(),
            b'\\' | b'{' | b'}' => self.emit(char::from(symbol)),
            b'~' => self.emit(' '),
            b'_' => self.emit('‑'),
            _ => {}
        }
    }

    /// Decodes hex escapes without consuming structural delimiters as digits.
    fn emit_hex_byte(&mut self) {
        let Some(first) = self.byte_at_offset(0) else {
            return;
        };
        if !first.is_ascii_hexdigit() {
            return;
        }
        let Some(second) = self.byte_at_offset(1) else {
            self.advance(1);
            return;
        };
        if !second.is_ascii_hexdigit() {
            self.advance(1);
            return;
        }
        let digits = [
            first, second,
        ];
        self.advance(2);
        let decoded_byte = std::str::from_utf8(&digits)
            .ok()
            .and_then(
                |value| {
                    u8::from_str_radix(
                        value, 16,
                    )
                    .ok()
                },
            );
        if let Some(ansi_byte) = decoded_byte {
            let character = self.decode_ansi_byte(ansi_byte);
            self.emit(character);
        }
    }

    /// Skips one complete ANSI fallback token after a Unicode control.
    fn skip_one_unicode_fallback(&mut self) -> bool {
        let Some(first) = self.byte_at_offset(0) else {
            return false;
        };
        if first != b'\\' {
            self.advance(1);
            return true;
        }
        match self.byte_at_offset(1) {
            Some(b'\'') => self.advance(4),
            Some(next) if next.is_ascii_alphabetic() => {
                self.advance(2);
                while self
                    .bytes
                    .get(self.pos)
                    .is_some_and(u8::is_ascii_alphabetic)
                {
                    self.advance(1);
                }
                if self
                    .bytes
                    .get(self.pos)
                    == Some(&b'-')
                {
                    self.advance(1);
                }
                while self
                    .bytes
                    .get(self.pos)
                    .is_some_and(u8::is_ascii_digit)
                {
                    self.advance(1);
                }
                if self
                    .bytes
                    .get(self.pos)
                    == Some(&b' ')
                {
                    self.advance(1);
                }
            }
            Some(_) => self.advance(2),
            None => self.advance(1),
        }
        true
    }

    /// Skips the scoped number of fallback characters after Unicode.
    fn skip_unicode_fallback(&mut self) {
        for _ in 0..self.current_unicode_fallback() {
            if !self.skip_one_unicode_fallback() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rtf_to_markdown;

    #[test]
    fn unicode_escape_honors_zero_fallback_count() {
        let markdown = rtf_to_markdown(br"{\rtf1\ansi\uc0\u233x}");

        assert_eq!(
            markdown,
            "éx\n"
        );
    }

    #[test]
    fn empty_document_stays_empty() {
        assert_eq!(
            rtf_to_markdown(b""),
            ""
        );
    }

    #[test]
    fn unicode_control_without_parameter_preserves_text() {
        let markdown = rtf_to_markdown(br"{\rtf1\ansi\u?x}");

        assert_eq!(
            markdown,
            "?x\n"
        );
    }

    #[test]
    fn unicode_escape_decodes_surrogate_pairs() {
        let markdown = rtf_to_markdown(br"{\rtf1\ansi\uc1\u-10179?\u-8704?}");

        assert_eq!(
            markdown,
            "😀\n"
        );
    }

    #[test]
    fn unicode_escape_skips_control_word_fallback() {
        let markdown = rtf_to_markdown(br"{\rtf1\ansi\uc1\u233\emdash X}");

        assert_eq!(
            markdown,
            "éX\n"
        );
    }

    #[test]
    fn binary_control_skips_declared_bytes() {
        let markdown = rtf_to_markdown(br"{\rtf1\ansi A\bin3 xyzB}");

        assert_eq!(
            markdown,
            "AB\n"
        );
    }

    #[test]
    fn standard_character_controls_are_preserved() {
        let input = concat!(
            r"{\rtf1 A\emdash B\endash C",
            r"\bullet D\lquote E\rquote F",
            r"\ldblquote G\rdblquote H}",
        );
        let markdown = rtf_to_markdown(input.as_bytes());

        assert_eq!(
            markdown,
            "A—B–C•D‘E’F“G”H\n"
        );
    }

    #[test]
    fn optional_hyphen_does_not_become_visible_text() {
        let markdown = rtf_to_markdown(br"{\rtf1 co\-operate}");

        assert_eq!(
            markdown,
            "cooperate\n"
        );
    }

    #[test]
    fn nonbreaking_space_remains_nonbreaking() {
        let markdown = rtf_to_markdown(br"{\rtf1 A\~B}");

        assert_eq!(
            markdown,
            "A B\n"
        );
    }

    #[test]
    fn truncated_hex_escape_does_not_leak_nibble() {
        let markdown = rtf_to_markdown(br"A\'4");

        assert_eq!(
            markdown,
            "A\n"
        );
    }

    #[test]
    fn page_break_separates_text_blocks() {
        let markdown = rtf_to_markdown(br"{\rtf1 first\page second}");

        assert_eq!(
            markdown,
            "first\nsecond\n"
        );
    }

    #[test]
    fn table_controls_separate_cells_and_rows() {
        let markdown = rtf_to_markdown(br"{\rtf1 one\cell two\row three}");

        assert_eq!(
            markdown,
            "one two\nthree\n"
        );
    }

    #[test]
    fn metadata_destinations_do_not_leak_content() {
        let input = concat!(
            r"{\rtf1 visible",
            r"{\filetbl file-data}",
            r"{\revtbl revision-data}",
            r"{\fldinst instruction-data}",
            r"{\datafield field-data}",
            r"end}",
        );
        let markdown = rtf_to_markdown(input.as_bytes());

        assert_eq!(
            markdown,
            "visibleend\n"
        );
    }

    #[test]
    fn raw_control_bytes_do_not_leak_into_text() {
        let input = [
            b'{', b'\\', b'r', b't', b'f', b'1', b' ', b'A', 0_u8, 1_u8,
            0x7F_u8, b'B', b'}',
        ];
        let markdown = rtf_to_markdown(&input);

        assert_eq!(
            markdown,
            "AB
"
        );
    }

    #[test]
    fn semantic_space_controls_preserve_width() {
        let markdown =
            rtf_to_markdown(br"{\rtf1 A\emspace B\enspace C\qmspace D}");

        assert_eq!(
            markdown,
            "A B C D\n"
        );
    }
}
