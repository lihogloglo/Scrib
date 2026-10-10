use super::*;

fn p(s: &str) -> VfsPath {
    VfsPath::new(s).unwrap()
}

#[test]
fn paths_normalize() {
    assert_eq!(p("Meshes\\X\\Foo.NIF").as_str(), "meshes/x/foo.nif");
    assert_eq!(p("/textures//./a.dds/").as_str(), "textures/a.dds");
    assert_eq!(VfsPath::new("a/../b"), Err(PathError::ParentSegment("a/../b".into())));
    assert_eq!(VfsPath::new("//"), Err(PathError::Empty));
}

#[test]
fn path_parts() {
    let path = p("meshes/x/foo.bar.nif");
    assert_eq!(path.extension(), Some("nif"));
    assert_eq!(path.file_name(), "foo.bar.nif");
    assert_eq!(path.parent(), Some("meshes/x"));
    assert_eq!(path.with_extension("GLB").as_str(), "meshes/x/foo.bar.glb");
    assert_eq!(p(".hidden").extension(), None);
    assert_eq!(p("noext").with_extension("a").as_str(), "noext.a");
    assert_eq!(p("top.txt").parent(), None);
}

fn layered() -> Vfs {
    Vfs::builder()
        .mount(MemSource::new("base").with("meshes/a.nif", "base-a").with("meshes/b.nif", "base-b").with("readme.txt", "r"))
        .mount(MemSource::new("mod").with("Meshes\\A.NIF", "mod-a").with("textures/t.dds", "t"))
        .build()
}

#[test]
fn last_mounted_source_wins() {
    let vfs = layered();
    assert_eq!(vfs.len(), 4);
    assert_eq!(vfs.read(&p("meshes/a.nif")).unwrap(), b"mod-a");
    assert_eq!(vfs.read(&p("meshes/b.nif")).unwrap(), b"base-b");
    assert_eq!(vfs.providers(&p("meshes/a.nif")), [SourceIndex(0), SourceIndex(1)]);
    assert_eq!(vfs.source(vfs.resolve(&p("meshes/b.nif")).unwrap()).label(), "base");
    assert_eq!(vfs.read(&p("missing")).unwrap_err().kind(), io::ErrorKind::NotFound);
}

#[test]
fn stats_count_files_and_wins() {
    let stats = layered().stats();
    assert_eq!((stats[0].files, stats[0].winning), (3, 2));
    assert_eq!((stats[1].files, stats[1].winning), (2, 2));
}

#[test]
fn read_dir_lists_immediate_children() {
    let vfs = Vfs::builder()
        .mount(
            MemSource::new("s")
                .with("meshes/a.nif", "")
                .with("meshes/x/b.nif", "")
                .with("meshes/x/c.nif", "")
                .with("meshes-extra/d.nif", "")
                .with("top.txt", ""),
        )
        .build();
    assert_eq!(
        vfs.read_dir("Meshes"),
        [DirEntry::File(p("meshes/a.nif")), DirEntry::Dir("x".into())]
    );
    assert_eq!(
        vfs.read_dir(""),
        [DirEntry::Dir("meshes".into()), DirEntry::Dir("meshes-extra".into()), DirEntry::File(p("top.txt"))]
    );
    assert!(vfs.is_dir("meshes/x"));
    assert!(vfs.is_dir(""));
    assert!(!vfs.is_dir("meshes/a.nif"));
    assert!(!vfs.is_dir("nope"));
    assert!(vfs.read_dir("../etc").is_empty());
}

#[test]
fn modern_formats_take_the_slot_when_not_outranked() {
    let vfs = Vfs::builder()
        .mount(MemSource::new("base").with("meshes/a.nif", "").with("meshes/b.nif", "").with("meshes/c.nif", ""))
        .mount(MemSource::new("hd").with("meshes/a.glb", "").with("meshes/b.glb", "").with("meshes/only.glb", ""))
        .mount(MemSource::new("later").with("meshes/b.nif", ""))
        .build();
    let alts = ["glb", "gltf"];
    // Higher-priority glb replaces the base nif.
    assert_eq!(vfs.resolve_preferring(&p("meshes/a.nif"), &alts), Some((p("meshes/a.glb"), SourceIndex(1))));
    // A later nif beats an earlier glb.
    assert_eq!(vfs.resolve_preferring(&p("meshes/b.nif"), &alts), Some((p("meshes/b.nif"), SourceIndex(2))));
    // No alternate: the original.
    assert_eq!(vfs.resolve_preferring(&p("meshes/c.nif"), &alts), Some((p("meshes/c.nif"), SourceIndex(0))));
    // Only an alternate exists.
    assert_eq!(vfs.resolve_preferring(&p("meshes/only.nif"), &alts), Some((p("meshes/only.glb"), SourceIndex(1))));
    assert_eq!(vfs.resolve_preferring(&p("meshes/none.nif"), &alts), None);
}

#[test]
fn alternate_wins_ties_within_one_source() {
    let vfs = Vfs::builder()
        .mount(MemSource::new("s").with("a.nif", "").with("a.gltf", "").with("a.glb", ""))
        .build();
    assert_eq!(vfs.resolve_preferring(&p("a.nif"), &["glb", "gltf"]), Some((p("a.glb"), SourceIndex(0))));
}

#[test]
fn dir_source_indexes_case_insensitively() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("Meshes/Sub")).unwrap();
    std::fs::write(dir.path().join("Meshes/Sub/Rock.NIF"), "rock").unwrap();
    std::fs::write(dir.path().join("Readme.txt"), "hi").unwrap();

    let source = DirSource::scan("loose", dir.path()).unwrap();
    assert_eq!(source.real_path(&p("meshes/sub/rock.nif")), Some(dir.path().join("Meshes/Sub/Rock.NIF")));
    let vfs = Vfs::builder().mount(source).build();
    assert_eq!(vfs.read(&p("MESHES/sub/rock.nif")).unwrap(), b"rock");
    assert_eq!(vfs.len(), 2);
}

#[test]
fn vfs_is_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<Vfs>();
}
