use gdmx::Vec3;

const EPS: f32 = 1e-6;

#[test]
fn unit_vectors_have_equal_length() {
    let x_len = Vec3::X.length();
    let y_len = Vec3::NEG_Y.length();
    let len_diff = x_len - y_len;
    assert!(len_diff.abs() < EPS);
}
