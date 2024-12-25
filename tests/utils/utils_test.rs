use rust_comp_geo::utils::utils::close_equal;

#[cfg(test)] // Only compiles when running tests
mod close_equal_test {
  use super::*; // Import root scope

  #[test] // This attribute signals that this function is a test
  fn test_near_zero() {
      let eq = close_equal(0.0, 0.00000000001, None);
      assert!(eq);
  }

  #[test]
  fn test_negative_zero() {
      let eq = close_equal(0.0, -0.0, None);
      assert!(eq);
  }

  #[test]
  fn near_equal() {
      let eq = close_equal(10.0, 9.9999999999, None);
      assert!(eq);
  }

  #[test]
  fn fail_near_equal() {
      let eq = close_equal(10.0, 9.998, None);
      assert!(!eq); // Verify that it is false
  }

  #[test]
  fn pass_near_equal_higher_tolerance() {
      let eq = close_equal(10.0, 9.998, Some(0.1));
      assert!(eq);
  }
}