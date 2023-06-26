mod encoding {
    mod for_label {
        use gix_filter::worktree;

        #[test]
        fn unknown() {
            assert_eq!(
                worktree::encoding::for_label("FOO").unwrap_err().to_string(),
                "An encoding named 'FOO' is not known"
            );
        }

        #[test]
        fn utf32_is_not_supported() {
            for enc in ["UTF-32BE", "UTF-32LE", "UTF-32", "UTF-32LE-BOM", "UTF-32BE-BOM"] {
                assert!(
                    matches!(
                        worktree::encoding::for_label(enc).unwrap_err(),
                        worktree::encoding::for_label::Error::Unknown { .. }
                    ),
                    "it's not needed for the web and this crate is meant for use in firefox"
                );
            }
        }

        #[test]
        fn various_spellings_of_utf_8_are_supported() {
            for enc in ["UTF8", "UTF-8", "utf-8", "utf8"] {
                let enc = worktree::encoding::for_label(enc).unwrap();
                assert_eq!(enc.name(), "UTF-8");
            }
        }

        #[test]
        fn various_utf_16_without_bom_suffix_are_supported() {
            for label in ["UTF-16BE", "UTF-16LE"] {
                let enc = worktree::encoding::for_label(label).unwrap();
                assert_eq!(enc.name(), label);
            }
        }

        #[test]
        fn various_utf_16_with_bom_suffix_are_unsupported() {
            for label in ["UTF-16BE-BOM", "UTF-16LE-BOM"] {
                assert!(
                    matches!(
                        worktree::encoding::for_label(label).unwrap_err(),
                        worktree::encoding::for_label::Error::Unknown { .. }
                    ),
                    "git supports these and has special handling, but we have not for now. Git has no tests for that either."
                );
            }
        }

        #[test]
        fn latin_1_is_supported_with_fallback() {
            let enc = worktree::encoding::for_label("latin-1").unwrap();
            assert_eq!(
                enc.name(),
                "windows-1252",
                "the encoding crate has its own fallback for ISO-8859-1 which we try to use"
            );
        }
    }
}

mod encode_to_git {
    use bstr::ByteSlice;
    use gix_filter::worktree;
    use gix_filter::worktree::encode_to_git::RoundTrip;

    #[test]
    fn simple() -> crate::Result {
        let input = &b"hello"[..];
        for round_trip in [RoundTrip::Ignore, RoundTrip::Validate] {
            let mut buf = Vec::new();
            worktree::encode_to_git(input, encoding("UTF-8"), &mut buf, round_trip)?;
            assert_eq!(buf.as_bstr(), input)
        }
        Ok(())
    }

    fn encoding(label: &str) -> &'static encoding_rs::Encoding {
        worktree::encoding::for_label(label).expect("encoding is valid and known at compile time")
    }
}
