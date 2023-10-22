use unicode_segmentation::UnicodeSegmentation;

/// Split up a Unicode source string into a collection of graphemes.
///
/// The grapheme is closest to what would be considered a text character in the human
/// sense, enabling us handle Unicode variable names and strings with relative ease.
pub fn convert_to_graphemes(input: String) -> Vec<String> {
    input.graphemes(true).map(|g| g.to_string()).collect()
}