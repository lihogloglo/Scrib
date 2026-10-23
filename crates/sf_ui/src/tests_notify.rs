//! Message box and toast queues (ui-text.md "Notifications").

use crate::{MessageBoxChoice, MessageBoxQueue, ToastConfig, ToastPhase, ToastQueue};

fn labels(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn message_boxes_answer_in_order() {
    let mut q = MessageBoxQueue::new();
    let v0 = q.version();
    let a = q.show("Rest here?", labels(&["Yes", "No"]), "OK");
    let b = q.show("The door is locked.", Vec::new(), "OK");
    assert_ne!(q.version(), v0);
    assert_eq!(q.current().unwrap().id, a);
    assert_eq!(q.pending(), 1);
    // The empty button list got the fallback.
    let v1 = q.version();
    assert_eq!(q.choose(2), None, "out of range");
    assert_eq!(q.version(), v1);
    assert_eq!(q.choose(1), Some(MessageBoxChoice { id: a, index: 1 }));
    assert_ne!(q.version(), v1);
    assert_eq!(q.current().unwrap().buttons, ["OK"]);
    // Back does nothing without a cancel button, then picks it.
    assert_eq!(q.cancel(), None);
    assert!(q.set_cancel(b, Some(0)));
    assert!(!q.set_cancel(b, Some(1)));
    assert_eq!(q.cancel(), Some(MessageBoxChoice { id: b, index: 0 }));
    assert!(q.is_empty());
    assert_eq!(q.choose(0), None);
}

#[test]
fn message_box_withdraw() {
    let mut q = MessageBoxQueue::new();
    let a = q.show("a", labels(&["x"]), "OK");
    let b = q.show("b", labels(&["x"]), "OK");
    let v = q.version();
    assert!(q.withdraw(b));
    assert_eq!(
        q.version(),
        v,
        "a hidden box leaving changes nothing on screen"
    );
    assert!(q.withdraw(a));
    assert_ne!(q.version(), v);
    assert!(!q.withdraw(a));
}

fn config() -> ToastConfig {
    ToastConfig {
        base: 1.0,
        per_char: 0.1,
        min: 2.0,
        max: 5.0,
        scale: 1.0,
        fade_in: 0.2,
        fade_out: 0.5,
        max_visible: 2,
        max_queued: 3,
        merge_repeats: true,
    }
}

#[test]
fn toast_durations_follow_text_length() {
    let c = config();
    assert!((c.duration("hi") - 2.0).abs() < 1e-6, "min");
    assert!((c.duration(&"x".repeat(25)) - 3.5).abs() < 1e-6);
    assert!((c.duration(&"x".repeat(100)) - 5.0).abs() < 1e-6, "max");
    let slow = ToastConfig { scale: 2.0, ..c };
    assert!((slow.duration("hi") - 4.0).abs() < 1e-6);
}

#[test]
fn toast_lifecycle_fades_in_holds_and_fades_out() {
    let mut q = ToastQueue::new(config());
    q.push("You picked up 3 gold."); // 21 chars → 3.1 s hold
    let c = q.config;
    let t = &q.visible()[0];
    assert_eq!(t.phase(&c), ToastPhase::FadingIn);
    assert!(t.opacity(&c) < 1e-6);
    q.tick(0.1);
    assert!((q.visible()[0].opacity(&c) - 0.5).abs() < 1e-4);
    q.tick(0.2);
    assert_eq!(q.visible()[0].phase(&c), ToastPhase::Shown);
    assert!((q.visible()[0].opacity(&c) - 1.0).abs() < 1e-6);
    // Fade-out starts at 0.2 + 3.1 = 3.3 s and ends at 3.8 s.
    q.tick(3.0); // 3.3
    q.tick(0.25); // 3.55
    let t = &q.visible()[0];
    assert_eq!(t.phase(&c), ToastPhase::FadingOut);
    assert!((t.opacity(&c) - 0.5).abs() < 1e-3);
    q.tick(0.3);
    assert!(q.visible().is_empty());
}

#[test]
fn toasts_stack_up_to_the_cap_then_retire_the_oldest() {
    let mut q = ToastQueue::new(config());
    let first = q.push("a");
    let b = q.push("b");
    q.tick(0.5);
    let c = q.push("c");
    // Full: c waits, a is retired early and fades out now.
    assert_eq!(q.visible().len(), 2);
    assert_eq!(q.waiting(), 1);
    assert_eq!(q.visible()[0].phase(&q.config), ToastPhase::FadingOut);
    // A second waiting toast doesn't retire b while a is still leaving.
    let d = q.push("d");
    assert_eq!(q.visible()[1].phase(&q.config), ToastPhase::Shown);
    q.tick(0.5); // a's fade (0.5 s) is over: c takes its slot, b retires for d
    let ids: Vec<_> = q.visible().iter().map(|t| t.id).collect();
    assert_eq!(ids, [b, c]);
    assert_eq!(q.visible()[0].phase(&q.config), ToastPhase::FadingOut);
    q.tick(0.5);
    let ids: Vec<_> = q.visible().iter().map(|t| t.id).collect();
    assert_eq!(ids, [c, d]);
    assert_eq!(q.waiting(), 0);
    assert!(!ids.contains(&first));
}

#[test]
fn toast_queue_is_bounded_and_repeats_merge() {
    let mut q = ToastQueue::new(config());
    let gold = q.push("gold");
    q.tick(1.5);
    assert_eq!(q.push("gold"), gold);
    let t = &q.visible()[0];
    assert_eq!(t.count, 2);
    assert_eq!(t.label(), "gold (x2)");
    assert!((t.age - q.config.fade_in).abs() < 1e-6, "restarted");
    // Flood: 1 more fits, then max_queued = 3 waiting, the oldest dropped.
    for i in 0..6 {
        q.push(format!("item {i}"));
    }
    assert_eq!(q.visible().len(), 2);
    assert_eq!(q.waiting(), 3);
    let waiting_repeat = q.push("item 5");
    q.dismiss(gold);
    q.dismiss(q.visible()[0].id);
    let texts: Vec<_> = q.visible().iter().map(crate::Toast::label).collect();
    assert_eq!(texts, ["item 3", "item 4"]);
    assert_eq!(q.waiting(), 1);
    q.dismiss(q.visible()[0].id);
    let last = q.visible().last().unwrap();
    assert_eq!((last.id, last.count), (waiting_repeat, 2));
}

#[test]
fn reduce_motion_toasts_snap() {
    let mut q = ToastQueue::new(config().without_fades());
    q.push("hello");
    let c = q.config;
    assert!((q.visible()[0].opacity(&c) - 1.0).abs() < 1e-6);
    q.tick(1.99);
    assert!((q.visible()[0].opacity(&c) - 1.0).abs() < 1e-6);
    q.tick(0.02);
    assert!(q.visible().is_empty());
}
