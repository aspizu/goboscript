use std::{
    env,
    fs::{self, read_dir, File},
    io::{self, BufWriter},
    path::PathBuf,
};

use anyhow::{bail, Result};
use colored::Colorize;
use fxhash::FxHashMap;
use smol_str::SmolStr;

use crate::{
    ast::{Project, Sprite},
    codegen::Sb3,
    config::Config,
    custom_toml_error::CustomTOMLError,
    diagnostic::{Diagnostic, LogLevel},
    parser::parse,
    visitors::{pass1, pass2},
};

pub fn build(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    compact: bool,
) -> Result<()> {
    let input = if let Some(input) = input { input } else { env::current_dir()? };
    let canonical_input = input.canonicalize()?;
    let project_name = canonical_input.file_name().unwrap().to_str().unwrap();
    let output = output.unwrap_or_else(|| input.join(format!("{project_name}.sb3")));
    let config_path = input.join("goboscript.toml");
    let config = if let Ok(config_src) = fs::read_to_string(&config_path) {
        match toml::from_str::<Config>(&config_src) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("{}", CustomTOMLError::new(config_path, config_src, err));
                bail!("cannot continue due to syntax errors")
            }
        }
    } else {
        Default::default()
    };
    let stage_path = input.join("stage.gs");
    let stage_src = match fs::read_to_string(&stage_path) {
        Ok(src) => src,
        Err(err) => {
            if matches!(err.kind(), io::ErrorKind::NotFound) {
                bail!("`stage.gs` not found, is this a goboscript project?")
            }
            bail!(err)
        }
    };
    let stage = match parse(&stage_src) {
        Ok(stage) => stage,
        Err(diag) => {
            diag.eprint(
                stage_path.to_str().unwrap(),
                &stage_src,
                &Default::default(),
                compact,
            );
            bail!("cannot continue due to syntax errors")
        }
    };
    let mut sprites: FxHashMap<SmolStr, Sprite> = Default::default();
    let mut stage_diags: Vec<Diagnostic> = Default::default();
    let mut srcs: FxHashMap<SmolStr, (PathBuf, String)> = Default::default();
    let mut diags: FxHashMap<SmolStr, Vec<Diagnostic>> = Default::default();
    for path in read_dir(&input)?.flatten().map(|entry| entry.path()) {
        if !(path.extension() == Some("gs".as_ref())
            && path.file_stem() != Some("stage".as_ref())
            && path.is_file())
        {
            continue;
        }
        let src = fs::read_to_string(&path)?;
        let sprite = match parse(&src) {
            Ok(sprite) => sprite,
            Err(diag) => {
                diag.eprint(path.to_str().unwrap(), &src, &Default::default(), compact);
                bail!("cannot continue due to syntax errors")
            }
        };
        let name = SmolStr::from(path.file_stem().unwrap().to_str().unwrap());
        sprites.insert(name.clone(), sprite);
        srcs.insert(name.clone(), (path, src));
        diags.insert(name.clone(), Default::default());
    }
    let mut project = Project::new(stage, sprites);
    pass1::visit_project(&mut project);
    pass2::visit_project(&mut project);
    let mut sb3 = Sb3::new(BufWriter::new(File::create(output)?));
    sb3.package(&project, &config, &input, &mut stage_diags, &mut diags)?;
    let mut errors = 0;
    let mut warnings = 0;
    for diag in stage_diags {
        diag.eprint(stage_path.to_str().unwrap(), &stage_src, &project.stage, compact);
        match diag.kind.log_level() {
            LogLevel::Error => errors += 1,
            LogLevel::Warning => warnings += 1,
        };
    }
    for (name, diags) in diags {
        for diag in diags {
            let (path, src) = &srcs[&name];
            diag.eprint(path.to_str().unwrap(), src, &project.sprites[&name], compact);
            match diag.kind.log_level() {
                LogLevel::Error => errors += 1,
                LogLevel::Warning => warnings += 1,
            };
        }
    }
    let err_summary = make_err_summary(errors, warnings);
    if errors > 0 {
        bail!(err_summary);
    }
    if warnings > 0 {
        eprintln!("{}: {}", "warning".yellow().bold(), err_summary.bold())
    }
    Ok(())
}

fn make_err_summary(errors: i32, warnings: i32) -> String {
    let error_text = if errors == 1 {
        Some(String::from("one error generated"))
    } else if errors > 1 {
        Some(format!("{errors} errors generated"))
    } else {
        None
    };
    let warning_text = if warnings == 1 {
        Some(String::from("one warning generated"))
    } else if warnings > 1 {
        Some(format!("{warnings} warnings generated"))
    } else {
        None
    };
    if let (Some(errs), Some(warns)) = (&error_text, &warning_text) {
        format!("{errs}; {warns}")
    } else if let Some(errs) = error_text {
        errs
    } else if let Some(warns) = warning_text {
        warns
    } else {
        "".into()
    }
}
