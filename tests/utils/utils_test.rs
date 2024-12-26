use rust_comp_geo::utils::utils::{close_equal, round, round_to_n_decimals};

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

  #[test]
  fn round_close_equal_test() {
    let test_cases: Vec<f64> = vec![
        4.815162342,
        9.999999,
        2.0,
        -7.0,
        12345.6789,
        0.0,
        -0.00000001,
    ];


    // Test that the rounded value is close to the original value
    // for the entirety of the list.
    for x in test_cases {
        let rounded = round(x);
        let eq = close_equal(x, rounded, None);
        assert!(eq);
    }
  }

  #[test]
  fn round_to_n_decimals_equality_test() {
    let test_cases: Vec<(f64, u32, bool)> = vec![
        (4.815162342, 2, false),
        (9.999999, 8, true),
        (2.0, 4, true),
        (-7.0, 1, true),
        (12345.6789, 1, false),
        (0.0, 7, true),
        (-0.00000001, 8, true),
    ];

    for (x, n, expected) in test_cases {
        let rounded = round(x);
        let rounded_n = round_to_n_decimals(x, n);
        let result = close_equal(rounded, rounded_n, None);
        assert_eq!(result, expected);
    }
  }

}