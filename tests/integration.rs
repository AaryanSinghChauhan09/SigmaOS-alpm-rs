use sigmaos_alpm_rs::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_alpm_rs::AlpmRs::new("sigmaos-alpm-rs").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-alpm-rs"));
}
