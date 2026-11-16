//! The blit shader without a GPU (CONTRIBUTING "Tests and visual checks"): `texview.wesl` linked by
//! `wesl` with each sample kind's def, as Bevy's shader cache does, and validated by naga; plus
//! its uniform against `crate::params`.

use std::borrow::Cow;

use wesl::syntax::{ModulePath, PathOrigin};

use crate::SampleKind;

fn compile(defs: &[&str]) -> Result<naga::Module, String> {
    let path = ModulePath { origin: PathOrigin::Package("sf_texview_bevy".into()), components: vec!["texview".into()] };
    let mut resolver = wesl::resolver::VirtualResolver::new();
    resolver.add_module(path.clone(), Cow::Borrowed(include_str!("texview.wesl")));
    let mut options = wesl::CompileOptions { imports: true, condcomp: true, visibility: false, strip: true, ..Default::default() };
    for d in defs {
        options.features.flags.insert((*d).to_string(), true.into());
    }
    let wgsl = wesl::compile(&path, &options, &resolver)
        .map_err(|e| format!("{defs:?}: link: {}", e.diagnostic().render_plain()))?
        .to_string();
    let module = naga::front::wgsl::parse_str(&wgsl).map_err(|e| format!("{defs:?}: parse: {}", e.emit_to_string(&wgsl)))?;
    naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
        .validate(&module)
        .map_err(|e| format!("{defs:?}: validate: {}", e.emit_to_string(&wgsl)))?;
    Ok(module)
}

/// Each sample kind's variant links and validates, binds the source with the matching scalar
/// type, and keeps the 32-byte uniform `crate::params` writes.
#[test]
fn every_sample_kind_compiles_with_its_texture_type() {
    for (kind, scalar) in [
        (SampleKind::Float, naga::ScalarKind::Float),
        (SampleKind::Uint, naga::ScalarKind::Uint),
        (SampleKind::Sint, naga::ScalarKind::Sint),
    ] {
        let module = compile(&kind.shader_def().into_iter().collect::<Vec<_>>()).unwrap_or_else(|e| panic!("{e}"));
        let source = module.global_variables.iter().find(|(_, g)| g.name.as_deref() == Some("source")).expect("source binding").1;
        match module.types[source.ty].inner {
            naga::TypeInner::Image { class: naga::ImageClass::Sampled { kind: k, multi: false }, .. } => {
                assert_eq!(k, scalar, "{kind:?}");
            }
            ref other => panic!("{kind:?}: source is {other:?}"),
        }
        let params = module.global_variables.iter().find(|(_, g)| g.name.as_deref() == Some("params")).expect("params").1;
        let size = module.types[params.ty].inner.size(module.to_ctx());
        assert_eq!(size, 32, "{kind:?}: uniform size");
        for entry in ["vertex", "fragment"] {
            assert!(module.entry_points.iter().any(|e| e.name == entry), "{kind:?}: {entry}");
        }
    }
}
