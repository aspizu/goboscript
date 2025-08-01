use std::{
    cell::RefCell,
    env,
    fs::File,
    io::{
        BufWriter,
        Seek,
        Write,
    },
    path::PathBuf,
    rc::Rc,
};

use anyhow::{
    anyhow,
    Context,
};
use directories::ProjectDirs;
use fxhash::FxHashMap;

use crate::{
    ast::{
        Project,
        Sprite,
    },
    codegen::sb3::Sb3,
    config::Config,
    diagnostic::{
        Artifact,
        SpriteDiagnostics,
    },
    misc::SmolStr,
    parser,
    standard_library::{
        fetch_standard_library,
        new_standard_library,
        standard_library_from_latest,
        StandardLibrary,
    },
    vfs::{
        RealFS,
        VFS,
    },
    visitor,
};

fn assign_layer_orders(project: &mut Project, config: &Config) {
    let mut layer_order: usize = 1;
    if let Some(layers) = &config.layers {
        for layer in layers {
            if let Some(sprite) = project.sprites.get_mut(&**layer) {
                sprite.layer_order = Some((layer_order.into(), 0..0));
                layer_order += 1;
            }
        }
    }
}

pub fn build(input: Option<PathBuf>, output: Option<PathBuf>) -> anyhow::Result<Artifact> {
    let input = input.unwrap_or_else(|| env::current_dir().unwrap());
    let canonical_input = input.canonicalize()?;
    let project_name = canonical_input.file_name().unwrap().to_str().unwrap();
    let output = output.unwrap_or_else(|| input.join(format!("{project_name}.sb3")));
    let sb3 = Sb3::new(BufWriter::new(File::create(&output)?));
    let fs = Rc::new(RefCell::new(RealFS::new()));
    build_impl(fs, canonical_input, sb3, None)
}

pub fn build_impl<T: Write + Seek>(
    fs: Rc<RefCell<dyn VFS>>,
    input: PathBuf,
    mut sb3: Sb3<T>,
    stdlib: Option<StandardLibrary>,
) -> anyhow::Result<Artifact> {
    let config_path = input.join("goboscript.toml");
    let config_src = fs
        .borrow_mut()
        .read_to_string(&config_path)
        .unwrap_or_default();
    let config: Config = toml::from_str(&config_src)
        .with_context(|| format!("failed to parse {}", config_path.display()))?;
    let stdlib = if let Some(stdlib) = stdlib {
        stdlib
    } else if let Some(std) = &config.std {
        let dirs = ProjectDirs::from("com", "aspizu", "goboscript").unwrap();
        let std = std
            .strip_prefix('v')
            .unwrap_or(std)
            .parse()
            .with_context(|| format!("std version `{}` is not a valid semver version", std))?;
        new_standard_library(std, &dirs.config_dir().join("std"))
    } else {
        let dirs = ProjectDirs::from("com", "aspizu", "goboscript").unwrap();
        standard_library_from_latest(&dirs.config_dir().join("std"))?
    };
    // v0.0.0 means stdlib is from wasm
    if stdlib.version.major != 0 {
        fetch_standard_library(&stdlib)?;
    }
    let stage_path = input.join("stage.gs");
    if !fs.borrow_mut().is_file(&stage_path) {
        return Err(anyhow!("{} not found", stage_path.display()));
    }
    let mut stage_diagnostics = SpriteDiagnostics::new(fs.clone(), stage_path, &stdlib);
    let (stage, parse_diagnostics) = parser::parse(&stage_diagnostics.translation_unit);
    stage_diagnostics.diagnostics.extend(parse_diagnostics);
    let mut sprites_diagnostics: FxHashMap<SmolStr, SpriteDiagnostics> = Default::default();
    let mut sprites: FxHashMap<SmolStr, Sprite> = Default::default();
    let files = fs.borrow_mut().read_dir(&input)?;
    for sprite_path in files {
        if sprite_path.file_stem().is_some_and(|stem| stem == "stage") {
            continue;
        }
        if sprite_path
            .extension()
            .is_none_or(|extension| extension != "gs")
        {
            continue;
        }
        if fs.borrow_mut().is_dir(&sprite_path) {
            continue;
        }
        let sprite_name: SmolStr = sprite_path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap()
            .into();
        let mut sprite_diagnostics = SpriteDiagnostics::new(fs.clone(), sprite_path, &stdlib);
        let (sprite, parse_diagnostics) = parser::parse(&sprite_diagnostics.translation_unit);
        sprite_diagnostics.diagnostics.extend(parse_diagnostics);
        sprites_diagnostics.insert(sprite_name.clone(), sprite_diagnostics);
        sprites.insert(sprite_name, sprite);
    }
    let mut project = Project { stage, sprites };
    if !(stage_diagnostics.diagnostics.is_empty()
        && sprites_diagnostics
            .values()
            .all(|sprite_diagnostics| sprite_diagnostics.diagnostics.is_empty()))
    {
        return Ok(Artifact {
            project,
            stage_diagnostics,
            sprites_diagnostics,
        });
    }
    visitor::pass0::visit_project(&input, &mut project);
    visitor::pass1::visit_project(&mut project);
    visitor::pass2::visit_project(
        &mut project,
        &mut stage_diagnostics,
        &mut sprites_diagnostics,
    );
    visitor::pass3::visit_project(&mut project);
    visitor::pass4::visit_project(&mut project);
    log::info!("{:#?}", project);
    assign_layer_orders(&mut project, &config);
    sb3.project(
        fs.clone(),
        &input,
        &project,
        &config,
        &mut stage_diagnostics,
        &mut sprites_diagnostics,
    )?;
    drop(sb3);
    Ok(Artifact {
        project,
        stage_diagnostics,
        sprites_diagnostics,
    })
}
