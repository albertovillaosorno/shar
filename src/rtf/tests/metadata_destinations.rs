// File:
//   - metadata_destinations.rs
// Path:
//   - src/rtf/tests/metadata_destinations.rs
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
//   - Deterministic RTF regression coverage for non-body destinations.
// - Must-Not:
//   - Depend on private documents or parser implementation details.
// - Allows:
//   - Public fixtures and caller-visible body-text assertions.
// - Split-When:
//   - Split when a destination family needs independent fixture infrastructure.
// - Merge-When:
//   - Another RTF test module owns the same metadata filtering contract.
// - Summary:
//   - Filters metadata destinations from generated Markdown body text.
// - Description:
//   - Exercises public conversion behavior for standard RTF destinations.
// - Usage:
//   - Executed through cargo test for the rtf crate.
// - Defaults:
//   - Fixtures remain deterministic and repository-local.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: The destination cases share one public filtering contract.
//   - Keeping them together makes destination coverage and omissions easier
//   - to audit than splitting one behavior across fragmented test modules.
//

//! Public regression coverage for RTF metadata destinations.
//!
//! Each fixture keeps non-body content out of caller-visible Markdown.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn first_page_headers_and_footers_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 A",
        r"{\headerf hidden-header}",
        r"{\footerf hidden-footer}",
        r"B}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn bookmark_names_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 A",
        r"{\bkmkstart bookmark-name}",
        r"{\bkmkend bookmark-name}",
        r"B}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn picture_alternative_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 A",
        r"{\shppict modern-picture-data}",
        r"{\nonshppict legacy-picture-data}",
        r"B}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn shape_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 A",
        r"{\shp shape-data}",
        r"{\shpinst shape-instructions}",
        r"{\shprslt shape-result}",
        r"B}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn index_entry_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 A",
        r"{\xe index-entry}",
        r"{\tc contents-entry}",
        r"{\rxe range-entry}",
        r"{\txe text-entry}",
        r"B}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn annotation_destinations_do_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\annotation reviewer-comment}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn list_picture_destinations_do_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\listpicture numbering-image-data}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn protected_user_tables_do_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\protusertbl protected-user-data}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn user_property_destinations_do_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\userprops custom-property-data}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn math_property_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\mmathPr hidden-0}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn note_separator_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\ftncn hidden-0}",
        r"{\ftnsep hidden-1}",
        r"{\ftnsepc hidden-2}",
        r"{\aftncn hidden-3}",
        r"{\aftnsep hidden-4}",
        r"{\aftnsepc hidden-5}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn custom_xml_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\xmlopen hidden-0}",
        r"{\xmlclose hidden-1}",
        r"{\xmlattrname hidden-2}",
        r"{\xmlattrvalue hidden-3}",
        r"{\xmlname hidden-4}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn smart_tag_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\smarttag hidden-0}",
        r"{\smarttagtype hidden-1}",
        r"{\factoidname hidden-2}",
        r"{\factoidtype hidden-3}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn mail_merge_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\mailmerge hidden-0}",
        r"{\mmconnectstr hidden-1}",
        r"{\mmconnectstrdata hidden-2}",
        r"{\mmquery hidden-3}",
        r"{\mmsource hidden-4}",
        r"{\mmodso hidden-5}",
        r"{\mmodsoudl hidden-6}",
        r"{\mmodsoudldata hidden-7}",
        r"{\mmodsorecipdata hidden-8}",
        r"{\mmodsofilter hidden-9}",
        r"{\mmodsofldmpdata hidden-10}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn annotation_metadata_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\atnauthor hidden-0}",
        r"{\atndate hidden-1}",
        r"{\atnicn hidden-2}",
        r"{\atnid hidden-3}",
        r"{\atnparent hidden-4}",
        r"{\atnref hidden-5}",
        r"{\atntime hidden-6}",
        r"{\atrfstart hidden-7}",
        r"{\atrfend hidden-8}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn list_definition_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\listname hidden-0}",
        r"{\liststylename hidden-1}",
        r"{\leveltext hidden-2}",
        r"{\levelnumbers hidden-3}",
        r"{\listlevel hidden-4}",
        r"{\listoverride hidden-5}",
        r"{\lfolevel hidden-6}",
        r"{\lsdlockedexcept hidden-7}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn font_metadata_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\falt hidden-0}",
        r"{\fname hidden-1}",
        r"{\fontemb hidden-2}",
        r"{\fontfile hidden-3}",
        r"{\fchars hidden-4}",
        r"{\lchars hidden-5}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn style_layout_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\keycode hidden-0}",
        r"{\gridtbl hidden-1}",
        r"{\pgptbl hidden-2}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn picture_metadata_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\blipuid hidden-0}",
        r"{\bliptag hidden-1}",
        r"{\blipupi hidden-2}",
        r"{\picprop hidden-3}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn shape_property_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\sp hidden-0}",
        r"{\sn hidden-1}",
        r"{\sv hidden-2}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn drawing_object_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\do hidden-0}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn html_metadata_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\htmltag hidden-0}",
        r"{\mhtmltag hidden-1}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn hyperlink_metadata_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\hl hidden-0}",
        r"{\hlloc hidden-1}",
        r"{\hlsrc hidden-2}",
        r"{\hlfr hidden-3}",
        r"{\hsv hidden-4}",
        r"{\linkval hidden-5}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn file_entry_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\file hidden-0}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn legacy_property_destinations_do_not_leak_into_body() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\oldcprops hidden-0}",
        r"{\oldpprops hidden-1}",
        r"{\oldsprops hidden-2}",
        r"{\oldtprops hidden-3}",
        r"end}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "visibleend\n"
    );
}

#[test]
fn author_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\author hidden-author}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn background_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\background hidden-background}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn backup_time_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\buptim hidden-backup-time}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn category_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\category hidden-category}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn comment_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\comment hidden-comment}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn company_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\company hidden-company}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn creation_time_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\creatim hidden-creation_time}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn document_comment_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\doccomm hidden-document_comment}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn document_variable_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\docvar hidden-document_variable}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn field_type_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\fldtype hidden-field_type}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn keywords_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\keywords hidden-keywords}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn manager_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\manager hidden-manager}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn operator_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\operator hidden-operator}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn print_time_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\printim hidden-print_time}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn revision_time_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\revtim hidden-revision_time}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn subject_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\subject hidden-subject}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn title_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\title hidden-title}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn template_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\template hidden-template}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn private_data_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\private hidden-private_data}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn panose_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\panose hidden-panose}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn next_file_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\nextfile hidden-next_file}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn form_default_text_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\ffdeftext hidden-form_default_text}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn form_entry_macro_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\ffentrymcr hidden-form_entry_macro}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn form_exit_macro_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\ffexitmcr hidden-form_exit_macro}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn form_format_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\ffformat hidden-form_format}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn form_help_text_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\ffhelptext hidden-form_help_text}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn form_list_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\ffl hidden-form_list}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn form_name_destination_does_not_leak_into_body() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\ffname hidden-form_name}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn form_status_text_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\ffstattext hidden-form_status_text}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn form_field_destination_does_not_leak_into_body() {
    let markdown =
        rtf_to_markdown(br"{\rtf1 A{\formfield hidden-form_field}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn nested_table_properties_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\nesttableprops hidden-nested-table-properties}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn default_character_properties_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\defchp hidden-default-character-properties}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn defpap_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\defpap hidden-defpap-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn dptxbxtext_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\dptxbxtext hidden-dptxbxtext-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ebcend_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\ebcend hidden-ebcend-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ebcstart_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\ebcstart hidden-ebcstart-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn footnote_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\footnote hidden-footnote-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn g_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\g hidden-g-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn hlinkbase_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\hlinkbase hidden-hlinkbase-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn list_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\list hidden-list-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn listtext_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\listtext hidden-listtext-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn macc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\macc hidden-macc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn macc_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\maccPr hidden-macc-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn maln_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\maln hidden-maln-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn maln_scr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\malnScr hidden-maln-scr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn marg_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\margPr hidden-marg-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mbar_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mbar hidden-mbar-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mbar_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mbarPr hidden-mbar-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mbase_jc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mbaseJc hidden-mbase-jc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mbeg_chr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mbegChr hidden-mbeg-chr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mborder_box_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mborderBox hidden-mborder-box-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mborder_box_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mborderBoxPr hidden-mborder-box-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mbox_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mbox hidden-mbox-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mbox_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mboxPr hidden-mbox-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mchr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mchr hidden-mchr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mcount_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mcount hidden-mcount-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mctrl_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mctrlPr hidden-mctrl-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn md_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\md hidden-md-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mdeg_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mdeg hidden-mdeg-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mdeg_hide_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mdegHide hidden-mdeg-hide-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mden_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mden hidden-mden-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mdiff_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mdiff hidden-mdiff-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn md_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mdPr hidden-md-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn me_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\me hidden-me-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mend_chr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mendChr hidden-mend-chr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn meq_arr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\meqArr hidden-meq-arr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn meq_arr_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\meqArrPr hidden-meq-arr-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mf_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mf hidden-mf-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mf_name_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mfName hidden-mf-name-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mf_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mfPr hidden-mf-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mfunc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mfunc hidden-mfunc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mfunc_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mfuncPr hidden-mfunc-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mgroup_chr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mgroupChr hidden-mgroup-chr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mgroup_chr_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mgroupChrPr hidden-mgroup-chr-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mgrow_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mgrow hidden-mgrow-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mhide_bot_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mhideBot hidden-mhide-bot-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mhide_left_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mhideLeft hidden-mhide-left-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mhide_right_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mhideRight hidden-mhide-right-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mhide_top_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mhideTop hidden-mhide-top-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mlim_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mlim hidden-mlim-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mlimloc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mlimloc hidden-mlimloc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mlimlow_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mlimlow hidden-mlimlow-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mlimlow_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mlimlowPr hidden-mlimlow-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mlimupp_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mlimupp hidden-mlimupp-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mlimupp_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mlimuppPr hidden-mlimupp-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mm_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mm hidden-mm-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmaddfieldname_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmaddfieldname hidden-mmaddfieldname-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmath_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmath hidden-mmath-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmath_pict_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmathPict hidden-mmath-pict-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmaxdist_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmaxdist hidden-mmaxdist-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmc hidden-mmc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmc_jc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmcJc hidden-mmc-jc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmc_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmcPr hidden-mmc-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmcs_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmcs hidden-mmcs-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmdatasource_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmdatasource hidden-mmdatasource-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmheadersource_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmheadersource hidden-mmheadersource-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmmailsubject_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmmailsubject hidden-mmmailsubject-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmodsomappedname_destination_does_not_leak_into_body() {
    let input =
        br"{\rtf1 A{\mmodsomappedname hidden-mmodsomappedname-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmodsoname_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmodsoname hidden-mmodsoname-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmodsosort_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmodsosort hidden-mmodsosort-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmodsosrc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmodsosrc hidden-mmodsosrc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmodsotable_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmodsotable hidden-mmodsotable-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmodsouniquetag_destination_does_not_leak_into_body() {
    let input =
        br"{\rtf1 A{\mmodsouniquetag hidden-mmodsouniquetag-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mm_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmPr hidden-mm-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mmr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mmr hidden-mmr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mnary_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mnary hidden-mnary-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mnary_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mnaryPr hidden-mnary-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mno_break_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mnoBreak hidden-mno-break-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mnum_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mnum hidden-mnum-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mobj_dist_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mobjDist hidden-mobj-dist-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mo_math_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\moMath hidden-mo-math-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mo_math_para_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\moMathPara hidden-mo-math-para-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mo_math_para_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\moMathParaPr hidden-mo-math-para-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mop_emu_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mopEmu hidden-mop-emu-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mphant_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mphant hidden-mphant-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mphant_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mphantPr hidden-mphant-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mplc_hide_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mplcHide hidden-mplc-hide-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mpos_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mpos hidden-mpos-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mr hidden-mr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mrad_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mrad hidden-mrad-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mrad_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mradPr hidden-mrad-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mr_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mrPr hidden-mr-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn msep_chr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msepChr hidden-msep-chr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mshow_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mshow hidden-mshow-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mshp_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mshp hidden-mshp-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ms_pre_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msPre hidden-ms-pre-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ms_pre_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msPrePr hidden-ms-pre-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ms_sub_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msSub hidden-ms-sub-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ms_sub_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msSubPr hidden-ms-sub-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ms_sub_sup_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msSubSup hidden-ms-sub-sup-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ms_sub_sup_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msSubSupPr hidden-ms-sub-sup-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ms_sup_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msSup hidden-ms-sup-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ms_sup_pr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msSupPr hidden-ms-sup-pr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mstrike_bltr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mstrikeBLTR hidden-mstrike-bltr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mstrike_h_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mstrikeH hidden-mstrike-h-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mstrike_tlbr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mstrikeTLBR hidden-mstrike-tlbr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mstrike_v_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mstrikeV hidden-mstrike-v-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn msub_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msub hidden-msub-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn msub_hide_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msubHide hidden-msub-hide-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn msup_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msup hidden-msup-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn msup_hide_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\msupHide hidden-msup-hide-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mtransp_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mtransp hidden-mtransp-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mtype_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mtype hidden-mtype-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mvert_jc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mvertJc hidden-mvert-jc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mvfmf_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mvfmf hidden-mvfmf-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mvfml_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mvfml hidden-mvfml-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mvtof_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mvtof hidden-mvtof-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mvtol_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mvtol hidden-mvtol-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mzero_asc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mzeroAsc hidden-mzero-asc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mzero_desc_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mzeroDesc hidden-mzero-desc-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn mzero_wid_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\mzeroWid hidden-mzero-wid-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn nonesttables_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\nonesttables hidden-nonesttables-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn objalias_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\objalias hidden-objalias-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn objclass_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\objclass hidden-objclass-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn objdata_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\objdata hidden-objdata-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn objname_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\objname hidden-objname-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn objsect_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\objsect hidden-objsect-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn objtime_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\objtime hidden-objtime-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn oleclsid_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\oleclsid hidden-oleclsid-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn password_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\password hidden-password-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn passwordhash_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\passwordhash hidden-passwordhash-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn pgp_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\pgp hidden-pgp-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn pn_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\pn hidden-pn-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn pnseclvl_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\pnseclvl hidden-pnseclvl-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn propname_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\propname hidden-propname-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn protend_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\protend hidden-protend-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn protstart_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\protstart hidden-protstart-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn pxe_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\pxe hidden-pxe-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn result_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\result hidden-result-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn shpgrp_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\shpgrp hidden-shpgrp-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn shptxt_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\shptxt hidden-shptxt-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn staticval_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\staticval hidden-staticval-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn svb_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\svb hidden-svb-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn ud_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\ud hidden-ud-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn upr_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\upr hidden-upr-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn wgrffmtfilter_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\wgrffmtfilter hidden-wgrffmtfilter-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn windowcaption_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\windowcaption hidden-windowcaption-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn writereservation_destination_does_not_leak_into_body() {
    let input =
        br"{\rtf1 A{\writereservation hidden-writereservation-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn writereservhash_destination_does_not_leak_into_body() {
    let input =
        br"{\rtf1 A{\writereservhash hidden-writereservhash-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}

#[test]
fn xform_destination_does_not_leak_into_body() {
    let input = br"{\rtf1 A{\xform hidden-xform-metadata}B}";
    let markdown = rtf_to_markdown(input);

    assert_eq!(
        markdown,
        "AB\n"
    );
}
