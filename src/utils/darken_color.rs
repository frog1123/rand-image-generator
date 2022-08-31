pub fn darken(color: [u8; 3], amount: u8) -> [u8; 3] {
  return [color[0] - amount, color[1] - amount, color[2] - amount];
}