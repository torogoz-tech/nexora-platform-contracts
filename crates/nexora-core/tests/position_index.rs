use nexora_core::{SourcePosition, SourcePositionIndex};

#[test]
fn indexed_positions_preserve_absolute_utf8_scalar_columns_and_lf_lines() {
    let source = "é🦀x\r\ny\n尾";
    let index = SourcePositionIndex::new(source).unwrap();
    for (offset, line, column) in [
        (0, 1, 1),
        (2, 1, 2),
        (6, 1, 3),
        (7, 1, 4),
        (8, 1, 5),
        (9, 2, 1),
        (10, 2, 2),
        (11, 3, 1),
        (14, 3, 2),
    ] {
        assert_eq!(
            index.position(offset).unwrap(),
            SourcePosition {
                offset,
                line,
                column
            }
        );
    }
    for offset in [1, 3, 4, 5, 12, 13, 15] {
        assert!(index.position(offset).is_err());
    }
    assert_eq!(index.source(), source);
}

#[test]
fn empty_and_large_source_indexes_preserve_position_boundaries() {
    let index = SourcePositionIndex::new("").unwrap();
    assert_eq!(index.position(0).unwrap(), SourcePosition::default());
    assert!(index.position(1).is_err());
    let source = format!("{}é\n尾", "x".repeat(1_048_577));
    let index = SourcePositionIndex::new(&source).unwrap();
    assert_eq!(
        index.position(source.len()).unwrap(),
        SourcePosition {
            offset: source.len(),
            line: 2,
            column: 2
        }
    );
    assert!(index.position(source.len() - 1).is_err());
}
