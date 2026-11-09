//! `service` tests (services.md "Tests"): synthetic records, exact assertions.
#![allow(clippy::many_single_char_names, reason = "test fixtures: w, s, r, c, t mirror the spec's examples")]

use std::cell::Cell;
use std::collections::BTreeMap;

use serde::Serialize;
use serde::de::DeserializeOwned;
use smallvec::{SmallVec, smallvec};

use super::*;
use crate::service::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn round_trip<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(value: &T) {
    let bytes = postcard::to_allocvec(value).expect("serialize");
    let back: T = postcard::from_bytes(&bytes).expect("deserialize");
    assert_eq!(&back, value);
}

const MERCHANT: ActorKey = ActorKey(7);
const CHEST: ActorKey = ActorKey(8);
const P1: PlayerId = PlayerId::HOST;

fn p2() -> PlayerId {
    PlayerId::new(1).expect("player 2")
}

fn kind(s: &str) -> ServiceKind {
    ServiceKind(id(s))
}

fn offer(k: &str, shape: ServiceShape) -> ServiceOffer {
    ServiceOffer { provider: MERCHANT, kind: kind(k), shape, label: LabelRef(id(k)) }
}

/// A fake adapter: fixed prices with a per-player sell ratio, flags as fields.
struct Rules {
    refuse: bool,
    veto: bool,
    haggle_ok: bool,
    purse: PurseRule,
    train_price: Cell<i64>,
    sell_permille: BTreeMap<PlayerId, u32>,
}

fn rules() -> Rules {
    Rules {
        refuse: false,
        veto: false,
        haggle_ok: true,
        purse: PurseRule::Pool { base: 500, reset_hours: 24 },
        train_price: Cell::new(10),
        sell_permille: BTreeMap::new(),
    }
}

fn unit_value(item: &RecordId) -> i64 {
    match item.as_str() {
        "sword" => 100,
        "gem" => 50,
        "ring" => 40,
        "potion" => 10,
        _ => 1,
    }
}

impl ServiceRules for Rules {
    fn offers(&self, _ctx: &ServiceCtx) -> SmallVec<[ServiceOffer; 8]> {
        smallvec![
            offer("trade", ServiceShape::Trade),
            offer("train", ServiceShape::List),
            offer("spellmaking", ServiceShape::Custom(id("spellmaker_screen"))),
        ]
    }

    fn refusal(&self, _ctx: &ServiceCtx, kind: &ServiceKind) -> Option<(RecordId, RecordId)> {
        (self.refuse && kind.0.as_str() == "trade").then(|| (id("service refusal"), id("r1")))
    }

    fn stock_sources(&self, _ctx: &ServiceCtx) -> SmallVec<[Holder; 4]> {
        smallvec![Holder::Actor(MERCHANT), Holder::Actor(CHEST)]
    }

    fn buys(&self, _ctx: &ServiceCtx, stack: &ItemStack) -> bool {
        stack.item.as_str() != "key"
    }

    fn sells(&self, _ctx: &ServiceCtx, _source: Holder, stack: &ItemStack) -> bool {
        stack.item.as_str() != "key"
    }

    fn restocking(&self, _ctx: &ServiceCtx, _source: Holder, stack: &ItemStack) -> bool {
        stack.item.as_str() == "bread"
    }

    fn quote(&self, ctx: &ServiceCtx, line: &QuoteLine<'_>) -> i64 {
        let rule = FixedPrice { sell_permille: self.sell_permille.get(&ctx.player).copied().unwrap_or(500) };
        rule.price(&FixedPriceIn { side: line.side, unit_value: unit_value(line.item), count: line.count })
    }

    fn haggle(&self, _ctx: &ServiceCtx, _deal: &DealTotals, _roll: u8) -> HaggleOutcome {
        if self.haggle_ok {
            HaggleOutcome {
                accepted: true,
                effects: vec![ServiceEffect::UseSkill { stat: id("barter"), use_kind: 0, scale: 1.0 }],
            }
        } else {
            HaggleOutcome { accepted: false, effects: vec![ServiceEffect::Disposition { delta: -1 }] }
        }
    }

    fn vet_deal(&self, _ctx: &ServiceCtx, _session: &TradeSession) -> Result<(), DealVeto> {
        if self.veto {
            return Err(DealVeto {
                reason: LabelRef(id("stolen")),
                effects: vec![ServiceEffect::Adapter { id: id("confiscate"), payload: AttrValue::Int(1) }],
            });
        }
        Ok(())
    }

    fn entries(&self, _ctx: &ServiceCtx, _kind: &ServiceKind) -> Vec<ServiceEntry> {
        vec![ServiceEntry {
            id: EntryId(id("blade")),
            label: LabelRef(id("blade")),
            detail: EntryDetail::Stat { stat: id("blade"), current: 10, next: 11 },
            price: self.train_price.get(),
            state: EntryState::Available,
        }]
    }

    fn buy_entry(&self, _ctx: &ServiceCtx, _kind: &ServiceKind, entry: &EntryId) -> Result<Vec<ServiceEffect>, EntryRefusal> {
        if entry.0.as_str() != "blade" {
            return Err(EntryRefusal::Unknown);
        }
        Ok(vec![ServiceEffect::Pay { amount: self.train_price.get() }, ServiceEffect::RaiseStat { stat: id("blade") }])
    }

    fn purse_base(&self, _ctx: &ServiceCtx) -> PurseRule {
        self.purse
    }
}

fn ctx(player: PlayerId) -> ServiceCtx {
    ServiceCtx { player, provider: MERCHANT, provider_base: id("merchant"), now: GameTime { day: 1, minute: 0 }, tick: 10 }
}

fn config() -> TradeConfig {
    TradeConfig { currency: Currency { item: id("gold") }, haggle: true }
}

type World = BTreeMap<Holder, CharacterRpg>;

fn add(world: &mut World, holder: Holder, item: &str, count: u32) -> StackId {
    world.entry(holder).or_default().loadout.add(id(item), count, Attrs::new()).expect("add")
}

fn count(world: &World, holder: Holder, item: &str) -> u64 {
    world.get(&holder).map_or(0, |c| c.loadout.count(&id(item)))
}

/// Stack ids of the fixture.
struct Stacks {
    gold: StackId,
    sword: StackId,
    potion: StackId,
    ring: StackId,
    key: StackId,
    gem: StackId,
    m_potion: StackId,
    bread: StackId,
}

fn world() -> (World, Stacks) {
    let mut w = World::new();
    let p = Holder::Player(P1);
    let m = Holder::Actor(MERCHANT);
    let stacks = Stacks {
        gold: add(&mut w, p, "gold", 100),
        sword: add(&mut w, p, "sword", 1),
        potion: add(&mut w, p, "potion", 5),
        ring: add(&mut w, p, "ring", 2),
        key: add(&mut w, p, "key", 1),
        gem: add(&mut w, m, "gem", 3),
        m_potion: add(&mut w, m, "potion", 10),
        bread: add(&mut w, Holder::Actor(CHEST), "bread", 1),
    };
    add(&mut w, m, "gold", 50);
    (w, stacks)
}

fn session(player: PlayerId) -> TradeSession {
    TradeSession::new(SessionId(0), player, MERCHANT, kind("trade"))
}

/// Applies item commands to the fixture (a stack move takes both characters out and back).
fn apply(world: &mut World, cmd: RpgCommand) -> Vec<RpgEvent> {
    let from = cmd.holder().expect("item command");
    if let Some(to) = cmd.target() {
        let mut source = world.remove(&from).unwrap_or_default();
        let mut target = world.remove(&to).unwrap_or_default();
        let events = CharacterRpg::apply_move(&mut source, &mut target, cmd);
        world.insert(from, source);
        world.insert(to, target);
        return events.expect("move");
    }
    world.entry(from).or_default().apply(cmd).expect("apply")
}

// ---------------------------------------------------------------- offers

#[test]
fn offers_in_order_and_refusal_opens_nothing() {
    let mut r = rules();
    let c = ctx(P1);
    let kinds: Vec<_> = r.offers(&c).into_iter().map(|o| o.kind).collect();
    assert_eq!(kinds, vec![kind("trade"), kind("train"), kind("spellmaking")]);

    let claims = Conversations::default();
    let mut services = WorldServices::default();
    let mut sessions = ServiceSessions::new();
    let opened = open(&r, &c, &kind("train"), &claims, &mut services, &mut sessions).expect("open");
    assert_eq!(opened, Opened { shape: ServiceShape::List, session: None, restocked: Some(500) });
    assert!(sessions.is_empty());

    r.refuse = true;
    let refused = open(&r, &c, &kind("trade"), &claims, &mut services, &mut sessions);
    assert_eq!(refused, Err(OpenError::Refused { topic: id("service refusal"), response: id("r1") }));
    assert!(sessions.is_empty());
    assert_eq!(open(&r, &c, &kind("repair"), &claims, &mut services, &mut sessions), Err(OpenError::NotOffered));

    r.refuse = false;
    let opened = open(&r, &c, &kind("trade"), &claims, &mut services, &mut sessions).expect("open");
    assert_eq!(opened.shape, ServiceShape::Trade);
    assert_eq!(opened.restocked, None, "same day: no restock");
    assert!(sessions.get(opened.session.expect("session")).is_some());
}

// ---------------------------------------------------------------- session math

#[test]
fn session_math_stage_unstage_cap_merge_and_haggle_margin() {
    let (w, s) = world();
    let r = rules();
    let c = ctx(P1);
    let cfg = config();
    let env = TradeEnv { ctx: &c, rules: &r, holdings: &w, config: &cfg };
    let p = Holder::Player(P1);
    let m = Holder::Actor(MERCHANT);
    let mut t = session(P1);

    assert_eq!(t.stage(&env, Side::Sell, p, s.sword, 1), Ok(1));
    assert_eq!(t.stage(&env, Side::Sell, p, s.potion, 3), Ok(3));
    assert_eq!(t.stage(&env, Side::Sell, p, s.ring, 2), Ok(2));
    assert_eq!(t.stage(&env, Side::Buy, m, s.gem, 2), Ok(2));
    assert_eq!(t.stage(&env, Side::Buy, m, s.m_potion, 2), Ok(2));
    let prices: Vec<i64> = t.lines.iter().map(|l| l.price).collect();
    assert_eq!(prices, vec![50, 15, 40, 100, 20]);
    assert_eq!(t.merchant_offer, 105 - 120);
    assert_eq!(t.balance, -15);
    assert!(!t.haggled);

    // Unstage restores; restaging merges into the same line.
    assert_eq!(t.unstage(&env, 3, 1), Ok(1));
    assert_eq!(t.merchant_offer, 105 - 70);
    assert_eq!(t.stage(&env, Side::Buy, m, s.gem, 1), Ok(1));
    assert_eq!(t.lines.len(), 5);
    assert_eq!(t.merchant_offer, -15);

    // Staging more than held is capped; then nothing is left.
    assert_eq!(t.stage(&env, Side::Sell, p, s.potion, 10), Ok(2));
    assert_eq!(t.lines[1].count, 5);
    assert_eq!(t.stage(&env, Side::Sell, p, s.potion, 1), Err(TradeRefusal::NothingLeft));
    assert_eq!(t.merchant_offer, -5);

    // Refusals while staging.
    assert_eq!(t.stage(&env, Side::Sell, p, s.gold, 1), Err(TradeRefusal::Currency));
    assert_eq!(t.stage(&env, Side::Sell, p, s.key, 1), Err(TradeRefusal::NotTraded));
    assert_eq!(t.stage(&env, Side::Buy, p, s.sword, 1), Err(TradeRefusal::WrongSource(p)));
    assert_eq!(t.stage(&env, Side::Sell, p, s.sword, 0), Err(TradeRefusal::ZeroCount));

    // A haggled balance keeps its margin as lines change, and keeps the deal's sign.
    assert_eq!(t.set_balance(&cfg, 0), Ok(0));
    assert!(t.haggled);
    assert_eq!(t.stage(&env, Side::Buy, m, s.m_potion, 1), Ok(1));
    assert_eq!((t.merchant_offer, t.balance), (-15, -10));
    assert_eq!(t.set_balance(&cfg, 50), Ok(0), "the player can't turn 'I pay' into 'you pay'");
    let off = TradeConfig { haggle: false, ..config() };
    assert_eq!(t.set_balance(&off, -15), Err(TradeRefusal::HaggleDisabled));

    // Unstaging everything resets the deal.
    while !t.lines.is_empty() {
        let n = t.lines[0].count;
        t.unstage(&env, 0, n).expect("unstage");
    }
    assert_eq!((t.merchant_offer, t.balance, t.haggled), (0, 0, false));
}

// ---------------------------------------------------------------- accept order

#[test]
fn accept_refusals_in_order_change_nothing() {
    let (mut w, s) = world();
    let mut r = rules();
    let c = ctx(P1);
    let cfg = config();
    let p = Holder::Player(P1);
    let m = Holder::Actor(MERCHANT);
    let pool = MerchantMoney::Pool(500);

    // 1. Nothing staged.
    let mut t = session(P1);
    let env = TradeEnv { ctx: &c, rules: &r, holdings: &w, config: &cfg };
    assert_eq!(t.accept(&env, pool, 0), Err(TradeRefusal::NothingStaged));

    // 2. A stack shrank behind the session's back.
    t.stage(&env, Side::Sell, p, s.potion, 3).expect("stage");
    let before = t.clone();
    w.get_mut(&p).expect("player").apply(RpgCommand::RemoveItem { holder: p, item: id("potion"), count: 3 }).expect("remove");
    let env = TradeEnv { ctx: &c, rules: &r, holdings: &w, config: &cfg };
    assert_eq!(t.accept(&env, pool, 0), Err(TradeRefusal::StockChanged { line: 0 }));
    assert_eq!(t, before);

    // 3. Money, before the deal check: the player can't afford (even with a veto pending).
    let (w, s) = world();
    r.veto = true;
    let env = TradeEnv { ctx: &c, rules: &r, holdings: &w, config: &cfg };
    let mut t = session(P1);
    t.stage(&env, Side::Buy, m, s.gem, 3).expect("stage");
    let before = t.clone();
    assert_eq!(t.accept(&env, pool, 0), Err(TradeRefusal::PlayerCantAfford { have: 100, want: 150 }));
    assert_eq!(t, before);

    // ... the merchant can't afford; unlimited never refuses.
    let mut t = session(P1);
    t.stage(&env, Side::Sell, p, s.sword, 1).expect("stage");
    assert_eq!(t.accept(&env, MerchantMoney::Pool(20), 0), Err(TradeRefusal::MerchantCantAfford { have: 20, want: 50 }));

    // 4. The veto (with its confiscation) closes the session.
    let refusal = t.accept(&env, MerchantMoney::Unlimited, 0).expect_err("veto");
    assert!(refusal.closes());
    assert!(matches!(refusal, TradeRefusal::Vetoed(DealVeto { ref effects, .. }) if effects.len() == 1));

    // 5. A failed haggle keeps the session open and counts the attempt.
    r.veto = false;
    r.haggle_ok = false;
    let env = TradeEnv { ctx: &c, rules: &r, holdings: &w, config: &cfg };
    t.set_balance(&cfg, 60).expect("haggle");
    let before = t.clone();
    let refusal = t.accept(&env, pool, 42).expect_err("haggle fails");
    assert_eq!(refusal, TradeRefusal::HaggleFailed { effects: vec![ServiceEffect::Disposition { delta: -1 }] });
    assert!(!refusal.closes());
    assert_eq!(t.attempts, before.attempts + 1);
    assert_eq!(t.lines, before.lines);
    assert_eq!(t.balance, 60);

    // Unlimited money settles a big sale.
    let mut big = session(P1);
    big.stage(&env, Side::Sell, p, s.sword, 1).expect("stage");
    assert_eq!(big.accept(&env, MerchantMoney::Unlimited, 0).map(|st| st.purse_delta), Ok(0));
}

// ---------------------------------------------------------------- settlement

#[test]
fn settlement_is_exact_commands_and_applies() {
    let (mut w, s) = world();
    let r = rules();
    let c = ctx(P1);
    let cfg = config();
    let p = Holder::Player(P1);
    let m = Holder::Actor(MERCHANT);
    let chest = Holder::Actor(CHEST);
    let env = TradeEnv { ctx: &c, rules: &r, holdings: &w, config: &cfg };
    let mut t = session(P1);
    t.stage(&env, Side::Sell, p, s.sword, 1).expect("sword");
    t.stage(&env, Side::Buy, m, s.gem, 2).expect("gem");
    t.stage(&env, Side::Buy, chest, s.bread, 1).expect("bread");
    assert_eq!(t.balance, 50 - 100 - 1);

    let settled = t.accept(&env, MerchantMoney::Pool(500), 0).expect("accept");
    assert_eq!(
        settled,
        Settlement {
            commands: vec![
                RpgCommand::MoveStack { from: p, to: m, stack: s.sword, count: 1 },
                RpgCommand::MoveStack { from: m, to: p, stack: s.gem, count: 2 },
                RpgCommand::AddItem { holder: p, item: id("bread"), count: 1, attrs: Attrs::new() },
                RpgCommand::RemoveItem { holder: p, item: id("gold"), count: 51 },
            ],
            purse_delta: 51,
            effects: Vec::new(),
        }
    );

    let mut services = WorldServices::default();
    services.open(MERCHANT, PurseRule::Pool { base: 500, reset_hours: 24 }, c.now);
    for cmd in settled.commands {
        apply(&mut w, cmd);
    }
    services.apply_delta(MERCHANT, settled.purse_delta);
    assert_eq!(count(&w, p, "gold"), 49);
    assert_eq!(count(&w, p, "gem"), 2);
    assert_eq!(count(&w, p, "sword"), 0);
    assert_eq!(count(&w, p, "bread"), 1);
    assert_eq!(count(&w, chest, "bread"), 1, "restocking stock is copied, not moved");
    assert_eq!(count(&w, m, "sword"), 1);
    assert_eq!(count(&w, m, "gem"), 1);
    assert_eq!(services.gold(MERCHANT), Some(551));

    // Inventory money: the currency also moves on the provider.
    let (w, s) = world();
    let env = TradeEnv { ctx: &c, rules: &r, holdings: &w, config: &cfg };
    let mut t = session(P1);
    t.stage(&env, Side::Sell, p, s.sword, 1).expect("sword");
    let settled = t.accept(&env, MerchantMoney::Inventory, 0).expect("accept");
    assert_eq!(
        settled.commands[1..],
        [
            RpgCommand::RemoveItem { holder: m, item: id("gold"), count: 50 },
            RpgCommand::AddItem { holder: p, item: id("gold"), count: 50, attrs: Attrs::new() },
        ]
    );
    assert_eq!(settled.purse_delta, 0);
}

#[test]
fn haggle_effects_reach_the_settlement() {
    let (w, s) = world();
    let r = rules();
    let c = ctx(P1);
    let cfg = config();
    let env = TradeEnv { ctx: &c, rules: &r, holdings: &w, config: &cfg };
    let mut t = session(P1);
    t.stage(&env, Side::Sell, Holder::Player(P1), s.sword, 1).expect("sword");
    t.set_balance(&cfg, 60).expect("haggle");
    let settled = t.accept(&env, MerchantMoney::Pool(500), 3).expect("accept");
    assert_eq!(settled.purse_delta, -60);
    assert_eq!(settled.effects, vec![ServiceEffect::UseSkill { stat: id("barter"), use_kind: 0, scale: 1.0 }]);
    assert_eq!(t.attempts, 1);
}

// ---------------------------------------------------------------- purses

#[test]
fn purse_restocks_lazily_at_the_boundary() {
    let mut purse = Purse { gold: 10, last_restock: GameTime { day: 1, minute: 600 } };
    assert!(!purse.restock(500, 24, GameTime { day: 2, minute: 599 }), "one minute early");
    assert_eq!(purse.gold, 10);
    assert!(purse.restock(500, 24, GameTime { day: 2, minute: 600 }), "boundary inclusive");
    assert_eq!(purse, Purse { gold: 500, last_restock: GameTime { day: 2, minute: 600 } });

    let rule = PurseRule::Pool { base: 500, reset_hours: 24 };
    let t0 = GameTime { day: 3, minute: 0 };
    let mut services = WorldServices::default();
    assert_eq!(services.open(MERCHANT, rule, t0), Some(500));
    services.apply_delta(MERCHANT, 30);
    assert_eq!(services.open(MERCHANT, rule, GameTime { day: 3, minute: 1439 }), None);
    assert_eq!(services.gold(MERCHANT), Some(530), "paid services raise the purse above base");
    assert_eq!(services.open(MERCHANT, rule, GameTime { day: 4, minute: 0 }), Some(500));

    assert_eq!(services.open(CHEST, PurseRule::Unlimited, t0), None);
    assert_eq!(MerchantMoney::of(PurseRule::Unlimited, &services, CHEST), MerchantMoney::Unlimited);
    assert_eq!(MerchantMoney::of(rule, &services, MERCHANT), MerchantMoney::Pool(500));
}

// ---------------------------------------------------------------- rolls

#[test]
fn rolls_are_seeded_by_provider_session_attempt_tick() {
    let a = service_seed(MERCHANT, SessionId(3), 0, 100);
    assert_eq!(a, service_seed(MERCHANT, SessionId(3), 0, 100));
    assert_ne!(a, service_seed(MERCHANT, SessionId(3), 1, 100), "a second attempt differs");
    assert_ne!(a, service_seed(CHEST, SessionId(3), 0, 100));
    assert_ne!(a, service_seed(MERCHANT, SessionId(4), 0, 100));
    assert_ne!(a, service_seed(MERCHANT, SessionId(3), 0, 101));
    assert_eq!(haggle_roll(MERCHANT, SessionId(3), 0, 100), u8::try_from(a % 100).expect("roll"));
    assert!((0..50).all(|i| haggle_roll(MERCHANT, SessionId(i), 0, 0) < 100));
}

// ---------------------------------------------------------------- list services

#[test]
fn list_purchase_reprices_at_purchase_and_pays_first() {
    let r = rules();
    let c = ctx(P1);
    let train = kind("train");
    let blade = EntryId(id("blade"));
    assert_eq!(r.entries(&c, &train)[0].price, 10);
    let bought = purchase(&r, &c, &train, &blade, 15).expect("buy");
    assert_eq!(bought.price, 10);
    assert_eq!(bought.effects[0], ServiceEffect::Pay { amount: 10 });

    // The row was listed at 10; the price rose before the purchase.
    r.train_price.set(20);
    assert_eq!(purchase(&r, &c, &train, &blade, 15), Err(EntryRefusal::TooExpensive));
    assert_eq!(purchase(&r, &c, &train, &EntryId(id("axe")), 99), Err(EntryRefusal::Unknown));

    let gold = Currency { item: id("gold") };
    assert_eq!(
        payment_command(P1, &gold, 10),
        Some(RpgCommand::RemoveItem { holder: Holder::Player(P1), item: id("gold"), count: 10 })
    );
    assert_eq!(payment_command(P1, &gold, 0), None);
}

// ---------------------------------------------------------------- commands

#[test]
fn move_stack_merges_at_the_target_and_is_atomic() {
    let (mut w, s) = world();
    let p = Holder::Player(P1);
    let m = Holder::Actor(MERCHANT);
    let events = apply(&mut w, RpgCommand::MoveStack { from: p, to: m, stack: s.potion, count: 2 });
    assert_eq!(
        events,
        vec![
            RpgEvent::ItemsChanged { holder: p, item: id("potion"), delta: -2 },
            RpgEvent::ItemsChanged { holder: m, item: id("potion"), delta: 2 },
        ]
    );
    let merchant = &w[&m].loadout;
    assert_eq!(merchant.inventory().get(s.m_potion).map(|st| st.count), Some(12));
    assert_eq!(merchant.inventory().stacks().iter().filter(|st| st.item == id("potion")).count(), 1);
    assert_eq!(count(&w, p, "potion"), 3);

    let mut source = w[&p].clone();
    let mut target = w[&m].clone();
    let before = (source.clone(), target.clone());
    let too_many = RpgCommand::MoveStack { from: p, to: m, stack: s.potion, count: 4 };
    assert!(matches!(
        CharacterRpg::apply_move(&mut source, &mut target, too_many.clone()),
        Err(RpgError::Inventory(InventoryError::NotEnough { .. }))
    ));
    assert_eq!((source.clone(), target.clone()), before);
    let same = RpgCommand::MoveStack { from: p, to: p, stack: s.potion, count: 1 };
    assert_eq!(CharacterRpg::apply_move(&mut source, &mut target, same), Err(RpgError::SameHolder));
    assert_eq!(source.apply(too_many), Err(RpgError::TwoHolders));
    assert_eq!(WorldRpg::default().apply(RpgCommand::MoveStack { from: p, to: m, stack: s.potion, count: 1 }), Err(RpgError::NotAWorldCommand));
}

#[test]
fn set_stack_attr_clearing_condition_remerges() {
    let p = Holder::Player(P1);
    let mut c = CharacterRpg::default();
    let plain = c.loadout.add(id("sword"), 1, Attrs::new()).expect("plain");
    let worn = c.loadout.add(id("sword"), 1, Attrs::new().with("condition", AttrValue::Int(40))).expect("worn");
    assert_ne!(plain, worn);

    let events = c
        .apply(RpgCommand::SetStackAttr { holder: p, stack: worn, key: id("condition"), value: Some(AttrValue::Int(60)) })
        .expect("set");
    assert_eq!(events, vec![RpgEvent::StackChanged { holder: p, stack: worn }]);
    assert_eq!(c.loadout.inventory().stacks().len(), 2);

    let events = c.apply(RpgCommand::SetStackAttr { holder: p, stack: worn, key: id("condition"), value: None }).expect("clear");
    assert_eq!(
        events,
        vec![RpgEvent::StacksMerged { holder: p, from: worn, into: plain }, RpgEvent::StackChanged { holder: p, stack: plain }]
    );
    assert_eq!(c.loadout.inventory().stacks().len(), 1);
    assert_eq!(c.loadout.inventory().get(plain).map(|st| st.count), Some(2));
    assert_eq!(
        c.apply(RpgCommand::SetStackAttr { holder: p, stack: StackId(99), key: id("condition"), value: None }),
        Err(RpgError::Inventory(InventoryError::NoStack(StackId(99))))
    );
}

// ---------------------------------------------------------------- saves

#[test]
fn world_services_round_trip_golden_and_drop_missing() {
    assert_eq!(SERVICES_SECTION, "sf/services");
    assert_eq!(SERVICES_VERSION, 1);
    let mut services = WorldServices::default();
    services.purses.insert(MERCHANT, Purse { gold: 500, last_restock: GameTime { day: 2, minute: 30 } });
    round_trip(&services);
    // Shape golden: map length, key (varint u128), gold (zigzag varint), day, minute.
    assert_eq!(postcard::to_allocvec(&services).expect("encode"), vec![0x01, 0x07, 0xE8, 0x07, 0x02, 0x1E]);

    services.purses.insert(CHEST, Purse { gold: 1, last_restock: GameTime::default() });
    assert_eq!(services.drop_missing(|k| k == MERCHANT), vec![CHEST]);
    assert_eq!(services.purses.keys().copied().collect::<Vec<_>>(), vec![MERCHANT]);
}

// ---------------------------------------------------------------- co-op

#[test]
fn coop_busy_provider_and_per_player_quotes() {
    let mut r = rules();
    let mut claims = Conversations::default();
    claims.claim(MERCHANT, P1).expect("claim");
    let mut services = WorldServices::default();
    let mut sessions = ServiceSessions::new();
    let busy = open(&r, &ctx(p2()), &kind("trade"), &claims, &mut services, &mut sessions);
    assert_eq!(busy, Err(OpenError::Busy { provider: MERCHANT, holder: P1 }));
    let opened = open(&r, &ctx(P1), &kind("trade"), &claims, &mut services, &mut sessions).expect("p1 opens");
    // Even without a dialogue claim, an open trade session keeps the provider busy.
    let busy = open(&r, &ctx(p2()), &kind("trade"), &Conversations::default(), &mut services, &mut sessions);
    assert_eq!(busy, Err(OpenError::Busy { provider: MERCHANT, holder: P1 }));
    let id1 = opened.session.expect("session");
    assert!(sessions.get_mut(id1, p2()).is_none(), "a guest can't drive another player's session");
    sessions.close_player(P1);
    assert!(sessions.is_empty());

    r.sell_permille.insert(P1, 500);
    r.sell_permille.insert(p2(), 800);
    let (mut w, _) = world();
    let s1 = add(&mut w, Holder::Player(P1), "sword", 1);
    let s2 = add(&mut w, Holder::Player(p2()), "sword", 1);
    let cfg = config();
    let (c1, c2) = (ctx(P1), ctx(p2()));
    let mut t1 = session(P1);
    let mut t2 = TradeSession::new(SessionId(1), p2(), MERCHANT, kind("trade"));
    t1.stage(&TradeEnv { ctx: &c1, rules: &r, holdings: &w, config: &cfg }, Side::Sell, Holder::Player(P1), s1, 1)
        .expect("p1");
    t2.stage(&TradeEnv { ctx: &c2, rules: &r, holdings: &w, config: &cfg }, Side::Sell, Holder::Player(p2()), s2, 1)
        .expect("p2");
    assert_eq!((t1.merchant_offer, t2.merchant_offer), (50, 80));
}

// ---------------------------------------------------------------- measurement

#[test]
#[ignore = "measurement: pane quoting for a 1,000-stack player and a 200-stack merchant (budget <= 1 ms/request)"]
fn measure_pane_quoting() {
    let mut w = World::new();
    let p = Holder::Player(P1);
    let m = Holder::Actor(MERCHANT);
    for i in 0..1000 {
        add(&mut w, p, &format!("item{i}"), 1 + i % 7);
    }
    for i in 0..200 {
        add(&mut w, m, &format!("ware{i}"), 1 + i % 5);
    }
    let r = rules();
    let c = ctx(P1);
    let cfg = config();
    let env = TradeEnv { ctx: &c, rules: &r, holdings: &w, config: &cfg };
    let start = std::time::Instant::now();
    let runs = 100;
    let mut priced = 0;
    for _ in 0..runs {
        priced += pane_prices(&env, Side::Sell, p).len() + pane_prices(&env, Side::Buy, m).len();
    }
    let open = start.elapsed() / runs;
    let stack = w[&p].loadout.inventory().stacks()[500].id;
    let start = std::time::Instant::now();
    let mut t = session(P1);
    for _ in 0..runs {
        t.stage(&env, Side::Sell, p, stack, 1).expect("stage");
        t.unstage(&env, 0, 1).expect("unstage");
    }
    let request = start.elapsed() / (2 * runs);
    println!("pane quoting (1,200 stacks): {open:?} per open; stage/unstage: {request:?} per request; {priced} prices");
}
