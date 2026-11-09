//! Barter rules (services-adapter.md "Tests": flags, offers, buys/sells, restocking, pool),
//! on synthetic records. Prices use equal party terms, so the buy term is exactly 1.0 and the
//! sell term exactly 0.5 in `f32` (`0.01 * 100.0` and `0.01 * 50.0` round to them).
#![allow(clippy::many_single_char_names, reason = "test fixtures: db, g, m, p, c, n")]

use std::collections::BTreeMap;

use mw_stats::MwGmst;
use sf_records::{RecordDb, RecordId};
use sf_rpg::service::{
    Currency, DealTotals, Holdings, LabelRef, MerchantMoney, PurseRule, QuoteLine, ServiceCtx, ServiceEffect,
    ServiceKind, ServiceRules, ServiceShape, SessionId, Side, TradeConfig, TradeEnv, TradeSession, WorldServices,
};
use sf_rpg::{
    ActorKey, AttrValue, Attrs, CharacterRpg, ConversationCtx, GameTime, Holder, ItemStack, Loadout, PlayerId,
    RpgCommand, StackId,
};
use smallvec::SmallVec;
use tes3::esp::{
    AiData, Alchemy, Armor, Class, ClassData, Clothing, Creature, GameSetting, GameSettingValue, MiscItem,
    MiscItemFlags, Npc, NpcFlags, ServiceFlags, TravelDestination, Weapon, WeaponType,
};

use crate::barter::{
    ATTR_HEALTH, BoundItems, MwService, MwServiceRules, MwTradeHost, SERVICE_REFUSAL_TOPIC, is_restocking,
    provider_facts, record_entry_stack,
};
use crate::equip::{MwEquipKind, MwEquipRules};
use crate::services::{BarterTerms, PartyTerms};

const P1: PlayerId = PlayerId::HOST;
const MERCHANT: ActorKey = ActorKey(7);
const CHEST: ActorKey = ActorKey(99);

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn even_terms(creature: bool) -> BarterTerms {
    let side = PartyTerms { mercantile: 50.0, luck: 50.0, personality: 50.0, fatigue_term: 1.0 };
    BarterTerms { merchant_is_creature: creature, disposition: 50, pc: side, npc: side }
}

/// The host fixture: inventories, terms, owned stock and refusals keyed by filter choice.
#[derive(Default)]
struct Host {
    world: BTreeMap<Holder, CharacterRpg>,
    terms: Option<BarterTerms>,
    owned: Vec<Holder>,
    refusals: Vec<(Option<i32>, RecordId)>,
}

impl Holdings for Host {
    fn loadout(&self, holder: Holder) -> Option<&Loadout> {
        self.world.get(&holder).map(|c| &c.loadout)
    }
}

impl MwTradeHost for Host {
    fn barter_terms(&self, _player: PlayerId, _provider: ActorKey) -> Option<BarterTerms> {
        self.terms
    }

    fn owned_stock(&self, _provider: ActorKey) -> SmallVec<[Holder; 4]> {
        self.owned.iter().copied().collect()
    }

    fn service_refusal(&self, ctx: &ConversationCtx) -> Option<RecordId> {
        self.refusals.iter().find(|(choice, _)| *choice == ctx.choice).map(|(_, info)| info.clone())
    }
}

fn npc(name: &str, services: ServiceFlags, gold: u32) -> Npc {
    Npc {
        id: name.into(),
        class: "trader_class".into(),
        ai_data: AiData { services, ..Default::default() },
        data: tes3::esp::NpcData { gold, ..Default::default() },
        ..Default::default()
    }
}

fn weapon(value: u32, health: u16, weapon_type: WeaponType, enchanting: &str) -> Weapon {
    let mut w = Weapon { enchanting: enchanting.into(), ..Default::default() };
    w.data.value = value;
    w.data.health = health;
    w.data.weapon_type = weapon_type;
    w
}

fn misc(value: u32, key: bool) -> MiscItem {
    let mut m = MiscItem::default();
    m.data.value = value;
    if key {
        m.data.flags = MiscItemFlags::KEY;
    }
    m
}

/// Providers and items. Class `trader_class` trains; `autocalc` is an autocalc NPC whose own
/// `AIDT` says weapons.
fn db() -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<Npc>("NPC_").unwrap();
    db.register::<Creature>("CREA").unwrap();
    db.register::<Class>("CLAS").unwrap();
    db.register::<Weapon>("WEAP").unwrap();
    db.register::<Armor>("ARMO").unwrap();
    db.register::<Clothing>("CLOT").unwrap();
    db.register::<MiscItem>("MISC").unwrap();
    db.register::<Alchemy>("ALCH").unwrap();
    db.register::<GameSetting>("GMST").unwrap();
    let src = db.add_source("test", &[]);

    let class = Class {
        id: "trader_class".into(),
        data: ClassData { services: ServiceFlags::OFFERS_TRAINING, ..Default::default() },
        ..Default::default()
    };
    db.define(src, id("trader_class"), class).unwrap();

    let mut autocalc = npc("autocalc", ServiceFlags::BARTERS_WEAPONS, 0);
    autocalc.npc_flags = NpcFlags::AUTO_CALCULATE;
    db.define(src, id("autocalc"), autocalc).unwrap();
    db.define(src, id("smith"), npc("smith", ServiceFlags::BARTERS_WEAPONS, 500)).unwrap();
    db.define(src, id("trader"), npc("trader", ServiceFlags::BARTERS_WEAPONS | ServiceFlags::BARTERS_MISC_ITEMS, 500))
        .unwrap();
    db.define(src, id("alchemist"), npc("alchemist", ServiceFlags::BARTERS_ALCHEMY, 100)).unwrap();
    db.define(src, id("enchanter"), npc("enchanter", ServiceFlags::BARTERS_ENCHANTED_ITEMS, 100)).unwrap();
    let mut everything = npc("everything", ServiceFlags::all(), 100);
    everything.travel_destinations.push(TravelDestination { cell: "Balmora".into(), ..Default::default() });
    db.define(src, id("everything"), everything).unwrap();

    let mut creeper = Creature { id: "scamp_creeper".into(), ..Default::default() };
    creeper.ai_data.services = ServiceFlags::BARTERS_MISC_ITEMS;
    creeper.data.gold = 300;
    db.define(src, id("scamp_creeper"), creeper).unwrap();

    db.define(src, id("iron dagger"), weapon(100, 100, WeaponType::ShortBladeOneHand, "")).unwrap();
    db.define(src, id("iron throwing knife"), weapon(10, 100, WeaponType::MarksmanThrown, "")).unwrap();
    db.define(src, id("bound_dagger"), weapon(0, 100, WeaponType::ShortBladeOneHand, "")).unwrap();
    db.define(src, id("iron cuirass"), Armor::default()).unwrap();
    db.define(src, id("plain ring"), Clothing::default()).unwrap();
    db.define(src, id("magic ring"), Clothing { enchanting: "shield_en".into(), ..Default::default() }).unwrap();
    db.define(src, id("bucket"), misc(4, false)).unwrap();
    db.define(src, id("door key"), misc(0, true)).unwrap();
    db.define(src, id("gold_001"), misc(1, false)).unwrap();
    db.define(src, id("potion"), Alchemy::default()).unwrap();
    db.define(
        src,
        id("sMagicBoundDaggerID"),
        GameSetting { id: "sMagicBoundDaggerID".into(), value: GameSettingValue::String("bound_dagger".into()), ..Default::default() },
    )
    .unwrap();
    db
}

fn ctx(provider_base: &str) -> ServiceCtx {
    ServiceCtx { player: P1, provider: MERCHANT, provider_base: id(provider_base), now: GameTime::default(), tick: 1 }
}

fn stack(item: &str) -> ItemStack {
    ItemStack { id: StackId(0), item: id(item), count: 1, attrs: Attrs::new() }
}

fn kinds(rules: &MwServiceRules<'_, Host>, base: &str) -> Vec<String> {
    rules.offers(&ctx(base)).into_iter().map(|o| o.kind.0.as_str().to_owned()).collect()
}

#[test]
fn autocalc_npc_takes_class_services_creature_its_aidt() {
    let (db, g, host) = (db(), MwGmst::vanilla(), Host::default());
    let bound = BoundItems::default();
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    assert_eq!(kinds(&rules, "autocalc"), ["mw.training"], "the class's bits, not the AIDT's weapons");
    assert_eq!(kinds(&rules, "smith"), ["mw.barter"]);
    assert_eq!(kinds(&rules, "scamp_creeper"), ["mw.barter"]);
    let creeper = provider_facts(&db, &id("scamp_creeper")).expect("creature");
    assert!(creeper.creature);
    assert_eq!(creeper.gold, 300);
    assert_eq!(rules.purse_base(&ctx("scamp_creeper")), PurseRule::Pool { base: 300, reset_hours: 24 });
    assert!(kinds(&rules, "nobody").is_empty());
}

#[test]
fn offers_in_openmw_button_order_with_shapes_and_labels() {
    let (db, g, host) = (db(), MwGmst::vanilla(), Host::default());
    let bound = BoundItems::default();
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    let offers = rules.offers(&ctx("everything"));
    let got: Vec<_> = offers.iter().map(|o| (o.kind.clone(), o.shape.clone(), o.label.clone())).collect();
    let want: Vec<_> = MwService::BUTTON_ORDER
        .into_iter()
        .map(|s| (s.kind(), s.shape(), LabelRef(id(s.label_gmst()))))
        .collect();
    assert_eq!(got, want);
    assert_eq!(
        offers.iter().map(|o| o.kind.0.as_str()).collect::<Vec<_>>(),
        ["mw.barter", "mw.spells", "mw.travel", "mw.spellmaking", "mw.enchanting", "mw.training", "mw.repair"]
    );
    assert_eq!(offers[0].shape, ServiceShape::Trade);
    assert_eq!(offers[3].shape, ServiceShape::Custom(id("mw.spellmaking")));
    assert_eq!(offers[0].label, LabelRef(id("sbarter")));
    assert!(offers.iter().all(|o| o.provider == MERCHANT));
}

#[test]
fn magic_items_bit_alone_buys_enchanted_but_shows_no_barter_button() {
    let (db, g, host) = (db(), MwGmst::vanilla(), Host::default());
    let bound = BoundItems::default();
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    assert!(kinds(&rules, "enchanter").is_empty(), "AllItems excludes MagicItems");
    let c = ctx("enchanter");
    assert!(rules.buys(&c, &stack("magic ring")));
    assert!(!rules.buys(&c, &stack("plain ring")));
}

#[test]
fn buys_filters_by_item_class_never_keys_or_gold() {
    let (db, g, host) = (db(), MwGmst::vanilla(), Host::default());
    let bound = BoundItems::from_records(&db);
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    let trader = ctx("trader");
    assert!(rules.buys(&trader, &stack("iron dagger")));
    assert!(rules.buys(&trader, &stack("bucket")));
    assert!(!rules.buys(&trader, &stack("door key")), "misc Key flag");
    assert!(!rules.buys(&trader, &stack("gold_001")), "isGold");
    assert!(!rules.buys(&trader, &stack("iron cuirass")));
    assert!(!rules.buys(&trader, &stack("potion")));
    assert!(!rules.buys(&trader, &stack("bound_dagger")), "bound item");
    assert!(!rules.buys(&trader, &stack("no such record")));
    assert!(rules.buys(&ctx("alchemist"), &stack("potion")));
    assert!(!rules.buys(&ctx("alchemist"), &stack("iron dagger")));
}

#[test]
fn sells_hides_equipped_bound_and_untraded_stock() {
    let db = db();
    let g = MwGmst::vanilla();
    let bound = BoundItems::from_records(&db);
    assert!(bound.contains(&id("bound_dagger")));
    let mut host = Host::default();
    let m = Holder::Actor(MERCHANT);
    let loadout = &mut host.world.entry(m).or_default().loadout;
    let worn = loadout.add(id("iron dagger"), 1, Attrs::new()).expect("add");
    let mut equip = MwEquipRules::default();
    equip.insert(id("iron dagger"), MwEquipKind::Weapon(WeaponType::ShortBladeOneHand));
    equip.equip(loadout, worn).expect("equip");
    let spare = loadout.add(id("iron dagger"), 2, Attrs::new().with("mw.spare", AttrValue::Int(1))).expect("add");
    let summoned = loadout.add(id("bound_dagger"), 1, Attrs::new()).expect("add");
    let cuirass = loadout.add(id("iron cuirass"), 1, Attrs::new()).expect("add");
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    let c = ctx("trader");
    let inv = host.world[&m].loadout.inventory();
    let sells = |s: StackId| rules.sells(&c, m, inv.get(s).expect("stack"));
    assert!(!sells(worn), "equipped");
    assert!(sells(spare));
    assert!(!sells(summoned), "bound");
    assert!(!sells(cuirass), "not a class the trader buys");
}

#[test]
fn stock_sources_merchant_first_then_owned() {
    let (db, g) = (db(), MwGmst::vanilla());
    let bound = BoundItems::default();
    let host = Host { owned: vec![Holder::Actor(CHEST), Holder::Actor(MERCHANT)], ..Default::default() };
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    let sources: Vec<Holder> = rules.stock_sources(&ctx("trader")).into_iter().collect();
    assert_eq!(sources, [Holder::Actor(MERCHANT), Holder::Actor(CHEST)]);
}

fn quote(rules: &MwServiceRules<'_, Host>, base: &str, side: Side, item: &str, attrs: &Attrs, count: u32) -> i64 {
    let item = id(item);
    let line = QuoteLine { side, source: Holder::Actor(MERCHANT), item: &item, attrs, count };
    rules.quote(&ctx(base), &line)
}

#[test]
fn quote_uses_condition_count_and_terms() {
    let (db, g) = (db(), MwGmst::vanilla());
    let bound = BoundItems::default();
    let host = Host { terms: Some(even_terms(false)), ..Default::default() };
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    let half = Attrs::new().with(ATTR_HEALTH, AttrValue::Int(50));
    let none = Attrs::new();
    // int(100 x 50/100 x 1) = 50; buy term 1.0, above the 75% cap (37).
    assert_eq!(quote(&rules, "trader", Side::Buy, "iron dagger", &half, 1), 50);
    // 4 x 5 = 20 at term 0.5 = 10, below the cap (15).
    assert_eq!(quote(&rules, "trader", Side::Sell, "bucket", &none, 5), 10);
    // A thrown weapon has no item health: the attr is ignored.
    assert_eq!(quote(&rules, "trader", Side::Buy, "iron throwing knife", &half, 3), 30);
    assert_eq!(quote(&rules, "trader", Side::Buy, "no such record", &none, 1), 0);

    let creature = Host { terms: Some(even_terms(true)), ..Default::default() };
    let rules = MwServiceRules::new(&db, &g, &bound, &creature);
    assert_eq!(quote(&rules, "scamp_creeper", Side::Sell, "bucket", &none, 5), 20, "creatures pay the base");
    let no_terms = Host::default();
    let rules = MwServiceRules::new(&db, &g, &bound, &no_terms);
    assert_eq!(quote(&rules, "trader", Side::Sell, "bucket", &none, 5), 20, "no terms: base value");
}

fn deal(merchant_offer: i64, balance: i64) -> DealTotals {
    DealTotals { merchant_offer, balance, buy_total: 0, sell_total: 0 }
}

#[test]
fn haggle_maps_the_roll_and_returns_disposition_effects() {
    let (db, g) = (db(), MwGmst::vanilla());
    let bound = BoundItems::default();
    let host = Host { terms: Some(even_terms(false)), ..Default::default() };
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    let c = ctx("trader");
    let plus = vec![ServiceEffect::Disposition { delta: 1 }];
    let minus = vec![ServiceEffect::Disposition { delta: -1 }];
    let asking_less = rules.haggle(&c, &deal(15, 10), 99);
    assert!(asking_less.accepted);
    assert_eq!(asking_less.effects, plus);
    // Selling for 110 against 100: d = 100 x 10 / 110 = 9; x = -4 x 9 + 50 + int(65 - 65) = 14.
    // `roll + 1` is OpenMW's rollDice(100) + 1, so roll 13 is 14 (accepted), roll 14 is 15.
    let won = rules.haggle(&c, &deal(100, 110), 13);
    assert!(won.accepted);
    assert_eq!(won.effects, plus);
    let lost = rules.haggle(&c, &deal(100, 110), 14);
    assert!(!lost.accepted);
    assert_eq!(lost.effects, minus);
    let buying_paid = rules.haggle(&c, &deal(-50, 10), 0);
    assert!(!buying_paid.accepted, "a buying player asking to be paid");
    assert_eq!(buying_paid.effects, minus);

    let creature = Host { terms: Some(even_terms(true)), ..Default::default() };
    let rules = MwServiceRules::new(&db, &g, &bound, &creature);
    let ok = rules.haggle(&ctx("scamp_creeper"), &deal(15, 10), 0);
    assert!(ok.accepted);
    assert!(ok.effects.is_empty(), "no disposition for creatures");
}

#[test]
fn refusal_checks_any_first_then_the_service_choice() {
    let (db, g) = (db(), MwGmst::vanilla());
    let bound = BoundItems::default();
    let topic = id(SERVICE_REFUSAL_TOPIC);
    let barter = MwService::Barter.kind();
    let training = MwService::Training.kind();

    let host = Host { refusals: vec![(Some(1), id("refuse_barter"))], ..Default::default() };
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    let c = ctx("everything");
    assert_eq!(rules.refusal(&c, &barter), Some((topic.clone(), id("refuse_barter"))));
    assert_eq!(rules.refusal(&c, &training), None, "Training is choice 4");
    assert_eq!(rules.refusal(&c, &ServiceKind(id("other.service"))), None);

    let host = Host {
        refusals: vec![(Some(1), id("refuse_barter")), (None, id("refuse_all"))],
        ..Default::default()
    };
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    assert_eq!(rules.refusal(&c, &barter), Some((topic.clone(), id("refuse_all"))), "the general check wins");
    assert_eq!(rules.refusal(&c, &training), Some((topic, id("refuse_all"))));
    assert_eq!(
        MwService::BUTTON_ORDER.map(MwService::choice),
        [1, 3, 5, 6, 7, 4, 2],
        "ServiceType: Barter 1, Repair 2, Spells 3, Training 4, Travel 5, Spellmaking 6, Enchanting 7"
    );
}

#[test]
fn record_entries_mark_restocking_stacks() {
    let (n, attrs) = record_entry_stack(-5);
    assert_eq!(n, 5);
    assert!(is_restocking(&attrs));
    let (n, attrs) = record_entry_stack(3);
    assert_eq!(n, 3);
    assert!(!is_restocking(&attrs));
}

#[test]
fn buying_from_a_restocking_stack_copies_it() {
    let (db, g) = (db(), MwGmst::vanilla());
    let bound = BoundItems::default();
    let mut host = Host { terms: Some(even_terms(false)), ..Default::default() };
    let (n, attrs) = record_entry_stack(-5);
    let m = Holder::Actor(MERCHANT);
    let p = Holder::Player(P1);
    let buckets = host.world.entry(m).or_default().loadout.add(id("bucket"), n, attrs).expect("add");
    host.world.entry(p).or_default().loadout.add(id("gold_001"), 100, Attrs::new()).expect("add");
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    let c = ctx("trader");
    let config = TradeConfig { currency: Currency { item: id("gold_001") }, haggle: false };
    let env = TradeEnv { ctx: &c, rules: &rules, holdings: &host, config: &config };
    let mut session = TradeSession::new(SessionId(1), P1, MERCHANT, MwService::Barter.kind());
    assert_eq!(session.stage(&env, Side::Buy, m, buckets, 5), Ok(5));
    assert_eq!(session.merchant_offer, -20, "4 x 5 at buy term 1.0");
    let settled = session.accept(&env, MerchantMoney::Pool(500), 0).expect("accept");
    assert!(
        settled.commands.iter().any(
            |cmd| matches!(cmd, RpgCommand::AddItem { holder, item, count: 5, .. } if *holder == p && *item == id("bucket"))
        ),
        "the player gets a copy: {:?}",
        settled.commands
    );
    assert!(
        !settled.commands.iter().any(|cmd| matches!(cmd, RpgCommand::MoveStack { .. })),
        "nothing leaves the merchant's restocking stack"
    );
    assert_eq!(settled.purse_delta, 20);
    assert!(
        settled.commands.iter().all(|cmd| match cmd {
            RpgCommand::AddItem { attrs, .. } => !is_restocking(attrs),
            _ => true,
        }),
        "the bought copy drops the restock marker: {:?}",
        settled.commands
    );
    // A deal at the merchant's own offer still gains iBarterSuccessDisposition (1), as in OpenMW.
    assert_eq!(settled.effects, vec![ServiceEffect::Disposition { delta: 1 }]);
}

#[test]
fn pool_resets_at_exactly_the_delay_and_payments_raise_it() {
    let (db, g, host) = (db(), MwGmst::vanilla(), Host::default());
    let bound = BoundItems::default();
    let rules = MwServiceRules::new(&db, &g, &bound, &host);
    let rule = rules.purse_base(&ctx("smith"));
    assert_eq!(rule, PurseRule::Pool { base: 500, reset_hours: 24 });
    let mut world = WorldServices::default();
    let t = |day, minute| GameTime { day, minute };
    assert_eq!(world.open(MERCHANT, rule, t(1, 0)), Some(500));
    world.apply_delta(MERCHANT, -200);
    assert_eq!(world.open(MERCHANT, rule, t(1, 1439)), None, "23:59 later: no reset");
    assert_eq!(world.gold(MERCHANT), Some(300));
    assert_eq!(world.open(MERCHANT, rule, t(2, 0)), Some(500), "exactly 24 h later");
    world.apply_delta(MERCHANT, 50);
    assert_eq!(world.gold(MERCHANT), Some(550), "a training payment lifts the pool above its base");
}
