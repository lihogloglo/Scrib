use super::*;

const WOOD: u16 = 1;
const STONE: u16 = 2;
const METAL: u16 = 3;
const DT: f32 = 1.0 / 60.0;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn map() -> ImpactMap {
    let mut m = ImpactMap::default();
    m.insert(None, None, ImpactCues::impact(0.0, id("thud")));
    m.insert(Some(METAL), None, ImpactCues::impact(0.1, id("clank_light")).with_band(2.0, id("clank_heavy")).with_slide(id("scrape_metal")));
    m.insert(Some(STONE), Some(WOOD), ImpactCues::impact(0.0, id("knock_wood_stone")));
    m.insert(Some(STONE), None, ImpactCues::default().with_slide(id("scrape_stone")));
    m
}

fn hit(body: u64, surfaces: [u16; 2], impulse: f32, speed: f32) -> ContactSample {
    ContactSample { bodies: [Some(body), None], surfaces, impulse, speed, slide: 0.0, at: [0.0; 3] }
}

fn slide(body: u64, surfaces: [u16; 2], slide: f32) -> ContactSample {
    ContactSample { bodies: [Some(body), None], surfaces, impulse: 0.1, speed: 0.16, slide, at: [0.0; 3] }
}

fn cues(frame: &ImpactFrame) -> Vec<&str> {
    frame.impacts.iter().map(|p| p.cue.as_str()).collect()
}

#[test]
fn pair_lookup_falls_back_from_exact_to_mover_to_other_to_any() {
    let m = map();
    // Exact pair, either order.
    assert_eq!(m.impact(WOOD, STONE, 1.0).map(RecordId::as_str), Some("knock_wood_stone"));
    assert_eq!(m.impact(STONE, WOOD, 1.0).map(RecordId::as_str), Some("knock_wood_stone"));
    // The mover's own entry, then the other side's.
    assert_eq!(m.impact(METAL, WOOD, 1.0).map(RecordId::as_str), Some("clank_light"));
    assert_eq!(m.impact(WOOD, METAL, 1.0).map(RecordId::as_str), Some("clank_light"));
    // STONE's entry has no bands: the catch-all plays.
    assert_eq!(m.impact(STONE, 0, 1.0).map(RecordId::as_str), Some("thud"));
    assert_eq!(m.slide(STONE, WOOD).map(RecordId::as_str), Some("scrape_stone"));
    assert_eq!(m.slide(METAL, STONE).map(RecordId::as_str), Some("scrape_metal"));
    assert!(m.slide(WOOD, 0).is_none());
}

#[test]
fn impulse_bands_pick_the_cue_and_light_hits_can_be_silent() {
    let m = map();
    assert_eq!(m.impact(METAL, 0, 0.05), None, "under the lowest band");
    assert_eq!(m.impact(METAL, 0, 0.5).map(RecordId::as_str), Some("clank_light"));
    assert_eq!(m.impact(METAL, 0, 2.0).map(RecordId::as_str), Some("clank_heavy"));
    assert_eq!(m.impact(METAL, 0, 40.0).map(RecordId::as_str), Some("clank_heavy"));
}

#[test]
fn speed_gates_and_scales_the_volume() {
    let mut a = ImpactAudio::new(map(), ImpactParams::default());
    // Holding a body up (g dt) is no hit.
    assert!(a.tick(DT, &[hit(1, [WOOD, 0], 0.05, 0.16)]).impacts.is_empty());
    let soft = a.tick(DT, &[hit(2, [WOOD, 0], 0.3, 0.5)]).impacts;
    let hard = a.tick(DT, &[hit(3, [WOOD, 0], 3.0, 6.0)]).impacts;
    let mid = a.tick(DT, &[hit(4, [WOOD, 0], 1.0, 3.25)]).impacts;
    assert!((soft[0].volume - 0.15).abs() < 1e-5, "{soft:?}");
    assert!((hard[0].volume - 1.0).abs() < 1e-5);
    assert!(mid[0].volume > soft[0].volume && mid[0].volume < hard[0].volume);
}

#[test]
fn a_body_jittering_in_contact_does_not_machine_gun() {
    let mut a = ImpactAudio::new(map(), ImpactParams::default());
    // A box chattering on a slope: a hit over the gate every step for one second.
    let played: usize = (0..60).map(|_| a.tick(DT, &[hit(7, [WOOD, 0], 0.4, 0.8)]).impacts.len()).sum();
    // At most one per 0.1 s cooldown: 10 in a second (6 steps apart at 60 Hz).
    assert!((9..=10).contains(&played), "{played}");
}

#[test]
fn a_much_harder_hit_overrides_the_cooldown() {
    let mut a = ImpactAudio::new(map(), ImpactParams::default());
    assert_eq!(a.tick(DT, &[hit(7, [WOOD, 0], 0.4, 0.8)]).impacts.len(), 1);
    assert!(a.tick(DT, &[hit(7, [WOOD, 0], 0.6, 1.2)]).impacts.is_empty(), "not twice as hard");
    assert_eq!(a.tick(DT, &[hit(7, [WOOD, 0], 0.9, 1.8)]).impacts.len(), 1, "over twice as hard as the last sound");
}

#[test]
fn a_bouncing_ball_sounds_each_bounce() {
    let mut a = ImpactAudio::new(map(), ImpactParams::default());
    // Bounces 0.4, 0.3, 0.2 s apart, each softer; in between it flies (no contacts).
    let mut played = Vec::new();
    let mut t = 0;
    for (gap, speed) in [(0, 5.0), (24, 3.0), (18, 1.8), (12, 1.0)] {
        for _ in 0..gap {
            a.tick(DT, &[]);
            t += 1;
        }
        played.extend(a.tick(DT, &[hit(1, [METAL, STONE], 0.3 * speed, speed)]).impacts.into_iter().map(|p| (t, p.volume)));
        t += 1;
    }
    assert_eq!(played.len(), 4, "{played:?}");
    assert!(played.windows(2).all(|w| w[1].1 < w[0].1), "softer each bounce: {played:?}");
}

#[test]
fn world_geometry_is_not_rate_limited() {
    let mut a = ImpactAudio::new(map(), ImpactParams::default());
    // Two balls land on the same floor in the same step: both heard.
    let f = a.tick(DT, &[hit(1, [WOOD, STONE], 1.0, 3.0), hit(2, [WOOD, STONE], 1.0, 3.0)]);
    assert_eq!(cues(&f), ["knock_wood_stone", "knock_wood_stone"]);
    // Two moving bodies hitting each other share the cooldown.
    let both = ContactSample { bodies: [Some(3), Some(4)], ..hit(3, [WOOD, WOOD], 1.0, 3.0) };
    assert_eq!(a.tick(DT, &[both]).impacts.len(), 1);
    assert!(a.tick(DT, &[hit(4, [WOOD, 0], 1.0, 3.0)]).impacts.is_empty());
}

#[test]
fn a_tick_plays_the_loudest_hits_up_to_the_cap() {
    let params = ImpactParams { max_per_tick: 3, ..ImpactParams::default() };
    let mut a = ImpactAudio::new(map(), params);
    let samples: Vec<_> = (0..10u8).map(|i| hit(u64::from(i), [WOOD, 0], 1.0, 0.5 + f32::from(i) * 0.5)).collect();
    let f = a.tick(DT, &samples);
    assert_eq!(f.impacts.len(), 3);
    // The three fastest (5, 4.5 and 4 m/s), loudest first.
    let volumes: Vec<f32> = f.impacts.iter().map(|p| p.volume).collect();
    let want: Vec<f32> = [5.0, 4.5, 4.0].iter().map(|s| ramp(*s, 0.5, 6.0, 0.15)).collect();
    assert_eq!(volumes, want);
}

#[test]
fn a_slide_loop_starts_follows_and_fades_out() {
    let mut a = ImpactAudio::new(map(), ImpactParams::default());
    let f = a.tick(DT, &[slide(5, [METAL, WOOD], 2.0)]);
    assert!(matches!(&f.slides[..], [SlideChange::Start { body: 5, cue, level, .. }] if cue.as_str() == "scrape_metal" && *level > 0.0));
    assert!(f.impacts.is_empty(), "sliding is no hit");
    // Keeps sliding: one loop, its level rises toward the speed's.
    let mut level = 0.0;
    for _ in 0..10 {
        let f = a.tick(DT, &[slide(5, [METAL, WOOD], 2.0)]);
        let [SlideChange::Update { body: 5, level: l, .. }] = f.slides[..] else { panic!("{:?}", f.slides) };
        assert!(l >= level);
        level = l;
    }
    let full = ramp(2.0, 0.4, 3.0, 0.3);
    assert!((level - full).abs() < 1e-5, "{level} vs {full}");
    // Stops sliding: held briefly, then a fast fade and a Stop.
    let mut stopped_at = None;
    for i in 0..30 {
        let f = a.tick(DT, &[]);
        if f.slides.iter().any(|c| matches!(c, SlideChange::Stop { body: 5 })) {
            stopped_at = Some(i);
            break;
        }
    }
    // Hold 0.12 s (7 ticks) + fade from ~0.79 at 1/0.1 s (5 ticks).
    assert!(stopped_at.is_some_and(|i| (9..=14).contains(&i)), "{stopped_at:?}");
    assert_eq!(a.slide_count(), 0);
}

#[test]
fn slow_slides_and_surfaces_without_a_loop_are_silent() {
    let mut a = ImpactAudio::new(map(), ImpactParams::default());
    assert!(a.tick(DT, &[slide(5, [METAL, WOOD], 0.2)]).slides.is_empty());
    assert!(a.tick(DT, &[slide(5, [WOOD, 0], 2.0)]).slides.is_empty());
}

#[test]
fn a_body_keeps_one_loop_and_switches_with_the_surface() {
    let mut a = ImpactAudio::new(map(), ImpactParams::default());
    // On two surfaces at once: the faster contact's loop only.
    let f = a.tick(DT, &[slide(5, [STONE, WOOD], 1.0), slide(5, [METAL, STONE], 2.0)]);
    assert!(matches!(&f.slides[..], [SlideChange::Start { cue, .. }] if cue.as_str() == "scrape_metal"), "{:?}", f.slides);
    // Another body's own loop.
    let f = a.tick(DT, &[slide(5, [METAL, STONE], 2.0), slide(6, [STONE, 0], 2.0)]);
    assert_eq!(f.slides.len(), 2);
    assert_eq!(a.slide_count(), 2);
    // Body 5 now slides on stone only (its own surface too): the old loop stops, the new one starts.
    let f = a.tick(DT, &[slide(5, [STONE, STONE], 2.0), slide(6, [STONE, 0], 2.0)]);
    assert!(f.slides.contains(&SlideChange::Stop { body: 5 }));
    assert!(f.slides.iter().any(|c| matches!(c, SlideChange::Start { body: 5, cue, .. } if cue.as_str() == "scrape_stone")));
}
