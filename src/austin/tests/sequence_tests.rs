use austin::result::Result;
use austin::node::Node;
use austin::sequence::{ Sequence };
use austin::node_collection::NodeCollection;
use austin::tests::helpers;
use austin::tests::helpers::TestTarget;

#[test]
fn constructor_new() {
    let sequence = Sequence::<TestTarget>::new("Test Sequence");
    assert!(sequence.name == "Test Sequence");
}

#[test]
fn evaluate_success_when_empty() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    let result = sequence.evaluate(&mut target);
    assert!(result == Result::Success)
}

#[test]
fn evaluate_success_with_success() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::success()));

    let result = sequence.evaluate(&mut target);
    assert!(result == Result::Success)
}

#[test]
fn evaluate_failure_with_failure() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::failure()));

    let result = sequence.evaluate(&mut target);
    assert!(result == Result::Failure)
}

#[test]
fn evaluate_pending_with_pending() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::pending()));

    let result = sequence.evaluate(&mut target);
    assert!(result == Result::Pending)
}

#[test]
fn evaluate_failure_short_circuits() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::success()));
    sequence.add(Box::new(helpers::failure()));
    sequence.add(Box::new(helpers::raise_error()));

    let result = sequence.evaluate(&mut target);
    assert!(result == Result::Failure)
}

#[test]
fn evaluate_pending_short_circuits() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::success()));
    sequence.add(Box::new(helpers::pending()));
    sequence.add(Box::new(helpers::raise_error()));

    let result = sequence.evaluate(&mut target);
    assert!(result == Result::Pending)
}