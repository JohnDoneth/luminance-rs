//! #TODO
/// The region to allow drawn shader fragments within when drawing.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ScissorRegion {
  /// The x position of the scissor region.
  pub x: u32,

  /// The y position of the scissor region.
  pub y: u32,

  /// The width of the scissor region.
  pub width: u32,

  /// The height of the scissor region.
  pub height: u32,
}
