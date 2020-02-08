/// A view is used to update how the score appears in the view widget.
pub trait View {
    /// Set group at measure number, beat number, part number.
    fn set(measure: u16, beat: u8, part: u8);
}
