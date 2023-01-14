use unicode_segmentation::UnicodeSegmentation;

pub fn convert_to_graphemes(input: &'static str) -> Vec<&'static str> {
    input.graphemes(true).collect::<Vec<&'static str>>()
}