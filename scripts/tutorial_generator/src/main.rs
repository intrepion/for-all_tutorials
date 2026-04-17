use askama::Template;
use regex::Regex;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const GITHUB_OWNER: &str = "intrepion";
const OUTPUT_REPO_PREFIX: &str = "fa_tut";
const GITHUB_REPO_CREATE_DELAY: Duration = Duration::from_secs(1);
const FOR_ALL_API_PORT: &str = "25664";
const FOR_ALL_FRONTEND_PORT: &str = "25616";

fn main() -> Result<(), AppError> {
    let app_root = Path::new(ROOT)
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| AppError::message("unable to resolve application root"))?
        .to_path_buf();

    let args = Args::parse(&app_root)?;
    match &args.command {
        CommandMode::Generate => {
            let shared_projects = Partial::load(&app_root.join("partials/projects/README.md"))?;
            generate_all(&app_root, &args.output_root, &shared_projects)?;
        }
        CommandMode::CreateOutputRepos { owner } => {
            create_output_repos(&app_root, owner)?;
        }
        CommandMode::CloneOutputRepos { repos_root, owner } => {
            clone_output_repos(&app_root, repos_root, owner)?;
        }
        CommandMode::BootstrapOutputRepos {
            repos_root,
            owner,
            sync_branch_name,
            project,
            selection_overrides,
        } => {
            bootstrap_output_repos(
                &app_root,
                repos_root,
                owner,
                sync_branch_name.as_deref(),
                project.as_deref(),
                selection_overrides,
            )?;
        }
        CommandMode::CleanupOutputRepos {
            repos_root,
            owner,
            apply,
        } => {
            cleanup_output_repos(&app_root, repos_root, owner, *apply)?;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Args {
    output_root: PathBuf,
    command: CommandMode,
}

#[derive(Debug)]
enum CommandMode {
    Generate,
    CreateOutputRepos {
        owner: String,
    },
    CloneOutputRepos {
        repos_root: PathBuf,
        owner: String,
    },
    BootstrapOutputRepos {
        repos_root: PathBuf,
        owner: String,
        sync_branch_name: Option<String>,
        project: Option<String>,
        selection_overrides: BootstrapSelectionOverrides,
    },
    CleanupOutputRepos {
        repos_root: PathBuf,
        owner: String,
        apply: bool,
    },
}

#[derive(Debug, Clone, Default)]
struct BootstrapSelectionOverrides {
    ecosystem: Option<String>,
    language: Option<String>,
    testing: Option<String>,
    mocking: Option<String>,
    storage: Option<String>,
    surface: Option<String>,
    target: Option<String>,
    framework: Option<String>,
    protocol: Option<String>,
}

impl Args {
    fn parse(app_root: &Path) -> Result<Self, AppError> {
        let mut output_root = app_root.join("tutorials");
        let mut command = CommandMode::Generate;
        let mut repos_root = app_root
            .parent()
            .unwrap_or(app_root)
            .join("output-repos");
        let mut owner = GITHUB_OWNER.to_string();
        let mut apply = false;
        let mut sync_branch_name = None;
        let mut project = None;
        let mut selection_overrides = BootstrapSelectionOverrides::default();
        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--output-root" => {
                    let path = args
                        .next()
                        .ok_or_else(|| AppError::message("missing value for --output-root"))?;
                    output_root = absolute_path(app_root, Path::new(&path));
                }
                "--create-output-repos" => {
                    command = CommandMode::CreateOutputRepos {
                        owner: owner.clone(),
                    };
                }
                "--clone-output-repos" => {
                    command = CommandMode::CloneOutputRepos {
                        repos_root: repos_root.clone(),
                        owner: owner.clone(),
                    };
                }
                "--bootstrap-output-repos" => {
                    command = CommandMode::BootstrapOutputRepos {
                        repos_root: repos_root.clone(),
                        owner: owner.clone(),
                        sync_branch_name: sync_branch_name.clone(),
                        project: project.clone(),
                        selection_overrides: selection_overrides.clone(),
                    };
                }
                "--cleanup-output-repos" => {
                    command = CommandMode::CleanupOutputRepos {
                        repos_root: repos_root.clone(),
                        owner: owner.clone(),
                        apply,
                    };
                }
                "--repos-root" => {
                    let path = args
                        .next()
                        .ok_or_else(|| AppError::message("missing value for --repos-root"))?;
                    repos_root = absolute_path(app_root, Path::new(&path));
                }
                "--owner" => {
                    owner = args
                        .next()
                        .ok_or_else(|| AppError::message("missing value for --owner"))?;
                }
                "--apply" => {
                    apply = true;
                }
                "--sync-branch-name" => {
                    sync_branch_name = Some(
                        args.next().ok_or_else(|| {
                            AppError::message("missing value for --sync-branch-name")
                        })?,
                    );
                }
                "--project" => {
                    project = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --project"))?,
                    );
                }
                "--ecosystem" => {
                    selection_overrides.ecosystem = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --ecosystem"))?,
                    );
                }
                "--language" => {
                    selection_overrides.language = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --language"))?,
                    );
                }
                "--testing" => {
                    selection_overrides.testing = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --testing"))?,
                    );
                }
                "--mocking" => {
                    selection_overrides.mocking = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --mocking"))?,
                    );
                }
                "--storage" => {
                    selection_overrides.storage = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --storage"))?,
                    );
                }
                "--surface" => {
                    selection_overrides.surface = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --surface"))?,
                    );
                }
                "--target" => {
                    selection_overrides.target = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --target"))?,
                    );
                }
                "--framework" => {
                    selection_overrides.framework = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --framework"))?,
                    );
                }
                "--protocol" => {
                    selection_overrides.protocol = Some(
                        args.next()
                            .ok_or_else(|| AppError::message("missing value for --protocol"))?,
                    );
                }
                other => return Err(AppError::message(format!("unknown argument: {other}"))),
            }
        }

        match command {
            CommandMode::CreateOutputRepos { .. } => {
                command = CommandMode::CreateOutputRepos { owner };
            }
            CommandMode::CloneOutputRepos { .. } => {
                command = CommandMode::CloneOutputRepos {
                    repos_root,
                    owner,
                };
            }
            CommandMode::BootstrapOutputRepos { .. } => {
                command = CommandMode::BootstrapOutputRepos {
                    repos_root,
                    owner,
                    sync_branch_name,
                    project,
                    selection_overrides,
                };
            }
            CommandMode::CleanupOutputRepos { .. } => {
                command = CommandMode::CleanupOutputRepos {
                    repos_root,
                    owner,
                    apply,
                };
            }
            CommandMode::Generate => {}
        }

        Ok(Self {
            output_root,
            command,
        })
    }
}

fn absolute_path(base: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

fn default_sync_branch_name() -> String {
    let output = Command::new("python3")
        .args([
            "-c",
            "from datetime import UTC, datetime; print(datetime.now(UTC).strftime('%Y%m%dT%H%M%S%fZ'))",
        ])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !value.is_empty() {
                return value;
            }
        }
    }

    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("utc-{}", duration.as_micros())
}

fn collect_manifest_paths(app_root: &Path) -> Result<Vec<PathBuf>, AppError> {
    let mut manifests = Vec::new();
    let projects_root = app_root.join("partials/projects");

    for entry in fs::read_dir(projects_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let manifest = path.join("manifest.yaml");
            if manifest.exists() {
                manifests.push(manifest);
            }
        }
    }

    manifests.sort();
    Ok(manifests)
}

fn generate_all(
    app_root: &Path,
    output_root: &Path,
    shared_projects: &Partial,
) -> Result<(), AppError> {
    let manifest_paths = collect_manifest_paths(app_root)?;

    for manifest_path in manifest_paths {
        generate_from_manifest(app_root, output_root, shared_projects, &manifest_path)?;
    }

    Ok(())
}

fn generate_from_manifest(
    app_root: &Path,
    output_root: &Path,
    shared_projects: &Partial,
    manifest_path: &Path,
) -> Result<(), AppError> {
    let manifest: Manifest = serde_yaml::from_str(&fs::read_to_string(manifest_path)?)?;
    let project_root_path = app_root
        .join("partials/projects")
        .join(&manifest.project)
        .join("README.md");
    let project_root = Partial::load(&project_root_path)?;
    let project_title = project_root.title.clone();

    let outputs = expanded_compiled_outputs(app_root, &manifest);

    for output in &outputs {
        let mut partials = Vec::new();
        for source in &output.sources {
            if source.ends_with(".md") {
                partials.push(Partial::load(&app_root.join(source))?);
            }
        }
        append_implicit_partials(&app_root, &mut partials, output)?;

        let rendered = build_readme(
            &project_title,
            &manifest.project,
            shared_projects,
            &partials,
            output,
        )?
        .render()?;

        let relative = output
            .tutorial_path
            .strip_prefix("tutorials/")
            .unwrap_or(&output.tutorial_path);
        let destination = output_root.join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(destination, format!("{rendered}\n"))?;
    }

    Ok(())
}

#[derive(Debug)]
struct OutputRepoSpec {
    repo_name: String,
    repo_description: String,
    project_slug: String,
    selections: OutputRepoSelections,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OutputRepoSelections {
    ecosystem: String,
    language: String,
    testing: String,
    mocking: String,
    storage: String,
    surface: String,
    target: String,
    framework: String,
    protocol: Option<String>,
}

fn is_go_saying_hello_output_repo(spec: &OutputRepoSpec) -> bool {
    spec.project_slug == "saying-hello" && spec.selections.ecosystem == "go"
}

fn is_go_todo_list_http_json_output_repo(spec: &OutputRepoSpec) -> bool {
    spec.project_slug == "todo-list"
        && spec.selections.ecosystem == "go"
        && spec.selections.language == "go"
        && spec.selections.testing == "testify"
        && spec.selections.mocking == "testify-mock"
        && spec.selections.storage == "local-files-json"
        && spec.selections.surface == "web"
        && spec.selections.target == "api"
        && spec.selections.framework == "echo"
        && spec.selections.protocol.as_deref() == Some("http-json")
}

fn is_go_todo_list_http_json_sqlite_output_repo(spec: &OutputRepoSpec) -> bool {
    spec.project_slug == "todo-list"
        && spec.selections.ecosystem == "go"
        && spec.selections.language == "go"
        && spec.selections.testing == "testify"
        && spec.selections.mocking == "testify-mock"
        && spec.selections.storage == "database-sqlite"
        && spec.selections.surface == "web"
        && spec.selections.target == "api"
        && spec.selections.framework == "echo"
        && spec.selections.protocol.as_deref() == Some("http-json")
}

fn is_go_todo_list_rest_json_postgres_output_repo(spec: &OutputRepoSpec) -> bool {
    spec.project_slug == "todo-list"
        && spec.selections.ecosystem == "go"
        && spec.selections.language == "go"
        && spec.selections.testing == "testify"
        && spec.selections.mocking == "testify-mock"
        && spec.selections.storage == "database-postgres"
        && spec.selections.surface == "web"
        && spec.selections.target == "api"
        && spec.selections.framework == "echo"
        && spec.selections.protocol.as_deref() == Some("rest-json")
}

fn is_go_todo_list_rest_json_output_repo(spec: &OutputRepoSpec) -> bool {
    is_go_todo_list_rest_json_postgres_output_repo(spec)
}

fn is_go_todo_list_output_repo(spec: &OutputRepoSpec) -> bool {
    is_go_todo_list_http_json_output_repo(spec)
        || is_go_todo_list_http_json_sqlite_output_repo(spec)
        || is_go_todo_list_rest_json_output_repo(spec)
}

fn is_go_output_repo(spec: &OutputRepoSpec) -> bool {
    is_go_saying_hello_output_repo(spec) || is_go_todo_list_output_repo(spec)
}

fn is_astro_saying_hello_output_repo(spec: &OutputRepoSpec) -> bool {
    spec.project_slug == "saying-hello"
        && spec.selections.ecosystem == "javascript"
        && spec.selections.language == "typescript"
        && spec.selections.testing == "vitest"
        && spec.selections.mocking == "vitest-built-in"
        && spec.selections.storage == "no-storage"
        && spec.selections.surface == "web"
        && spec.selections.target == "front-end"
        && spec.selections.framework == "astro"
        && spec.selections.protocol.as_deref() == Some("http-json")
}

fn is_flutter_saying_hello_output_repo(spec: &OutputRepoSpec) -> bool {
    is_flutter_local_saying_hello_output_repo(spec)
        || is_flutter_http_saying_hello_output_repo(spec)
}

fn is_flutter_todo_list_rest_json_output_repo(spec: &OutputRepoSpec) -> bool {
    spec.project_slug == "todo-list"
        && spec.selections.ecosystem == "dart"
        && spec.selections.language == "dart"
        && spec.selections.testing == "test"
        && spec.selections.mocking == "mocktail"
        && spec.selections.storage == "no-storage"
        && spec.selections.surface == "client"
        && spec.selections.target == "all"
        && spec.selections.framework == "flutter"
        && spec.selections.protocol.as_deref() == Some("rest-json")
}

fn is_flutter_todo_list_output_repo(spec: &OutputRepoSpec) -> bool {
    is_flutter_todo_list_rest_json_output_repo(spec)
}

fn is_flutter_output_repo(spec: &OutputRepoSpec) -> bool {
    is_flutter_saying_hello_output_repo(spec) || is_flutter_todo_list_output_repo(spec)
}

fn is_flutter_http_output_repo(spec: &OutputRepoSpec) -> bool {
    is_flutter_http_saying_hello_output_repo(spec) || is_flutter_todo_list_output_repo(spec)
}

fn is_flutter_local_saying_hello_output_repo(spec: &OutputRepoSpec) -> bool {
    spec.project_slug == "saying-hello"
        && spec.selections.ecosystem == "dart"
        && spec.selections.language == "dart"
        && spec.selections.testing == "test"
        && spec.selections.mocking == "mocktail"
        && spec.selections.storage == "no-storage"
        && spec.selections.surface == "client"
        && spec.selections.target == "all"
        && spec.selections.framework == "flutter"
        && spec.selections.protocol.is_none()
}

fn is_flutter_http_saying_hello_output_repo(spec: &OutputRepoSpec) -> bool {
    spec.project_slug == "saying-hello"
        && spec.selections.ecosystem == "dart"
        && spec.selections.language == "dart"
        && spec.selections.testing == "test"
        && spec.selections.mocking == "mocktail"
        && spec.selections.storage == "no-storage"
        && spec.selections.surface == "client"
        && spec.selections.target == "all"
        && spec.selections.framework == "flutter"
        && spec.selections.protocol.as_deref() == Some("http-json")
}

fn collect_output_repo_specs(app_root: &Path) -> Result<Vec<OutputRepoSpec>, AppError> {
    collect_output_repo_specs_for_project(app_root, None, &BootstrapSelectionOverrides::default())
}

fn collect_output_repo_specs_for_project(
    app_root: &Path,
    project_filter: Option<&str>,
    selection_overrides: &BootstrapSelectionOverrides,
) -> Result<Vec<OutputRepoSpec>, AppError> {
    let manifest_paths = collect_manifest_paths(app_root)?;
    let mut specs = Vec::new();

    for manifest_path in manifest_paths {
        let manifest: Manifest = serde_yaml::from_str(&fs::read_to_string(&manifest_path)?)?;
        if let Some(project_filter) = project_filter {
            if manifest.project != project_filter {
                continue;
            }
        }
        let project_root_path = app_root
            .join("partials/projects")
            .join(&manifest.project)
            .join("README.md");
        let project_root = Partial::load(&project_root_path)?;
        let project_title = project_root.title.clone();
        let selections =
            output_repo_selections_for_project(&manifest.project, selection_overrides)?;
        let repo_description = output_repo_description(&project_title, &selections);

        specs.push(OutputRepoSpec {
            repo_name: output_repo_name(&manifest.project),
            repo_description,
            project_slug: manifest.project.clone(),
            selections,
        });
    }

    specs.sort_by(|left, right| left.repo_name.cmp(&right.repo_name));
    Ok(specs)
}

fn output_repo_selections_for_project(
    project_slug: &str,
    selection_overrides: &BootstrapSelectionOverrides,
) -> Result<OutputRepoSelections, AppError> {
    let mut selections = selection_overrides
        .ecosystem
        .as_deref()
        .and_then(|ecosystem| supported_output_repo_selections(project_slug, ecosystem))
        .unwrap_or_else(|| default_output_repo_selections(project_slug));

    if let Some(value) = &selection_overrides.ecosystem {
        selections.ecosystem = value.clone();
    }
    if let Some(value) = &selection_overrides.language {
        selections.language = value.clone();
    }
    if let Some(value) = &selection_overrides.testing {
        selections.testing = value.clone();
    }
    if let Some(value) = &selection_overrides.mocking {
        selections.mocking = value.clone();
    }
    if let Some(value) = &selection_overrides.storage {
        selections.storage = value.clone();
    }
    if let Some(value) = &selection_overrides.surface {
        selections.surface = value.clone();
    }
    if let Some(value) = &selection_overrides.target {
        selections.target = value.clone();
    }
    if let Some(value) = &selection_overrides.framework {
        selections.framework = value.clone();
    }
    if let Some(value) = &selection_overrides.protocol {
        selections.protocol = if value == "none" {
            None
        } else {
            Some(value.clone())
        };
    }

    validate_output_repo_selections(project_slug, &selections)?;
    Ok(selections)
}

fn default_output_repo_selections(project_slug: &str) -> OutputRepoSelections {
    let _ = project_slug;
    OutputRepoSelections {
        ecosystem: "dotnet".to_string(),
        language: "csharp".to_string(),
        testing: "xunit".to_string(),
        mocking: "nsubstitute".to_string(),
        storage: "no-storage".to_string(),
        surface: "command-line".to_string(),
        target: "all".to_string(),
        framework: "no-framework".to_string(),
        protocol: None,
    }
}

fn supported_output_repo_selections(
    project_slug: &str,
    ecosystem: &str,
) -> Option<OutputRepoSelections> {
    match (project_slug, ecosystem) {
        ("saying-hello", "go") => Some(OutputRepoSelections {
            ecosystem: "go".to_string(),
            language: "go".to_string(),
            testing: "testify".to_string(),
            mocking: "testify-mock".to_string(),
            storage: "no-storage".to_string(),
            surface: "web".to_string(),
            target: "api".to_string(),
            framework: "echo".to_string(),
            protocol: Some("http-json".to_string()),
        }),
        ("todo-list", "go") => Some(OutputRepoSelections {
            ecosystem: "go".to_string(),
            language: "go".to_string(),
            testing: "testify".to_string(),
            mocking: "testify-mock".to_string(),
            storage: "local-files-json".to_string(),
            surface: "web".to_string(),
            target: "api".to_string(),
            framework: "echo".to_string(),
            protocol: Some("http-json".to_string()),
        }),
        ("saying-hello", "javascript") => Some(OutputRepoSelections {
            ecosystem: "javascript".to_string(),
            language: "typescript".to_string(),
            testing: "vitest".to_string(),
            mocking: "vitest-built-in".to_string(),
            storage: "no-storage".to_string(),
            surface: "web".to_string(),
            target: "front-end".to_string(),
            framework: "astro".to_string(),
            protocol: Some("http-json".to_string()),
        }),
        ("saying-hello", "dart") => Some(OutputRepoSelections {
            ecosystem: "dart".to_string(),
            language: "dart".to_string(),
            testing: "test".to_string(),
            mocking: "mocktail".to_string(),
            storage: "no-storage".to_string(),
            surface: "client".to_string(),
            target: "all".to_string(),
            framework: "flutter".to_string(),
            protocol: Some("http-json".to_string()),
        }),
        ("todo-list", "dart") => Some(OutputRepoSelections {
            ecosystem: "dart".to_string(),
            language: "dart".to_string(),
            testing: "test".to_string(),
            mocking: "mocktail".to_string(),
            storage: "no-storage".to_string(),
            surface: "client".to_string(),
            target: "all".to_string(),
            framework: "flutter".to_string(),
            protocol: Some("rest-json".to_string()),
        }),
        (_, "dotnet") => Some(OutputRepoSelections {
            ecosystem: "dotnet".to_string(),
            language: "csharp".to_string(),
            testing: "xunit".to_string(),
            mocking: "nsubstitute".to_string(),
            storage: "no-storage".to_string(),
            surface: "command-line".to_string(),
            target: "all".to_string(),
            framework: "no-framework".to_string(),
            protocol: None,
        }),
        _ => None,
    }
}

fn validate_output_repo_selections(
    project_slug: &str,
    selections: &OutputRepoSelections,
) -> Result<(), AppError> {
    let supported_go = OutputRepoSelections {
        ecosystem: "go".to_string(),
        language: "go".to_string(),
        testing: "testify".to_string(),
        mocking: "testify-mock".to_string(),
        storage: "no-storage".to_string(),
        surface: "web".to_string(),
        target: "api".to_string(),
        framework: "echo".to_string(),
        protocol: Some("http-json".to_string()),
    };

    let supported_go_todo_list = OutputRepoSelections {
        ecosystem: "go".to_string(),
        language: "go".to_string(),
        testing: "testify".to_string(),
        mocking: "testify-mock".to_string(),
        storage: "local-files-json".to_string(),
        surface: "web".to_string(),
        target: "api".to_string(),
        framework: "echo".to_string(),
        protocol: Some("http-json".to_string()),
    };

    let supported_go_todo_list_http_json_sqlite = OutputRepoSelections {
        ecosystem: "go".to_string(),
        language: "go".to_string(),
        testing: "testify".to_string(),
        mocking: "testify-mock".to_string(),
        storage: "database-sqlite".to_string(),
        surface: "web".to_string(),
        target: "api".to_string(),
        framework: "echo".to_string(),
        protocol: Some("http-json".to_string()),
    };

    let supported_go_todo_list_rest_json_postgres = OutputRepoSelections {
        ecosystem: "go".to_string(),
        language: "go".to_string(),
        testing: "testify".to_string(),
        mocking: "testify-mock".to_string(),
        storage: "database-postgres".to_string(),
        surface: "web".to_string(),
        target: "api".to_string(),
        framework: "echo".to_string(),
        protocol: Some("rest-json".to_string()),
    };

    let supported_astro = OutputRepoSelections {
        ecosystem: "javascript".to_string(),
        language: "typescript".to_string(),
        testing: "vitest".to_string(),
        mocking: "vitest-built-in".to_string(),
        storage: "no-storage".to_string(),
        surface: "web".to_string(),
        target: "front-end".to_string(),
        framework: "astro".to_string(),
        protocol: Some("http-json".to_string()),
    };

    let supported_flutter_http = OutputRepoSelections {
        ecosystem: "dart".to_string(),
        language: "dart".to_string(),
        testing: "test".to_string(),
        mocking: "mocktail".to_string(),
        storage: "no-storage".to_string(),
        surface: "client".to_string(),
        framework: "flutter".to_string(),
        target: "all".to_string(),
        protocol: Some("http-json".to_string()),
    };

    let supported_flutter_todo_list_rest_json = OutputRepoSelections {
        ecosystem: "dart".to_string(),
        language: "dart".to_string(),
        testing: "test".to_string(),
        mocking: "mocktail".to_string(),
        storage: "no-storage".to_string(),
        surface: "client".to_string(),
        target: "all".to_string(),
        framework: "flutter".to_string(),
        protocol: Some("rest-json".to_string()),
    };

    let supported_flutter_local = OutputRepoSelections {
        ecosystem: "dart".to_string(),
        language: "dart".to_string(),
        testing: "test".to_string(),
        mocking: "mocktail".to_string(),
        storage: "no-storage".to_string(),
        surface: "client".to_string(),
        target: "all".to_string(),
        framework: "flutter".to_string(),
        protocol: None,
    };

    let supported_dotnet = OutputRepoSelections {
        ecosystem: "dotnet".to_string(),
        language: "csharp".to_string(),
        testing: "xunit".to_string(),
        mocking: "nsubstitute".to_string(),
        storage: "no-storage".to_string(),
        surface: "command-line".to_string(),
        target: "all".to_string(),
        framework: "no-framework".to_string(),
        protocol: None,
    };

    if selections == &supported_dotnet {
        return Ok(());
    }

    if project_slug == "saying-hello" && selections == &supported_go {
        return Ok(());
    }

    if project_slug == "todo-list" && selections == &supported_go_todo_list {
        return Ok(());
    }

    if project_slug == "todo-list" && selections == &supported_go_todo_list_http_json_sqlite {
        return Ok(());
    }

    if project_slug == "todo-list" && selections == &supported_go_todo_list_rest_json_postgres {
        return Ok(());
    }

    if project_slug == "saying-hello" && selections == &supported_astro {
        return Ok(());
    }

    if project_slug == "saying-hello" && selections == &supported_flutter_http {
        return Ok(());
    }

    if project_slug == "todo-list" && selections == &supported_flutter_todo_list_rest_json {
        return Ok(());
    }

    if project_slug == "saying-hello" && selections == &supported_flutter_local {
        return Ok(());
    }

    Err(AppError::message(format!(
        "unsupported bootstrap selections for {project_slug}: {}/{}/{}/{}/{}/{}/{}/{}{}",
        selections.ecosystem,
        selections.language,
        selections.testing,
        selections.mocking,
        selections.storage,
        selections.surface,
        selections.target,
        selections.framework,
        selections
            .protocol
            .as_ref()
            .map(|value| format!("/{}", value))
            .unwrap_or_default()
    )))
}

fn expanded_compiled_outputs(app_root: &Path, manifest: &Manifest) -> Vec<CompiledOutput> {
    let mut outputs = manifest.compiled_outputs.clone();
    let mut seen_contract_pairs = BTreeSet::new();

    for output in &outputs {
        if output.kind == OutputKind::Contracts {
            seen_contract_pairs.insert((
                output.selections.ecosystem.clone(),
                output.selections.language.clone(),
            ));
        }
    }

    let mut implied_pairs = BTreeSet::new();
    for output in &outputs {
        implied_pairs.insert((
            output.selections.ecosystem.clone(),
            output.selections.language.clone(),
        ));
    }

    for (ecosystem, language) in implied_pairs {
        if ecosystem != "dotnet" || seen_contract_pairs.contains(&(ecosystem.clone(), language.clone()))
        {
            continue;
        }

        outputs.insert(
            0,
            implicit_contract_output(app_root, manifest, &ecosystem, &language),
        );
    }

    outputs
}

fn implicit_contract_output(
    app_root: &Path,
    manifest: &Manifest,
    ecosystem: &str,
    language: &str,
) -> CompiledOutput {
    let mut sources = vec![
        format!("partials/projects/{}/README.md", manifest.project),
        format!("partials/projects/{}/spec/README.md", manifest.project),
        format!("partials/projects/{}/instructions/README.md", manifest.project),
        format!("partials/setups/code/{ecosystem}/README.md"),
        format!("partials/setups/code/{ecosystem}/languages/{language}.md"),
        format!("partials/setups/code/{ecosystem}/toolchain/sdk.md"),
        format!("partials/setups/code/{ecosystem}/toolchain/dotnet-cli.md"),
    ];

    let project_contracts_partial = format!(
        "partials/projects/{}/instructions/contracts.md",
        manifest.project
    );
    if app_root.join(&project_contracts_partial).exists() {
        sources.insert(3, project_contracts_partial);
    }

    CompiledOutput {
        id: format!("{ecosystem}-{language}-contracts"),
        kind: OutputKind::Contracts,
        tutorial_path: format!(
            "tutorials/{}/{}/{}/contracts/README.md",
            manifest.project, ecosystem, language
        ),
        selections: Selections {
            ecosystem: ecosystem.to_string(),
            language: language.to_string(),
            testing: None,
            storage: None,
            surface: None,
            target: None,
            framework: None,
        },
        sources,
    }
}

fn create_output_repos(app_root: &Path, owner: &str) -> Result<(), AppError> {
    let specs = collect_output_repo_specs(app_root)?;
    ensure_github_repos_exist(owner, &specs)
}

fn clone_output_repos(app_root: &Path, repos_root: &Path, owner: &str) -> Result<(), AppError> {
    let specs = collect_output_repo_specs(app_root)?;
    fs::create_dir_all(repos_root)?;

    for spec in &specs {
        clone_output_repo(repos_root, owner, spec)?;
    }

    Ok(())
}

fn bootstrap_output_repos(
    app_root: &Path,
    repos_root: &Path,
    owner: &str,
    sync_branch_name: Option<&str>,
    project_filter: Option<&str>,
    selection_overrides: &BootstrapSelectionOverrides,
) -> Result<(), AppError> {
    let specs = collect_output_repo_specs_for_project(app_root, project_filter, selection_overrides)?;
    let sync_branch_name = sync_branch_name
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
        .unwrap_or_else(default_sync_branch_name);
    println!("sync branch: {sync_branch_name}");

    for spec in &specs {
        bootstrap_output_repo(app_root, repos_root, owner, spec, &sync_branch_name)?;
    }

    Ok(())
}

fn cleanup_output_repos(
    app_root: &Path,
    repos_root: &Path,
    owner: &str,
    apply: bool,
) -> Result<(), AppError> {
    let specs = collect_output_repo_specs(app_root)?;

    for spec in specs {
        cleanup_output_repo(repos_root, owner, &spec, apply)?;
    }

    Ok(())
}

fn ensure_github_repos_exist(owner: &str, specs: &[OutputRepoSpec]) -> Result<(), AppError> {
    let mut created_repo_count = 0usize;

    for spec in specs {
        let repo_full_name = format!("{owner}/{}", spec.repo_name);
        if gh_repo_exists(&repo_full_name)? {
            continue;
        }

        if created_repo_count > 0 {
            sleep(GITHUB_REPO_CREATE_DELAY);
        }

        create_github_repo(&repo_full_name, &spec.repo_description)?;
        created_repo_count += 1;
    }

    Ok(())
}

fn clone_output_repo(
    repos_root: &Path,
    owner: &str,
    spec: &OutputRepoSpec,
) -> Result<(), AppError> {
    let repo_full_name = format!("{owner}/{}", spec.repo_name);
    let clone_path = repos_root.join(&spec.repo_name);

    if clone_path.exists() {
        ensure_git_repo(&clone_path)?;
        println!("already cloned: {repo_full_name}");
        return Ok(());
    }

    clone_github_repo(&repo_full_name, &clone_path)?;
    println!("cloned: {repo_full_name}");
    Ok(())
}

fn bootstrap_output_repo(
    app_root: &Path,
    repos_root: &Path,
    owner: &str,
    spec: &OutputRepoSpec,
    sync_branch_name: &str,
) -> Result<(), AppError> {
    let repo_full_name = format!("{owner}/{}", spec.repo_name);
    let clone_path = repos_root.join(&spec.repo_name);
    let managed_files = build_managed_repo_files(app_root, owner, spec)?;

    if !clone_path.exists() {
        return Err(AppError::message(format!(
            "local clone missing at {} for {}. Run clone-output-repos first.",
            clone_path.display(),
            repo_full_name
        )));
    }

    ensure_git_repo(&clone_path)?;
    ensure_clean_worktree(&clone_path)?;
    let baseline_commit = ensure_main_license_baseline(&clone_path, owner)?;
    switch_to_sync_branch_from_commit(&clone_path, sync_branch_name, &baseline_commit)?;
    let file_changes = planned_file_changes(&clone_path, &managed_files)?;

    if file_changes.is_empty() {
        println!("up to date: {repo_full_name}");
        return Ok(());
    }

    println!("repo: {repo_full_name}");
    for change in &file_changes {
        println!("  - {} {}", change.action, change.path);
    }

    write_managed_files(&clone_path, &managed_files)?;
    git_add_managed_files(&clone_path, &managed_files)?;
    git_commit_managed_files(&clone_path, false)?;
    git_push_branch(&clone_path, sync_branch_name)?;

    println!("synced: {repo_full_name}");
    Ok(())
}

fn cleanup_output_repo(
    repos_root: &Path,
    owner: &str,
    spec: &OutputRepoSpec,
    apply: bool,
) -> Result<(), AppError> {
    let repo_full_name = format!("{owner}/{}", spec.repo_name);
    let clone_path = repos_root.join(&spec.repo_name);
    let repo_exists = gh_repo_exists(&repo_full_name)?;
    let clone_exists = clone_path.exists();

    if !apply {
        println!("repo: {repo_full_name}");
        if repo_exists {
            println!("  - would delete GitHub repo");
        } else {
            println!("  - GitHub repo already absent");
        }

        if clone_exists {
            println!("  - would delete local clone at {}", clone_path.display());
        } else {
            println!("  - local clone already absent");
        }

        return Ok(());
    }

    if clone_exists {
        fs::remove_dir_all(&clone_path)?;
        println!("deleted local clone: {}", clone_path.display());
    }

    if repo_exists {
        delete_github_repo(&repo_full_name)?;
        println!("deleted GitHub repo: {repo_full_name}");
    }

    if !clone_exists && !repo_exists {
        println!("already absent: {repo_full_name}");
    }

    Ok(())
}

fn gh_repo_exists(repo_full_name: &str) -> Result<bool, AppError> {
    let status = Command::new("gh")
        .args(["repo", "view", repo_full_name, "--json", "name"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    Ok(status.success())
}

#[derive(Debug)]
struct ManagedRepoFile {
    relative_path: String,
    contents: Vec<u8>,
}

#[derive(Debug)]
struct PlannedFileChange {
    action: &'static str,
    path: String,
}

fn build_managed_repo_files(
    app_root: &Path,
    owner: &str,
    spec: &OutputRepoSpec,
) -> Result<Vec<ManagedRepoFile>, AppError> {
    let mut files = vec![
        ManagedRepoFile {
            relative_path: "README.md".to_string(),
            contents: render_output_repo_readme_content(owner, spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "justfile".to_string(),
            contents: render_output_repo_root_justfile_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "LICENSE".to_string(),
            contents: mit_license_text(owner).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: ".gitignore".to_string(),
            contents: starter_gitignore_content(&spec.selections.ecosystem).into_bytes(),
        },
    ];

    if spec.selections.ecosystem == "dotnet"
        || is_go_output_repo(spec)
        || is_flutter_output_repo(spec)
        || is_astro_saying_hello_output_repo(spec)
    {
        files.push(ManagedRepoFile {
            relative_path: ".github/workflows/ci.yml".to_string(),
            contents: render_output_repo_ci_workflow_content(spec).into_bytes(),
        });
    }

    files.extend(build_output_repo_tutorial_files(app_root, spec));

    Ok(files)
}

fn build_main_baseline_files(owner: &str) -> Vec<ManagedRepoFile> {
    vec![ManagedRepoFile {
        relative_path: "LICENSE".to_string(),
        contents: mit_license_text(owner).into_bytes(),
    }]
}

fn planned_file_changes(
    repo_path: &Path,
    managed_files: &[ManagedRepoFile],
) -> Result<Vec<PlannedFileChange>, AppError> {
    let mut changes = Vec::new();

    for managed_file in managed_files {
        let destination = repo_path.join(&managed_file.relative_path);
        let action = match fs::read(&destination) {
            Ok(existing) if existing == managed_file.contents => None,
            Ok(_) => Some("update"),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Some("create"),
            Err(error) => return Err(AppError::Io(error)),
        };

        if let Some(action) = action {
            changes.push(PlannedFileChange {
                action,
                path: managed_file.relative_path.clone(),
            });
        }
    }

    Ok(changes)
}

fn write_managed_files(repo_path: &Path, managed_files: &[ManagedRepoFile]) -> Result<(), AppError> {
    for managed_file in managed_files {
        let destination = repo_path.join(&managed_file.relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(destination, &managed_file.contents)?;
    }

    Ok(())
}

fn render_root_readme_content(owner: &str, repo_name: &str, repo_description: &str) -> String {
    let workflow_url = format!("https://github.com/{owner}/{repo_name}/actions/workflows/ci.yml");
    let badge_url = format!("{workflow_url}/badge.svg");

    format!("# {repo_name}\n{repo_description}\n\n[![CI]({badge_url})]({workflow_url})\n")
}

fn render_output_repo_readme_content(owner: &str, spec: &OutputRepoSpec) -> String {
    render_root_readme_content(owner, &spec.repo_name, &spec.repo_description)
}

fn render_output_repo_root_justfile_content(spec: &OutputRepoSpec) -> String {
    if is_go_todo_list_rest_json_postgres_output_repo(spec) {
        return format!(
            "set shell := [\"bash\", \"-eu\", \"-c\"]\n\n\
default:\n\
\t@just --list\n\n\
workspace := \"workspace\"\n\
database_url := \"postgres://postgres@localhost:5432/todo_list?sslmode=disable\"\n\n\
restore:\n\
\t(cd {{{{workspace}}}} && go mod download)\n\n\
generate:\n\
\t(cd {{{{workspace}}}} && ./bin/sqlc generate)\n\n\
format:\n\
\tfind {{{{workspace}}}} -name '*.go' -exec gofmt -w {{}} +\n\n\
check-formatting:\n\
\ttest -z \"$(find {{{{workspace}}}} -name '*.go' -exec gofmt -l {{}} +)\"\n\n\
check-tests:\n\
\tjust generate\n\
\t(cd {{{{workspace}}}} && go test ./...)\n\n\
run:\n\
\tjust generate\n\
\t(cd {{{{workspace}}}} && TODO_LIST_DATABASE_URL={{{{database_url}}}} go run ./cmd/server)\n\n\
check-all:\n\
\tjust check-formatting\n\
\tjust check-tests\n"
        );
    }

    if is_go_output_repo(spec) {
        return "set shell := [\"bash\", \"-eu\", \"-c\"]\n\n\
default:\n\
\t@just --list\n\n\
workspace := \"workspace\"\n\n\
restore:\n\
\t(cd {{workspace}} && go mod download)\n\n\
format:\n\
\tfind {{workspace}} -name '*.go' -exec gofmt -w {} +\n\n\
check-formatting:\n\
\ttest -z \"$(find {{workspace}} -name '*.go' -exec gofmt -l {} +)\"\n\n\
check-tests:\n\
\t(cd {{workspace}} && go test ./...)\n\n\
run:\n\
\t(cd {{workspace}} && go run ./cmd/server)\n\n\
check-all:\n\
\tjust check-formatting\n\
\tjust check-tests\n"
            .to_string();
    }

    if is_flutter_output_repo(spec) {
        let extra_variable = if is_flutter_http_output_repo(spec) {
            format!("api_base_url := \"http://localhost:{FOR_ALL_API_PORT}\"\n\n")
        } else {
            String::new()
        };
        let web_define = if is_flutter_http_output_repo(spec) {
            " --dart-define=API_BASE_URL={{api_base_url}}"
        } else {
            ""
        };
        let desktop_define = if is_flutter_http_output_repo(spec) {
            " --dart-define=API_BASE_URL={{api_base_url}}"
        } else {
            ""
        };
        let mobile_define = if is_flutter_http_output_repo(spec) {
            " --dart-define=API_BASE_URL={{api_base_url}}"
        } else {
            ""
        };

        return format!(
            "set shell := [\"bash\", \"-eu\", \"-c\"]\n\n\
default:\n\
\t@just --list\n\n\
workspace := \"workspace\"\n\n\
{extra_variable}\
restore:\n\
\t(cd {{{{workspace}}}} && flutter pub get)\n\n\
format:\n\
\t(cd {{{{workspace}}}} && dart format lib test integration_test)\n\n\
check-formatting:\n\
\t(cd {{{{workspace}}}} && dart format --output=none --set-exit-if-changed lib test integration_test)\n\n\
check-tests:\n\
\t(cd {{{{workspace}}}} && flutter test)\n\n\
devices:\n\
\t(cd {{{{workspace}}}} && flutter devices)\n\n\
emulators:\n\
\t(cd {{{{workspace}}}} && flutter emulators)\n\n\
run:\n\
\tjust run-web\n\n\
run-web:\n\
\t(cd {{{{workspace}}}} && flutter run -d chrome --web-port {FOR_ALL_FRONTEND_PORT}{web_define})\n\n\
run-ios device=\"\":\n\
\t#!/usr/bin/env bash\n\
\tset -euo pipefail\n\
\tnormalized_device='{{{{device}}}}'\n\
\tnormalized_device=\"${{normalized_device#\\\"}}\"\n\
\tnormalized_device=\"${{normalized_device%\\\"}}\"\n\
\tnormalized_device=\"${{normalized_device#device=}}\"\n\
\tif [ -n \"$normalized_device\" ]; then\n\
\t  (cd {{{{workspace}}}} && flutter run -d \"$normalized_device\"{mobile_define})\n\
\telse\n\
\t  echo 'Use `just devices` and rerun with device=\"<ios-device-id-or-name>\".' >&2\n\
\t  exit 1\n\
\tfi\n\n\
run-android device=\"\":\n\
\t#!/usr/bin/env bash\n\
\tset -euo pipefail\n\
\tnormalized_device='{{{{device}}}}'\n\
\tnormalized_device=\"${{normalized_device#\\\"}}\"\n\
\tnormalized_device=\"${{normalized_device%\\\"}}\"\n\
\tnormalized_device=\"${{normalized_device#device=}}\"\n\
\tif [ -n \"$normalized_device\" ]; then\n\
\t  (cd {{{{workspace}}}} && flutter run -d \"$normalized_device\"{mobile_define})\n\
\telse\n\
\t  echo 'Use `just devices` and rerun with device=\"<android-device-id-or-name>\".' >&2\n\
\t  exit 1\n\
\tfi\n\n\
run-macos:\n\
\t(cd {{{{workspace}}}} && flutter run -d macos{desktop_define})\n\n\
run-windows:\n\
\t(cd {{{{workspace}}}} && flutter run -d windows{desktop_define})\n\n\
run-linux:\n\
\t(cd {{{{workspace}}}} && flutter run -d linux{desktop_define})\n\n\
check-all:\n\
\tjust check-formatting\n\
\tjust check-tests\n"
        );
    }

    if is_astro_saying_hello_output_repo(spec) {
        return "set shell := [\"bash\", \"-eu\", \"-c\"]\n\n\
default:\n\
\t@just --list\n\n\
workspace := \"workspace\"\n\n\
restore:\n\
\tnpm --prefix {{workspace}} ci\n\n\
format:\n\
\tnpm --prefix {{workspace}} run format\n\n\
check-formatting:\n\
\tnpm --prefix {{workspace}} run check-formatting\n\n\
check-tests:\n\
\tnpm --prefix {{workspace}} run test\n\n\
run:\n\
\tnpm --prefix {{workspace}} run dev\n\n\
check-all:\n\
\tjust check-formatting\n\
\tjust check-tests\n"
            .to_string();
    }

    let solution_name = workspace_solution_name(&spec.project_slug);
    let adapter_project_name = adapter_project_name(&spec.project_slug);
    format!(
        "set shell := [\"bash\", \"-eu\", \"-c\"]\n\n\
         default:\n\
         \t@just --list\n\n\
         workspace := \"workspace\"\n\
         solution := \"{solution_name}.sln\"\n\
         adapter_project := \"workspace/src/{adapter_project_name}\"\n\n\
         restore:\n\
         \tdotnet restore {{{{workspace}}}}/{{{{solution}}}}\n\n\
         format:\n\
         \tdotnet format {{{{workspace}}}}/{{{{solution}}}}\n\n\
         check-formatting:\n\
         \tdotnet format {{{{workspace}}}}/{{{{solution}}}} --verify-no-changes\n\n\
         check-tests:\n\
         \tdotnet test {{{{workspace}}}}/{{{{solution}}}}\n\n\
         run *args:\n\
         \tdotnet run --project {{{{adapter_project}}}} -- {{{{args}}}}\n\n\
         check-all:\n\
         \tjust check-formatting\n\
         \tjust check-tests\n"
    )
}

fn render_output_repo_ci_workflow_content(spec: &OutputRepoSpec) -> String {
    if is_go_output_repo(spec) {
        return r#"name: CI

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - name: Check out code
        uses: actions/checkout@v6

      - name: Set up Go
        if: ${{ hashFiles('workspace/go.mod') != '' }}
        uses: actions/setup-go@v5
        with:
          go-version-file: workspace/go.mod

      - name: Verify formatting
        if: ${{ hashFiles('workspace/go.mod') != '' }}
        run: test -z "$(find workspace -name '*.go' -exec gofmt -l {} +)"

      - name: Test
        if: ${{ hashFiles('workspace/go.mod') != '' }}
        working-directory: workspace
        run: go test ./...
"#
        .to_string();
    }

    if is_flutter_output_repo(spec) {
        return r#"name: CI

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - name: Check out code
        uses: actions/checkout@v6

      - name: Set up Flutter
        if: ${{ hashFiles('workspace/pubspec.yaml') != '' }}
        uses: subosito/flutter-action@v2
        with:
          channel: stable
          cache: true

      - name: Restore
        if: ${{ hashFiles('workspace/pubspec.yaml') != '' }}
        working-directory: workspace
        run: flutter pub get

      - name: Verify formatting
        if: ${{ hashFiles('workspace/pubspec.yaml') != '' }}
        working-directory: workspace
        run: dart format --output=none --set-exit-if-changed lib test integration_test

      - name: Test
        if: ${{ hashFiles('workspace/pubspec.yaml') != '' }}
        working-directory: workspace
        run: flutter test
"#
        .to_string();
    }

    if is_astro_saying_hello_output_repo(spec) {
        return r#"name: CI

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - name: Check out code
        uses: actions/checkout@v6

      - name: Set up Node.js
        if: ${{ hashFiles('workspace/package-lock.json') != '' }}
        uses: actions/setup-node@v6
        with:
          node-version: 24
          cache: npm
          cache-dependency-path: workspace/package-lock.json

      - name: Install dependencies
        if: ${{ hashFiles('workspace/package-lock.json') != '' }}
        working-directory: workspace
        run: npm ci

      - name: Verify formatting
        if: ${{ hashFiles('workspace/package-lock.json') != '' }}
        working-directory: workspace
        run: npm run check-formatting

      - name: Test
        if: ${{ hashFiles('workspace/package-lock.json') != '' }}
        working-directory: workspace
        run: npm test
"#
        .to_string();
    }

    let solution_name = workspace_solution_name(&spec.project_slug);
    format!(
        r#"name: CI

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - name: Check out code
        uses: actions/checkout@v6

      - name: Set up .NET
        uses: actions/setup-dotnet@v5
        with:
          dotnet-version: 10.0.x

      - name: Verify formatting
        working-directory: workspace
        run: dotnet format {solution_name}.sln --verify-no-changes

      - name: Restore
        working-directory: workspace
        run: dotnet restore {solution_name}.sln

      - name: Test
        working-directory: workspace
        run: dotnet test {solution_name}.sln
"#
    )
}

fn build_output_repo_tutorial_files(app_root: &Path, spec: &OutputRepoSpec) -> Vec<ManagedRepoFile> {
    if is_go_saying_hello_output_repo(spec) {
        return build_go_saying_hello_output_repo_tutorial_files(app_root, spec);
    }

    if is_go_todo_list_output_repo(spec) {
        return build_go_todo_list_output_repo_tutorial_files(app_root, spec);
    }

    if is_flutter_saying_hello_output_repo(spec) {
        return build_flutter_saying_hello_output_repo_tutorial_files(app_root, spec);
    }

    if is_flutter_todo_list_output_repo(spec) {
        return build_flutter_todo_list_output_repo_tutorial_files(app_root, spec);
    }

    if is_astro_saying_hello_output_repo(spec) {
        return build_astro_saying_hello_output_repo_tutorial_files(app_root, spec);
    }

    let project_root = app_root.join("partials/projects").join(&spec.project_slug);
    let spec_partial =
        Partial::load(&project_root.join("spec/README.md")).expect("spec partial should exist");
    let code_partial =
        Partial::load(&project_root.join("instructions/core.md")).expect("core partial should exist");

    let adapter_xunit_path = project_root.join("instructions/adapter-xunit.md");
    let adapter_partial = if adapter_xunit_path.exists() {
        Partial::load(&adapter_xunit_path).expect("xUnit adapter partial should exist")
    } else {
        Partial::load(&project_root.join("instructions/adapter.md"))
            .expect("adapter partial should exist")
    };

    let contracts_path = project_root.join("instructions/contracts.md");
    let contracts_contents = if contracts_path.exists() {
        let partial = Partial::load(&contracts_path).expect("contracts partial should exist");
        let contracts_body = format!(
            "{}\n\n{}",
            render_output_repo_contracts_scaffold(spec),
            rewrite_for_single_repo_tutorial(&partial.body)
        );
        tutorial_file_markdown(
            "Contracts",
            &rewrite_stage_commit_checkpoints(&rewrite_touch_creation_checkpoints(&contracts_body)),
        )
    } else {
        generic_contracts_tutorial(spec, &spec_partial.body)
    };

    vec![
        ManagedRepoFile {
            relative_path: "tutorial/README.md".to_string(),
            contents: render_output_repo_tutorial_readme_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/setup.md".to_string(),
            contents: render_output_repo_setup_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/spec.md".to_string(),
            contents: tutorial_file_markdown(
                "Spec",
                &rewrite_for_single_repo_tutorial(&spec_partial.body),
            )
            .into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/contracts.md".to_string(),
            contents: contracts_contents.into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/code.md".to_string(),
            contents: tutorial_file_markdown(
                "Code",
                &rewrite_stage_commit_checkpoints(&rewrite_touch_creation_checkpoints(&format!(
                    "{}\n\n{}",
                    render_output_repo_code_scaffold(spec),
                    rewrite_output_repo_code_body(&rewrite_for_single_repo_tutorial(&code_partial.body))
                ))),
            )
            .into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/adapter.md".to_string(),
            contents: tutorial_file_markdown(
                "Adapter",
                &rewrite_stage_commit_checkpoints(&rewrite_touch_creation_checkpoints(&format!(
                    "{}\n\n{}",
                    render_output_repo_adapter_scaffold(spec),
                    rewrite_output_repo_adapter_body(&rewrite_for_single_repo_tutorial(&adapter_partial.body))
                ))),
            )
            .into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/finish.md".to_string(),
            contents: render_output_repo_finish_content(spec).into_bytes(),
        },
    ]
}

fn build_go_saying_hello_output_repo_tutorial_files(
    app_root: &Path,
    spec: &OutputRepoSpec,
) -> Vec<ManagedRepoFile> {
    let project_root = app_root.join("partials/projects").join(&spec.project_slug);
    let spec_partial =
        Partial::load(&project_root.join("spec/README.md")).expect("spec partial should exist");

    vec![
        ManagedRepoFile {
            relative_path: "tutorial/README.md".to_string(),
            contents: render_output_repo_tutorial_readme_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/setup.md".to_string(),
            contents: render_output_repo_setup_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/spec.md".to_string(),
            contents: tutorial_file_markdown(
                "Spec",
                &rewrite_for_single_repo_tutorial(&spec_partial.body),
            )
            .into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/contracts.md".to_string(),
            contents: render_go_saying_hello_contracts_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/code.md".to_string(),
            contents: render_go_saying_hello_code_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/adapter.md".to_string(),
            contents: render_go_saying_hello_adapter_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/finish.md".to_string(),
            contents: render_output_repo_finish_content(spec).into_bytes(),
        },
    ]
}

fn build_go_todo_list_output_repo_tutorial_files(
    app_root: &Path,
    spec: &OutputRepoSpec,
) -> Vec<ManagedRepoFile> {
    let project_root = app_root.join("partials/projects").join(&spec.project_slug);
    let spec_partial =
        Partial::load(&project_root.join("spec/README.md")).expect("spec partial should exist");
    let spec_contents = if is_go_todo_list_http_json_sqlite_output_repo(spec) {
        render_go_todo_list_rest_json_sqlite_spec_content()
    } else if is_go_todo_list_rest_json_postgres_output_repo(spec) {
        render_go_todo_list_rest_json_postgres_spec_content()
    } else {
        tutorial_file_markdown(
            "Spec",
            &rewrite_for_single_repo_tutorial(&spec_partial.body),
        )
    };

    vec![
        ManagedRepoFile {
            relative_path: "tutorial/README.md".to_string(),
            contents: render_output_repo_tutorial_readme_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/setup.md".to_string(),
            contents: render_output_repo_setup_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/spec.md".to_string(),
            contents: spec_contents.into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/contracts.md".to_string(),
            contents: render_go_todo_list_contracts_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/code.md".to_string(),
            contents: render_go_todo_list_code_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/adapter.md".to_string(),
            contents: render_go_todo_list_adapter_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/finish.md".to_string(),
            contents: render_output_repo_finish_content(spec).into_bytes(),
        },
    ]
}

fn build_flutter_saying_hello_output_repo_tutorial_files(
    app_root: &Path,
    spec: &OutputRepoSpec,
) -> Vec<ManagedRepoFile> {
    let project_root = app_root.join("partials/projects").join(&spec.project_slug);
    let spec_partial =
        Partial::load(&project_root.join("spec/README.md")).expect("spec partial should exist");

    vec![
        ManagedRepoFile {
            relative_path: "tutorial/README.md".to_string(),
            contents: render_output_repo_tutorial_readme_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/setup.md".to_string(),
            contents: render_output_repo_setup_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/spec.md".to_string(),
            contents: tutorial_file_markdown(
                "Spec",
                &rewrite_for_single_repo_tutorial(&spec_partial.body),
            )
            .into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/contracts.md".to_string(),
            contents: render_flutter_saying_hello_contracts_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/code.md".to_string(),
            contents: render_flutter_saying_hello_code_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/adapter.md".to_string(),
            contents: render_flutter_saying_hello_adapter_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/finish.md".to_string(),
            contents: render_output_repo_finish_content(spec).into_bytes(),
        },
    ]
}

fn build_flutter_todo_list_output_repo_tutorial_files(
    _app_root: &Path,
    spec: &OutputRepoSpec,
) -> Vec<ManagedRepoFile> {
    vec![
        ManagedRepoFile {
            relative_path: "tutorial/README.md".to_string(),
            contents: render_output_repo_tutorial_readme_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/setup.md".to_string(),
            contents: render_output_repo_setup_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/spec.md".to_string(),
            contents: render_go_todo_list_rest_json_sqlite_spec_content().into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/contracts.md".to_string(),
            contents: render_flutter_todo_list_rest_json_contracts_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/code.md".to_string(),
            contents: render_flutter_todo_list_rest_json_code_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/adapter.md".to_string(),
            contents: render_flutter_todo_list_rest_json_adapter_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/finish.md".to_string(),
            contents: render_output_repo_finish_content(spec).into_bytes(),
        },
    ]
}

fn build_astro_saying_hello_output_repo_tutorial_files(
    app_root: &Path,
    spec: &OutputRepoSpec,
) -> Vec<ManagedRepoFile> {
    let project_root = app_root.join("partials/projects").join(&spec.project_slug);
    let spec_partial =
        Partial::load(&project_root.join("spec/README.md")).expect("spec partial should exist");

    vec![
        ManagedRepoFile {
            relative_path: "tutorial/README.md".to_string(),
            contents: render_output_repo_tutorial_readme_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/setup.md".to_string(),
            contents: render_output_repo_setup_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/spec.md".to_string(),
            contents: tutorial_file_markdown(
                "Spec",
                &rewrite_for_single_repo_tutorial(&spec_partial.body),
            )
            .into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/contracts.md".to_string(),
            contents: render_astro_saying_hello_contracts_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/code.md".to_string(),
            contents: render_astro_saying_hello_code_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/adapter.md".to_string(),
            contents: render_astro_saying_hello_adapter_content(spec).into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/finish.md".to_string(),
            contents: render_output_repo_finish_content(spec).into_bytes(),
        },
    ]
}

fn render_output_repo_tutorial_readme_content(spec: &OutputRepoSpec) -> String {
    if is_flutter_output_repo(spec) {
        let mut choices = vec![
            format!("- Project: `{}`", spec.project_slug),
            "- Workspace folder: `workspace/`".to_string(),
            "- Ecosystem: `Dart`".to_string(),
            "- Language: `Dart`".to_string(),
            "- Unit testing: `test`".to_string(),
            "- Widget testing: `flutter_test`".to_string(),
            "- Integration testing: `integration_test`".to_string(),
            "- Mocking: `mocktail`".to_string(),
            format!(
                "- Storage: `{}`",
                repo_choice_display(&spec.selections.storage)
            ),
            format!(
                "- Surface: `{}`",
                repo_choice_display(&spec.selections.surface)
            ),
            format!(
                "- Target: `{}`",
                repo_choice_display(&spec.selections.target)
            ),
            "- Framework: `Flutter`".to_string(),
            "- Platforms: `web`, `ios`, `android`, `macos`, `windows`, `linux`".to_string(),
        ];

        if let Some(protocol) = &spec.selections.protocol {
            choices.push(format!("- Protocol: `{}`", repo_choice_display(protocol)));
        }

        if is_flutter_http_output_repo(spec) {
            choices.push(format!("- API Port: `{FOR_ALL_API_PORT}`"));
        }

        choices.push(format!("- App Port: `{FOR_ALL_FRONTEND_PORT}`"));

        return format!(
            "# Tutorial\n\nChoices for this repo:\n\n{}\n\nWork through these files in order:\n\n1. [Spec](spec.md)\n2. [Setup](setup.md)\n3. [Contracts](contracts.md)\n4. [Code](code.md)\n5. [Adapter](adapter.md)\n6. [Finish](finish.md)\n",
            choices.join("\n")
        );
    }

    let mut choices = vec![
        format!("- Project: `{}`", spec.project_slug),
        "- Workspace folder: `workspace/`".to_string(),
        format!(
            "- Ecosystem: `{}`",
            repo_choice_display(&spec.selections.ecosystem)
        ),
        format!(
            "- Language: `{}`",
            repo_choice_display(&spec.selections.language)
        ),
        format!(
            "- Testing: `{}`",
            repo_choice_display(&spec.selections.testing)
        ),
        format!(
            "- Mocking: `{}`",
            repo_choice_display(&spec.selections.mocking)
        ),
        format!(
            "- Storage: `{}`",
            repo_choice_display(&spec.selections.storage)
        ),
        format!(
            "- Surface: `{}`",
            repo_choice_display(&spec.selections.surface)
        ),
        format!(
            "- Target: `{}`",
            repo_choice_display(&spec.selections.target)
        ),
        format!(
            "- Framework: `{}`",
            repo_choice_display(&spec.selections.framework)
        ),
    ];

    if let Some(protocol) = &spec.selections.protocol {
        choices.push(format!("- Protocol: `{}`", repo_choice_display(protocol)));
    }

    if is_go_output_repo(spec) {
        choices.push(format!("- API Port: `{FOR_ALL_API_PORT}`"));
        choices.push(format!("- App Port: `{FOR_ALL_FRONTEND_PORT}`"));
    }

    if is_astro_saying_hello_output_repo(spec) {
        choices.push(format!("- API Port: `{FOR_ALL_API_PORT}`"));
        choices.push(format!("- App Port: `{FOR_ALL_FRONTEND_PORT}`"));
    }

    format!(
        "# Tutorial\n\nChoices for this repo:\n\n{}\n\nWork through these files in order:\n\n1. [Spec](spec.md)\n2. [Setup](setup.md)\n3. [Contracts](contracts.md)\n4. [Code](code.md)\n5. [Adapter](adapter.md)\n6. [Finish](finish.md)\n",
        choices.join("\n"),
    )
}

fn render_output_repo_setup_content(spec: &OutputRepoSpec) -> String {
    if is_go_saying_hello_output_repo(spec) {
        let module_path = format!(
            "github.com/{}/{}/workspace",
            GITHUB_OWNER, spec.repo_name
        );
        let setup_commands = vec![
            "curl -L -s https://raw.githubusercontent.com/github/gitignore/refs/heads/main/Go.gitignore > workspace/.gitignore".to_string(),
            "printf '\\n# Repo-local tools\\nbin/\\n\\n# Local runtime data\\ndata/\\n' >> workspace/.gitignore".to_string(),
            format!("(cd workspace && go mod init {module_path})"),
            "(cd workspace && go get github.com/labstack/echo/v4)".to_string(),
            "(cd workspace && go get github.com/labstack/echo/v4/middleware)".to_string(),
            "(cd workspace && go get github.com/stretchr/testify/assert github.com/stretchr/testify/mock)".to_string(),
        ];
        return tutorial_file_markdown(
            "Setup",
            &format!(
                "Keep the repository root for shared files like `README.md`, `LICENSE`, `.gitignore`, `.github/`, `justfile`, and `tutorial/`.\n\nPut all Go code inside a single `workspace/` folder.\n\nFrom the repository root, run each setup command and checkpoint it before moving to the next one:\n\n```bash\n{}\n```\n\nThis gives you:\n\n- a root-level `.gitignore` for operating-system noise and editor leftovers\n- a `workspace/.gitignore` for standard Go build output, local tooling files, and local runtime data\n\nWhen the full workspace is finished, it should contain these files:\n\n```text\nworkspace/\n  .gitignore\n  go.mod\n  go.sum\n  cmd/\n    server/\n      main.go\n  internal/\n    contracts/\n      greeting.go\n    code/\n      greeting_service.go\n      greeting_service_test.go\n    adapter/\n      http/\n        greeting_handler.go\n        greeting_handler_test.go\n```",
                render_setup_commands_with_commits(&setup_commands, 0)
            ),
        );
    }

    if is_go_todo_list_http_json_sqlite_output_repo(spec) {
        let module_path = format!(
            "github.com/{}/{}/workspace",
            GITHUB_OWNER, spec.repo_name
        );
        let setup_commands = vec![
            "curl -L -s https://raw.githubusercontent.com/github/gitignore/refs/heads/main/Go.gitignore > workspace/.gitignore".to_string(),
            "printf '\\n# Repo-local tools\\nbin/\\n\\n# Local runtime data\\ndata/\\n' >> workspace/.gitignore".to_string(),
            format!("(cd workspace && go mod init {module_path})"),
            "(cd workspace && go get github.com/labstack/echo/v4)".to_string(),
            "(cd workspace && go get github.com/labstack/echo/v4/middleware)".to_string(),
            "(cd workspace && go get modernc.org/sqlite)".to_string(),
            "(cd workspace && go get github.com/stretchr/testify/assert github.com/stretchr/testify/mock)".to_string(),
            "mkdir -p workspace/data".to_string(),
        ];
        return tutorial_file_markdown(
            "Setup",
            &format!(
                "Keep the repository root for shared files like `README.md`, `LICENSE`, `.gitignore`, `.github/`, `justfile`, and `tutorial/`.\n\nPut all Go code inside a single `workspace/` folder.\n\nFrom the repository root, run each setup command and checkpoint it before moving to the next one:\n\n```bash\n{}\n```\n\nThis gives you:\n\n- a root-level `.gitignore` for operating-system noise and editor leftovers\n- a `workspace/.gitignore` for standard Go build output, local tooling files, and local runtime data\n- a `workspace/data/tasks.db` file path for the durable SQLite task store used by the REST API adapter\n\nWhen the full workspace is finished, it should contain these files:\n\n```text\nworkspace/\n  .gitignore\n  go.mod\n  go.sum\n  cmd/\n    server/\n      main.go\n  data/\n    tasks.db\n  internal/\n    contracts/\n      task_api.go\n    code/\n      task_service.go\n      task_service_test.go\n    adapter/\n      http/\n        task_handler.go\n        task_handler_test.go\n      storage/\n        sqlite_task_store.go\n        sqlite_task_store_test.go\n```",
                render_setup_commands_with_commits(&setup_commands, 0)
            ),
        );
    }

    if is_go_todo_list_rest_json_postgres_output_repo(spec) {
        let module_path = format!(
            "github.com/{}/{}/workspace",
            GITHUB_OWNER, spec.repo_name
        );
        let setup_commands = vec![
            "curl -L -s https://raw.githubusercontent.com/github/gitignore/refs/heads/main/Go.gitignore > workspace/.gitignore".to_string(),
            "printf '\\n# Repo-local tools\\nbin/\\n\\n# Local runtime data\\ndata/\\n' >> workspace/.gitignore".to_string(),
            format!("(cd workspace && go mod init {module_path})"),
            "(cd workspace && GOBIN=$(pwd)/bin go install github.com/sqlc-dev/sqlc/cmd/sqlc@v1.30.0)".to_string(),
            "(cd workspace && go get github.com/labstack/echo/v4)".to_string(),
            "(cd workspace && go get github.com/labstack/echo/v4/middleware)".to_string(),
            "(cd workspace && go get github.com/jackc/pgx/v5/stdlib)".to_string(),
            "(cd workspace && go get github.com/DATA-DOG/go-sqlmock)".to_string(),
            "(cd workspace && go get github.com/stretchr/testify/assert github.com/stretchr/testify/mock)".to_string(),
            "mkdir -p workspace/db/query".to_string(),
            "touch workspace/sqlc.yaml".to_string(),
            "touch workspace/db/schema.sql".to_string(),
            "touch workspace/db/query/tasks.sql".to_string(),
        ];
        return tutorial_file_markdown(
            "Setup",
            &format!(
                "Keep the repository root for shared files like `README.md`, `LICENSE`, `.gitignore`, `.github/`, `justfile`, and `tutorial/`.\n\nPut all Go code inside a single `workspace/` folder.\n\nFrom the repository root, run each setup command and checkpoint it before moving to the next one:\n\n```bash\n{}\n```\n\nPut this exact content in `workspace/sqlc.yaml`:\n\n```yaml\nversion: \"2\"\nsql:\n  - engine: \"postgresql\"\n    schema: \"db/schema.sql\"\n    queries: \"db/query/tasks.sql\"\n    gen:\n      go:\n        package: \"storedb\"\n        out: \"internal/adapter/storage/db\"\n        sql_package: \"database/sql\"\n```\n\nPut this exact content in `workspace/db/schema.sql`:\n\n```sql\nCREATE TABLE IF NOT EXISTS tasks (\n  id TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,\n  text TEXT NOT NULL\n);\n```\n\nPut this exact content in `workspace/db/query/tasks.sql`:\n\n```sql\n-- name: ListTasks :many\nSELECT id, text\nFROM tasks\nORDER BY text ASC, id ASC;\n\n-- name: CreateTask :one\nINSERT INTO tasks (text)\nVALUES ($1)\nRETURNING id, text;\n\n-- name: GetTask :one\nSELECT id, text\nFROM tasks\nWHERE id = $1\nLIMIT 1;\n\n-- name: DeleteTask :execrows\nDELETE FROM tasks\nWHERE id = $1;\n```\n\nThen run:\n\n```bash\njust format\ngit add --all\ngit commit --message \"Add sqlc configuration and queries\"\n```\n\nThis gives you:\n\n- a root-level `.gitignore` for operating-system noise and editor leftovers\n- a `workspace/.gitignore` for standard Go build output, local tooling files, and local runtime data\n- a Postgres-backed REST adapter that reads its connection string from `TODO_LIST_DATABASE_URL`\n- a generated root `justfile` that defaults `database_url` to `postgres://postgres@localhost:5432/todo_list?sslmode=disable`\n- a repo-local `workspace/bin/sqlc` installation for generating Go query code from `workspace/sqlc.yaml`, `workspace/db/schema.sql`, and `workspace/db/query/tasks.sql`\n\nBefore you run the server, create the default tutorial database with:\n\n```bash\ncreatedb --host localhost --username postgres todo_list\n```\n\nIf that does not match your local Postgres setup, create an equivalent database your user can access and override the generated `database_url` value in the root `justfile`.\n\nWhen the full workspace is finished, it should contain these files:\n\n```text\nworkspace/\n  .gitignore\n  bin/\n    sqlc\n  go.mod\n  go.sum\n  sqlc.yaml\n  db/\n    schema.sql\n    query/\n      tasks.sql\n  cmd/\n    server/\n      main.go\n  internal/\n    contracts/\n      task_api.go\n    code/\n      task_service.go\n      task_service_test.go\n    adapter/\n      http/\n        task_handler.go\n        task_handler_test.go\n      storage/\n        postgres_task_store.go\n        postgres_task_store_test.go\n        db/\n          ...generated Go files from sqlc...\n```",
                render_setup_commands_with_commits(&setup_commands, 0)
            ),
        );
    }

    if is_go_todo_list_http_json_output_repo(spec) {
        let module_path = format!(
            "github.com/{}/{}/workspace",
            GITHUB_OWNER, spec.repo_name
        );
        let setup_commands = vec![
            "curl -L -s https://raw.githubusercontent.com/github/gitignore/refs/heads/main/Go.gitignore > workspace/.gitignore".to_string(),
            "printf '\\n# Repo-local tools\\nbin/\\n\\n# Local runtime data\\ndata/\\n' >> workspace/.gitignore".to_string(),
            format!("(cd workspace && go mod init {module_path})"),
            "(cd workspace && go get github.com/labstack/echo/v4)".to_string(),
            "(cd workspace && go get github.com/labstack/echo/v4/middleware)".to_string(),
            "(cd workspace && go get github.com/stretchr/testify/assert github.com/stretchr/testify/mock)".to_string(),
            "mkdir -p workspace/data".to_string(),
        ];
        return tutorial_file_markdown(
            "Setup",
            &format!(
                "Keep the repository root for shared files like `README.md`, `LICENSE`, `.gitignore`, `.github/`, `justfile`, and `tutorial/`.\n\nPut all Go code inside a single `workspace/` folder.\n\nFrom the repository root, run each setup command and checkpoint it before moving to the next one:\n\n```bash\n{}\n```\n\nThis gives you:\n\n- a root-level `.gitignore` for operating-system noise and editor leftovers\n- a `workspace/.gitignore` for standard Go build output, local tooling files, and local runtime data\n- a `workspace/data/tasks.json` file path for the durable local JSON task store used by the API adapter\n\nWhen the full workspace is finished, it should contain these files:\n\n```text\nworkspace/\n  .gitignore\n  go.mod\n  go.sum\n  cmd/\n    server/\n      main.go\n  data/\n    tasks.json\n  internal/\n    contracts/\n      task_list_service.go\n    code/\n      task_list_service.go\n      task_list_service_test.go\n    adapter/\n      http/\n        task_handler.go\n        task_handler_test.go\n      storage/\n        json_task_store.go\n        json_task_store_test.go\n```",
                render_setup_commands_with_commits(&setup_commands, 0)
            ),
        );
    }

    if is_flutter_todo_list_output_repo(spec) {
        let package_name = flutter_package_name(&spec.project_slug);
        let setup_commands = vec![
            format!("flutter create --platforms=web,android,ios,macos,windows,linux --org com.intrepion --project-name {package_name} workspace"),
            "rm workspace/test/widget_test.dart".to_string(),
            "(cd workspace && flutter pub add http)".to_string(),
            "(cd workspace && flutter pub add --dev test)".to_string(),
            "(cd workspace && flutter pub add --dev mocktail)".to_string(),
            "(cd workspace && flutter pub add --dev integration_test --sdk flutter)".to_string(),
        ];
        let macos_entitlements_note = r#"Put this exact content in `workspace/macos/Runner/DebugProfile.entitlements`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>com.apple.security.app-sandbox</key>
	<true/>
	<key>com.apple.security.cs.allow-jit</key>
	<true/>
	<key>com.apple.security.network.client</key>
	<true/>
	<key>com.apple.security.network.server</key>
	<true/>
</dict>
</plist>
```

Put this exact content in `workspace/macos/Runner/Release.entitlements`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>com.apple.security.app-sandbox</key>
	<true/>
	<key>com.apple.security.network.client</key>
	<true/>
</dict>
</plist>
```

Then run:

```bash
git add --all
git commit --message "Enable macOS network client entitlement"
```"#;
        let macos_http_note = format!(
            "\nIf the macOS desktop app does not appear to reach the API, rerun it with the host URL made explicit:\n\n```bash\njust --set api_base_url http://localhost:{FOR_ALL_API_PORT} run-macos\n```"
        );

        return tutorial_file_markdown(
            "Setup",
            &format!(
                "Keep the repository root for shared files like `README.md`, `LICENSE`, `.gitignore`, `.github/`, `justfile`, and `tutorial/`.\n\nPut all Flutter code inside a single `workspace/` folder.\n\nFrom the repository root, run each setup command and checkpoint it before moving to the next one:\n\n```bash\n{}\n```\n\n{}\n\nWhen the full workspace is finished, it should contain these files:\n\n```text\nworkspace/\n  pubspec.yaml\n  lib/\n    contracts/\n      task.dart\n      task_api.dart\n      task_list_response.dart\n    code/\n      task_list_controller.dart\n    adapter/\n      http_task_api.dart\n      todo_list_page.dart\n  test/\n    code/\n      task_list_controller_test.dart\n    adapter/\n      http_task_api_test.dart\n      todo_list_page_test.dart\n  integration_test/\n    app_test.dart\n  lib/main.dart\n```\n\nBefore you try any run command, make sure Flutter can see a supported target:\n\n```bash\njust devices\n```\n\nFor web, use the default web command:\n\n```bash\njust run\n```\n\nor, explicitly:\n\n```bash\njust run-web\n```\n\nOn macOS for iOS, install CocoaPods first if you have not already:\n\n```bash\nsudo gem install cocoapods\n```\n\nThen open the simulator, list devices, and run the iOS app with an actual simulator id or name:\n\n```bash\nopen -a Simulator\njust devices\njust run-ios device=\"<ios-device-id-or-name>\"\n```\n\nFlutter does not accept bare `ios` as a generic simulator target, so if `just devices` does not show your simulator yet, wait a moment and run it again.\n\nFor Android, list available emulators, launch one, list devices again, and then run the Android app:\n\n```bash\njust emulators\nflutter emulators --launch <emulator-id>\njust devices\njust --set api_base_url http://10.0.2.2:{FOR_ALL_API_PORT} run-android device=\"<android-device-id-or-name>\"\n```\n\nFor macOS desktop, use:\n\n```bash\njust run-macos\n```\n{}{newline}For Windows or Linux, run the matching command on that host platform:\n\n```bash\njust run-windows\njust run-linux\n```\n\nAfter your first successful iOS run, if CocoaPods added shared iOS project files like these:\n\n- `workspace/ios/Runner.xcodeproj/project.pbxproj`\n- `workspace/ios/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/ios/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add iOS CocoaPods workspace files\"\n```\n\nAfter your first successful macOS run, if CocoaPods added shared macOS project files like these:\n\n- `workspace/macos/Runner.xcodeproj/project.pbxproj`\n- `workspace/macos/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/macos/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add macOS CocoaPods workspace files\"\n```\n\nDo not commit local machine output like these:\n\n- `workspace/ios/Pods/`\n- `workspace/macos/Pods/`\n- `workspace/build/`\n- `workspace/.dart_tool/`\n\nFor Android, a normal first run usually should not add shared tracked files. If it does change shared files under `workspace/android/`, review them carefully and commit only the project-level changes. Do not commit machine-specific files like:\n\n- `workspace/android/local.properties`\n- `workspace/.gradle/`\n- `workspace/build/`",
                render_setup_commands_with_commits(&setup_commands, 2),
                macos_entitlements_note,
                macos_http_note,
                newline = if macos_http_note.is_empty() { "" } else { "\n" },
            ),
        );
    }

    if is_flutter_saying_hello_output_repo(spec) {
        let mut setup_commands = vec![
            "flutter create --platforms=web,android,ios,macos,windows,linux --org com.intrepion --project-name saying_hello workspace".to_string(),
            "rm workspace/test/widget_test.dart".to_string(),
            "(cd workspace && flutter pub add --dev test)".to_string(),
            "(cd workspace && flutter pub add --dev mocktail)".to_string(),
            "(cd workspace && flutter pub add --dev integration_test --sdk flutter)".to_string(),
        ];

        if is_flutter_http_saying_hello_output_repo(spec) {
            setup_commands.insert(2, "(cd workspace && flutter pub add http)".to_string());
        }

        let workspace_tree = if is_flutter_local_saying_hello_output_repo(spec) {
            r#"workspace/
  pubspec.yaml
  lib/
    contracts/
      greeting_service.dart
    code/
      default_greeting_service.dart
    adapter/
      greeting_page.dart
  test/
    code/
      default_greeting_service_test.dart
    adapter/
      greeting_page_test.dart
  integration_test/
    app_test.dart
  lib/main.dart"#
                .to_string()
        } else {
            r#"workspace/
  pubspec.yaml
  lib/
    contracts/
      greeting_api.dart
      greeting_response.dart
    code/
      load_greeting.dart
    adapter/
      http_greeting_api.dart
      greeting_page.dart
  test/
    code/
      load_greeting_test.dart
    adapter/
      http_greeting_api_test.dart
      greeting_page_test.dart
  integration_test/
      app_test.dart
  lib/main.dart"#
                .to_string()
        };

        let android_http_note = if is_flutter_http_saying_hello_output_repo(spec) {
            format!(
                "\nIf you are running the `http-json` variant on Android, use the host-machine override when you launch the app:\n\n```bash\njust --set api_base_url http://10.0.2.2:{FOR_ALL_API_PORT} run-android device=\"<android-device-id-or-name>\"\n```"
            )
        } else {
            String::new()
        };
        let macos_http_note = if is_flutter_http_saying_hello_output_repo(spec) {
            format!(
                "\nIf the macOS desktop app does not appear to reach the API, rerun it with the host URL made explicit:\n\n```bash\njust --set api_base_url http://localhost:{FOR_ALL_API_PORT} run-macos\n```"
            )
        } else {
            String::new()
        };
        let macos_entitlements_note = if is_flutter_http_saying_hello_output_repo(spec) {
            r#"Put this exact content in `workspace/macos/Runner/DebugProfile.entitlements`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>com.apple.security.app-sandbox</key>
	<true/>
	<key>com.apple.security.cs.allow-jit</key>
	<true/>
	<key>com.apple.security.network.client</key>
	<true/>
	<key>com.apple.security.network.server</key>
	<true/>
</dict>
</plist>
```

Put this exact content in `workspace/macos/Runner/Release.entitlements`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>com.apple.security.app-sandbox</key>
	<true/>
	<key>com.apple.security.network.client</key>
	<true/>
</dict>
</plist>
```

Then run:

```bash
git add --all
git commit --message "Enable macOS network client entitlement"
```"#
                .to_string()
        } else {
            String::new()
        };

        return tutorial_file_markdown(
            "Setup",
            &format!(
                "Keep the repository root for shared files like `README.md`, `LICENSE`, `.gitignore`, `.github/`, `justfile`, and `tutorial/`.\n\nPut all Flutter code inside a single `workspace/` folder.\n\nFrom the repository root, run each setup command and checkpoint it before moving to the next one:\n\n```bash\n{}\n```\n\n{}{entitlement_newline}When the full workspace is finished, it should contain these files:\n\n```text\n{workspace_tree}\n```\n\nBefore you try any run command, make sure Flutter can see a supported target:\n\n```bash\njust devices\n```\n\nFor web, use the default web command:\n\n```bash\njust run\n```\n\nor, explicitly:\n\n```bash\njust run-web\n```\n\nOn macOS for iOS, install CocoaPods first if you have not already:\n\n```bash\nsudo gem install cocoapods\n```\n\nThen open the simulator, list devices, and run the iOS app with an actual simulator id or name:\n\n```bash\nopen -a Simulator\njust devices\njust run-ios device=\"<ios-device-id-or-name>\"\n```\n\nFlutter does not accept bare `ios` as a generic simulator target, so if `just devices` does not show your simulator yet, wait a moment and run it again.\n\nFor Android, list available emulators, launch one, list devices again, and then run the Android app:\n\n```bash\njust emulators\nflutter emulators --launch <emulator-id>\njust devices\njust run-android device=\"<android-device-id-or-name>\"\n```\n{android_http_note}{newline}For macOS desktop, use:\n\n```bash\njust run-macos\n```\n{macos_http_note}{macos_newline}For Windows or Linux, run the matching command on that host platform:\n\n```bash\njust run-windows\njust run-linux\n```\n\nAfter your first successful iOS run, if CocoaPods added shared iOS project files like these:\n\n- `workspace/ios/Runner.xcodeproj/project.pbxproj`\n- `workspace/ios/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/ios/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add iOS CocoaPods workspace files\"\n```\n\nAfter your first successful macOS run, if CocoaPods added shared macOS project files like these:\n\n- `workspace/macos/Runner.xcodeproj/project.pbxproj`\n- `workspace/macos/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/macos/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add macOS CocoaPods workspace files\"\n```\n\nDo not commit local machine output like these:\n\n- `workspace/ios/Pods/`\n- `workspace/macos/Pods/`\n- `workspace/build/`\n- `workspace/.dart_tool/`\n\nFor Android, a normal first run usually should not add shared tracked files. If it does change shared files under `workspace/android/`, review them carefully and commit only the project-level changes. Do not commit machine-specific files like:\n\n- `workspace/android/local.properties`\n- `workspace/.gradle/`\n- `workspace/build/`",
                render_setup_commands_with_commits(&setup_commands, 1),
                macos_entitlements_note,
                workspace_tree = workspace_tree,
                android_http_note = android_http_note,
                macos_http_note = macos_http_note,
                entitlement_newline = if macos_entitlements_note.is_empty() { "" } else { "\n\n" },
                newline = if android_http_note.is_empty() { "" } else { "\n" },
                macos_newline = if macos_http_note.is_empty() { "" } else { "\n" },
            ),
        );
    }

    if is_astro_saying_hello_output_repo(spec) {
        let setup_commands = vec![
            "curl -L -s https://raw.githubusercontent.com/github/gitignore/refs/heads/main/Node.gitignore > workspace/.gitignore".to_string(),
            "printf '\\n# Astro\\n.astro/\\ndist/\\n\\n# Vitest\\ncoverage/\\n' >> workspace/.gitignore".to_string(),
            "(cd workspace && npm init --yes)".to_string(),
            "(cd workspace && npm install astro)".to_string(),
            "(cd workspace && npm install --save-dev typescript vitest jsdom prettier @types/node)".to_string(),
            "(cd workspace && npm pkg set private=true)".to_string(),
            "(cd workspace && npm pkg set type=module)".to_string(),
            "(cd workspace && npm pkg delete main)".to_string(),
            format!(
                "(cd workspace && npm pkg set scripts.dev=\"astro dev --host 0.0.0.0 --port {FOR_ALL_FRONTEND_PORT}\")"
            ),
            "(cd workspace && npm pkg set scripts.build=\"astro build\")".to_string(),
            format!(
                "(cd workspace && npm pkg set scripts.preview=\"astro preview --host 0.0.0.0 --port {FOR_ALL_FRONTEND_PORT}\")"
            ),
            "(cd workspace && npm pkg set scripts.format=\"prettier --write .\")".to_string(),
            "(cd workspace && npm pkg set scripts.check-formatting=\"prettier --check .\")".to_string(),
            "(cd workspace && npm pkg set scripts.test=\"vitest run\")".to_string(),
            "touch workspace/astro.config.mjs".to_string(),
            "touch workspace/tsconfig.json".to_string(),
            "touch workspace/vitest.config.ts".to_string(),
            "touch workspace/src/env.d.ts".to_string(),
        ];
        return tutorial_file_markdown(
            "Setup",
            &format!(
                "Keep the repository root for shared files like `README.md`, `LICENSE`, `.gitignore`, `.github/`, `justfile`, and `tutorial/`.\n\nPut all Astro code inside a single `workspace/` folder.\n\nFrom the repository root, run each setup command and checkpoint it before moving to the next one:\n\n```bash\n{}\n```\n\nPut this exact content in `workspace/astro.config.mjs`:\n\n```js\nimport {{ defineConfig }} from 'astro/config';\n\nexport default defineConfig({{}});\n```\n\nPut this exact content in `workspace/tsconfig.json`:\n\n```json\n{{\n  \"extends\": \"astro/tsconfigs/strict\"\n}}\n```\n\nPut this exact content in `workspace/vitest.config.ts`:\n\n```ts\n/// <reference types=\"vitest/config\" />\n\nimport {{ getViteConfig }} from 'astro/config';\n\nexport default getViteConfig({{\n  test: {{\n    environment: 'node',\n  }},\n}});\n```\n\nPut this exact content in `workspace/src/env.d.ts`:\n\n```ts\n/// <reference types=\"astro/client\" />\n```\n\nAfter those setup files have their final contents, run:\n\n```bash\njust format\ngit add --all\ngit commit --message \"Add Astro workspace configuration files\"\n```\n\nThe browser-binding test later in `tutorial/adapter.md` opts into `jsdom` explicitly. Everything else can stay on the default Node test environment.\n\nWhen the full workspace is finished, it should contain these files:\n\n```text\nworkspace/\n  .gitignore\n  astro.config.mjs\n  package.json\n  package-lock.json\n  tsconfig.json\n  vitest.config.ts\n  src/\n    env.d.ts\n    contracts/\n      greeting-api.ts\n      greeting-response.ts\n    code/\n      load-greeting.ts\n      load-greeting.test.ts\n    adapter/\n      http-greeting-api.ts\n      http-greeting-api.test.ts\n      bind-greeting-form.ts\n      bind-greeting-form.test.ts\n      index-page.test.ts\n    pages/\n      index.astro\n```",
                render_setup_commands_with_commits(&setup_commands, 11)
            ),
        );
    }

    let solution_name = workspace_solution_name(&spec.project_slug);
    let contracts_project_name = contracts_project_name(&spec.project_slug);
    let code_project_name = code_project_name(&spec.project_slug);
    let code_test_project_name = code_test_project_name(&spec.project_slug);
    let adapter_project_name = adapter_project_name(&spec.project_slug);
    let adapter_test_project_name = adapter_test_project_name(&spec.project_slug);
    let setup_commands = vec![
        format!(
            "dotnet new sln --format sln --name {solution_name} --output workspace"
        ),
        "dotnet new gitignore --output workspace".to_string(),
    ];

    tutorial_file_markdown(
        "Setup",
        &format!(
            "Keep the repository root for shared files like `README.md`, `LICENSE`, `.gitignore`, `.github/`, and `tutorial/`.\n\nPut all .NET code inside a single `workspace/` folder.\n\nFrom the repository root, run each setup command and checkpoint it before moving to the next one:\n\n```bash\n{}\n```\n\nThis gives you:\n\n- a root-level `.gitignore` for operating-system noise and editor leftovers\n- a `workspace/.gitignore` for standard `.NET` build output and local tooling files\n\nWhen the full workspace is finished, it should contain these projects:\n\n- `workspace/src/{contracts_project_name}`\n- `workspace/src/{code_project_name}`\n- `workspace/tests/{code_test_project_name}`\n- `workspace/src/{adapter_project_name}`\n- `workspace/tests/{adapter_test_project_name}`\n\nThe next files assume this layout:\n\n```text\nworkspace/\n  .gitignore\n  {solution_name}.sln\n  src/\n    {contracts_project_name}/\n    {code_project_name}/\n    {adapter_project_name}/\n  tests/\n    {code_test_project_name}/\n    {adapter_test_project_name}/\n```",
            render_setup_commands_with_commits(&setup_commands, 0)
        ),
    )
}

fn render_setup_commands_with_commits(commands: &[String], format_start_index: usize) -> String {
    let mut ensured_touch_dirs = BTreeSet::new();
    let mut rendered_blocks = vec!["rm -rf workspace".to_string(), "mkdir -p workspace".to_string()];

    for (index, command) in commands.iter().enumerate() {
        let mut command_lines = Vec::new();
        if let Some(dir) = touch_parent_dir(command) {
            if ensured_touch_dirs.insert(dir.clone()) {
                command_lines.push(format!("mkdir -p {dir}"));
            }
        }
        command_lines.push(command.clone());

        if command.starts_with("mkdir -p ") {
            rendered_blocks.push(command_lines.join("\n"));
            continue;
        }

        let maybe_format = if index >= format_start_index {
            "just format\n"
        } else {
            ""
        };
        rendered_blocks.push(format!(
            "{}\n{maybe_format}git add --all\ngit commit --message \"{}\"",
            command_lines.join("\n"),
            escape_shell_double_quoted(command)
        ));
    }

    rendered_blocks.join("\n\n")
}

fn touch_parent_dir(command: &str) -> Option<String> {
    let path = command.strip_prefix("touch ")?;
    let parent = Path::new(path.trim()).parent()?;
    let parent = parent.to_string_lossy().to_string();
    if parent.is_empty() || parent == "." {
        None
    } else {
        Some(parent)
    }
}

fn escape_shell_double_quoted(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$")
        .replace('`', "\\`")
}

fn tutorial_file_markdown(title: &str, body: &str) -> String {
    format!(
        "# {title}\n\n{}\n",
        rewrite_tutorial_checkpoint_commands(&normalize_text(body))
    )
}

fn rewrite_tutorial_checkpoint_commands(text: &str) -> String {
    text.replace(
        "Run:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"",
        "Run:\n\n```bash\njust format\njust check-all\ngit add --all\ngit commit --message \"",
    )
}

fn rewrite_touch_creation_checkpoints(text: &str) -> String {
    rewrite_touch_creation_checkpoints_with_check_all(text, false)
}

fn rewrite_touch_creation_stage_only(text: &str) -> String {
    rewrite_touch_creation_checkpoints_with_check_all(text, false)
}

fn rewrite_touch_creation_checkpoints_with_check_all(
    text: &str,
    include_check_all: bool,
) -> String {
    let fenced_block_re = Regex::new(r"(?s)```bash\n(?P<body>.*?)\n```").expect("valid fenced bash regex");

    fenced_block_re
        .replace_all(text, |captures: &regex::Captures| {
            let body = captures
                .name("body")
                .map(|value| value.as_str())
                .unwrap_or_default();

            if !body.lines().any(|line| line.starts_with("touch ")) {
                return captures
                    .get(0)
                    .map(|value| value.as_str())
                    .unwrap_or_default()
                    .to_string();
            }

            let mut ensured_dirs = BTreeSet::new();
            let rewritten_body = body
                .lines()
                .flat_map(|line| {
                    if let Some(dir) = line.strip_prefix("mkdir -p ") {
                        ensured_dirs.insert(dir.trim().to_string());
                        return vec![line.to_string()];
                    }

                    if line.starts_with("touch ") {
                        let mut lines = Vec::new();
                        if let Some(dir) = touch_parent_dir(line) {
                            if ensured_dirs.insert(dir.clone()) {
                                lines.push(format!("mkdir -p {dir}"));
                            }
                        }
                        lines.push(line.to_string());
                        lines.push("just format".to_string());
                        if include_check_all {
                            lines.push("just check-all".to_string());
                        }
                        lines.push("git add --all".to_string());
                        lines.push(format!("git commit --message '{}'", line));
                        lines
                    } else {
                        vec![line.to_string()]
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            format!("```bash\n{rewritten_body}\n```")
        })
        .to_string()
}

fn generic_contracts_tutorial(spec: &OutputRepoSpec, spec_body: &str) -> String {
    let contract_section = extract_section(spec_body, "Core Logic Contract").unwrap_or_else(|| {
        "Define the shared interfaces, request and response types, enums, and small value objects that both the code layer and adapter layer need.".to_string()
    });
    let body = format!(
        "{}\n\nUse this file to define the shared contracts that the code layer implements and the adapter layer depends on.\n\nDo not add tests here. Keep this layer limited to interfaces, request and response types, enums, and small shared value objects.\n\n## Core Logic Contract\n\n{}\n\nAfter the contract files are in place, run:\n\n```bash\njust format\ngit add --all\ngit commit --message \"Define contracts\"\n```",
        render_output_repo_contracts_scaffold(spec),
        normalize_text(&contract_section)
    );

    tutorial_file_markdown(
        "Contracts",
        &rewrite_stage_commit_checkpoints(&body),
    )
}

fn rewrite_stage_commit_checkpoints(text: &str) -> String {
    let fenced_block_re = Regex::new(r"(?s)```bash\n(?P<body>.*?)\n```").expect("valid fenced bash regex");

    fenced_block_re
        .replace_all(text, |captures: &regex::Captures| {
            let body = captures
                .name("body")
                .map(|value| value.as_str())
                .unwrap_or_default();

            if !body.lines().any(|line| line.starts_with("git commit")) {
                return captures
                    .get(0)
                    .map(|value| value.as_str())
                    .unwrap_or_default()
                    .to_string();
            }

            let mut lines = body
                .lines()
                .map(|line| {
                    if line.starts_with("git add ") {
                        "git add --all".to_string()
                    } else if line.starts_with("git commit -m ") {
                        line.replacen("git commit -m ", "git commit --message ", 1)
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>();

            if !lines.iter().any(|line| line == "just format") {
                if let Some(insert_index) = lines.iter().position(|line| line.starts_with("git add ")) {
                    lines.insert(insert_index, "just format".to_string());
                }
            }

            let is_touch_checkpoint = lines.iter().any(|line| line.starts_with("touch "));

            if !is_touch_checkpoint && !lines.iter().any(|line| line == "just check-all") {
                if let Some(insert_index) = lines.iter().position(|line| line.starts_with("git add ")) {
                    lines.insert(insert_index, "just check-all".to_string());
                }
            }

            format!("```bash\n{}\n```", lines.join("\n"))
        })
        .to_string()
}

fn rewrite_for_single_repo_tutorial(text: &str) -> String {
    normalize_text(
        &text
            .replace("In a separate adapter repo, ", "In the adapter layer in this repo, ")
            .replace("separate adapter repo", "adapter layer in this repo")
            .replace("adapter repos", "adapter layer")
            .replace("adapter repo", "adapter layer in this repo")
            .replace("core repos", "code layer")
            .replace("core repo", "code layer in this repo")
            .replace("The core repo owns", "The code layer owns")
            .replace("the core repo owns", "the code layer owns")
            .replace(
                "The matching core tutorial is the next step.",
                "Because `tutorial/code.md` comes before this file, the code layer should already exist before you finish wiring the adapter.",
            )
            .replace("src/", "workspace/src/")
            .replace("tests/", "workspace/tests/"),
    )
}

fn rewrite_output_repo_adapter_body(text: &str) -> String {
    let with_real_program = text.replace(
        "```csharp\nusing SayingHello.CommandLine;\nusing SayingHello.Contracts;\n\nvar greetingService = new NotImplementedGreetingService();\nvar adapter = new CommandLineGreeting(greetingService);\n\nConsole.WriteLine(adapter.BuildMessage(args));\n\ninternal sealed class NotImplementedGreetingService : IGreetingService\n{\n    public string Greet(string name)\n    {\n        throw new NotImplementedException(\n            \"Finish the matching core tutorial, then replace this placeholder with the real core implementation.\"\n        );\n    }\n}\n```",
        "```csharp\nusing SayingHello;\nusing SayingHello.CommandLine;\n\nvar greetingService = new GreetingService();\nvar adapter = new CommandLineGreeting(greetingService);\n\nConsole.WriteLine(adapter.BuildMessage(args));\n```",
    );

    let checkpointed = rewrite_output_repo_test_checkpoints(&with_real_program);
    let stop_section_re =
        Regex::new(r"(?s)\n### 5\. Stop At The Contract Boundary.*\z").expect("valid adapter strip regex");
    stop_section_re.replace(&checkpointed, "").to_string()
}

fn rewrite_output_repo_code_body(text: &str) -> String {
    rewrite_output_repo_test_checkpoints(text)
}

fn rewrite_output_repo_test_checkpoints(text: &str) -> String {
    let heading_re = Regex::new(r"(?m)^### (?P<title>.+)$").expect("valid heading regex");
    let mut rewritten = String::new();
    let mut last_index = 0usize;
    let matches: Vec<_> = heading_re.captures_iter(text).collect();

    for (index, capture) in matches.iter().enumerate() {
        let whole_match = capture.get(0).expect("heading match");
        let title = capture
            .name("title")
            .expect("heading title")
            .as_str()
            .replace('"', "\\\"");
        let section_start = whole_match.start();
        let section_end = matches
            .get(index + 1)
            .and_then(|next| next.get(0))
            .map(|next| next.start())
            .unwrap_or(text.len());

        rewritten.push_str(&text[last_index..section_start]);
        let section_text = &text[section_start..section_end];
        let replacement = format!(
            "Run:\n\n```bash\njust format\njust check-all\ngit add --all\ngit commit --message \"{title}\"\n```"
        );
        rewritten.push_str(&section_text.replace("Run:\n\n```bash\ndotnet test\n```", &replacement));
        last_index = section_end;
    }

    rewritten.push_str(&text[last_index..]);
    rewritten
}

fn render_output_repo_finish_content(spec: &OutputRepoSpec) -> String {
    if is_go_saying_hello_output_repo(spec) {
        return tutorial_file_markdown(
            "Finish",
            &format!(
                "Start the API server from the repository root:\n\n```bash\njust run\n```\n\nThis API is configured to accept browser requests from `http://localhost:{FOR_ALL_FRONTEND_PORT}`.\n\nIn another terminal, try these requests:\n\n```bash\ncurl \"http://localhost:{FOR_ALL_API_PORT}/api/greeting\"\ncurl \"http://localhost:{FOR_ALL_API_PORT}/api/greeting?name=Ada\"\n```\n\nYou should get:\n\n```json\n{{\"message\":\"Hello!\"}}\n```\n\nand:\n\n```json\n{{\"message\":\"Hello, Ada!\"}}\n```"
            ),
        );
    }

    if is_go_todo_list_http_json_sqlite_output_repo(spec) {
        return tutorial_file_markdown(
            "Finish",
            &format!(
                "Start the API server from the repository root:\n\n```bash\njust run\n```\n\nThis API is configured to accept browser requests from `http://localhost:{FOR_ALL_FRONTEND_PORT}` and to persist tasks in `workspace/data/tasks.db`.\n\nIn another terminal, try these requests:\n\n```bash\ncurl \"http://localhost:{FOR_ALL_API_PORT}/api/tasks\"\ncurl -X POST \"http://localhost:{FOR_ALL_API_PORT}/api/tasks\" \\\n  -H \"Content-Type: application/json\" \\\n  -d '{{\"text\":\"Buy milk\"}}'\ncurl \"http://localhost:{FOR_ALL_API_PORT}/api/tasks/1\"\ncurl -i -X DELETE \"http://localhost:{FOR_ALL_API_PORT}/api/tasks/1\"\n```\n\nWith a fresh database, the first `GET` should return an empty list:\n\n```json\n{{\"tasks\":[]}}\n```\n\nThe `POST` should return the created task resource:\n\n```json\n{{\"id\":1,\"text\":\"Buy milk\"}}\n```\n\nThe next `GET /api/tasks/1` should return the same task resource. The `DELETE` should return `204 No Content`.\n\nIf you already have rows in the database, SQLite may assign a different numeric id than `1`."
            ),
        );
    }

    if is_go_todo_list_rest_json_postgres_output_repo(spec) {
        return tutorial_file_markdown(
            "Finish",
            &format!(
                "Make sure you have a local Postgres database named `todo_list`. The default tutorial command for that is:\n\n```bash\ncreatedb --host localhost --username postgres todo_list\n```\n\nStart the API server from the repository root:\n\n```bash\njust run\n```\n\nThe generated `just run` and `just check-tests` commands call `sqlc generate` for you before compiling the app.\n\nIf your local Postgres uses a different connection string, run:\n\n```bash\njust --set database_url \"postgres://<user>:<password>@localhost:5432/<database>?sslmode=disable\" run\n```\n\nThis API is configured to accept browser requests from `http://localhost:{FOR_ALL_FRONTEND_PORT}`.\n\nIn another terminal, try these requests:\n\n```bash\ncurl \"http://localhost:{FOR_ALL_API_PORT}/api/tasks\"\ncurl -X POST \"http://localhost:{FOR_ALL_API_PORT}/api/tasks\" \\\n  -H \"Content-Type: application/json\" \\\n  -d '{{\"text\":\"Buy milk\"}}'\ncurl \"http://localhost:{FOR_ALL_API_PORT}/api/tasks/11111111-1111-1111-1111-111111111111\"\ncurl -i -X DELETE \"http://localhost:{FOR_ALL_API_PORT}/api/tasks/11111111-1111-1111-1111-111111111111\"\n```\n\nWith a fresh database, the first `GET` should return an empty list:\n\n```json\n{{\"tasks\":[]}}\n```\n\nThe `POST` should return a created task resource with a UUID id, for example:\n\n```json\n{{\"id\":\"11111111-1111-1111-1111-111111111111\",\"text\":\"Buy milk\"}}\n```\n\nThe next `GET /api/tasks/<uuid>` should return the same task resource. The `DELETE` should return `204 No Content`.\n\nIf you already have rows in the database, Postgres will assign a different UUID than the example above."
            ),
        );
    }

    if is_go_todo_list_http_json_output_repo(spec) {
        return tutorial_file_markdown(
            "Finish",
            &format!(
                "Start the API server from the repository root:\n\n```bash\njust run\n```\n\nThis API is configured to accept browser requests from `http://localhost:{FOR_ALL_FRONTEND_PORT}` and to persist tasks in `workspace/data/tasks.json`.\n\nIn another terminal, try these requests:\n\n```bash\ncurl \"http://localhost:{FOR_ALL_API_PORT}/api/tasks\"\ncurl -X POST \"http://localhost:{FOR_ALL_API_PORT}/api/tasks\" \\\n  -H \"Content-Type: application/json\" \\\n  -d '{{\"task\":\"Buy milk\"}}'\ncurl -X DELETE \"http://localhost:{FOR_ALL_API_PORT}/api/tasks?task=Buy%20milk\"\n```\n\nThe initial `GET` should return an empty task list:\n\n```json\n{{\"tasks\":[],\"lines\":[]}}\n```\n\nAfter the `POST`, you should get:\n\n```json\n{{\"tasks\":[\"Buy milk\"],\"lines\":[\"Buy milk\"]}}\n```\n\nAfter the `DELETE`, the task list should be empty again.\n\nIf you later build a browser client at `http://localhost:{FOR_ALL_FRONTEND_PORT}`, it can call this API without additional CORS setup."
            ),
        );
    }

    if is_flutter_todo_list_output_repo(spec) {
        return tutorial_file_markdown(
            "Finish",
            &format!(
                "Make sure the matching Todo List API is running on your development machine at port `{FOR_ALL_API_PORT}`.\n\nFor web, start the Flutter app from the repository root with:\n\n```bash\njust run\n```\n\nor:\n\n```bash\njust run-web\n```\n\nThen open `http://localhost:{FOR_ALL_FRONTEND_PORT}` in your browser.\n\nFor iOS, open the simulator, list devices, and run with an actual simulator id or name:\n\n```bash\nopen -a Simulator\njust devices\njust run-ios device=\"<ios-device-id-or-name>\"\n```\n\nFor Android, use:\n\n```bash\njust --set api_base_url http://10.0.2.2:{FOR_ALL_API_PORT} run-android device=\"<android-device-id-or-name>\"\n```\n\nFor macOS desktop, use:\n\n```bash\njust run-macos\n```\n\nIf the macOS desktop app does not appear to reach the API, rerun it with the host URL made explicit:\n\n```bash\njust --set api_base_url http://localhost:{FOR_ALL_API_PORT} run-macos\n```\n\nFor Windows or Linux, run the matching command on that host platform:\n\n```bash\njust run-windows\njust run-linux\n```\n\nAfter the first successful iOS run, if CocoaPods added shared iOS project files like these:\n\n- `workspace/ios/Runner.xcodeproj/project.pbxproj`\n- `workspace/ios/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/ios/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add iOS CocoaPods workspace files\"\n```\n\nAfter the first successful macOS run, if CocoaPods added shared macOS project files like these:\n\n- `workspace/macos/Runner.xcodeproj/project.pbxproj`\n- `workspace/macos/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/macos/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add macOS CocoaPods workspace files\"\n```\n\nA normal Android run usually should not add shared tracked files. Do not commit machine-specific files like `workspace/android/local.properties`, `workspace/.gradle/`, `workspace/build/`, or `workspace/macos/Pods/`.\n\nTry this flow:\n\n- load the current task list\n- add `Buy milk`\n- remove `Buy milk`\n\nIf the API is unavailable, the app should show `Sorry, the task API is unavailable right now.`"
            ),
        );
    }

    if is_flutter_saying_hello_output_repo(spec) {
        let body = if is_flutter_http_saying_hello_output_repo(spec) {
            format!(
                "Make sure the matching Saying Hello API is running on your development machine at port `{FOR_ALL_API_PORT}`.\n\nFor web, start the Flutter app from the repository root with:\n\n```bash\njust run\n```\n\nor:\n\n```bash\njust run-web\n```\n\nThen open `http://localhost:{FOR_ALL_FRONTEND_PORT}` in your browser.\n\nFor iOS, open the simulator, list devices, and run with an actual simulator id or name:\n\n```bash\nopen -a Simulator\njust devices\njust run-ios device=\"<ios-device-id-or-name>\"\n```\n\nFor Android, use:\n\n```bash\njust --set api_base_url http://10.0.2.2:{FOR_ALL_API_PORT} run-android device=\"<android-device-id-or-name>\"\n```\n\nFor macOS desktop, use:\n\n```bash\njust run-macos\n```\n\nIf the macOS desktop app does not appear to reach the API, rerun it with the host URL made explicit:\n\n```bash\njust --set api_base_url http://localhost:{FOR_ALL_API_PORT} run-macos\n```\n\nFor Windows or Linux, run the matching command on that host platform:\n\n```bash\njust run-windows\njust run-linux\n```\n\nAfter the first successful iOS run, if CocoaPods added shared iOS project files like these:\n\n- `workspace/ios/Runner.xcodeproj/project.pbxproj`\n- `workspace/ios/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/ios/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add iOS CocoaPods workspace files\"\n```\n\nAfter the first successful macOS run, if CocoaPods added shared macOS project files like these:\n\n- `workspace/macos/Runner.xcodeproj/project.pbxproj`\n- `workspace/macos/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/macos/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add macOS CocoaPods workspace files\"\n```\n\nA normal Android run usually should not add shared tracked files. Do not commit machine-specific files like `workspace/android/local.properties`, `workspace/.gradle/`, `workspace/build/`, or `workspace/macos/Pods/`.\n\nTry these inputs:\n\n- enter `Ada` and expect `Hello, Ada!`\n- submit an empty value and expect `Hello!`\n\nIf the API is unavailable, the app should show `Sorry, the greeting API is unavailable right now.`"
            )
        } else {
            format!(
                "For web, start the Flutter app from the repository root with:\n\n```bash\njust run\n```\n\nor:\n\n```bash\njust run-web\n```\n\nThen open `http://localhost:{FOR_ALL_FRONTEND_PORT}` in your browser.\n\nFor iOS, open the simulator, list devices, and run with an actual simulator id or name:\n\n```bash\nopen -a Simulator\njust devices\njust run-ios device=\"<ios-device-id-or-name>\"\n```\n\nFor Android, use:\n\n```bash\njust run-android device=\"<android-device-id-or-name>\"\n```\n\nFor macOS desktop, use:\n\n```bash\njust run-macos\n```\n\nAfter the first successful iOS run, if CocoaPods added shared iOS project files like these:\n\n- `workspace/ios/Runner.xcodeproj/project.pbxproj`\n- `workspace/ios/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/ios/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add iOS CocoaPods workspace files\"\n```\n\nAfter the first successful macOS run, if CocoaPods added shared macOS project files like these:\n\n- `workspace/macos/Runner.xcodeproj/project.pbxproj`\n- `workspace/macos/Runner.xcworkspace/contents.xcworkspacedata`\n- `workspace/macos/Podfile.lock`\n\nthen run:\n\n```bash\ngit add --all\ngit commit --message \"Add macOS CocoaPods workspace files\"\n```\n\nA normal Android run usually should not add shared tracked files. Do not commit machine-specific files like `workspace/android/local.properties`, `workspace/.gradle/`, `workspace/build/`, or `workspace/macos/Pods/`.\n\nTry these inputs:\n\n- enter `Ada` and expect `Hello, Ada!`\n- submit an empty value and expect `Hello!`"
            )
        };
        return tutorial_file_markdown("Finish", &body);
    }

    if is_astro_saying_hello_output_repo(spec) {
        return tutorial_file_markdown(
            "Finish",
            &format!(
                "Make sure the matching Saying Hello API is running at `http://localhost:{FOR_ALL_API_PORT}`.\n\nThen start the Astro app from the repository root:\n\n```bash\njust run\n```\n\nOpen `http://localhost:{FOR_ALL_FRONTEND_PORT}` in your browser.\n\nTry these inputs:\n\n- submit the form with `Ada` and expect `Hello, Ada!`\n- submit the form with an empty input and expect `Hello!`\n\nIf the API is unavailable, the page should show `Sorry, the greeting API is unavailable right now.`"
            ),
        );
    }

    let body = if spec.project_slug == "saying-hello" {
        "From the repository root, use the generated root `justfile` to run the finished command-line application.\n\nTry these commands:\n\n```bash\njust run\njust run Ada\n```\n\nFor `saying-hello`, the expected behavior is:\n\n- `just run` prints `Hello!`\n- `just run Ada` prints `Hello, Ada!`"
            .to_string()
    } else {
        "From the repository root, use the generated root `justfile` to run the finished command-line application.\n\nTry these commands:\n\n```bash\njust run\njust run <your-arguments>\n```".to_string()
    };

    tutorial_file_markdown("Finish", &body)
}

fn workspace_solution_name(project_slug: &str) -> String {
    pascal_case_slug(project_slug)
}

fn contracts_project_name(project_slug: &str) -> String {
    format!("{}.Contracts", workspace_solution_name(project_slug))
}

fn code_project_name(project_slug: &str) -> String {
    workspace_solution_name(project_slug)
}

fn code_test_project_name(project_slug: &str) -> String {
    format!("{}.Tests", workspace_solution_name(project_slug))
}

fn adapter_project_name(project_slug: &str) -> String {
    format!("{}.CommandLine", workspace_solution_name(project_slug))
}

fn adapter_test_project_name(project_slug: &str) -> String {
    format!("{}.Tests", adapter_project_name(project_slug))
}

fn render_output_repo_contracts_scaffold(spec: &OutputRepoSpec) -> String {
    let solution_name = workspace_solution_name(&spec.project_slug);
    let project_name = contracts_project_name(&spec.project_slug);
    let project_path = format!("workspace/src/{project_name}");
    let commands = vec![
        format!(
            "dotnet new classlib --language C# --output {project_path} --name {project_name}"
        ),
        format!(
            "dotnet sln workspace/{solution_name}.sln add {project_path}/{project_name}.csproj"
        ),
    ];

    format!(
        "From the repository root, run:\n\n```bash\n{}\n```",
        render_checkpointed_command_sequence(&commands)
    )
}

fn render_output_repo_code_scaffold(spec: &OutputRepoSpec) -> String {
    let solution_name = workspace_solution_name(&spec.project_slug);
    let library_project_name = code_project_name(&spec.project_slug);
    let test_project_name = code_test_project_name(&spec.project_slug);
    let contracts_project_name = contracts_project_name(&spec.project_slug);
    let library_project_path = format!("workspace/src/{library_project_name}");
    let test_project_path = format!("workspace/tests/{test_project_name}");
    let contracts_project_path =
        format!("workspace/src/{contracts_project_name}/{contracts_project_name}.csproj");
    let commands = vec![
        format!(
            "dotnet new classlib --language C# --output {library_project_path} --name {library_project_name}"
        ),
        format!(
            "dotnet new xunit --language C# --output {test_project_path} --name {test_project_name}"
        ),
        format!(
            "dotnet sln workspace/{solution_name}.sln add {library_project_path}/{library_project_name}.csproj"
        ),
        format!(
            "dotnet sln workspace/{solution_name}.sln add {test_project_path}/{test_project_name}.csproj"
        ),
        format!(
            "dotnet add {library_project_path}/{library_project_name}.csproj reference {contracts_project_path}"
        ),
        format!(
            "dotnet add {test_project_path}/{test_project_name}.csproj reference {contracts_project_path}"
        ),
        format!(
            "dotnet add {test_project_path}/{test_project_name}.csproj reference {library_project_path}/{library_project_name}.csproj"
        ),
    ];

    format!(
        "Both the code library and the code test library should reference the contracts library.\n\nFrom the repository root, run:\n\n```bash\n{}\n```",
        render_checkpointed_command_sequence(&commands)
    )
}

fn render_output_repo_adapter_scaffold(spec: &OutputRepoSpec) -> String {
    let solution_name = workspace_solution_name(&spec.project_slug);
    let adapter_project_name = adapter_project_name(&spec.project_slug);
    let adapter_test_project_name = adapter_test_project_name(&spec.project_slug);
    let contracts_project_name = contracts_project_name(&spec.project_slug);
    let code_project_name = code_project_name(&spec.project_slug);
    let adapter_project_path = format!("workspace/src/{adapter_project_name}");
    let adapter_test_project_path = format!("workspace/tests/{adapter_test_project_name}");
    let contracts_project_path =
        format!("workspace/src/{contracts_project_name}/{contracts_project_name}.csproj");
    let code_project_path = format!("workspace/src/{code_project_name}/{code_project_name}.csproj");
    let commands = vec![
        format!(
            "dotnet new console --language C# --output {adapter_project_path} --name {adapter_project_name}"
        ),
        format!(
            "dotnet new xunit --language C# --output {adapter_test_project_path} --name {adapter_test_project_name}"
        ),
        format!(
            "dotnet sln workspace/{solution_name}.sln add {adapter_project_path}/{adapter_project_name}.csproj"
        ),
        format!(
            "dotnet sln workspace/{solution_name}.sln add {adapter_test_project_path}/{adapter_test_project_name}.csproj"
        ),
        format!(
            "dotnet add {adapter_project_path}/{adapter_project_name}.csproj reference {contracts_project_path}"
        ),
        format!(
            "dotnet add {adapter_test_project_path}/{adapter_test_project_name}.csproj reference {contracts_project_path}"
        ),
        format!(
            "dotnet add {adapter_project_path}/{adapter_project_name}.csproj reference {code_project_path}"
        ),
        format!(
            "dotnet add {adapter_test_project_path}/{adapter_test_project_name}.csproj reference {adapter_project_path}/{adapter_project_name}.csproj"
        ),
    ];

    format!(
        "Both the adapter library and the adapter test library should reference the contracts library. The adapter library should also reference the code library.\n\nFrom the repository root, run:\n\n```bash\n{}\n```",
        render_checkpointed_command_sequence(&commands)
    )
}

fn render_checkpointed_command_sequence(commands: &[String]) -> String {
    commands
        .iter()
        .map(|command| {
            format!(
                "{command}\njust format\njust check-all\ngit add --all\ngit commit --message '{}'",
                command
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn mit_license_text(owner: &str) -> String {
    [
        "MIT License",
        "",
        &format!("Copyright (c) 2026 {owner}"),
        "",
        "Permission is hereby granted, free of charge, to any person obtaining a copy",
        "of this software and associated documentation files (the \"Software\"), to deal",
        "in the Software without restriction, including without limitation the rights",
        "to use, copy, modify, merge, publish, distribute, sublicense, and/or sell",
        "copies of the Software, and to permit persons to whom the Software is",
        "furnished to do so, subject to the following conditions:",
        "",
        "The above copyright notice and this permission notice shall be included in all",
        "copies or substantial portions of the Software.",
        "",
        "THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR",
        "IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,",
        "FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE",
        "AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER",
        "LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,",
        "OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE",
        "SOFTWARE.",
    ]
    .join("\n")
        + "\n"
}

fn starter_gitignore_content(_ecosystem: &str) -> String {
    format!(
        "{}{}{}",
        wrap_gitignore_section(
            "https://github.com/github/gitignore/blob/main/Global/Linux.gitignore",
            github_global_linux_gitignore(),
        ),
        wrap_gitignore_section(
            "https://github.com/github/gitignore/blob/main/Global/macOS.gitignore",
            github_global_macos_gitignore(),
        ),
        wrap_gitignore_section(
            "https://github.com/github/gitignore/blob/main/Global/Windows.gitignore",
            github_global_windows_gitignore(),
        ),
    )
}

fn wrap_gitignore_section(source_url: &str, contents: &str) -> String {
    format!(
        "#### START {source_url}\n\n{contents}\n#### END {source_url}\n\n"
    )
}

fn github_global_linux_gitignore() -> &'static str {
    "*~\n\n# temporary files which can be created if a process still has a handle open of a deleted file\n.fuse_hidden*\n\n# Metadata left by Dolphin file manager, which comes with KDE Plasma\n.directory\n\n# Linux trash folder which might appear on any partition or disk\n.Trash-*\n\n# .nfs files are created when an open file is removed but is still being accessed\n.nfs*\n\n# Log files created by default by the nohup command\nnohup.out\n"
}

fn github_global_macos_gitignore() -> &'static str {
    "# General\n.DS_Store\n.localized\n__MACOSX/\n.AppleDouble\n.LSOverride\nIcon[\r]\n\n# Thumbnails\n._*\n\n# Files that might appear in the root of a volume\n.DocumentRevisions-V100\n.fseventsd\n.Spotlight-V100\n.TemporaryItems\n.Trashes\n.VolumeIcon.icns\n.com.apple.timemachine.donotpresent\n\n# Directories potentially created on remote AFP share\n.AppleDB\n.AppleDesktop\nNetwork Trash Folder\nTemporary Items\n.apdisk\n"
}

fn github_global_windows_gitignore() -> &'static str {
    "# Windows thumbnail cache files\nThumbs.db\nThumbs.db:encryptable\nehthumbs.db\nehthumbs_vista.db\n\n# Dump file\n*.stackdump\n\n# Folder config file\n[Dd]esktop.ini\n\n# Recycle Bin used on file shares\n$RECYCLE.BIN/\n\n# Windows Installer files\n*.cab\n*.msi\n*.msix\n*.msm\n*.msp\n\n# Windows shortcuts\n*.lnk\n"
}

#[allow(dead_code)]
fn load_dotnet_ci_workflow(app_root: &Path) -> Result<String, AppError> {
    let partial = Partial::load(&app_root.join("partials/setups/code/dotnet/toolchain/github-actions.md"))?;
    extract_fenced_code_block(&partial.body, "yaml").ok_or_else(|| {
        AppError::message("failed to extract YAML workflow from .NET GitHub Actions partial")
    })
}

#[allow(dead_code)]
fn dotnet_contracts_ci_workflow() -> String {
    "name: CI

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - name: Check out code
        uses: actions/checkout@v6

      - name: Set up .NET
        uses: actions/setup-dotnet@v5
        with:
          dotnet-version: 10.0.x

      - name: Verify formatting
        run: dotnet format --verify-no-changes

      - name: Restore
        run: dotnet restore

      - name: Build
        run: dotnet build
"
    .to_string()
}

#[allow(dead_code)]
fn extract_fenced_code_block(markdown: &str, info_string: &str) -> Option<String> {
    let fence = format!("```{info_string}\n");
    let after_start = markdown.split_once(&fence)?.1;
    let (block, _) = after_start.split_once("\n```")?;
    Some(format!("{block}\n"))
}

fn create_github_repo(repo_full_name: &str, description: &str) -> Result<(), AppError> {
    let status = Command::new("gh")
        .args(["repo", "create", repo_full_name, "--public", "--description"])
        .arg(description)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to create GitHub repo {repo_full_name}"
        )))
    }
}

fn delete_github_repo(repo_full_name: &str) -> Result<(), AppError> {
    let status = Command::new("gh")
        .args(["repo", "delete", repo_full_name, "--yes"])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to delete GitHub repo {repo_full_name}"
        )))
    }
}

fn clone_github_repo(repo_full_name: &str, clone_path: &Path) -> Result<(), AppError> {
    let clone_url = format!("https://github.com/{repo_full_name}.git");
    let status = Command::new("git")
        .arg("clone")
        .arg(&clone_url)
        .arg(clone_path)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to clone GitHub repo {repo_full_name} from {clone_url} into {}",
            clone_path.display()
        )))
    }
}

fn ensure_git_repo(repo_path: &Path) -> Result<(), AppError> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "{} exists but is not a git repository",
            repo_path.display()
        )))
    }
}

fn ensure_clean_worktree(repo_path: &Path) -> Result<(), AppError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["status", "--short"])
        .output()?;
    if !output.status.success() {
        return Err(AppError::message(format!(
            "failed to inspect worktree status for {}",
            repo_path.display()
        )));
    }

    let status_text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if status_text.is_empty() {
        return Ok(());
    }

    println!(
        "resetting dirty output repo {} before bootstrap",
        repo_path.display()
    );

    let reset_status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["reset", "--hard"])
        .status()?;
    if !reset_status.success() {
        return Err(AppError::message(format!(
            "failed to hard reset dirty output repo {}",
            repo_path.display()
        )));
    }

    let clean_status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["clean", "-fd"])
        .status()?;
    if !clean_status.success() {
        return Err(AppError::message(format!(
            "failed to remove untracked files in dirty output repo {}",
            repo_path.display()
        )));
    }

    let verify_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["status", "--short"])
        .output()?;
    if !verify_output.status.success() {
        return Err(AppError::message(format!(
            "failed to verify worktree status for {} after cleanup",
            repo_path.display()
        )));
    }

    if String::from_utf8_lossy(&verify_output.stdout).trim().is_empty() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "{} is still dirty after reset and clean",
            repo_path.display()
        )))
    }
}

fn ensure_main_license_baseline(repo_path: &Path, owner: &str) -> Result<String, AppError> {
    let has_remote_main = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["ls-remote", "--exit-code", "--heads", "origin", "main"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?
        .success();
    let has_local_main = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["show-ref", "--verify", "--quiet", "refs/heads/main"])
        .status()?
        .success();

    if has_remote_main {
        switch_to_local_main_and_pull_if_present(repo_path, has_local_main)?;
        return root_commit_of_head(repo_path);
    }

    if has_local_main {
        let switch_status = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .args(["switch", "main"])
            .status()?;
        if !switch_status.success() {
            return Err(AppError::message(format!(
                "failed to switch to existing main in {}",
                repo_path.display()
            )));
        }

        if !repo_has_head(repo_path)? {
            create_main_license_baseline(repo_path, owner)?;
        } else {
            git_push_main(repo_path)?;
        }
        return root_commit_of_head(repo_path);
    }

    create_main_license_baseline(repo_path, owner)?;
    root_commit_of_head(repo_path)
}

fn switch_to_local_main_and_pull_if_present(
    repo_path: &Path,
    has_local_main: bool,
) -> Result<(), AppError> {
    let switch_status = if has_local_main {
        Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .args(["switch", "main"])
            .status()?
    } else {
        Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .args(["switch", "-c", "main", "--track", "origin/main"])
            .status()?
    };
    if !switch_status.success() {
        return Err(AppError::message(format!(
            "failed to switch to main in {}",
            repo_path.display()
        )));
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["pull", "--ff-only", "origin", "main"])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to fast-forward pull main in {}",
            repo_path.display()
        )))
    }
}

fn create_main_license_baseline(repo_path: &Path, owner: &str) -> Result<(), AppError> {
    let orphan_status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["switch", "--orphan", "main"])
        .status()?;
    if !orphan_status.success() {
        return Err(AppError::message(format!(
            "failed to create orphan main in {}",
            repo_path.display()
        )));
    }

    clear_repo_worktree(repo_path)?;
    let baseline_files = build_main_baseline_files(owner);
    write_managed_files(repo_path, &baseline_files)?;
    git_add_managed_files(repo_path, &baseline_files)?;
    git_commit(repo_path, "Add LICENSE")?;
    git_push_main(repo_path)
}

fn clear_repo_worktree(repo_path: &Path) -> Result<(), AppError> {
    for entry in fs::read_dir(repo_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().is_some_and(|name| name == ".git") {
            continue;
        }

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

fn repo_has_head(repo_path: &Path) -> Result<bool, AppError> {
    Ok(Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["rev-parse", "--verify", "HEAD"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?
        .success())
}

fn root_commit_of_head(repo_path: &Path) -> Result<String, AppError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["rev-list", "--max-parents=0", "HEAD"])
        .output()?;
    if !output.status.success() {
        return Err(AppError::message(format!(
            "failed to resolve root commit in {}",
            repo_path.display()
        )));
    }

    let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if commit.is_empty() {
        Err(AppError::message(format!(
            "root commit was empty in {}",
            repo_path.display()
        )))
    } else {
        Ok(commit)
    }
}

fn switch_to_sync_branch_from_commit(
    repo_path: &Path,
    sync_branch_name: &str,
    base_commit: &str,
) -> Result<(), AppError> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["switch", "-C", sync_branch_name, base_commit])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to switch {} to branch {}",
            repo_path.display(),
            sync_branch_name
        )))
    }
}

#[allow(dead_code)]
fn switch_to_main(repo_path: &Path) -> Result<(), AppError> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["switch", "main"])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to switch {} to main",
            repo_path.display()
        )))
    }
}

#[allow(dead_code)]
fn run_command_in_dir(repo_path: &Path, command: &[String]) -> Result<(), AppError> {
    let (program, args) = command.split_first().ok_or_else(|| {
        AppError::message(format!(
            "attempted to run an empty command in {}",
            repo_path.display()
        ))
    })?;

    let status = Command::new(program)
        .current_dir(repo_path)
        .args(args)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "command failed in {}: {}",
            repo_path.display(),
            command.join(" ")
        )))
    }
}

#[allow(dead_code)]
fn git_add_paths(repo_path: &Path, paths: &[String]) -> Result<(), AppError> {
    let mut command = Command::new("git");
    command.arg("-C").arg(repo_path).arg("add");
    for path in paths {
        command.arg(path);
    }

    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to stage selected paths in {}",
            repo_path.display()
        )))
    }
}

fn git_add_managed_files(repo_path: &Path, managed_files: &[ManagedRepoFile]) -> Result<(), AppError> {
    let mut command = Command::new("git");
    command.arg("-C").arg(repo_path).arg("add");
    for managed_file in managed_files {
        command.arg(&managed_file.relative_path);
    }

    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to stage managed files in {}",
            repo_path.display()
        )))
    }
}

fn git_commit_managed_files(repo_path: &Path, had_head_before_commit: bool) -> Result<(), AppError> {
    let commit_message = if had_head_before_commit {
        "Update tutorial workspace files"
    } else {
        "Bootstrap repository with tutorial workspace"
    };

    git_commit(repo_path, commit_message)
}

fn git_commit(repo_path: &Path, commit_message: &str) -> Result<(), AppError> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["commit", "-m", commit_message])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to commit managed files in {}",
            repo_path.display()
        )))
    }
}

fn git_push_main(repo_path: &Path) -> Result<(), AppError> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["push", "-u", "origin", "main"])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to push main from {}",
            repo_path.display()
        )))
    }
}

fn git_push_branch(repo_path: &Path, branch_name: &str) -> Result<(), AppError> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["push", "-u", "origin", branch_name])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AppError::message(format!(
            "failed to push branch {} from {}",
            branch_name,
            repo_path.display()
        )))
    }
}

fn append_implicit_partials(
    app_root: &Path,
    partials: &mut Vec<Partial>,
    output: &CompiledOutput,
) -> Result<(), AppError> {
    if output.kind == OutputKind::Core && output.selections.ecosystem == "dotnet" {
        partials.push(Partial::load(
            &app_root.join("partials/setups/code/dotnet/toolchain/github-actions.md"),
        )?);
    }
    if output.selections.ecosystem == "dotnet" {
        partials.push(Partial::load(
            &app_root.join("partials/setups/code/dotnet/toolchain/justfile.md"),
        )?);
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Manifest {
    project: String,
    compiled_outputs: Vec<CompiledOutput>,
}

#[derive(Debug, Deserialize, Clone)]
struct CompiledOutput {
    #[allow(dead_code)]
    id: String,
    kind: OutputKind,
    tutorial_path: String,
    selections: Selections,
    sources: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum OutputKind {
    Contracts,
    Core,
    Adapter,
}

#[derive(Debug, Deserialize, Clone)]
struct Selections {
    ecosystem: String,
    language: String,
    #[serde(default)]
    testing: Option<String>,
    #[serde(default)]
    storage: Option<String>,
    #[serde(default)]
    surface: Option<String>,
    #[serde(default)]
    target: Option<String>,
    #[serde(default)]
    framework: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    partial_kind: PartialKind,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum PartialKind {
    PartialsRoot,
    ProjectsRoot,
    ProjectRoot,
    ProjectSpec,
    InstructionsIndex,
    ContractsInstructions,
    CoreInstructions,
    AdapterInstructions,
    EcosystemRoot,
    StorageRoot,
    StoragePartial,
    LanguagePartial,
    ToolchainItem,
    TestingPartial,
    AdapterPartial,
    FrameworkPartial,
    TestingIndex,
    JustfilePartial,
    CiPartial,
}

#[derive(Debug)]
struct Partial {
    meta: Frontmatter,
    title: String,
    body: String,
}

impl Partial {
    fn load(path: &Path) -> Result<Self, AppError> {
        let raw = fs::read_to_string(path)?;
        let (frontmatter_text, markdown) = split_frontmatter(&raw).ok_or_else(|| {
            AppError::message(format!("missing frontmatter in {}", path.display()))
        })?;
        let meta: Frontmatter = serde_yaml::from_str(frontmatter_text)?;
        let (title, body) = split_title(markdown)
            .ok_or_else(|| AppError::message(format!("missing title in {}", path.display())))?;

        Ok(Self {
            meta,
            title: title.to_string(),
            body: body.trim_start_matches('\n').to_string(),
        })
    }
}

fn split_frontmatter(raw: &str) -> Option<(&str, &str)> {
    let rest = raw.strip_prefix("---\n")?;
    let index = rest.find("\n---\n")?;
    let frontmatter = &rest[..index];
    let markdown = &rest[(index + 5)..];
    Some((frontmatter, markdown))
}

fn split_title(markdown: &str) -> Option<(&str, &str)> {
    let markdown = markdown.trim_start_matches('\n');
    let markdown = markdown.strip_prefix("# ")?;
    let index = markdown.find('\n')?;
    let title = &markdown[..index];
    let body = &markdown[(index + 1)..];
    Some((title.trim(), body))
}

#[derive(Template)]
#[template(path = "tutorial.md", escape = "none")]
struct ReadmeTemplate {
    title: String,
    project_slug: String,
    role_label: String,
    repo_name: String,
    repo_description: String,
    generated_path: String,
    selected_stack: Vec<KeyValue>,
    sections: Vec<RenderedSection>,
}

#[derive(Debug)]
struct KeyValue {
    key: String,
    value: String,
}

#[derive(Debug)]
struct RenderedSection {
    heading: String,
    body: String,
}

fn build_readme(
    project_title: &str,
    project_slug: &str,
    shared_projects: &Partial,
    partials: &[Partial],
    output: &CompiledOutput,
) -> Result<ReadmeTemplate, AppError> {
    let role_label = match output.kind {
        OutputKind::Contracts => "contracts library".to_string(),
        OutputKind::Core => "core library".to_string(),
        OutputKind::Adapter => "adapter".to_string(),
    };

    let mut selected_stack = vec![
        KeyValue {
            key: "Ecosystem".to_string(),
            value: format_selection_value(&output.selections.ecosystem),
        },
        KeyValue {
            key: "Language".to_string(),
            value: format_selection_value(&output.selections.language),
        },
    ];

    if let Some(testing) = output.selections.testing.as_deref() {
        selected_stack.push(KeyValue {
            key: "Testing".to_string(),
            value: format_selection_value(testing),
        });
    }

    if let Some(storage) = &output.selections.storage {
        selected_stack.push(KeyValue {
            key: "Storage".to_string(),
            value: format_selection_value(storage),
        });
    }
    if let Some(surface) = &output.selections.surface {
        selected_stack.push(KeyValue {
            key: "Surface".to_string(),
            value: format_selection_value(surface),
        });
    }
    if let Some(target) = &output.selections.target {
        selected_stack.push(KeyValue {
            key: "Target".to_string(),
            value: format_selection_value(target),
        });
    }
    if let Some(framework) = &output.selections.framework {
        selected_stack.push(KeyValue {
            key: "Framework".to_string(),
            value: format_selection_value(framework),
        });
    }

    let spec = find_partial(partials, PartialKind::ProjectSpec)?;
    let role_instructions = match output.kind {
        OutputKind::Contracts => find_optional_partial(partials, PartialKind::ContractsInstructions)
            .map(|partial| (partial.title.clone(), partial.body.clone())),
        OutputKind::Core => Some({
            let partial = find_partial(partials, PartialKind::CoreInstructions)?;
            (partial.title.clone(), partial.body.clone())
        }),
        OutputKind::Adapter => Some({
            let partial = find_partial(partials, PartialKind::AdapterInstructions)?;
            (partial.title.clone(), partial.body.clone())
        }),
    };
    let storage_root = find_optional_partial(partials, PartialKind::StorageRoot);
    let justfile_partial = find_optional_partial(partials, PartialKind::JustfilePartial);
    let ci_partial = find_optional_partial(partials, PartialKind::CiPartial);

    let mut sections = Vec::new();
    push_section(
        &mut sections,
        "Project Spec",
        Some(shift_headings(&spec.body, 1)),
    );
    push_section(
        &mut sections,
        "Create Root README",
        Some(render_root_readme(
            &repo_name(project_slug, output),
            &repo_description(project_title, output),
        )),
    );
    push_section(
        &mut sections,
        "Prepare .NET",
        prepare_dotnet_environment(project_slug, output),
    );
    push_section(
        &mut sections,
        "Create the Solution and Projects",
        recommended_dotnet_core_scaffold(project_slug, output),
    );
    push_section(
        &mut sections,
        "Create the Contracts Solution and Project",
        recommended_dotnet_contracts_scaffold(project_slug, output),
    );
    push_section(
        &mut sections,
        "Create the Adapter Solution and Projects",
        recommended_dotnet_command_line_adapter_scaffold(project_slug, output),
    );
    push_section(
        &mut sections,
        "Add Root Justfile",
        render_dotnet_root_justfile(justfile_partial, project_slug, output),
    );
    if output.kind == OutputKind::Adapter {
        push_section(
            &mut sections,
            "Storage",
            storage_root.map(|partial| partial.body.clone()),
        );
    }
    for partial in partials
        .iter()
        .filter(|partial| partial.meta.partial_kind == PartialKind::StoragePartial)
    {
        push_subsection(&mut sections, &partial.title, &partial.body, 2);
    }
    for partial in partials
        .iter()
        .filter(|partial| partial.meta.partial_kind == PartialKind::AdapterPartial)
    {
        push_subsection(&mut sections, &partial.title, &partial.body, 2);
    }
    for partial in partials
        .iter()
        .filter(|partial| partial.meta.partial_kind == PartialKind::FrameworkPartial)
    {
        push_subsection(&mut sections, &partial.title, &partial.body, 2);
    }
    if let Some((title, body)) = role_instructions {
        push_section(&mut sections, &title, Some(body));
    }
    if output.kind == OutputKind::Core {
        if let Some(ci_partial) = ci_partial {
            push_section(
                &mut sections,
                &ci_partial.title,
                Some(ci_partial.body.clone()),
            );
        }
    }
    if output.kind != OutputKind::Contracts {
        push_section(
            &mut sections,
            "Shared Finish Checklist",
            extract_section(&shared_projects.body, "Shared Finish Checklist"),
        );
    }

    if output.kind == OutputKind::Adapter {
        push_section(
            &mut sections,
            "Next Step",
            Some(render_matching_core_tutorial_link(project_slug, output)),
        );
    }

    Ok(ReadmeTemplate {
        title: project_title.to_string(),
        project_slug: project_slug.to_string(),
        role_label,
        repo_name: repo_name(project_slug, output),
        repo_description: repo_description(project_title, output),
        generated_path: output.tutorial_path.clone(),
        selected_stack,
        sections,
    })
}

fn prepare_dotnet_environment(project_slug: &str, output: &CompiledOutput) -> Option<String> {
    if output.selections.ecosystem != "dotnet" {
        return None;
    }

    let mut body = String::from(
        "Before you scaffold this repo:\n\n\
         - from the repo root, verify the CLI is available:\n\n\
         ```bash\n\
         dotnet --version\n\
         ```\n\n\
         If `dotnet` is not found, install the SDK with the official Microsoft steps for your operating system:\n\n\
         ### macOS\n\n\
         1. Open [Install .NET on macOS](https://learn.microsoft.com/en-us/dotnet/core/install/macos).\n\
         2. Open [Download .NET](https://dotnet.microsoft.com/download/dotnet).\n\
         3. Choose the latest .NET version and open the SDK download table.\n\
         4. Pick `Arm64` for Apple Silicon or `x64` for Intel.\n\
         5. Run the installer package.\n\n\
         ### Windows\n\n\
         1. Open [Install .NET on Windows](https://learn.microsoft.com/en-us/dotnet/core/install/windows).\n\
         2. Open [Download .NET](https://dotnet.microsoft.com/download/dotnet).\n\
         3. Choose the latest .NET version and open the SDK download table.\n\
         4. Pick your CPU architecture. If you are unsure, choose `x64`.\n\
         5. Run the installer.\n\n\
         ### Linux\n\n\
         For the most common Linux families, start with Microsoft's distro-specific install page:\n\n\
         #### [Arch Linux](https://archlinux.org/)\n\n\
         1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).\n\
         2. Choose `Arch Linux`.\n\
         3. Follow the package-manager steps for `dotnet-sdk`.\n\n\
         #### [Debian](https://www.debian.org/)\n\n\
         1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).\n\
         2. Choose `Debian`.\n\
         3. Follow the package-manager steps for `dotnet-sdk`.\n\n\
         #### [Fedora](https://fedoraproject.org/)\n\n\
         1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).\n\
         2. Choose `Fedora`.\n\
         3. Follow the package-manager steps for `dotnet-sdk`.\n\n\
         #### [openSUSE](https://www.opensuse.org/)\n\n\
         1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).\n\
         2. Choose `openSUSE`.\n\
         3. Follow the package-manager steps for `dotnet-sdk`.\n\n\
         #### [NixOS](https://nixos.org/)\n\n\
         1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).\n\
         2. Check whether the current Microsoft page links to a maintained `NixOS` path.\n\
         3. If not, use the `nixpkgs` package instructions from the NixOS project site.\n\n\
         #### [Alpine Linux](https://alpinelinux.org/)\n\n\
         1. Open [Install .NET on Linux distributions](https://learn.microsoft.com/en-us/dotnet/core/install/linux).\n\
         2. Choose `Alpine`.\n\
         3. Follow the package-manager steps for `dotnet-sdk`.\n\n\
         For the other Linux distributions and platform projects you mentioned, start with the project site, then follow its package or container guidance for `.NET`:\n\n\
         - [AerynOS](https://aerynos.com/)\n\
         - [Puppy Linux](https://puppylinux-woof-ce.github.io/)\n\
         - [Void Linux](https://voidlinux.org/)\n\
         - [PCLinuxOS](https://www.pclinuxos.com/)\n\
         - [Solus](https://getsol.us/)\n\
         - [ZimaOS](https://www.zimaos.com/)\n\
         - [KDE Linux](https://kde.org/linux-ready/)\n\
         - [Gentoo Linux](https://www.gentoo.org/)\n\
         - [Mageia](https://www.mageia.org/)\n\
         - [EasyOS](https://easyos.org/)\n\
         - [GNOME OS](https://os.gnome.org/)\n\
         - [MocaccinoOS](https://www.mocaccino.org/)\n\
         - [Slackware Linux](https://www.slackware.com/)\n\
         - [Tiny Core Linux](https://www.tinycorelinux.net/)\n\
         - [MagOS Linux](http://magos-linux.ru/)\n\
         - [KaOS](https://kaosx.us/)\n\
         - [Talos Linux](https://www.talos.dev/)\n\
         - [ALT Linux](https://www.altlinux.org/)\n\
         - [OpenMandriva Lx](https://www.openmandriva.org/)\n\n\
         ### BSD\n\n\
         These are not mainstream first-class `.NET` workstation paths, so use the project site and current community guidance:\n\n\
         - [FreeBSD](https://www.freebsd.org/)\n\
         - [OpenBSD](https://www.openbsd.org/)\n\n\
         ### Android\n\n\
         Use the platform site as your starting point:\n\n\
         - [Android](https://www.android.com/)\n\n\
         ### iOS\n\n\
         Use the platform site as your starting point:\n\n\
         - [iOS](https://www.apple.com/os/ios/)\n\n\
         ### ChromeOS\n\n\
         Use the platform site as your starting point:\n\n\
         - [ChromeOS](https://www.google.com/chromebook/chrome-os/)\n\n\
         ### ReactOS\n\n\
         Use the project site as your starting point:\n\n\
         - [ReactOS](https://reactos.org/)\n\n\
         - after installation, open a fresh shell and run `dotnet --version` again\n\
         - use the SDK default target framework unless this tutorial explicitly tells you to pin a different one",
    );

    if output.kind == OutputKind::Core || output.kind == OutputKind::Adapter {
        body.push_str(&format!(
            "\n- keep this repo next to the matching contracts repo working copy so this local reference shape works:\n  `{}`",
            format!("../{}", contracts_repo_name(project_slug, output))
        ));
    }

    Some(body)
}

fn render_root_readme(repo_name: &str, repo_description: &str) -> String {
    format!(
        "Create the file:\n\n\
         ```bash\n\
         touch README.md\n\
         ```\n\n\
         Then put this exact content in `README.md`:\n\n\
         ```md\n\
         {}\
         ```"
        ,
        render_root_readme_content(GITHUB_OWNER, repo_name, repo_description)
    )
}

#[derive(Debug)]
#[allow(dead_code)]
struct DotnetScaffoldBootstrapPlan {
    commands: Vec<Vec<String>>,
    git_add_paths: Vec<String>,
    commit_message: &'static str,
}

#[derive(Debug)]
#[allow(dead_code)]
struct DotnetRootJustfilePlan {
    pre_commands: Vec<Vec<String>>,
    files: Vec<ManagedRepoFile>,
    git_add_paths: Vec<String>,
    commit_message: &'static str,
}

#[allow(dead_code)]
fn dotnet_scaffold_bootstrap_plan(
    project_slug: &str,
    output: &CompiledOutput,
) -> Option<DotnetScaffoldBootstrapPlan> {
    if output.selections.ecosystem != "dotnet" {
        return None;
    }

    if output.kind == OutputKind::Contracts {
        let solution_name = format!("{}.Contracts", pascal_case_slug(project_slug));
        let project_name = solution_name.clone();
        let solution_file = format!("{solution_name}.sln");
        let project_path = format!("src/{project_name}");

        return Some(DotnetScaffoldBootstrapPlan {
            commands: vec![
                vec![
                    "dotnet".to_string(),
                    "new".to_string(),
                    "sln".to_string(),
                    "--format".to_string(),
                    "sln".to_string(),
                    "--name".to_string(),
                    solution_name.clone(),
                ],
                vec![
                    "dotnet".to_string(),
                    "new".to_string(),
                    "classlib".to_string(),
                    "--name".to_string(),
                    project_name.clone(),
                    "--output".to_string(),
                    project_path.clone(),
                ],
                vec![
                    "dotnet".to_string(),
                    "sln".to_string(),
                    solution_file.clone(),
                    "add".to_string(),
                    format!("{project_path}/{project_name}.csproj"),
                ],
            ],
            git_add_paths: vec![solution_file, "src".to_string()],
            commit_message: "Create the Contracts Solution and Project",
        });
    }

    let (test_template, template_install_command) =
        dotnet_test_template_short_name(testing_selection(output)?);

    if output.kind == OutputKind::Core {
        let solution_name = pascal_case_slug(project_slug);
        let library_project_name = solution_name.clone();
        let test_project_name = format!("{solution_name}.Tests");
        let solution_file = format!("{solution_name}.sln");
        let library_project_path = format!("src/{library_project_name}");
        let test_project_path = format!("tests/{test_project_name}");
        let contracts_project_name = format!("{}.Contracts", pascal_case_slug(project_slug));
        let contracts_project_reference_path = format!(
            "../{}/src/{contracts_project_name}/{contracts_project_name}.csproj",
            contracts_repo_name(project_slug, output)
        );
        let mut commands = Vec::new();

        commands.push(vec![
            "dotnet".to_string(),
            "new".to_string(),
            "sln".to_string(),
            "--format".to_string(),
            "sln".to_string(),
            "--name".to_string(),
            solution_name.clone(),
        ]);

        if let Some(command) = template_install_command {
            commands.push(command.split_whitespace().map(str::to_string).collect());
        }

        commands.push(vec![
            "dotnet".to_string(),
            "new".to_string(),
            "classlib".to_string(),
            "--name".to_string(),
            library_project_name.clone(),
            "--output".to_string(),
            library_project_path.clone(),
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "new".to_string(),
            test_template.to_string(),
            "--name".to_string(),
            test_project_name.clone(),
            "--output".to_string(),
            test_project_path.clone(),
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "sln".to_string(),
            solution_file.clone(),
            "add".to_string(),
            format!("{library_project_path}/{library_project_name}.csproj"),
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "sln".to_string(),
            solution_file.clone(),
            "add".to_string(),
            format!("{test_project_path}/{test_project_name}.csproj"),
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "add".to_string(),
            format!("{library_project_path}/{library_project_name}.csproj"),
            "reference".to_string(),
            contracts_project_reference_path,
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "add".to_string(),
            format!("{test_project_path}/{test_project_name}.csproj"),
            "reference".to_string(),
            format!("{library_project_path}/{library_project_name}.csproj"),
        ]);

        return Some(DotnetScaffoldBootstrapPlan {
            commands,
            git_add_paths: vec![solution_file, "src".to_string(), "tests".to_string()],
            commit_message: "Create the Solution and Projects",
        });
    }

    if output.selections.surface.as_deref() == Some("command-line") {
        let solution_name = format!("{}.CommandLine", pascal_case_slug(project_slug));
        let adapter_name = solution_name.clone();
        let adapter_test_project_name = format!("{adapter_name}.Tests");
        let solution_file = format!("{solution_name}.sln");
        let adapter_project_path = format!("src/{adapter_name}");
        let adapter_test_project_path = format!("tests/{adapter_test_project_name}");
        let contracts_project_name = format!("{}.Contracts", pascal_case_slug(project_slug));
        let contracts_project_reference_path = format!(
            "../{}/src/{contracts_project_name}/{contracts_project_name}.csproj",
            contracts_repo_name(project_slug, output)
        );
        let mut commands = Vec::new();

        commands.push(vec![
            "dotnet".to_string(),
            "new".to_string(),
            "sln".to_string(),
            "--format".to_string(),
            "sln".to_string(),
            "--name".to_string(),
            solution_name.clone(),
        ]);

        if let Some(command) = template_install_command {
            commands.push(command.split_whitespace().map(str::to_string).collect());
        }

        commands.push(vec![
            "dotnet".to_string(),
            "new".to_string(),
            "console".to_string(),
            "--name".to_string(),
            adapter_name.clone(),
            "--output".to_string(),
            adapter_project_path.clone(),
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "new".to_string(),
            test_template.to_string(),
            "--name".to_string(),
            adapter_test_project_name.clone(),
            "--output".to_string(),
            adapter_test_project_path.clone(),
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "sln".to_string(),
            solution_file.clone(),
            "add".to_string(),
            format!("{adapter_project_path}/{adapter_name}.csproj"),
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "sln".to_string(),
            solution_file.clone(),
            "add".to_string(),
            format!("{adapter_test_project_path}/{adapter_test_project_name}.csproj"),
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "add".to_string(),
            format!("{adapter_project_path}/{adapter_name}.csproj"),
            "reference".to_string(),
            contracts_project_reference_path,
        ]);
        commands.push(vec![
            "dotnet".to_string(),
            "add".to_string(),
            format!("{adapter_test_project_path}/{adapter_test_project_name}.csproj"),
            "reference".to_string(),
            format!("{adapter_project_path}/{adapter_name}.csproj"),
        ]);

        return Some(DotnetScaffoldBootstrapPlan {
            commands,
            git_add_paths: vec![solution_file, "src".to_string(), "tests".to_string()],
            commit_message: "Create the Adapter Solution and Projects",
        });
    }

    None
}

#[allow(dead_code)]
fn run_dotnet_scaffold_bootstrap_plan(
    repo_path: &Path,
    plan: &DotnetScaffoldBootstrapPlan,
) -> Result<(), AppError> {
    for command in &plan.commands {
        run_command_in_dir(repo_path, command)?;
    }

    git_add_paths(repo_path, &plan.git_add_paths)?;
    git_commit(repo_path, plan.commit_message)
}

#[allow(dead_code)]
fn dotnet_root_justfile_plan(
    project_slug: &str,
    output: &CompiledOutput,
) -> Option<DotnetRootJustfilePlan> {
    if output.selections.ecosystem != "dotnet" {
        return None;
    }

    if output.kind == OutputKind::Contracts {
        return Some(DotnetRootJustfilePlan {
            pre_commands: Vec::new(),
            files: vec![ManagedRepoFile {
                relative_path: "justfile".to_string(),
                contents: dotnet_contracts_root_justfile_contents(project_slug).into_bytes(),
            }],
            git_add_paths: vec!["justfile".to_string()],
            commit_message: "Add Root Justfile",
        });
    }

    let test_project_path = dotnet_test_project_csproj_path(project_slug, output)?;
    let testing = testing_selection(output)?;
    let justfile_contents = dotnet_root_justfile_contents(&test_project_path, testing);

    if testing == "tunit" {
        return Some(DotnetRootJustfilePlan {
            pre_commands: Vec::new(),
            files: vec![
                ManagedRepoFile {
                    relative_path: "justfile".to_string(),
                    contents: justfile_contents.into_bytes(),
                },
                ManagedRepoFile {
                    relative_path: "global.json".to_string(),
                    contents: dotnet_test_runner_global_json().into_bytes(),
                },
            ],
            git_add_paths: vec!["justfile".to_string(), "global.json".to_string()],
            commit_message: "Add Root Justfile",
        });
    }

    Some(DotnetRootJustfilePlan {
        pre_commands: vec![vec![
            "dotnet".to_string(),
            "add".to_string(),
            test_project_path.clone(),
            "package".to_string(),
            "coverlet.msbuild".to_string(),
        ]],
        files: vec![ManagedRepoFile {
            relative_path: "justfile".to_string(),
            contents: justfile_contents.into_bytes(),
        }],
        git_add_paths: vec!["justfile".to_string(), test_project_path],
        commit_message: "Add Root Justfile",
    })
}

#[allow(dead_code)]
fn run_dotnet_root_justfile_plan(
    repo_path: &Path,
    plan: &DotnetRootJustfilePlan,
) -> Result<(), AppError> {
    for command in &plan.pre_commands {
        run_command_in_dir(repo_path, command)?;
    }

    write_managed_files(repo_path, &plan.files)?;
    git_add_paths(repo_path, &plan.git_add_paths)?;
    git_commit(repo_path, plan.commit_message)
}

fn recommended_dotnet_core_scaffold(project_slug: &str, output: &CompiledOutput) -> Option<String> {
    if output.kind != OutputKind::Core || output.selections.ecosystem != "dotnet" {
        return None;
    }

    let (test_template, template_install_command) =
        dotnet_test_template_short_name(testing_selection(output)?);
    let solution_name = pascal_case_slug(project_slug);
    let library_project_name = solution_name.clone();
    let test_project_name = format!("{solution_name}.Tests");
    let solution_file = format!("{solution_name}.sln");
    let library_project_path = format!("src/{library_project_name}");
    let test_project_path = format!("tests/{test_project_name}");
    let contracts_repo_name = contracts_repo_name(project_slug, output);
    let contracts_project_name = format!("{solution_name}.Contracts");
    let contracts_project_reference_path = format!(
        "../{contracts_repo_name}/src/{contracts_project_name}/{contracts_project_name}.csproj"
    );
    let template_install_line = template_install_command
        .map(|command| format!("{command}\n"))
        .unwrap_or_default();

    Some(format!(
        "- Solution name: `{solution_name}`\n\
         - Solution file: `{solution_file}`\n\
         - Library project name: `{library_project_name}`\n\
         - Library project path: `{library_project_path}`\n\
         - Test project name: `{test_project_name}`\n\
         - Test project path: `{test_project_path}`\n\n\
         - Local contracts repo assumption: sibling checkout at `../{contracts_repo_name}`\n\
         - Local contracts project reference path: `{contracts_project_reference_path}`\n\n\
         Use these names and paths, then run:\n\n\
         ```bash\n\
         dotnet new sln --format sln --name {solution_name}\n\
         dotnet new gitignore\n\
         {template_install_line}\
         dotnet new classlib --name {library_project_name} --output {library_project_path}\n\
         dotnet new {test_template} --name {test_project_name} --output {test_project_path}\n\
         dotnet sln {solution_file} add {library_project_path}/{library_project_name}.csproj\n\
         dotnet sln {solution_file} add {test_project_path}/{test_project_name}.csproj\n\
         dotnet add {library_project_path}/{library_project_name}.csproj reference {contracts_project_reference_path}\n\
         dotnet add {test_project_path}/{test_project_name}.csproj reference {library_project_path}/{library_project_name}.csproj\n\
         ```\n\n\
         After those files exist, make this commit:\n\n\
         ```bash\n\
         git add {solution_file} src tests\n\
         git commit --message \"Create the Solution and Projects\"\n\
         ```"
    ))
}

fn recommended_dotnet_contracts_scaffold(
    project_slug: &str,
    output: &CompiledOutput,
) -> Option<String> {
    if output.kind != OutputKind::Contracts || output.selections.ecosystem != "dotnet" {
        return None;
    }

    let solution_name = format!("{}.Contracts", pascal_case_slug(project_slug));
    let project_name = solution_name.clone();
    let solution_file = format!("{solution_name}.sln");
    let project_path = format!("src/{project_name}");

    Some(format!(
        "- Solution name: `{solution_name}`\n\
         - Solution file: `{solution_file}`\n\
         - Contracts project name: `{project_name}`\n\
         - Contracts project path: `{project_path}`\n\n\
         Use these names and paths, then run:\n\n\
         ```bash\n\
         dotnet new sln --format sln --name {solution_name}\n\
         dotnet new gitignore\n\
         dotnet new classlib --name {project_name} --output {project_path}\n\
         dotnet sln {solution_file} add {project_path}/{project_name}.csproj\n\
         ```\n\n\
         After those files exist, make this commit:\n\n\
         ```bash\n\
         git add {solution_file} src\n\
         git commit --message \"Create the Contracts Solution and Project\"\n\
         ```"
    ))
}

fn recommended_dotnet_command_line_adapter_scaffold(
    project_slug: &str,
    output: &CompiledOutput,
) -> Option<String> {
    if output.kind != OutputKind::Adapter
        || output.selections.ecosystem != "dotnet"
        || output.selections.surface.as_deref() != Some("command-line")
    {
        return None;
    }

    let (test_template, template_install_command) =
        dotnet_test_template_short_name(testing_selection(output)?);
    let contracts_repo_name = contracts_repo_name(project_slug, output);
    let contracts_project_name = format!("{}.Contracts", pascal_case_slug(project_slug));
    let adapter_name = format!("{}.CommandLine", pascal_case_slug(project_slug));
    let adapter_test_project_name = format!("{adapter_name}.Tests");
    let solution_name = adapter_name.clone();
    let solution_file = format!("{solution_name}.sln");
    let adapter_project_path = format!("src/{adapter_name}");
    let adapter_test_project_path = format!("tests/{adapter_test_project_name}");
    let contracts_project_reference_path = format!(
        "../{contracts_repo_name}/src/{contracts_project_name}/{contracts_project_name}.csproj"
    );
    let template_install_line = template_install_command
        .map(|command| format!("{command}\n"))
        .unwrap_or_default();

    Some(format!(
        "- Solution name: `{solution_name}`\n\
         - Solution file: `{solution_file}`\n\
         - Adapter project name: `{adapter_name}`\n\
         - Adapter project path: `{adapter_project_path}`\n\
         - Adapter test project name: `{adapter_test_project_name}`\n\
         - Adapter test project path: `{adapter_test_project_path}`\n\
         - Local contracts repo assumption: sibling checkout at `../{contracts_repo_name}`\n\
         - Local contracts project reference path: `{contracts_project_reference_path}`\n\n\
         Use these names and paths, then run:\n\n\
         ```bash\n\
         dotnet new sln --format sln --name {solution_name}\n\
         dotnet new gitignore\n\
         {template_install_line}\
         dotnet new console --name {adapter_name} --output {adapter_project_path}\n\
         dotnet new {test_template} --name {adapter_test_project_name} --output {adapter_test_project_path}\n\
         dotnet sln {solution_file} add {adapter_project_path}/{adapter_name}.csproj\n\
         dotnet sln {solution_file} add {adapter_test_project_path}/{adapter_test_project_name}.csproj\n\
         dotnet add {adapter_project_path}/{adapter_name}.csproj reference {contracts_project_reference_path}\n\
         dotnet add {adapter_test_project_path}/{adapter_test_project_name}.csproj reference {adapter_project_path}/{adapter_name}.csproj\n\
         ```\n\n\
         After those files exist, make this commit:\n\n\
         ```bash\n\
         git add {solution_file} src tests\n\
         git commit --message \"Create the Adapter Solution and Projects\"\n\
         ```"
    ))
}

fn render_dotnet_root_justfile(
    justfile_partial: Option<&Partial>,
    project_slug: &str,
    output: &CompiledOutput,
) -> Option<String> {
    if output.selections.ecosystem != "dotnet" {
        return None;
    }

    let intro = justfile_partial
        .map(|partial| normalize_text(&partial.body))
        .filter(|body| !body.is_empty())
        .unwrap_or_else(|| {
            "Add a repo-root `justfile` so local developer commands and CI checks use the same entry points."
                .to_string()
        });

    if output.kind == OutputKind::Contracts {
        let justfile_contents = dotnet_contracts_root_justfile_contents(project_slug);
        return Some(format!(
            "{intro}\n\n\
             Create the file:\n\n\
             ```bash\n\
             touch justfile\n\
             ```\n\n\
             Then put this exact content in `justfile`:\n\n\
             ```just\n\
             {justfile_contents}\
             ```\n\n\
             After `justfile` is in place, make this commit:\n\n\
             ```bash\n\
             git add justfile\n\
             git commit --message \"Add Root Justfile\"\n\
             ```"
        ));
    }

    let test_project_path = dotnet_test_project_csproj_path(project_slug, output)?;
    let testing = testing_selection(output)?;
    let justfile_contents = dotnet_root_justfile_contents(&test_project_path, testing);
    let commit_add_paths = if testing == "tunit" {
        "justfile global.json".to_string()
    } else {
        format!("justfile {test_project_path}")
    };

    let body = if testing == "tunit" {
        format!(
            "{intro}\n\n\
             Create a root `global.json` so `dotnet test` uses Microsoft.Testing.Platform with current .NET SDKs:\n\n\
             ```bash\n\
             touch global.json\n\
             ```\n\n\
             Then put this exact content in `global.json`:\n\n\
             ```json\n\
             {}\
             ```\n\n\
             Create the file:\n\n\
             ```bash\n\
             touch justfile\n\
             ```\n\n\
             Then put this exact content in `justfile`:\n\n\
             ```just\n\
             {justfile_contents}\
             ```\n\n\
             For `TUnit`, both coverage checks collect one Cobertura report with Microsoft.Testing.Platform, then validate either the line-rate or the branch-rate from that same report.\n\n\
             After `justfile` is in place, make this commit:\n\n\
             ```bash\n\
             git add {commit_add_paths}\n\
             git commit --message \"Add Root Justfile\"\n\
             ```"
            ,
            dotnet_test_runner_global_json()
        )
    } else {
        format!(
            "{intro}\n\n\
             Install Coverlet's MSBuild package once for the test project:\n\n\
             ```bash\n\
             dotnet add {test_project_path} package coverlet.msbuild\n\
             ```\n\n\
             Create the file:\n\n\
             ```bash\n\
             touch justfile\n\
             ```\n\n\
             Then put this exact content in `justfile`:\n\n\
             ```just\n\
             {justfile_contents}\
             ```\n\n\
             After `justfile` is in place, make this commit:\n\n\
             ```bash\n\
             git add {commit_add_paths}\n\
             git commit --message \"Add Root Justfile\"\n\
             ```"
        )
    };

    Some(body)
}

fn dotnet_test_runner_global_json() -> String {
    "{\n  \"test\": {\n    \"runner\": \"Microsoft.Testing.Platform\"\n  }\n}\n".to_string()
}

fn dotnet_contracts_root_justfile_contents(project_slug: &str) -> String {
    let solution_name = format!("{}.Contracts", pascal_case_slug(project_slug));
    let solution_file = format!("{solution_name}.sln");
    format!(
        "restore:\n\
         \tdotnet restore\n\n\
         format:\n\
         \tdotnet format\n\n\
         check-formatting:\n\
         \tdotnet format --verify-no-changes\n\n\
         check-build:\n\
         \tdotnet build {solution_file}\n\n\
         check-all:\n\
         \tjust check-formatting\n\
         \tjust check-build\n"
    )
}

fn dotnet_root_justfile_contents(test_project_path: &str, testing: &str) -> String {
    if testing == "tunit" {
        return format!(
            "restore:\n\
             \tdotnet restore\n\n\
             format:\n\
             \tdotnet format\n\n\
             check-formatting:\n\
             \tdotnet format --verify-no-changes\n\n\
             check-tests:\n\
             \tdotnet test\n\n\
             check-code-cover:\n\
             \tmkdir -p TestResults\n\
             \tdotnet test {test_project_path} -c Release -- --coverage --coverage-output TestResults/coverage.cobertura.xml --coverage-output-format cobertura\n\
             \truby -rrexml/document -e 'doc = REXML::Document.new(File.read(\"TestResults/coverage.cobertura.xml\")); rate = doc.root.attributes[\"line-rate\"].to_f * 100; abort(format(\"Line coverage %.2f%% is below 90%%\", rate)) if rate < 90'\n\n\
             check-branch-cover:\n\
             \tmkdir -p TestResults\n\
             \tdotnet test {test_project_path} -c Release -- --coverage --coverage-output TestResults/coverage.cobertura.xml --coverage-output-format cobertura\n\
             \truby -rrexml/document -e 'doc = REXML::Document.new(File.read(\"TestResults/coverage.cobertura.xml\")); rate = doc.root.attributes[\"branch-rate\"].to_f * 100; abort(format(\"Branch coverage %.2f%% is below 85%%\", rate)) if rate < 85'\n\n\
             check-all:\n\
             \tjust check-formatting\n\
             \tjust check-tests\n\
             \tjust check-code-cover\n\
             \tjust check-branch-cover\n"
        );
    }

    format!(
        "restore:\n\
         \tdotnet restore\n\n\
         format:\n\
         \tdotnet format\n\n\
         check-formatting:\n\
         \tdotnet format --verify-no-changes\n\n\
         check-tests:\n\
         \tdotnet test\n\n\
         check-code-cover:\n\
         \tdotnet test {test_project_path} /p:CollectCoverage=true /p:CoverletOutputFormat=cobertura /p:ThresholdType=line /p:Threshold=90 /p:ThresholdStat=total\n\n\
         check-branch-cover:\n\
         \tdotnet test {test_project_path} /p:CollectCoverage=true /p:CoverletOutputFormat=cobertura /p:ThresholdType=branch /p:Threshold=85 /p:ThresholdStat=total\n\
         \n\
         check-all:\n\
         \tjust check-formatting\n\
         \tjust check-tests\n\
         \tjust check-code-cover\n\
         \tjust check-branch-cover\n"
    )
}

fn dotnet_test_project_csproj_path(project_slug: &str, output: &CompiledOutput) -> Option<String> {
    let solution_name = pascal_case_slug(project_slug);

    if output.kind == OutputKind::Core {
        let test_project_name = format!("{solution_name}.Tests");
        return Some(format!("tests/{test_project_name}/{test_project_name}.csproj"));
    }

    if output.selections.surface.as_deref() == Some("command-line") {
        let adapter_name = format!("{solution_name}.CommandLine");
        let test_project_name = format!("{adapter_name}.Tests");
        return Some(format!("tests/{test_project_name}/{test_project_name}.csproj"));
    }

    None
}

fn dotnet_test_template_short_name(testing: &str) -> (&'static str, Option<&'static str>) {
    match testing {
        "xunit" => ("xunit", None),
        "nunit" => ("nunit", None),
        "mstest" => ("mstest", None),
        "tunit" => ("TUnit", Some("dotnet new install TUnit.Templates")),
        _ => ("xunit", None),
    }
}

fn find_partial<'a>(partials: &'a [Partial], kind: PartialKind) -> Result<&'a Partial, AppError> {
    partials
        .iter()
        .find(|partial| partial.meta.partial_kind == kind)
        .ok_or_else(|| AppError::message(format!("missing required partial: {:?}", kind)))
}

fn find_optional_partial<'a>(partials: &'a [Partial], kind: PartialKind) -> Option<&'a Partial> {
    partials
        .iter()
        .find(|partial| partial.meta.partial_kind == kind)
}

fn push_section(sections: &mut Vec<RenderedSection>, heading: &str, body: Option<String>) {
    let Some(body) = body else { return };
    let normalized = normalize_text(&body);
    if normalized.is_empty() {
        return;
    }
    sections.push(RenderedSection {
        heading: heading.to_string(),
        body: normalized,
    });
}

fn push_subsection(
    sections: &mut Vec<RenderedSection>,
    heading: &str,
    body: &str,
    heading_shift: usize,
) {
    let body = if heading_shift == 0 {
        body.to_string()
    } else {
        shift_headings(body, heading_shift)
    };
    let normalized = normalize_text(&format!("### {heading}\n\n{body}"));
    if normalized.is_empty() {
        return;
    }
    sections.push(RenderedSection {
        heading: String::new(),
        body: normalized,
    });
}

fn extract_section(markdown_body: &str, heading: &str) -> Option<String> {
    let lines: Vec<&str> = markdown_body.lines().collect();
    let start = lines
        .iter()
        .position(|line| line.trim() == format!("## {heading}"))?;
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find_map(|(index, line)| line.starts_with("## ").then_some(index))
        .unwrap_or(lines.len());
    Some(lines[(start + 1)..end].join("\n").trim().to_string())
}

fn shift_headings(text: &str, amount: usize) -> String {
    text.lines()
        .map(|line| {
            let hashes = line.chars().take_while(|ch| *ch == '#').count();
            if hashes == 0 || !line.chars().nth(hashes).is_some_and(|ch| ch == ' ') {
                line.to_string()
            } else {
                format!("{}{}", "#".repeat(hashes + amount), &line[hashes..])
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_text(text: &str) -> String {
    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    let collapsed = link_regex.replace_all(text, |captures: &regex::Captures| {
        let label = captures
            .get(1)
            .map(|value| value.as_str())
            .unwrap_or_default();
        let target = captures
            .get(2)
            .map(|value| value.as_str())
            .unwrap_or_default();
        if target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with('#')
        {
            captures
                .get(0)
                .map(|value| value.as_str())
                .unwrap_or_default()
                .to_string()
        } else {
            label.to_string()
        }
    });
    let pruned = collapsed
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.starts_with("The supported compiled tutorial outputs for this pilot slice")
                && !trimmed.starts_with("This file is a source index, not a compiled tutorial")
                && !trimmed.starts_with("Use the shared finish checklist in Projects together with")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let rewritten = pruned
        .replace(
            "This project follows the shared workflow and output model in Projects and Projects.",
            "This project follows the shared workflow and output model in the shared project guidance.",
        )
        .replace(
            "This spec follows the shared surface and setup-path rules in Projects.",
            "This spec follows the shared surface and setup-path rules in the shared project guidance.",
        )
        .replace(
            "This project follows the shared output model in Projects.",
            "This project follows the shared output model in the shared project guidance.",
        )
        .replace(
            "This spec follows the shared coverage policy in Projects.",
            "This spec follows the shared coverage policy in the shared project guidance.",
        )
        .replace(
            "Follow the shared repository-creation and instruction workflow in Projects and Projects, then use this project's instruction files:",
            "Follow the shared repository-creation and instruction workflow in the shared project guidance, then use this project's instruction files:",
        );

    collapse_blank_lines(&rewritten).trim().to_string()
}

fn collapse_blank_lines(text: &str) -> String {
    let mut result = Vec::new();
    let mut previous_blank = false;

    for line in text.lines() {
        let blank = line.trim().is_empty();
        if blank && previous_blank {
            continue;
        }
        result.push(line);
        previous_blank = blank;
    }

    result.join("\n")
}

fn testing_selection(output: &CompiledOutput) -> Option<&str> {
    output.selections.testing.as_deref()
}

fn output_repo_name(project_slug: &str) -> String {
    format!("{}_{}", OUTPUT_REPO_PREFIX, project_slug)
}

fn flutter_package_name(project_slug: &str) -> String {
    project_slug.replace('-', "_")
}

fn output_repo_description(project_title: &str, selections: &OutputRepoSelections) -> String {
    let mut parts = vec![
        repo_choice_display(&selections.ecosystem),
        repo_choice_display(&selections.language),
        repo_choice_display(&selections.testing),
        repo_choice_display(&selections.mocking),
        repo_choice_display(&selections.storage),
        repo_choice_display(&selections.surface),
        repo_choice_display(&selections.target),
        repo_choice_display(&selections.framework),
    ];

    if let Some(protocol) = &selections.protocol {
        parts.push(repo_choice_display(protocol));
    }

    format!(
        "Tutorial workspace for the {project_title} project with {} choices.",
        parts.join(" / ")
    )
}

fn repo_choice_display(value: &str) -> String {
    match value {
        "dart" => "Dart".to_string(),
        "database-postgres" => "Postgres".to_string(),
        "database-sqlite" => "SQLite".to_string(),
        "javascript" => "JavaScript".to_string(),
        "typescript" => "TypeScript".to_string(),
        "dotnet" => ".NET".to_string(),
        "csharp" => "C#".to_string(),
        "flutter" => "Flutter".to_string(),
        "grpc" => "gRPC".to_string(),
        "vitest" => "Vitest".to_string(),
        "vitest-built-in" => "Vitest built-in".to_string(),
        "xunit" => "xUnit".to_string(),
        "nsubstitute" => "NSubstitute".to_string(),
        "api" => "API".to_string(),
        "astro" => "Astro".to_string(),
        "http-json" => "http-json".to_string(),
        "rest-json" => "rest-json".to_string(),
        "testify-mock" => "testify-mock".to_string(),
        other => other.to_string(),
    }
}

fn go_module_path(spec: &OutputRepoSpec) -> String {
    format!("github.com/{}/{}/workspace", GITHUB_OWNER, spec.repo_name)
}

fn render_go_todo_list_rest_json_sqlite_spec_content() -> String {
    tutorial_file_markdown(
        "Spec",
        "Build a Todo List API that uses HTTP JSON routes and persists task resources in SQLite.\n\n## Goal\n\nBuild a small project app that introduces:\n\n- adapter-owned JSON over HTTP routes\n- database-generated numeric task ids\n- durable SQLite persistence\n- test-first service and adapter logic\n- thin adapters\n\n## Canonical Resource Shape\n\nEvery tutorial run for this path should treat the task resource as:\n\n```text\nid: integer\ntext: string\n```\n\nA fresh SQLite database should create ids in increasing numeric order beginning with `1`.\n\n## Core Logic Contract\n\nThe shared contracts for this path are:\n\n```text\nlist_tasks() -> task[]\ncreate_task(task_text: string) -> task\nget_task(task_id: integer) -> task | not_found\ndelete_task(task_id: integer) -> deleted | not_found\n```\n\nCanonical behavior:\n\n- blank or whitespace-only task text is rejected\n- task ids are generated by SQLite when rows are inserted\n- `GET /api/tasks` returns all tasks ordered by id ascending\n- `POST /api/tasks` creates one task and returns the created task resource\n- `GET /api/tasks/:id` returns the matching task resource or `404`\n- `DELETE /api/tasks/:id` deletes the matching task resource or returns `404`\n\nExamples with a fresh database:\n\n- `POST /api/tasks` with `{\"text\":\"Buy milk\"}` returns `{\"id\":1,\"text\":\"Buy milk\"}`\n- `GET /api/tasks` returns `{\"tasks\":[{\"id\":1,\"text\":\"Buy milk\"}]}`\n- `GET /api/tasks/1` returns `{\"id\":1,\"text\":\"Buy milk\"}`\n- `DELETE /api/tasks/1` returns `204 No Content`\n\n## Non-Goals\n\nThis path does not include:\n\n- authentication or authorization\n- task editing\n- due dates\n- priorities\n- categories or tags\n- multiple lists\n- syncing tasks across machines\n\n## Surface Expectations\n\nFor this path, the API adapter should expose these canonical routes:\n\n- `GET /api/tasks`\n- `POST /api/tasks`\n- `GET /api/tasks/:id`\n- `DELETE /api/tasks/:id`\n\nThe adapter should keep request parsing, route handling, and SQLite wiring thin, and it should delegate task validation and not-found behavior to the code layer.\n\n## Testing And Coverage Contract\n\nMinimum test expectations:\n\n- a test that blank task text is rejected\n- a test that tasks are listed in id order\n- a test that the first created task in a fresh database gets id `1`\n- a test that missing ids return `404`\n- tests for every adapter built in the chosen run that prove it returns `201` for create, `200` for list and fetch, and `204` for delete",
    )
}

fn render_go_todo_list_rest_json_postgres_spec_content() -> String {
    tutorial_file_markdown(
        "Spec",
        "Build a Todo List API that uses REST-style JSON routes and persists task resources in Postgres.\n\n## Goal\n\nBuild a small project app that introduces:\n\n- resource-oriented task routes\n- database-generated UUID task ids\n- durable Postgres persistence\n- `sqlc`-generated Go query code\n- test-first service and adapter logic\n- thin adapters\n\n## Canonical Resource Shape\n\nEvery tutorial run for this path should treat the task resource as:\n\n```text\nid: uuid string\ntext: string\n```\n\nA fresh Postgres database should create a new UUID string every time a task row is inserted.\n\n## Core Logic Contract\n\nThe shared contracts for this path are:\n\n```text\nlist_tasks() -> task[]\ncreate_task(task_text: string) -> task\nget_task(task_id: uuid string) -> task | not_found\ndelete_task(task_id: uuid string) -> deleted | not_found\n```\n\nCanonical behavior:\n\n- blank or whitespace-only task text is rejected\n- task ids are generated by Postgres as UUID strings when rows are inserted\n- the Postgres adapter uses `sqlc` to generate its CRUD query package from checked-in SQL files\n- `GET /api/tasks` returns all tasks ordered by text ascending, then id ascending\n- `POST /api/tasks` creates one task and returns the created task resource\n- `GET /api/tasks/:id` returns the matching task resource or `404`\n- `DELETE /api/tasks/:id` deletes the matching task resource or returns `404`\n\nExamples with a fresh database:\n\n- `POST /api/tasks` with `{\"text\":\"Buy milk\"}` returns `{\"id\":\"11111111-1111-1111-1111-111111111111\",\"text\":\"Buy milk\"}`\n- `GET /api/tasks` returns `{\"tasks\":[{\"id\":\"11111111-1111-1111-1111-111111111111\",\"text\":\"Buy milk\"}]}`\n- `GET /api/tasks/11111111-1111-1111-1111-111111111111` returns `{\"id\":\"11111111-1111-1111-1111-111111111111\",\"text\":\"Buy milk\"}`\n- `DELETE /api/tasks/11111111-1111-1111-1111-111111111111` returns `204 No Content`\n\n## Non-Goals\n\nThis path does not include:\n\n- authentication or authorization\n- task editing\n- due dates\n- priorities\n- categories or tags\n- multiple lists\n- syncing tasks across machines\n\n## Surface Expectations\n\nFor this path, the API adapter should expose these canonical routes:\n\n- `GET /api/tasks`\n- `POST /api/tasks`\n- `GET /api/tasks/:id`\n- `DELETE /api/tasks/:id`\n\nThe adapter should keep request parsing, route handling, and Postgres wiring thin, and it should delegate task validation and not-found behavior to the code layer.\n\n## Testing And Coverage Contract\n\nMinimum test expectations:\n\n- a test that blank task text is rejected\n- a test that tasks are listed in stable order\n- a test that created tasks receive non-empty UUID ids\n- a test that missing ids return `404`\n- tests for every adapter built in the chosen run that prove it returns `201` for create, `200` for list and fetch, and `204` for delete",
    )
}

fn render_go_saying_hello_contracts_content(_spec: &OutputRepoSpec) -> String {
    let contracts_file = "workspace/internal/contracts/greeting.go";
    tutorial_file_markdown(
        "Contracts",
        &rewrite_stage_commit_checkpoints(&rewrite_touch_creation_stage_only(&format!(
            "Create the shared contract file:\n\n```bash\ntouch {contracts_file}\n```\n\nPut this exact content in `{contracts_file}`:\n\n```go\npackage contracts\n\ntype GreetingService interface {{\n\tGreet(name string) string\n}}\n\ntype GreetingResponse struct {{\n\tMessage string `json:\"message\"`\n}}\n```\n\nDo not add tests here. Keep this layer limited to interfaces and small shared types.\n\nThen run:\n\n```bash\ngit add --all\ngit commit --message \"Define greeting contracts\"\n```"
        ))),
    )
}

fn render_go_saying_hello_code_content(_spec: &OutputRepoSpec) -> String {
    tutorial_file_markdown(
        "Code",
        &rewrite_touch_creation_stage_only("### 1. Red: Add The First Failing Code Test\n\nCreate the first test file:\n\n```bash\ntouch workspace/internal/code/greeting_service_test.go\n```\n\nPut this exact content in `workspace/internal/code/greeting_service_test.go`:\n\n```go\npackage code\n\nimport (\n\t\"testing\"\n\n\t\"github.com/stretchr/testify/assert\"\n)\n\nfunc TestGreetingService_GreetReturnsPersonalGreetingForNonEmptyName(t *testing.T) {\n\tservice := GreetingService{}\n\n\tresult := service.Greet(\"Ada\")\n\n\tassert.Equal(t, \"Hello, Ada!\", result)\n}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"1. Red: Add The First Failing Code Test\"\n```\n\n### 2. Green: Return The Personalized Greeting\n\nCreate the first production file:\n\n```bash\ntouch workspace/internal/code/greeting_service.go\n```\n\nPut this exact content in `workspace/internal/code/greeting_service.go`:\n\n```go\npackage code\n\ntype GreetingService struct{}\n\nfunc (s GreetingService) Greet(name string) string {\n\treturn \"Hello, \" + name + \"!\"\n}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"2. Green: Return The Personalized Greeting\"\n```\n\n### 3. Red: Add The Trimming Test\n\nReplace `workspace/internal/code/greeting_service_test.go` with:\n\n```go\npackage code\n\nimport (\n\t\"testing\"\n\n\t\"github.com/stretchr/testify/assert\"\n)\n\nfunc TestGreetingService_GreetReturnsPersonalGreetingForNonEmptyName(t *testing.T) {\n\tservice := GreetingService{}\n\n\tresult := service.Greet(\"Ada\")\n\n\tassert.Equal(t, \"Hello, Ada!\", result)\n}\n\nfunc TestGreetingService_GreetTrimsWhitespaceBeforeGreeting(t *testing.T) {\n\tservice := GreetingService{}\n\n\tresult := service.Greet(\"  Ada  \")\n\n\tassert.Equal(t, \"Hello, Ada!\", result)\n}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"3. Red: Add The Trimming Test\"\n```\n\n### 4. Green: Trim The Name Before Greeting\n\nReplace `workspace/internal/code/greeting_service.go` with:\n\n```go\npackage code\n\nimport \"strings\"\n\ntype GreetingService struct{}\n\nfunc (s GreetingService) Greet(name string) string {\n\ttrimmed := strings.TrimSpace(name)\n\treturn \"Hello, \" + trimmed + \"!\"\n}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"4. Green: Trim The Name Before Greeting\"\n```\n\n### 5. Red: Add Empty-Input Tests\n\nReplace `workspace/internal/code/greeting_service_test.go` with:\n\n```go\npackage code\n\nimport (\n\t\"testing\"\n\n\t\"github.com/stretchr/testify/assert\"\n)\n\nfunc TestGreetingService_GreetReturnsPersonalGreetingForNonEmptyName(t *testing.T) {\n\tservice := GreetingService{}\n\n\tresult := service.Greet(\"Ada\")\n\n\tassert.Equal(t, \"Hello, Ada!\", result)\n}\n\nfunc TestGreetingService_GreetTrimsWhitespaceBeforeGreeting(t *testing.T) {\n\tservice := GreetingService{}\n\n\tresult := service.Greet(\"  Ada  \")\n\n\tassert.Equal(t, \"Hello, Ada!\", result)\n}\n\nfunc TestGreetingService_GreetReturnsGenericGreetingForEmptyName(t *testing.T) {\n\tservice := GreetingService{}\n\n\tresult := service.Greet(\"\")\n\n\tassert.Equal(t, \"Hello!\", result)\n}\n\nfunc TestGreetingService_GreetReturnsGenericGreetingForWhitespaceOnlyName(t *testing.T) {\n\tservice := GreetingService{}\n\n\tresult := service.Greet(\"   \")\n\n\tassert.Equal(t, \"Hello!\", result)\n}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"5. Red: Add Empty-Input Tests\"\n```\n\n### 6. Green: Return The Generic Greeting For Empty Input\n\nReplace `workspace/internal/code/greeting_service.go` with:\n\n```go\npackage code\n\nimport \"strings\"\n\ntype GreetingService struct{}\n\nfunc (s GreetingService) Greet(name string) string {\n\ttrimmed := strings.TrimSpace(name)\n\tif trimmed == \"\" {\n\t\treturn \"Hello!\"\n\t}\n\n\treturn \"Hello, \" + trimmed + \"!\"\n}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"6. Green: Return The Generic Greeting For Empty Input\"\n```"),
    )
}

fn render_go_saying_hello_adapter_content(spec: &OutputRepoSpec) -> String {
    let module_path = go_module_path(spec);
    let body = r#"### 1. Red: Add The First Failing Adapter Test

Create the first adapter test file:

```bash
touch workspace/internal/adapter/http/greeting_handler_test.go
```

Put this exact content in `workspace/internal/adapter/http/greeting_handler_test.go`:

```go
package httpadapter

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockGreetingService struct {
	mock.Mock
}

func (m *MockGreetingService) Greet(name string) string {
	args := m.Called(name)
	return args.String(0)
}

func TestGreetingHandler_GetGreetingReturnsCanonicalJson(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/greeting?name=Ada", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockGreetingService)
	service.On("Greet", "Ada").Return("Hello, Ada!")

	handler := NewGreetingHandler(service)
	err := handler.GetGreeting(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)

	var body contracts.GreetingResponse
	err = json.Unmarshal(rec.Body.Bytes(), &body)

	assert.NoError(t, err)
	assert.Equal(t, "Hello, Ada!", body.Message)
	service.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The First Failing Adapter Test"
```

### 2. Green: Return The Canonical JSON Response

Create the first adapter production file:

```bash
touch workspace/internal/adapter/http/greeting_handler.go
```

Put this exact content in `workspace/internal/adapter/http/greeting_handler.go`:

```go
package httpadapter

import (
	"net/http"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
)

type GreetingHandler struct {
	service contracts.GreetingService
}

func NewGreetingHandler(service contracts.GreetingService) *GreetingHandler {
	return &GreetingHandler{service: service}
}

func (h *GreetingHandler) GetGreeting(c echo.Context) error {
	name := c.QueryParam("name")

	return c.JSON(http.StatusOK, contracts.GreetingResponse{
		Message: h.service.Greet(name),
	})
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Return The Canonical JSON Response"
```

### 3. Red: Add The Empty-Name Adapter Test

Replace `workspace/internal/adapter/http/greeting_handler_test.go` with:

```go
package httpadapter

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockGreetingService struct {
	mock.Mock
}

func (m *MockGreetingService) Greet(name string) string {
	args := m.Called(name)
	return args.String(0)
}

func TestGreetingHandler_GetGreetingReturnsCanonicalJson(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/greeting?name=Ada", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockGreetingService)
	service.On("Greet", "Ada").Return("Hello, Ada!")

	handler := NewGreetingHandler(service)
	err := handler.GetGreeting(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)

	var body contracts.GreetingResponse
	err = json.Unmarshal(rec.Body.Bytes(), &body)

	assert.NoError(t, err)
	assert.Equal(t, "Hello, Ada!", body.Message)
	service.AssertExpectations(t)
}

func TestGreetingHandler_GetGreetingDelegatesEmptyNameForGenericGreeting(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/greeting", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockGreetingService)
	service.On("Greet", "").Return("Hello!")

	handler := NewGreetingHandler(service)
	err := handler.GetGreeting(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)

	var body contracts.GreetingResponse
	err = json.Unmarshal(rec.Body.Bytes(), &body)

	assert.NoError(t, err)
	assert.Equal(t, "Hello!", body.Message)
	service.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Add The Empty-Name Adapter Test"
```

### 4. Green: Wire The Server Entry Point

Create the server entry point:

```bash
touch workspace/cmd/server/main.go
```

Put this exact content in `workspace/cmd/server/main.go`:

```go
package main

import (
	"log"

	httpadapter "__MODULE_PATH__/internal/adapter/http"
	"__MODULE_PATH__/internal/code"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func main() {
	e := echo.New()
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"http://localhost:25616"},
	}))
	service := code.GreetingService{}
	handler := httpadapter.NewGreetingHandler(service)

	e.GET("/api/greeting", handler.GetGreeting)

	log.Fatal(e.Start(":25664"))
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Wire The Server Entry Point"
```"#;
    tutorial_file_markdown(
        "Adapter",
        &rewrite_touch_creation_stage_only(&body.replace("__MODULE_PATH__", &module_path)),
    )
}

fn render_go_todo_list_contracts_content(_spec: &OutputRepoSpec) -> String {
    if is_go_todo_list_http_json_sqlite_output_repo(_spec)
        || is_go_todo_list_rest_json_output_repo(_spec)
    {
        let contracts_file = "workspace/internal/contracts/task_api.go";
        let task_id_type = if is_go_todo_list_rest_json_postgres_output_repo(_spec) {
            "string"
        } else {
            "int64"
        };
        let commit_message = if is_go_todo_list_rest_json_postgres_output_repo(_spec) {
            "Define todo-list REST contracts"
        } else {
            "Define todo-list HTTP JSON contracts"
        };
        return tutorial_file_markdown(
            "Contracts",
            &rewrite_stage_commit_checkpoints(&rewrite_touch_creation_stage_only(&format!(
                "Create the shared contract file:\n\n```bash\ntouch {contracts_file}\n```\n\nPut this exact content in `{contracts_file}`:\n\n```go\npackage contracts\n\nimport \"errors\"\n\nvar (\n\tErrTaskTextBlank = errors.New(\"task text must not be blank\")\n\tErrTaskNotFound  = errors.New(\"task not found\")\n)\n\ntype Task struct {{\n\tID   {task_id_type}  `json:\"id\"`\n\tText string `json:\"text\"`\n}}\n\ntype TaskStore interface {{\n\tListTasks() ([]Task, error)\n\tCreateTask(taskText string) (Task, error)\n\tGetTask(taskID {task_id_type}) (Task, bool, error)\n\tDeleteTask(taskID {task_id_type}) (bool, error)\n}}\n\ntype TaskService interface {{\n\tListTasks() (TaskListResponse, error)\n\tCreateTask(taskText string) (Task, error)\n\tGetTask(taskID {task_id_type}) (Task, error)\n\tDeleteTask(taskID {task_id_type}) error\n}}\n\ntype TaskListResponse struct {{\n\tTasks []Task `json:\"tasks\"`\n}}\n\ntype CreateTaskRequest struct {{\n\tText string `json:\"text\"`\n}}\n\ntype ErrorResponse struct {{\n\tMessage string `json:\"message\"`\n}}\n```\n\nDo not add tests here. Keep this layer limited to interfaces, small shared types, and canonical error values.\n\nThen run:\n\n```bash\ngit add --all\ngit commit --message \"{commit_message}\"\n```"
            ))),
        );
    }

    let contracts_file = "workspace/internal/contracts/task_list_service.go";
    tutorial_file_markdown(
        "Contracts",
        &rewrite_stage_commit_checkpoints(&rewrite_touch_creation_stage_only(&format!(
            "Create the shared contract file:\n\n```bash\ntouch {contracts_file}\n```\n\nPut this exact content in `{contracts_file}`:\n\n```go\npackage contracts\n\ntype TaskStore interface {{\n\tLoadTaskStorage() (string, error)\n\tSaveTaskStorage(storageText string) error\n}}\n\ntype TaskListService interface {{\n\tListTasks() (TaskListResult, error)\n\tAddTask(taskText string) (TaskListResult, error)\n\tRemoveTask(completedTaskText string) (TaskListResult, error)\n}}\n\ntype TaskListResult struct {{\n\tTasks []string `json:\"tasks\"`\n\tLines []string `json:\"lines\"`\n}}\n\ntype AddTaskRequest struct {{\n\tTask string `json:\"task\"`\n}}\n\ntype ErrorResponse struct {{\n\tMessage string `json:\"message\"`\n}}\n```\n\nDo not add tests here. Keep this layer limited to interfaces and small shared types.\n\nThen run:\n\n```bash\ngit add --all\ngit commit --message \"Define task-list contracts\"\n```"
        ))),
    )
}

fn render_go_todo_list_code_content(_spec: &OutputRepoSpec) -> String {
    if is_go_todo_list_http_json_sqlite_output_repo(_spec)
        || is_go_todo_list_rest_json_output_repo(_spec)
    {
        let task_id_type = if is_go_todo_list_rest_json_postgres_output_repo(_spec) {
            "string"
        } else {
            "int64"
        };
        let task_one_id = if is_go_todo_list_rest_json_postgres_output_repo(_spec) {
            "\"11111111-1111-1111-1111-111111111111\""
        } else {
            "1"
        };
        let task_two_id = if is_go_todo_list_rest_json_postgres_output_repo(_spec) {
            "\"22222222-2222-2222-2222-222222222222\""
        } else {
            "2"
        };
        let missing_task_id = if is_go_todo_list_rest_json_postgres_output_repo(_spec) {
            "\"99999999-9999-9999-9999-999999999999\""
        } else {
            "int64(9)"
        };
        let missing_task_call = if is_go_todo_list_rest_json_postgres_output_repo(_spec) {
            "\"99999999-9999-9999-9999-999999999999\""
        } else {
            "9"
        };
        let body = r#"### 1. Red: List Tasks From The Store

Create the first code test file:

```bash
touch workspace/internal/code/task_service_test.go
```

Put this exact content in `workspace/internal/code/task_service_test.go`:

```go
package code

import (
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

type MockTaskStore struct {
	mock.Mock
}

func (m *MockTaskStore) ListTasks() ([]contracts.Task, error) {
	args := m.Called()
	return args.Get(0).([]contracts.Task), args.Error(1)
}

func (m *MockTaskStore) CreateTask(taskText string) (contracts.Task, error) {
	args := m.Called(taskText)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskStore) GetTask(taskID int64) (contracts.Task, bool, error) {
	args := m.Called(taskID)
	return args.Get(0).(contracts.Task), args.Bool(1), args.Error(2)
}

func (m *MockTaskStore) DeleteTask(taskID int64) (bool, error) {
	args := m.Called(taskID)
	return args.Bool(0), args.Error(1)
}

func TestTaskServiceListTasksReturnsTasksInOrder(t *testing.T) {
	store := new(MockTaskStore)
	store.On("ListTasks").Return([]contracts.Task{
		{ID: 1, Text: "Learn how to invert binary trees"},
		{ID: 2, Text: "Buy milk"},
	}, nil)

	service := NewTaskService(store)
	result, err := service.ListTasks()

	require.NoError(t, err)
	assert.Equal(t, []contracts.Task{
		{ID: 1, Text: "Learn how to invert binary trees"},
		{ID: 2, Text: "Buy milk"},
	}, result.Tasks)
	store.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: List Tasks From The Store"
```

### 2. Green: Return The Current Task List

Create the first production file:

```bash
touch workspace/internal/code/task_service.go
```

Put this exact content in `workspace/internal/code/task_service.go`:

```go
package code

import "__MODULE_PATH__/internal/contracts"

type DefaultTaskService struct {
	store contracts.TaskStore
}

func NewTaskService(store contracts.TaskStore) contracts.TaskService {
	return DefaultTaskService{store: store}
}

func (s DefaultTaskService) ListTasks() (contracts.TaskListResponse, error) {
	tasks, err := s.store.ListTasks()
	if err != nil {
		return contracts.TaskListResponse{}, err
	}

	return contracts.TaskListResponse{
		Tasks: append([]contracts.Task{}, tasks...),
	}, nil
}

func (s DefaultTaskService) CreateTask(taskText string) (contracts.Task, error) {
	panic("not implemented")
}

func (s DefaultTaskService) GetTask(taskID int64) (contracts.Task, error) {
	panic("not implemented")
}

func (s DefaultTaskService) DeleteTask(taskID int64) error {
	panic("not implemented")
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Return The Current Task List"
```

### 3. Red: Reject Blank Task Creation

Replace `workspace/internal/code/task_service_test.go` with:

```go
package code

import (
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

type MockTaskStore struct {
	mock.Mock
}

func (m *MockTaskStore) ListTasks() ([]contracts.Task, error) {
	args := m.Called()
	return args.Get(0).([]contracts.Task), args.Error(1)
}

func (m *MockTaskStore) CreateTask(taskText string) (contracts.Task, error) {
	args := m.Called(taskText)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskStore) GetTask(taskID int64) (contracts.Task, bool, error) {
	args := m.Called(taskID)
	return args.Get(0).(contracts.Task), args.Bool(1), args.Error(2)
}

func (m *MockTaskStore) DeleteTask(taskID int64) (bool, error) {
	args := m.Called(taskID)
	return args.Bool(0), args.Error(1)
}

func TestTaskServiceListTasksReturnsTasksInOrder(t *testing.T) {
	store := new(MockTaskStore)
	store.On("ListTasks").Return([]contracts.Task{
		{ID: 1, Text: "Learn how to invert binary trees"},
		{ID: 2, Text: "Buy milk"},
	}, nil)

	service := NewTaskService(store)
	result, err := service.ListTasks()

	require.NoError(t, err)
	assert.Equal(t, []contracts.Task{
		{ID: 1, Text: "Learn how to invert binary trees"},
		{ID: 2, Text: "Buy milk"},
	}, result.Tasks)
	store.AssertExpectations(t)
}

func TestTaskServiceCreateTaskRejectsBlankText(t *testing.T) {
	store := new(MockTaskStore)
	service := NewTaskService(store)

	_, err := service.CreateTask("   ")

	assert.ErrorIs(t, err, contracts.ErrTaskTextBlank)
	store.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Reject Blank Task Creation"
```

### 4. Green: Create Tasks And Trim Input

Replace `workspace/internal/code/task_service.go` with:

```go
package code

import (
	"strings"

	"__MODULE_PATH__/internal/contracts"
)

type DefaultTaskService struct {
	store contracts.TaskStore
}

func NewTaskService(store contracts.TaskStore) contracts.TaskService {
	return DefaultTaskService{store: store}
}

func (s DefaultTaskService) ListTasks() (contracts.TaskListResponse, error) {
	tasks, err := s.store.ListTasks()
	if err != nil {
		return contracts.TaskListResponse{}, err
	}

	return contracts.TaskListResponse{
		Tasks: append([]contracts.Task{}, tasks...),
	}, nil
}

func (s DefaultTaskService) CreateTask(taskText string) (contracts.Task, error) {
	trimmed := strings.TrimSpace(taskText)
	if trimmed == "" {
		return contracts.Task{}, contracts.ErrTaskTextBlank
	}

	return s.store.CreateTask(trimmed)
}

func (s DefaultTaskService) GetTask(taskID int64) (contracts.Task, error) {
	panic("not implemented")
}

func (s DefaultTaskService) DeleteTask(taskID int64) error {
	panic("not implemented")
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Create Tasks And Trim Input"
```

### 5. Red: Add Lookup And Delete Not-Found Behavior

Replace `workspace/internal/code/task_service_test.go` with:

```go
package code

import (
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

type MockTaskStore struct {
	mock.Mock
}

func (m *MockTaskStore) ListTasks() ([]contracts.Task, error) {
	args := m.Called()
	return args.Get(0).([]contracts.Task), args.Error(1)
}

func (m *MockTaskStore) CreateTask(taskText string) (contracts.Task, error) {
	args := m.Called(taskText)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskStore) GetTask(taskID int64) (contracts.Task, bool, error) {
	args := m.Called(taskID)
	return args.Get(0).(contracts.Task), args.Bool(1), args.Error(2)
}

func (m *MockTaskStore) DeleteTask(taskID int64) (bool, error) {
	args := m.Called(taskID)
	return args.Bool(0), args.Error(1)
}

func TestTaskServiceListTasksReturnsTasksInOrder(t *testing.T) {
	store := new(MockTaskStore)
	store.On("ListTasks").Return([]contracts.Task{
		{ID: 1, Text: "Learn how to invert binary trees"},
		{ID: 2, Text: "Buy milk"},
	}, nil)

	service := NewTaskService(store)
	result, err := service.ListTasks()

	require.NoError(t, err)
	assert.Equal(t, []contracts.Task{
		{ID: 1, Text: "Learn how to invert binary trees"},
		{ID: 2, Text: "Buy milk"},
	}, result.Tasks)
	store.AssertExpectations(t)
}

func TestTaskServiceCreateTaskRejectsBlankText(t *testing.T) {
	store := new(MockTaskStore)
	service := NewTaskService(store)

	_, err := service.CreateTask("   ")

	assert.ErrorIs(t, err, contracts.ErrTaskTextBlank)
	store.AssertExpectations(t)
}

func TestTaskServiceGetTaskReturnsNotFoundForMissingIds(t *testing.T) {
	store := new(MockTaskStore)
	store.On("GetTask", int64(9)).Return(contracts.Task{}, false, nil)

	service := NewTaskService(store)
	_, err := service.GetTask(9)

	assert.ErrorIs(t, err, contracts.ErrTaskNotFound)
	store.AssertExpectations(t)
}

func TestTaskServiceDeleteTaskReturnsNotFoundForMissingIds(t *testing.T) {
	store := new(MockTaskStore)
	store.On("DeleteTask", int64(9)).Return(false, nil)

	service := NewTaskService(store)
	err := service.DeleteTask(9)

	assert.ErrorIs(t, err, contracts.ErrTaskNotFound)
	store.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "5. Red: Add Lookup And Delete Not-Found Behavior"
```

### 6. Green: Return Tasks By Id And Delete Existing Tasks

Replace `workspace/internal/code/task_service.go` with:

```go
package code

import (
	"strings"

	"__MODULE_PATH__/internal/contracts"
)

type DefaultTaskService struct {
	store contracts.TaskStore
}

func NewTaskService(store contracts.TaskStore) contracts.TaskService {
	return DefaultTaskService{store: store}
}

func (s DefaultTaskService) ListTasks() (contracts.TaskListResponse, error) {
	tasks, err := s.store.ListTasks()
	if err != nil {
		return contracts.TaskListResponse{}, err
	}

	return contracts.TaskListResponse{
		Tasks: append([]contracts.Task{}, tasks...),
	}, nil
}

func (s DefaultTaskService) CreateTask(taskText string) (contracts.Task, error) {
	trimmed := strings.TrimSpace(taskText)
	if trimmed == "" {
		return contracts.Task{}, contracts.ErrTaskTextBlank
	}

	return s.store.CreateTask(trimmed)
}

func (s DefaultTaskService) GetTask(taskID int64) (contracts.Task, error) {
	task, found, err := s.store.GetTask(taskID)
	if err != nil {
		return contracts.Task{}, err
	}
	if !found {
		return contracts.Task{}, contracts.ErrTaskNotFound
	}

	return task, nil
}

func (s DefaultTaskService) DeleteTask(taskID int64) error {
	deleted, err := s.store.DeleteTask(taskID)
	if err != nil {
		return err
	}
	if !deleted {
		return contracts.ErrTaskNotFound
	}

	return nil
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "6. Green: Return Tasks By Id And Delete Existing Tasks"
```"#;

        let body = body
            .replace("int64(9)", missing_task_id)
            .replace("GetTask(9)", &format!("GetTask({missing_task_call})"))
            .replace("DeleteTask(9)", &format!("DeleteTask({missing_task_call})"))
            .replace("int64", task_id_type)
            .replace("ID: 1,", &format!("ID: {task_one_id},"))
            .replace("ID: 2,", &format!("ID: {task_two_id},"));

        return tutorial_file_markdown(
            "Code",
            &rewrite_touch_creation_stage_only(&body.replace("__MODULE_PATH__", &go_module_path(_spec))),
        );
    }

    let body = r#"### 1. Red: Parse The Canonical Stored Task Data

Create the first code test file:

```bash
touch workspace/internal/code/task_list_service_test.go
```

Put this exact content in `workspace/internal/code/task_list_service_test.go`:

```go
package code

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseTaskStoragePreservesTheCanonicalTasksInOrder(t *testing.T) {
	storageText := "[\n  \"Learn how to invert binary trees\",\n  \"Buy milk\",\n  \"Clean kitchen\"\n]"

	result := parseTaskStorage(storageText)

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, result)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Parse The Canonical Stored Task Data"
```

### 2. Green: Parse The Canonical Stored Task Data

Create the first production file:

```bash
touch workspace/internal/code/task_list_service.go
```

Put this exact content in `workspace/internal/code/task_list_service.go`:

```go
package code

import "encoding/json"

func parseTaskStorage(storageText string) []string {
	var tasks []string
	_ = json.Unmarshal([]byte(storageText), &tasks)
	return tasks
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Parse The Canonical Stored Task Data"
```

### 3. Red: Append And Remove Tasks

Replace `workspace/internal/code/task_list_service_test.go` with:

```go
package code

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseTaskStoragePreservesTheCanonicalTasksInOrder(t *testing.T) {
	storageText := "[\n  \"Learn how to invert binary trees\",\n  \"Buy milk\",\n  \"Clean kitchen\"\n]"

	result := parseTaskStorage(storageText)

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, result)
}

func TestAppendTaskAppendsTheTaskToTheEndOfANewList(t *testing.T) {
	result := appendTask([]string{
		"Learn how to invert binary trees",
		"Buy milk",
	}, "Clean kitchen")

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, result)
}

func TestRemoveTaskByExactTextRemovesTheFirstExactMatch(t *testing.T) {
	result := removeTaskByExactText([]string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, "Buy milk")

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Clean kitchen",
	}, result)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Append And Remove Tasks"
```

### 4. Green: Append And Remove Tasks

Replace `workspace/internal/code/task_list_service.go` with:

```go
package code

import "encoding/json"

func parseTaskStorage(storageText string) []string {
	var tasks []string
	_ = json.Unmarshal([]byte(storageText), &tasks)
	return tasks
}

func appendTask(taskList []string, taskText string) []string {
	next := append([]string{}, taskList...)
	return append(next, taskText)
}

func removeTaskByExactText(taskList []string, completedTaskText string) []string {
	next := append([]string{}, taskList...)
	for index, task := range next {
		if task == completedTaskText {
			return append(next[:index], next[index+1:]...)
		}
	}
	return next
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Append And Remove Tasks"
```

### 5. Red: Format And Serialize The Task List

Replace `workspace/internal/code/task_list_service_test.go` with:

```go
package code

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseTaskStoragePreservesTheCanonicalTasksInOrder(t *testing.T) {
	storageText := "[\n  \"Learn how to invert binary trees\",\n  \"Buy milk\",\n  \"Clean kitchen\"\n]"

	result := parseTaskStorage(storageText)

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, result)
}

func TestAppendTaskAppendsTheTaskToTheEndOfANewList(t *testing.T) {
	result := appendTask([]string{
		"Learn how to invert binary trees",
		"Buy milk",
	}, "Clean kitchen")

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, result)
}

func TestRemoveTaskByExactTextRemovesTheFirstExactMatch(t *testing.T) {
	result := removeTaskByExactText([]string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, "Buy milk")

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Clean kitchen",
	}, result)
}

func TestFormatTaskListReturnsOneLinePerTaskInOrder(t *testing.T) {
	result := formatTaskList([]string{
		"Learn how to invert binary trees",
		"Buy milk",
	})

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Buy milk",
	}, result)
}

func TestSerializeTaskStorageReturnsAJsonArrayInOrder(t *testing.T) {
	result := serializeTaskStorage([]string{
		"Learn how to invert binary trees",
		"Buy milk",
	})

	assert.JSONEq(t, "[\"Learn how to invert binary trees\",\"Buy milk\"]", result)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "5. Red: Format And Serialize The Task List"
```

### 6. Green: Format And Serialize The Task List

Replace `workspace/internal/code/task_list_service.go` with:

```go
package code

import "encoding/json"

func parseTaskStorage(storageText string) []string {
	var tasks []string
	_ = json.Unmarshal([]byte(storageText), &tasks)
	return tasks
}

func appendTask(taskList []string, taskText string) []string {
	next := append([]string{}, taskList...)
	return append(next, taskText)
}

func removeTaskByExactText(taskList []string, completedTaskText string) []string {
	next := append([]string{}, taskList...)
	for index, task := range next {
		if task == completedTaskText {
			return append(next[:index], next[index+1:]...)
		}
	}
	return next
}

func formatTaskList(taskList []string) []string {
	return append([]string{}, taskList...)
}

func serializeTaskStorage(taskList []string) string {
	storageBytes, _ := json.Marshal(taskList)
	return string(storageBytes)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "6. Green: Format And Serialize The Task List"
```

### 7. Red: Add The Task Service Tests

Replace `workspace/internal/code/task_list_service_test.go` with:

```go
package code

import (
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockTaskStore struct {
	mock.Mock
}

func (m *MockTaskStore) LoadTaskStorage() (string, error) {
	args := m.Called()
	return args.String(0), args.Error(1)
}

func (m *MockTaskStore) SaveTaskStorage(storageText string) error {
	args := m.Called(storageText)
	return args.Error(0)
}

func TestParseTaskStoragePreservesTheCanonicalTasksInOrder(t *testing.T) {
	storageText := "[\n  \"Learn how to invert binary trees\",\n  \"Buy milk\",\n  \"Clean kitchen\"\n]"

	result := parseTaskStorage(storageText)

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, result)
}

func TestAppendTaskAppendsTheTaskToTheEndOfANewList(t *testing.T) {
	result := appendTask([]string{
		"Learn how to invert binary trees",
		"Buy milk",
	}, "Clean kitchen")

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, result)
}

func TestRemoveTaskByExactTextRemovesTheFirstExactMatch(t *testing.T) {
	result := removeTaskByExactText([]string{
		"Learn how to invert binary trees",
		"Buy milk",
		"Clean kitchen",
	}, "Buy milk")

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Clean kitchen",
	}, result)
}

func TestFormatTaskListReturnsOneLinePerTaskInOrder(t *testing.T) {
	result := formatTaskList([]string{
		"Learn how to invert binary trees",
		"Buy milk",
	})

	assert.Equal(t, []string{
		"Learn how to invert binary trees",
		"Buy milk",
	}, result)
}

func TestSerializeTaskStorageReturnsAJsonArrayInOrder(t *testing.T) {
	result := serializeTaskStorage([]string{
		"Learn how to invert binary trees",
		"Buy milk",
	})

	assert.JSONEq(t, "[\"Learn how to invert binary trees\",\"Buy milk\"]", result)
}

func TestTaskListServiceListTasksLoadsAndFormatsTasks(t *testing.T) {
	store := new(MockTaskStore)
	store.On("LoadTaskStorage").Return("[\"Learn how to invert binary trees\",\"Buy milk\"]", nil)

	service := NewTaskListService(store)
	result, err := service.ListTasks()

	assert.NoError(t, err)
	assert.Equal(t, contracts.TaskListResult{
		Tasks: []string{"Learn how to invert binary trees", "Buy milk"},
		Lines: []string{"Learn how to invert binary trees", "Buy milk"},
	}, result)
	store.AssertExpectations(t)
}

func TestTaskListServiceAddTaskPersistsUpdatedTasks(t *testing.T) {
	store := new(MockTaskStore)
	store.On("LoadTaskStorage").Return("[\"Learn how to invert binary trees\"]", nil)
	store.On("SaveTaskStorage", "[\"Learn how to invert binary trees\",\"Buy milk\"]").Return(nil)

	service := NewTaskListService(store)
	result, err := service.AddTask("Buy milk")

	assert.NoError(t, err)
	assert.Equal(t, contracts.TaskListResult{
		Tasks: []string{"Learn how to invert binary trees", "Buy milk"},
		Lines: []string{"Learn how to invert binary trees", "Buy milk"},
	}, result)
	store.AssertExpectations(t)
}

func TestTaskListServiceRemoveTaskPersistsUpdatedTasks(t *testing.T) {
	store := new(MockTaskStore)
	store.On("LoadTaskStorage").Return("[\"Learn how to invert binary trees\",\"Buy milk\"]", nil)
	store.On("SaveTaskStorage", "[\"Learn how to invert binary trees\"]").Return(nil)

	service := NewTaskListService(store)
	result, err := service.RemoveTask("Buy milk")

	assert.NoError(t, err)
	assert.Equal(t, contracts.TaskListResult{
		Tasks: []string{"Learn how to invert binary trees"},
		Lines: []string{"Learn how to invert binary trees"},
	}, result)
	store.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "7. Red: Add The Task Service Tests"
```

### 8. Green: Add The Task Service

Replace `workspace/internal/code/task_list_service.go` with:

```go
package code

import (
	"encoding/json"

	"__MODULE_PATH__/internal/contracts"
)

type DefaultTaskListService struct {
	store contracts.TaskStore
}

func NewTaskListService(store contracts.TaskStore) contracts.TaskListService {
	return DefaultTaskListService{store: store}
}

func parseTaskStorage(storageText string) []string {
	var tasks []string
	_ = json.Unmarshal([]byte(storageText), &tasks)
	return tasks
}

func appendTask(taskList []string, taskText string) []string {
	next := append([]string{}, taskList...)
	return append(next, taskText)
}

func removeTaskByExactText(taskList []string, completedTaskText string) []string {
	next := append([]string{}, taskList...)
	for index, task := range next {
		if task == completedTaskText {
			return append(next[:index], next[index+1:]...)
		}
	}
	return next
}

func formatTaskList(taskList []string) []string {
	return append([]string{}, taskList...)
}

func serializeTaskStorage(taskList []string) string {
	storageBytes, _ := json.Marshal(taskList)
	return string(storageBytes)
}

func (s DefaultTaskListService) ListTasks() (contracts.TaskListResult, error) {
	storageText, err := s.store.LoadTaskStorage()
	if err != nil {
		return contracts.TaskListResult{}, err
	}

	tasks := parseTaskStorage(storageText)
	return buildTaskListResult(tasks), nil
}

func (s DefaultTaskListService) AddTask(taskText string) (contracts.TaskListResult, error) {
	storageText, err := s.store.LoadTaskStorage()
	if err != nil {
		return contracts.TaskListResult{}, err
	}

	nextTasks := appendTask(parseTaskStorage(storageText), taskText)
	if err := s.store.SaveTaskStorage(serializeTaskStorage(nextTasks)); err != nil {
		return contracts.TaskListResult{}, err
	}

	return buildTaskListResult(nextTasks), nil
}

func (s DefaultTaskListService) RemoveTask(completedTaskText string) (contracts.TaskListResult, error) {
	storageText, err := s.store.LoadTaskStorage()
	if err != nil {
		return contracts.TaskListResult{}, err
	}

	nextTasks := removeTaskByExactText(parseTaskStorage(storageText), completedTaskText)
	if err := s.store.SaveTaskStorage(serializeTaskStorage(nextTasks)); err != nil {
		return contracts.TaskListResult{}, err
	}

	return buildTaskListResult(nextTasks), nil
}

func buildTaskListResult(taskList []string) contracts.TaskListResult {
	return contracts.TaskListResult{
		Tasks: append([]string{}, taskList...),
		Lines: formatTaskList(taskList),
	}
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "8. Green: Add The Task Service"
```"#
        .replace("__MODULE_PATH__", &go_module_path(_spec));

    tutorial_file_markdown(
        "Code",
        &rewrite_touch_creation_stage_only(&body),
    )
}

fn render_go_todo_list_adapter_content(spec: &OutputRepoSpec) -> String {
    if is_go_todo_list_rest_json_postgres_output_repo(spec) {
        let module_path = go_module_path(spec);
        let body = r#"### 1. Red: Add The REST Handler Tests

Create the first adapter test file:

```bash
touch workspace/internal/adapter/http/task_handler_test.go
```

Put this exact content in `workspace/internal/adapter/http/task_handler_test.go`:

```go
package httpadapter

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockTaskService struct {
	mock.Mock
}

func (m *MockTaskService) ListTasks() (contracts.TaskListResponse, error) {
	args := m.Called()
	return args.Get(0).(contracts.TaskListResponse), args.Error(1)
}

func (m *MockTaskService) CreateTask(taskText string) (contracts.Task, error) {
	args := m.Called(taskText)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskService) GetTask(taskID string) (contracts.Task, error) {
	args := m.Called(taskID)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskService) DeleteTask(taskID string) error {
	args := m.Called(taskID)
	return args.Error(0)
}

func TestTaskHandlerListTasksReturnsTheCurrentTaskResources(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/tasks", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskService)
	service.On("ListTasks").Return(contracts.TaskListResponse{
		Tasks: []contracts.Task{
			{ID: "11111111-1111-1111-1111-111111111111", Text: "Learn how to invert binary trees"},
			{ID: "22222222-2222-2222-2222-222222222222", Text: "Buy milk"},
		},
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.ListTasks(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)

	var body contracts.TaskListResponse
	err = json.Unmarshal(rec.Body.Bytes(), &body)

	assert.NoError(t, err)
	assert.Equal(t, []contracts.Task{
		{ID: "11111111-1111-1111-1111-111111111111", Text: "Learn how to invert binary trees"},
		{ID: "22222222-2222-2222-2222-222222222222", Text: "Buy milk"},
	}, body.Tasks)
	service.AssertExpectations(t)
}

func TestTaskHandlerCreateTaskReturnsCreatedResource(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(
		http.MethodPost,
		"/api/tasks",
		bytes.NewBufferString("{\"text\":\"Buy milk\"}"),
	)
	req.Header.Set(echo.HeaderContentType, echo.MIMEApplicationJSON)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskService)
	service.On("CreateTask", "Buy milk").Return(contracts.Task{
		ID:   "11111111-1111-1111-1111-111111111111",
		Text: "Buy milk",
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.CreateTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, rec.Code)
	service.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The REST Handler Tests"
```

### 2. Green: Return REST Resources

Create the first adapter production file:

```bash
touch workspace/internal/adapter/http/task_handler.go
```

Put this exact content in `workspace/internal/adapter/http/task_handler.go`:

```go
package httpadapter

import (
	"errors"
	"net/http"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
)

type TaskHandler struct {
	service contracts.TaskService
}

func NewTaskHandler(service contracts.TaskService) *TaskHandler {
	return &TaskHandler{service: service}
}

func (h *TaskHandler) ListTasks(c echo.Context) error {
	result, err := h.service.ListTasks()
	if err != nil {
		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusOK, result)
}

func (h *TaskHandler) CreateTask(c echo.Context) error {
	var request contracts.CreateTaskRequest
	if err := c.Bind(&request); err != nil {
		return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
			Message: "invalid request body",
		})
	}

	task, err := h.service.CreateTask(request.Text)
	if err != nil {
		if errors.Is(err, contracts.ErrTaskTextBlank) {
			return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
				Message: err.Error(),
			})
		}

		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusCreated, task)
}

func (h *TaskHandler) GetTask(c echo.Context) error {
	taskID := c.Param("id")
	if taskID == "" {
		return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
			Message: "task id must not be empty",
		})
	}

	task, err := h.service.GetTask(taskID)
	if err != nil {
		if errors.Is(err, contracts.ErrTaskNotFound) {
			return c.JSON(http.StatusNotFound, contracts.ErrorResponse{
				Message: err.Error(),
			})
		}

		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusOK, task)
}

func (h *TaskHandler) DeleteTask(c echo.Context) error {
	taskID := c.Param("id")
	if taskID == "" {
		return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
			Message: "task id must not be empty",
		})
	}

	err := h.service.DeleteTask(taskID)
	if err != nil {
		if errors.Is(err, contracts.ErrTaskNotFound) {
			return c.JSON(http.StatusNotFound, contracts.ErrorResponse{
				Message: err.Error(),
			})
		}

		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.NoContent(http.StatusNoContent)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Return REST Resources"
```

### 3. Red: Add Fetch, Delete, And Postgres Store Tests

Replace `workspace/internal/adapter/http/task_handler_test.go` with:

```go
package httpadapter

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockTaskService struct {
	mock.Mock
}

func (m *MockTaskService) ListTasks() (contracts.TaskListResponse, error) {
	args := m.Called()
	return args.Get(0).(contracts.TaskListResponse), args.Error(1)
}

func (m *MockTaskService) CreateTask(taskText string) (contracts.Task, error) {
	args := m.Called(taskText)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskService) GetTask(taskID string) (contracts.Task, error) {
	args := m.Called(taskID)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskService) DeleteTask(taskID string) error {
	args := m.Called(taskID)
	return args.Error(0)
}

func TestTaskHandlerListTasksReturnsTheCurrentTaskResources(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/tasks", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskService)
	service.On("ListTasks").Return(contracts.TaskListResponse{
		Tasks: []contracts.Task{
			{ID: "11111111-1111-1111-1111-111111111111", Text: "Learn how to invert binary trees"},
			{ID: "22222222-2222-2222-2222-222222222222", Text: "Buy milk"},
		},
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.ListTasks(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)
	service.AssertExpectations(t)
}

func TestTaskHandlerCreateTaskReturnsCreatedResource(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(
		http.MethodPost,
		"/api/tasks",
		bytes.NewBufferString("{\"text\":\"Buy milk\"}"),
	)
	req.Header.Set(echo.HeaderContentType, echo.MIMEApplicationJSON)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskService)
	service.On("CreateTask", "Buy milk").Return(contracts.Task{
		ID:   "11111111-1111-1111-1111-111111111111",
		Text: "Buy milk",
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.CreateTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, rec.Code)
	service.AssertExpectations(t)
}

func TestTaskHandlerGetTaskReturnsTheRequestedResource(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/tasks/11111111-1111-1111-1111-111111111111", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)
	ctx.SetPath("/api/tasks/:id")
	ctx.SetParamNames("id")
	ctx.SetParamValues("11111111-1111-1111-1111-111111111111")

	service := new(MockTaskService)
	service.On("GetTask", "11111111-1111-1111-1111-111111111111").Return(contracts.Task{
		ID:   "11111111-1111-1111-1111-111111111111",
		Text: "Buy milk",
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.GetTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)
	service.AssertExpectations(t)
}

func TestTaskHandlerDeleteTaskReturnsNoContent(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodDelete, "/api/tasks/11111111-1111-1111-1111-111111111111", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)
	ctx.SetPath("/api/tasks/:id")
	ctx.SetParamNames("id")
	ctx.SetParamValues("11111111-1111-1111-1111-111111111111")

	service := new(MockTaskService)
	service.On("DeleteTask", "11111111-1111-1111-1111-111111111111").Return(nil)

	handler := NewTaskHandler(service)
	err := handler.DeleteTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusNoContent, rec.Code)
	service.AssertExpectations(t)
}
```

Create the Postgres store test file:

```bash
touch workspace/internal/adapter/storage/postgres_task_store_test.go
```

Put this exact content in `workspace/internal/adapter/storage/postgres_task_store_test.go`:

```go
package storageadapter

import (
	"regexp"
	"testing"

	storedb "__MODULE_PATH__/internal/adapter/storage/db"
	"__MODULE_PATH__/internal/contracts"
	sqlmock "github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestPostgresTaskStoreCreatesTasksWithDatabaseGeneratedUuids(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	t.Cleanup(func() { _ = db.Close() })

	store := &PostgresTaskStore{queries: storedb.New(db)}
	mock.ExpectQuery(regexp.QuoteMeta(`INSERT INTO tasks (text) VALUES ($1) RETURNING id, text`)).
		WithArgs("Learn how to invert binary trees").
		WillReturnRows(sqlmock.NewRows([]string{"id", "text"}).AddRow("11111111-1111-1111-1111-111111111111", "Learn how to invert binary trees"))
	mock.ExpectQuery(regexp.QuoteMeta(`INSERT INTO tasks (text) VALUES ($1) RETURNING id, text`)).
		WithArgs("Buy milk").
		WillReturnRows(sqlmock.NewRows([]string{"id", "text"}).AddRow("22222222-2222-2222-2222-222222222222", "Buy milk"))

	first, err := store.CreateTask("Learn how to invert binary trees")
	require.NoError(t, err)

	second, err := store.CreateTask("Buy milk")
	require.NoError(t, err)

	assert.Equal(t, contracts.Task{ID: "11111111-1111-1111-1111-111111111111", Text: "Learn how to invert binary trees"}, first)
	assert.Equal(t, contracts.Task{ID: "22222222-2222-2222-2222-222222222222", Text: "Buy milk"}, second)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestPostgresTaskStoreListsGetsAndDeletesTasks(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	t.Cleanup(func() { _ = db.Close() })

	store := &PostgresTaskStore{queries: storedb.New(db)}
	mock.ExpectQuery(regexp.QuoteMeta(`SELECT id, text FROM tasks ORDER BY text ASC, id ASC`)).
		WillReturnRows(sqlmock.NewRows([]string{"id", "text"}).AddRow("11111111-1111-1111-1111-111111111111", "Buy milk"))

	listed, err := store.ListTasks()
	require.NoError(t, err)
	assert.Equal(t, []contracts.Task{{ID: "11111111-1111-1111-1111-111111111111", Text: "Buy milk"}}, listed)

	mock.ExpectQuery(regexp.QuoteMeta(`SELECT id, text FROM tasks WHERE id = $1 LIMIT 1`)).
		WithArgs("11111111-1111-1111-1111-111111111111").
		WillReturnRows(sqlmock.NewRows([]string{"id", "text"}).AddRow("11111111-1111-1111-1111-111111111111", "Buy milk"))

	got, found, err := store.GetTask("11111111-1111-1111-1111-111111111111")
	require.NoError(t, err)
	assert.True(t, found)
	assert.Equal(t, contracts.Task{ID: "11111111-1111-1111-1111-111111111111", Text: "Buy milk"}, got)

	mock.ExpectExec(regexp.QuoteMeta(`DELETE FROM tasks WHERE id = $1`)).
		WithArgs("11111111-1111-1111-1111-111111111111").
		WillReturnResult(sqlmock.NewResult(0, 1))

	deleted, err := store.DeleteTask("11111111-1111-1111-1111-111111111111")
	require.NoError(t, err)
	assert.True(t, deleted)
	assert.NoError(t, mock.ExpectationsWereMet())
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Add Fetch, Delete, And Postgres Store Tests"
```

### 4. Green: Add The Postgres Store And Server Wiring

Use the `sqlc` configuration and SQL files that you created in `tutorial/setup.md`. The generated `just check-tests` and `just run` commands already call `sqlc generate` for you before they compile the app.

Create the Postgres store production file:

```bash
touch workspace/internal/adapter/storage/postgres_task_store.go
```

Put this exact content in `workspace/internal/adapter/storage/postgres_task_store.go`:

```go
package storageadapter

import (
	"context"
	"database/sql"
	"errors"

	storedb "__MODULE_PATH__/internal/adapter/storage/db"
	"__MODULE_PATH__/internal/contracts"
	_ "github.com/jackc/pgx/v5/stdlib"
)

type PostgresTaskStore struct {
	db      *sql.DB
	queries *storedb.Queries
}

func NewPostgresTaskStore(databaseURL string) (*PostgresTaskStore, error) {
	db, err := sql.Open("pgx", databaseURL)
	if err != nil {
		return nil, err
	}

	store := &PostgresTaskStore{
		db:      db,
		queries: storedb.New(db),
	}
	if err := store.ensureSchema(); err != nil {
		_ = db.Close()
		return nil, err
	}

	return store, nil
}

func (s *PostgresTaskStore) Close() error {
	return s.db.Close()
}

func (s *PostgresTaskStore) ensureSchema() error {
	_, err := s.db.Exec(`
		CREATE TABLE IF NOT EXISTS tasks (
			id TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
			text TEXT NOT NULL
		)
	`)
	return err
}

func (s *PostgresTaskStore) ListTasks() ([]contracts.Task, error) {
	rows, err := s.queries.ListTasks(context.Background())
	if err != nil {
		return nil, err
	}

	tasks := make([]contracts.Task, 0, len(rows))
	for _, row := range rows {
		tasks = append(tasks, contracts.Task{
			ID:   row.ID,
			Text: row.Text,
		})
	}

	return tasks, nil
}

func (s *PostgresTaskStore) CreateTask(taskText string) (contracts.Task, error) {
	row, err := s.queries.CreateTask(context.Background(), taskText)
	if err != nil {
		return contracts.Task{}, err
	}

	return contracts.Task{
		ID:   row.ID,
		Text: row.Text,
	}, nil
}

func (s *PostgresTaskStore) GetTask(taskID string) (contracts.Task, bool, error) {
	row, err := s.queries.GetTask(context.Background(), taskID)
	if errors.Is(err, sql.ErrNoRows) {
		return contracts.Task{}, false, nil
	}
	if err != nil {
		return contracts.Task{}, false, err
	}

	return contracts.Task{
		ID:   row.ID,
		Text: row.Text,
	}, true, nil
}

func (s *PostgresTaskStore) DeleteTask(taskID string) (bool, error) {
	rowsAffected, err := s.queries.DeleteTask(context.Background(), taskID)
	if err != nil {
		return false, err
	}

	return rowsAffected > 0, nil
}
```

Create the server entry point:

```bash
touch workspace/cmd/server/main.go
```

Put this exact content in `workspace/cmd/server/main.go`:

```go
package main

import (
	"log"
	"os"

	httpadapter "__MODULE_PATH__/internal/adapter/http"
	storageadapter "__MODULE_PATH__/internal/adapter/storage"
	"__MODULE_PATH__/internal/code"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func main() {
	e := echo.New()
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"http://localhost:25616"},
	}))

	databaseURL := os.Getenv("TODO_LIST_DATABASE_URL")
	if databaseURL == "" {
		log.Fatal("TODO_LIST_DATABASE_URL must not be empty")
	}

	store, err := storageadapter.NewPostgresTaskStore(databaseURL)
	if err != nil {
		log.Fatal(err)
	}
	defer store.Close()

	service := code.NewTaskService(store)
	handler := httpadapter.NewTaskHandler(service)

	e.GET("/api/tasks", handler.ListTasks)
	e.POST("/api/tasks", handler.CreateTask)
	e.GET("/api/tasks/:id", handler.GetTask)
	e.DELETE("/api/tasks/:id", handler.DeleteTask)

	log.Fatal(e.Start(":25664"))
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Add The Postgres Store And Server Wiring"
```"#;

        return tutorial_file_markdown(
            "Adapter",
            &rewrite_touch_creation_stage_only(&body.replace("__MODULE_PATH__", &module_path)),
        );
    }

    if is_go_todo_list_http_json_sqlite_output_repo(spec) {
        let module_path = go_module_path(spec);
        let body = r#"### 1. Red: Add The REST Handler Tests

Create the first adapter test file:

```bash
touch workspace/internal/adapter/http/task_handler_test.go
```

Put this exact content in `workspace/internal/adapter/http/task_handler_test.go`:

```go
package httpadapter

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockTaskService struct {
	mock.Mock
}

func (m *MockTaskService) ListTasks() (contracts.TaskListResponse, error) {
	args := m.Called()
	return args.Get(0).(contracts.TaskListResponse), args.Error(1)
}

func (m *MockTaskService) CreateTask(taskText string) (contracts.Task, error) {
	args := m.Called(taskText)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskService) GetTask(taskID int64) (contracts.Task, error) {
	args := m.Called(taskID)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskService) DeleteTask(taskID int64) error {
	args := m.Called(taskID)
	return args.Error(0)
}

func TestTaskHandlerListTasksReturnsTheCurrentTaskResources(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/tasks", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskService)
	service.On("ListTasks").Return(contracts.TaskListResponse{
		Tasks: []contracts.Task{
			{ID: 1, Text: "Learn how to invert binary trees"},
			{ID: 2, Text: "Buy milk"},
		},
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.ListTasks(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)

	var body contracts.TaskListResponse
	err = json.Unmarshal(rec.Body.Bytes(), &body)

	assert.NoError(t, err)
	assert.Equal(t, []contracts.Task{
		{ID: 1, Text: "Learn how to invert binary trees"},
		{ID: 2, Text: "Buy milk"},
	}, body.Tasks)
	service.AssertExpectations(t)
}

func TestTaskHandlerCreateTaskReturnsCreatedResource(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(
		http.MethodPost,
		"/api/tasks",
		bytes.NewBufferString("{\"text\":\"Buy milk\"}"),
	)
	req.Header.Set(echo.HeaderContentType, echo.MIMEApplicationJSON)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskService)
	service.On("CreateTask", "Buy milk").Return(contracts.Task{
		ID:   1,
		Text: "Buy milk",
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.CreateTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, rec.Code)
	service.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The REST Handler Tests"
```

### 2. Green: Return REST Resources

Create the first adapter production file:

```bash
touch workspace/internal/adapter/http/task_handler.go
```

Put this exact content in `workspace/internal/adapter/http/task_handler.go`:

```go
package httpadapter

import (
	"errors"
	"net/http"
	"strconv"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
)

type TaskHandler struct {
	service contracts.TaskService
}

func NewTaskHandler(service contracts.TaskService) *TaskHandler {
	return &TaskHandler{service: service}
}

func (h *TaskHandler) ListTasks(c echo.Context) error {
	result, err := h.service.ListTasks()
	if err != nil {
		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusOK, result)
}

func (h *TaskHandler) CreateTask(c echo.Context) error {
	var request contracts.CreateTaskRequest
	if err := c.Bind(&request); err != nil {
		return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
			Message: "invalid request body",
		})
	}

	task, err := h.service.CreateTask(request.Text)
	if err != nil {
		if errors.Is(err, contracts.ErrTaskTextBlank) {
			return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
				Message: err.Error(),
			})
		}

		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusCreated, task)
}

func (h *TaskHandler) GetTask(c echo.Context) error {
	taskID, err := strconv.ParseInt(c.Param("id"), 10, 64)
	if err != nil {
		return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
			Message: "task id must be an integer",
		})
	}

	task, err := h.service.GetTask(taskID)
	if err != nil {
		if errors.Is(err, contracts.ErrTaskNotFound) {
			return c.JSON(http.StatusNotFound, contracts.ErrorResponse{
				Message: err.Error(),
			})
		}

		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusOK, task)
}

func (h *TaskHandler) DeleteTask(c echo.Context) error {
	taskID, err := strconv.ParseInt(c.Param("id"), 10, 64)
	if err != nil {
		return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
			Message: "task id must be an integer",
		})
	}

	err = h.service.DeleteTask(taskID)
	if err != nil {
		if errors.Is(err, contracts.ErrTaskNotFound) {
			return c.JSON(http.StatusNotFound, contracts.ErrorResponse{
				Message: err.Error(),
			})
		}

		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.NoContent(http.StatusNoContent)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Return REST Resources"
```

### 3. Red: Add Fetch, Delete, And SQLite Store Tests

Replace `workspace/internal/adapter/http/task_handler_test.go` with:

```go
package httpadapter

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockTaskService struct {
	mock.Mock
}

func (m *MockTaskService) ListTasks() (contracts.TaskListResponse, error) {
	args := m.Called()
	return args.Get(0).(contracts.TaskListResponse), args.Error(1)
}

func (m *MockTaskService) CreateTask(taskText string) (contracts.Task, error) {
	args := m.Called(taskText)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskService) GetTask(taskID int64) (contracts.Task, error) {
	args := m.Called(taskID)
	return args.Get(0).(contracts.Task), args.Error(1)
}

func (m *MockTaskService) DeleteTask(taskID int64) error {
	args := m.Called(taskID)
	return args.Error(0)
}

func TestTaskHandlerListTasksReturnsTheCurrentTaskResources(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/tasks", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskService)
	service.On("ListTasks").Return(contracts.TaskListResponse{
		Tasks: []contracts.Task{
			{ID: 1, Text: "Learn how to invert binary trees"},
			{ID: 2, Text: "Buy milk"},
		},
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.ListTasks(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)
	service.AssertExpectations(t)
}

func TestTaskHandlerCreateTaskReturnsCreatedResource(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(
		http.MethodPost,
		"/api/tasks",
		bytes.NewBufferString("{\"text\":\"Buy milk\"}"),
	)
	req.Header.Set(echo.HeaderContentType, echo.MIMEApplicationJSON)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskService)
	service.On("CreateTask", "Buy milk").Return(contracts.Task{
		ID:   1,
		Text: "Buy milk",
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.CreateTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusCreated, rec.Code)
	service.AssertExpectations(t)
}

func TestTaskHandlerGetTaskReturnsTheRequestedResource(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/tasks/1", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)
	ctx.SetPath("/api/tasks/:id")
	ctx.SetParamNames("id")
	ctx.SetParamValues("1")

	service := new(MockTaskService)
	service.On("GetTask", int64(1)).Return(contracts.Task{
		ID:   1,
		Text: "Buy milk",
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.GetTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)
	service.AssertExpectations(t)
}

func TestTaskHandlerDeleteTaskReturnsNoContent(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodDelete, "/api/tasks/1", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)
	ctx.SetPath("/api/tasks/:id")
	ctx.SetParamNames("id")
	ctx.SetParamValues("1")

	service := new(MockTaskService)
	service.On("DeleteTask", int64(1)).Return(nil)

	handler := NewTaskHandler(service)
	err := handler.DeleteTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusNoContent, rec.Code)
	service.AssertExpectations(t)
}
```

Create the SQLite store test file:

```bash
touch workspace/internal/adapter/storage/sqlite_task_store_test.go
```

Put this exact content in `workspace/internal/adapter/storage/sqlite_task_store_test.go`:

```go
package storageadapter

import (
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSQLiteTaskStoreCreatesTasksWithIncreasingIds(t *testing.T) {
	store, err := NewSQLiteTaskStore(filepath.Join(t.TempDir(), "tasks.db"))
	require.NoError(t, err)
	t.Cleanup(func() { _ = store.Close() })

	first, err := store.CreateTask("Learn how to invert binary trees")
	require.NoError(t, err)

	second, err := store.CreateTask("Buy milk")
	require.NoError(t, err)

	assert.Equal(t, int64(1), first.ID)
	assert.Equal(t, int64(2), second.ID)
}

func TestSQLiteTaskStoreListsGetsAndDeletesTasks(t *testing.T) {
	store, err := NewSQLiteTaskStore(filepath.Join(t.TempDir(), "tasks.db"))
	require.NoError(t, err)
	t.Cleanup(func() { _ = store.Close() })

	created, err := store.CreateTask("Buy milk")
	require.NoError(t, err)

	listed, err := store.ListTasks()
	require.NoError(t, err)
	assert.Len(t, listed, 1)

	got, found, err := store.GetTask(created.ID)
	require.NoError(t, err)
	assert.True(t, found)
	assert.Equal(t, created, got)

	deleted, err := store.DeleteTask(created.ID)
	require.NoError(t, err)
	assert.True(t, deleted)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Add Fetch, Delete, And SQLite Store Tests"
```

### 4. Green: Add The SQLite Store And Server Wiring

Create the SQLite store production file:

```bash
touch workspace/internal/adapter/storage/sqlite_task_store.go
```

Put this exact content in `workspace/internal/adapter/storage/sqlite_task_store.go`:

```go
package storageadapter

import (
	"database/sql"
	"os"
	"path/filepath"

	"__MODULE_PATH__/internal/contracts"
	_ "modernc.org/sqlite"
)

type SQLiteTaskStore struct {
	db *sql.DB
}

func NewSQLiteTaskStore(path string) (*SQLiteTaskStore, error) {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return nil, err
	}

	db, err := sql.Open("sqlite", path)
	if err != nil {
		return nil, err
	}

	store := &SQLiteTaskStore{db: db}
	if err := store.ensureSchema(); err != nil {
		_ = db.Close()
		return nil, err
	}

	return store, nil
}

func (s *SQLiteTaskStore) Close() error {
	return s.db.Close()
}

func (s *SQLiteTaskStore) ensureSchema() error {
	_, err := s.db.Exec(`
		CREATE TABLE IF NOT EXISTS tasks (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			text TEXT NOT NULL
		)
	`)
	return err
}

func (s *SQLiteTaskStore) ListTasks() ([]contracts.Task, error) {
	rows, err := s.db.Query(`SELECT id, text FROM tasks ORDER BY id ASC`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []contracts.Task
	for rows.Next() {
		var task contracts.Task
		if err := rows.Scan(&task.ID, &task.Text); err != nil {
			return nil, err
		}
		tasks = append(tasks, task)
	}

	return tasks, rows.Err()
}

func (s *SQLiteTaskStore) CreateTask(taskText string) (contracts.Task, error) {
	result, err := s.db.Exec(`INSERT INTO tasks (text) VALUES (?)`, taskText)
	if err != nil {
		return contracts.Task{}, err
	}

	taskID, err := result.LastInsertId()
	if err != nil {
		return contracts.Task{}, err
	}

	return contracts.Task{
		ID:   taskID,
		Text: taskText,
	}, nil
}

func (s *SQLiteTaskStore) GetTask(taskID int64) (contracts.Task, bool, error) {
	var task contracts.Task
	err := s.db.QueryRow(`SELECT id, text FROM tasks WHERE id = ?`, taskID).Scan(&task.ID, &task.Text)
	if err == sql.ErrNoRows {
		return contracts.Task{}, false, nil
	}
	if err != nil {
		return contracts.Task{}, false, err
	}

	return task, true, nil
}

func (s *SQLiteTaskStore) DeleteTask(taskID int64) (bool, error) {
	result, err := s.db.Exec(`DELETE FROM tasks WHERE id = ?`, taskID)
	if err != nil {
		return false, err
	}

	rowsAffected, err := result.RowsAffected()
	if err != nil {
		return false, err
	}

	return rowsAffected > 0, nil
}
```

Create the server entry point:

```bash
touch workspace/cmd/server/main.go
```

Put this exact content in `workspace/cmd/server/main.go`:

```go
package main

import (
	"log"

	httpadapter "__MODULE_PATH__/internal/adapter/http"
	storageadapter "__MODULE_PATH__/internal/adapter/storage"
	"__MODULE_PATH__/internal/code"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func main() {
	e := echo.New()
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"http://localhost:25616"},
	}))

	store, err := storageadapter.NewSQLiteTaskStore("data/tasks.db")
	if err != nil {
		log.Fatal(err)
	}
	defer store.Close()

	service := code.NewTaskService(store)
	handler := httpadapter.NewTaskHandler(service)

	e.GET("/api/tasks", handler.ListTasks)
	e.POST("/api/tasks", handler.CreateTask)
	e.GET("/api/tasks/:id", handler.GetTask)
	e.DELETE("/api/tasks/:id", handler.DeleteTask)

	log.Fatal(e.Start(":25664"))
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Add The SQLite Store And Server Wiring"
```"#;

        return tutorial_file_markdown(
            "Adapter",
            &rewrite_touch_creation_stage_only(&body.replace("__MODULE_PATH__", &module_path)),
        );
    }

    let module_path = go_module_path(spec);
    let body = r#"### 1. Red: Add The List-Tasks Handler Test

Create the first adapter test file:

```bash
touch workspace/internal/adapter/http/task_handler_test.go
```

Put this exact content in `workspace/internal/adapter/http/task_handler_test.go`:

```go
package httpadapter

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockTaskListService struct {
	mock.Mock
}

func (m *MockTaskListService) ListTasks() (contracts.TaskListResult, error) {
	args := m.Called()
	return args.Get(0).(contracts.TaskListResult), args.Error(1)
}

func (m *MockTaskListService) AddTask(taskText string) (contracts.TaskListResult, error) {
	args := m.Called(taskText)
	return args.Get(0).(contracts.TaskListResult), args.Error(1)
}

func (m *MockTaskListService) RemoveTask(completedTaskText string) (contracts.TaskListResult, error) {
	args := m.Called(completedTaskText)
	return args.Get(0).(contracts.TaskListResult), args.Error(1)
}

func TestTaskHandlerGetTasksReturnsTheCurrentTaskList(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/tasks", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskListService)
	service.On("ListTasks").Return(contracts.TaskListResult{
		Tasks: []string{"Learn how to invert binary trees", "Buy milk"},
		Lines: []string{"Learn how to invert binary trees", "Buy milk"},
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.GetTasks(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)

	var body contracts.TaskListResult
	err = json.Unmarshal(rec.Body.Bytes(), &body)

	assert.NoError(t, err)
	assert.Equal(t, []string{"Learn how to invert binary trees", "Buy milk"}, body.Tasks)
	assert.Equal(t, []string{"Learn how to invert binary trees", "Buy milk"}, body.Lines)
	service.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The List-Tasks Handler Test"
```

### 2. Green: Return The Current Task List As JSON

Create the first adapter production file:

```bash
touch workspace/internal/adapter/http/task_handler.go
```

Put this exact content in `workspace/internal/adapter/http/task_handler.go`:

```go
package httpadapter

import (
	"net/http"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
)

type TaskHandler struct {
	service contracts.TaskListService
}

func NewTaskHandler(service contracts.TaskListService) *TaskHandler {
	return &TaskHandler{service: service}
}

func (h *TaskHandler) GetTasks(c echo.Context) error {
	result, err := h.service.ListTasks()
	if err != nil {
		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusOK, result)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Return The Current Task List As JSON"
```

### 3. Red: Add The Task-Mutation Handler Tests

Replace `workspace/internal/adapter/http/task_handler_test.go` with:

```go
package httpadapter

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockTaskListService struct {
	mock.Mock
}

func (m *MockTaskListService) ListTasks() (contracts.TaskListResult, error) {
	args := m.Called()
	return args.Get(0).(contracts.TaskListResult), args.Error(1)
}

func (m *MockTaskListService) AddTask(taskText string) (contracts.TaskListResult, error) {
	args := m.Called(taskText)
	return args.Get(0).(contracts.TaskListResult), args.Error(1)
}

func (m *MockTaskListService) RemoveTask(completedTaskText string) (contracts.TaskListResult, error) {
	args := m.Called(completedTaskText)
	return args.Get(0).(contracts.TaskListResult), args.Error(1)
}

func TestTaskHandlerGetTasksReturnsTheCurrentTaskList(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/api/tasks", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskListService)
	service.On("ListTasks").Return(contracts.TaskListResult{
		Tasks: []string{"Learn how to invert binary trees", "Buy milk"},
		Lines: []string{"Learn how to invert binary trees", "Buy milk"},
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.GetTasks(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)

	var body contracts.TaskListResult
	err = json.Unmarshal(rec.Body.Bytes(), &body)

	assert.NoError(t, err)
	assert.Equal(t, []string{"Learn how to invert binary trees", "Buy milk"}, body.Tasks)
	assert.Equal(t, []string{"Learn how to invert binary trees", "Buy milk"}, body.Lines)
	service.AssertExpectations(t)
}

func TestTaskHandlerAddTaskAppendsTheSubmittedTask(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(
		http.MethodPost,
		"/api/tasks",
		bytes.NewBufferString("{\"task\":\"Buy milk\"}"),
	)
	req.Header.Set(echo.HeaderContentType, echo.MIMEApplicationJSON)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskListService)
	service.On("AddTask", "Buy milk").Return(contracts.TaskListResult{
		Tasks: []string{"Learn how to invert binary trees", "Buy milk"},
		Lines: []string{"Learn how to invert binary trees", "Buy milk"},
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.AddTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)
	service.AssertExpectations(t)
}

func TestTaskHandlerAddTaskRejectsBlankInput(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(
		http.MethodPost,
		"/api/tasks",
		bytes.NewBufferString("{\"task\":\"   \"}"),
	)
	req.Header.Set(echo.HeaderContentType, echo.MIMEApplicationJSON)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskListService)
	handler := NewTaskHandler(service)
	err := handler.AddTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusBadRequest, rec.Code)
	service.AssertExpectations(t)
}

func TestTaskHandlerRemoveTaskRemovesTheChosenTask(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodDelete, "/api/tasks?task=Buy%20milk", nil)
	rec := httptest.NewRecorder()
	ctx := e.NewContext(req, rec)

	service := new(MockTaskListService)
	service.On("RemoveTask", "Buy milk").Return(contracts.TaskListResult{
		Tasks: []string{"Learn how to invert binary trees"},
		Lines: []string{"Learn how to invert binary trees"},
	}, nil)

	handler := NewTaskHandler(service)
	err := handler.RemoveTask(ctx)

	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, rec.Code)
	service.AssertExpectations(t)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Add The Task-Mutation Handler Tests"
```

### 4. Green: Handle Add And Remove Task Requests

Replace `workspace/internal/adapter/http/task_handler.go` with:

```go
package httpadapter

import (
	"net/http"
	"strings"

	"__MODULE_PATH__/internal/contracts"
	"github.com/labstack/echo/v4"
)

type TaskHandler struct {
	service contracts.TaskListService
}

func NewTaskHandler(service contracts.TaskListService) *TaskHandler {
	return &TaskHandler{service: service}
}

func (h *TaskHandler) GetTasks(c echo.Context) error {
	result, err := h.service.ListTasks()
	if err != nil {
		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusOK, result)
}

func (h *TaskHandler) AddTask(c echo.Context) error {
	var request contracts.AddTaskRequest
	if err := c.Bind(&request); err != nil {
		return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
			Message: "invalid request body",
		})
	}

	if strings.TrimSpace(request.Task) == "" {
		return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
			Message: "task must not be blank",
		})
	}

	result, err := h.service.AddTask(request.Task)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusOK, result)
}

func (h *TaskHandler) RemoveTask(c echo.Context) error {
	taskText := c.QueryParam("task")
	if strings.TrimSpace(taskText) == "" {
		return c.JSON(http.StatusBadRequest, contracts.ErrorResponse{
			Message: "task query parameter must not be blank",
		})
	}

	result, err := h.service.RemoveTask(taskText)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, contracts.ErrorResponse{
			Message: "internal server error",
		})
	}

	return c.JSON(http.StatusOK, result)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Handle Add And Remove Task Requests"
```

### 5. Red: Add The JSON Task Store Test

Create the storage test file:

```bash
touch workspace/internal/adapter/storage/json_task_store_test.go
```

Put this exact content in `workspace/internal/adapter/storage/json_task_store_test.go`:

```go
package storageadapter

import (
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestJSONTaskStoreLoadsEmptyJsonArrayWhenTheFileDoesNotExist(t *testing.T) {
	store := NewJSONTaskStore(filepath.Join(t.TempDir(), "tasks.json"))

	result, err := store.LoadTaskStorage()

	require.NoError(t, err)
	assert.JSONEq(t, "[]", result)
}

func TestJSONTaskStoreWritesAndReadsTaskStorage(t *testing.T) {
	store := NewJSONTaskStore(filepath.Join(t.TempDir(), "tasks.json"))

	require.NoError(t, store.SaveTaskStorage("[\"Buy milk\"]"))

	result, err := store.LoadTaskStorage()

	require.NoError(t, err)
	assert.JSONEq(t, "[\"Buy milk\"]", result)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "5. Red: Add The JSON Task Store Test"
```

### 6. Green: Read And Write The Local JSON Task Store

Create the storage production file:

```bash
touch workspace/internal/adapter/storage/json_task_store.go
```

Put this exact content in `workspace/internal/adapter/storage/json_task_store.go`:

```go
package storageadapter

import (
	"errors"
	"os"
	"path/filepath"
)

type JSONTaskStore struct {
	path string
}

func NewJSONTaskStore(path string) *JSONTaskStore {
	return &JSONTaskStore{path: path}
}

func (s *JSONTaskStore) LoadTaskStorage() (string, error) {
	storageBytes, err := os.ReadFile(s.path)
	if errors.Is(err, os.ErrNotExist) {
		return "[]", nil
	}
	if err != nil {
		return "", err
	}

	return string(storageBytes), nil
}

func (s *JSONTaskStore) SaveTaskStorage(storageText string) error {
	if err := os.MkdirAll(filepath.Dir(s.path), 0o755); err != nil {
		return err
	}

	return os.WriteFile(s.path, []byte(storageText), 0o644)
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "6. Green: Read And Write The Local JSON Task Store"
```

### 7. Green: Wire The Server Entry Point

Create the server entry point:

```bash
touch workspace/cmd/server/main.go
```

Put this exact content in `workspace/cmd/server/main.go`:

```go
package main

import (
	"log"

	httpadapter "__MODULE_PATH__/internal/adapter/http"
	storageadapter "__MODULE_PATH__/internal/adapter/storage"
	"__MODULE_PATH__/internal/code"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func main() {
	e := echo.New()
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"http://localhost:25616"},
	}))

	store := storageadapter.NewJSONTaskStore("data/tasks.json")
	service := code.NewTaskListService(store)
	handler := httpadapter.NewTaskHandler(service)

	e.GET("/api/tasks", handler.GetTasks)
	e.POST("/api/tasks", handler.AddTask)
	e.DELETE("/api/tasks", handler.RemoveTask)

	log.Fatal(e.Start(":25664"))
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "7. Green: Wire The Server Entry Point"
```"#;

    tutorial_file_markdown(
        "Adapter",
        &rewrite_touch_creation_stage_only(&body.replace("__MODULE_PATH__", &module_path)),
    )
}

fn render_flutter_todo_list_rest_json_contracts_content(_spec: &OutputRepoSpec) -> String {
    tutorial_file_markdown(
        "Contracts",
        &rewrite_stage_commit_checkpoints(&rewrite_touch_creation_stage_only(&format!(
            r#"Create the shared contract files:

```bash
touch workspace/lib/contracts/task.dart
touch workspace/lib/contracts/task_list_response.dart
touch workspace/lib/contracts/task_api.dart
```

Put this exact content in `workspace/lib/contracts/task.dart`:

```dart
class Task {{
  final String id;
  final String text;

  const Task({{required this.id, required this.text}});

  factory Task.fromJson(Map<String, dynamic> json) {{
    return Task(
      id: json['id'] as String,
      text: json['text'] as String,
    );
  }}
}}
```

Put this exact content in `workspace/lib/contracts/task_list_response.dart`:

```dart
import 'task.dart';

class TaskListResponse {{
  final List<Task> tasks;

  const TaskListResponse({{required this.tasks}});

  factory TaskListResponse.fromJson(Map<String, dynamic> json) {{
    return TaskListResponse(
      tasks: (json['tasks'] as List<dynamic>)
          .map((taskJson) => Task.fromJson(taskJson as Map<String, dynamic>))
          .toList(),
    );
  }}
}}
```

Put this exact content in `workspace/lib/contracts/task_api.dart`:

```dart
import 'task.dart';
import 'task_list_response.dart';

abstract class TaskApi {{
  Future<TaskListResponse> getTasks();
  Future<Task> createTask(String text);
  Future<void> deleteTask(String taskId);
}}
```

Do not add tests here. Keep this layer limited to interfaces and small shared types.

Then run:

```bash
just format
git add --all
git commit --message "Define todo-list Flutter contracts"
```"#,
        ))),
    )
}

fn render_flutter_todo_list_rest_json_code_content(spec: &OutputRepoSpec) -> String {
    let package_name = flutter_package_name(&spec.project_slug);
    let body = format!(
        r#"### 1. Red: Load The Current Task List

Create the first code test file:

```bash
touch workspace/test/code/task_list_controller_test.dart
```

Put this exact content in `workspace/test/code/task_list_controller_test.dart`:

```dart
import 'package:mocktail/mocktail.dart';
import 'package:{package_name}/code/task_list_controller.dart';
import 'package:{package_name}/contracts/task.dart';
import 'package:{package_name}/contracts/task_api.dart';
import 'package:{package_name}/contracts/task_list_response.dart';
import 'package:test/test.dart';

class MockTaskApi extends Mock implements TaskApi {{}}

void main() {{
  test('loads the current task list', () async {{
    final api = MockTaskApi();
    when(() => api.getTasks()).thenAnswer(
      (_) async => const TaskListResponse(
        tasks: [
          Task(id: 1, text: 'Learn how to invert binary trees'),
          Task(id: 2, text: 'Buy milk'),
        ],
      ),
    );

    final result = await loadTasks(api);

    expect(
      result.tasks.map((task) => task.text).toList(),
      ['Learn how to invert binary trees', 'Buy milk'],
    );
    expect(result.errorMessage, isNull);
  }});
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Load The Current Task List"
```

### 2. Green: Load The Current Task List

Create the first production file:

```bash
touch workspace/lib/code/task_list_controller.dart
```

Put this exact content in `workspace/lib/code/task_list_controller.dart`:

```dart
import '../contracts/task.dart';
import '../contracts/task_api.dart';
import '../contracts/task_list_response.dart';

class TaskListViewModel {{
  final List<Task> tasks;
  final String? errorMessage;

  const TaskListViewModel({{
    required this.tasks,
    this.errorMessage,
  }});

  factory TaskListViewModel.fromResponse(TaskListResponse response) {{
    return TaskListViewModel(
      tasks: response.tasks,
    );
  }}
}}

Future<TaskListViewModel> loadTasks(TaskApi api) async {{
  final response = await api.getTasks();
  return TaskListViewModel.fromResponse(response);
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Load The Current Task List"
```

### 3. Red: Trim Submitted Tasks Before Calling The API

Replace `workspace/test/code/task_list_controller_test.dart` with:

```dart
import 'package:mocktail/mocktail.dart';
import 'package:{package_name}/code/task_list_controller.dart';
import 'package:{package_name}/contracts/task.dart';
import 'package:{package_name}/contracts/task_api.dart';
import 'package:{package_name}/contracts/task_list_response.dart';
import 'package:test/test.dart';

class MockTaskApi extends Mock implements TaskApi {{}}

void main() {{
  test('loads the current task list', () async {{
    final api = MockTaskApi();
    when(() => api.getTasks()).thenAnswer(
      (_) async => const TaskListResponse(
        tasks: [
          Task(id: 1, text: 'Learn how to invert binary trees'),
          Task(id: 2, text: 'Buy milk'),
        ],
      ),
    );

    final result = await loadTasks(api);

    expect(
      result.tasks.map((task) => task.text).toList(),
      ['Learn how to invert binary trees', 'Buy milk'],
    );
    expect(result.errorMessage, isNull);
  }});

  test('trims submitted tasks before calling the REST API', () async {{
    final api = MockTaskApi();
    when(() => api.createTask('Buy milk')).thenAnswer(
      (_) async => const Task(id: 2, text: 'Buy milk'),
    );
    when(() => api.getTasks()).thenAnswer(
      (_) async => const TaskListResponse(
        tasks: [
          Task(id: 1, text: 'Learn how to invert binary trees'),
          Task(id: 2, text: 'Buy milk'),
        ],
      ),
    );

    final result = await addTask('  Buy milk  ', api);

    expect(
      result.tasks.map((task) => task.text).toList(),
      ['Learn how to invert binary trees', 'Buy milk'],
    );
    verify(() => api.createTask('Buy milk')).called(1);
  }});
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Trim Submitted Tasks Before Calling The API"
```

### 4. Green: Trim Submitted Tasks Before Calling The API

Replace `workspace/lib/code/task_list_controller.dart` with:

```dart
import '../contracts/task.dart';
import '../contracts/task_api.dart';
import '../contracts/task_list_response.dart';

class TaskListViewModel {{
  final List<Task> tasks;
  final String? errorMessage;

  const TaskListViewModel({{
    required this.tasks,
    this.errorMessage,
  }});

  factory TaskListViewModel.fromResponse(TaskListResponse response) {{
    return TaskListViewModel(
      tasks: response.tasks,
    );
  }}
}}

Future<TaskListViewModel> loadTasks(TaskApi api) async {{
  final response = await api.getTasks();
  return TaskListViewModel.fromResponse(response);
}}

Future<TaskListViewModel> addTask(String task, TaskApi api) async {{
  await api.createTask(task.trim());
  final response = await api.getTasks();
  return TaskListViewModel.fromResponse(response);
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Trim Submitted Tasks Before Calling The API"
```

### 5. Red: Reject Blank Tasks And Handle API Errors

Replace `workspace/test/code/task_list_controller_test.dart` with:

```dart
import 'package:mocktail/mocktail.dart';
import 'package:{package_name}/code/task_list_controller.dart';
import 'package:{package_name}/contracts/task.dart';
import 'package:{package_name}/contracts/task_api.dart';
import 'package:{package_name}/contracts/task_list_response.dart';
import 'package:test/test.dart';

class MockTaskApi extends Mock implements TaskApi {{}}

void main() {{
  test('loads the current task list', () async {{
    final api = MockTaskApi();
    when(() => api.getTasks()).thenAnswer(
      (_) async => const TaskListResponse(
        tasks: [
          Task(id: 1, text: 'Learn how to invert binary trees'),
          Task(id: 2, text: 'Buy milk'),
        ],
      ),
    );

    final result = await loadTasks(api);

    expect(
      result.tasks.map((task) => task.text).toList(),
      ['Learn how to invert binary trees', 'Buy milk'],
    );
    expect(result.errorMessage, isNull);
  }});

  test('trims submitted tasks before calling the REST API', () async {{
    final api = MockTaskApi();
    when(() => api.createTask('Buy milk')).thenAnswer(
      (_) async => const Task(id: 2, text: 'Buy milk'),
    );
    when(() => api.getTasks()).thenAnswer(
      (_) async => const TaskListResponse(
        tasks: [
          Task(id: 1, text: 'Learn how to invert binary trees'),
          Task(id: 2, text: 'Buy milk'),
        ],
      ),
    );

    final result = await addTask('  Buy milk  ', api);

    expect(
      result.tasks.map((task) => task.text).toList(),
      ['Learn how to invert binary trees', 'Buy milk'],
    );
    verify(() => api.createTask('Buy milk')).called(1);
  }});

  test('rejects blank submitted tasks without calling the API', () async {{
    final api = MockTaskApi();

    final result = await addTask('   ', api);

    expect(result.errorMessage, 'Task must not be blank.');
    verifyNever(() => api.createTask(any()));
  }});

  test('returns a friendly message when the task API is unavailable', () async {{
    final api = MockTaskApi();
    when(() => api.getTasks()).thenThrow(Exception('boom'));

    final result = await loadTasks(api);

    expect(result.errorMessage, 'Sorry, the task API is unavailable right now.');
  }});

  test('removes the chosen task through the REST API', () async {{
    final api = MockTaskApi();
    when(() => api.deleteTask(2)).thenAnswer((_) async {{}});
    when(() => api.getTasks()).thenAnswer(
      (_) async => const TaskListResponse(
        tasks: [
          Task(id: 1, text: 'Learn how to invert binary trees'),
        ],
      ),
    );

    final result = await removeTask(2, api);

    expect(
      result.tasks.map((task) => task.text).toList(),
      ['Learn how to invert binary trees'],
    );
    verify(() => api.deleteTask(2)).called(1);
  }});
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "5. Red: Reject Blank Tasks And Handle API Errors"
```

### 6. Green: Reject Blank Tasks And Handle API Errors

Replace `workspace/lib/code/task_list_controller.dart` with:

```dart
import '../contracts/task.dart';
import '../contracts/task_api.dart';
import '../contracts/task_list_response.dart';

class TaskListViewModel {{
  final List<Task> tasks;
  final String? errorMessage;

  const TaskListViewModel({{
    required this.tasks,
    this.errorMessage,
  }});

  factory TaskListViewModel.fromResponse(TaskListResponse response) {{
    return TaskListViewModel(
      tasks: response.tasks,
    );
  }}
}}

Future<TaskListViewModel> loadTasks(TaskApi api) async {{
  try {{
    final response = await api.getTasks();
    return TaskListViewModel.fromResponse(response);
  }} catch (_) {{
    return const TaskListViewModel(
      tasks: [],
      errorMessage: 'Sorry, the task API is unavailable right now.',
    );
  }}
}}

Future<TaskListViewModel> addTask(String task, TaskApi api) async {{
  final trimmed = task.trim();
  if (trimmed.isEmpty) {{
    return const TaskListViewModel(
      tasks: [],
      errorMessage: 'Task must not be blank.',
    );
  }}

  try {{
    await api.createTask(trimmed);
    final response = await api.getTasks();
    return TaskListViewModel.fromResponse(response);
  }} catch (_) {{
    return const TaskListViewModel(
      tasks: [],
      errorMessage: 'Sorry, the task API is unavailable right now.',
    );
  }}
}}

Future<TaskListViewModel> removeTask(int taskId, TaskApi api) async {{
  try {{
    await api.deleteTask(taskId);
    final response = await api.getTasks();
    return TaskListViewModel.fromResponse(response);
  }} catch (_) {{
    return const TaskListViewModel(
      tasks: [],
      errorMessage: 'Sorry, the task API is unavailable right now.',
    );
  }}
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "6. Green: Reject Blank Tasks And Handle API Errors"
```"#,
    );
    let body = body
        .replace("Task(id: 1,", "Task(id: '11111111-1111-1111-1111-111111111111',")
        .replace("Task(id: 2,", "Task(id: '22222222-2222-2222-2222-222222222222',")
        .replace("api.deleteTask(2)", "api.deleteTask('22222222-2222-2222-2222-222222222222')")
        .replace("removeTask(2, api)", "removeTask('22222222-2222-2222-2222-222222222222', api)")
        .replace(
            "Future<TaskListViewModel> removeTask(int taskId, TaskApi api) async {",
            "Future<TaskListViewModel> removeTask(String taskId, TaskApi api) async {",
        );

    tutorial_file_markdown(
        "Code",
        &rewrite_touch_creation_stage_only(&body),
    )
}

fn render_flutter_todo_list_rest_json_adapter_content(spec: &OutputRepoSpec) -> String {
    let package_name = flutter_package_name(&spec.project_slug);
    let body = format!(
        r#"### 1. Red: Add The HTTP Task API Test

Create the HTTP adapter test file:

```bash
touch workspace/test/adapter/http_task_api_test.dart
```

Put this exact content in `workspace/test/adapter/http_task_api_test.dart`:

```dart
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:{package_name}/adapter/http_task_api.dart';
import 'package:{package_name}/contracts/task.dart';
import 'package:{package_name}/contracts/task_list_response.dart';
import 'package:test/test.dart';

void main() {{
  test('loads the current tasks from the canonical endpoint', () async {{
    late Uri requestedUri;
    final client = MockClient((request) async {{
      requestedUri = request.url;
      return http.Response(
        '{{"tasks":[{{"id":1,"text":"Learn how to invert binary trees"}},{{"id":2,"text":"Buy milk"}}]}}',
        200,
        headers: {{'content-type': 'application/json'}},
      );
    }});

    final api = HttpTaskApi(
      baseUrl: 'http://localhost:{FOR_ALL_API_PORT}',
      client: client,
    );
    final result = await api.getTasks();

    expect(requestedUri.toString(), 'http://localhost:{FOR_ALL_API_PORT}/api/tasks');
    expect(
      result.tasks.map((task) => task.text).toList(),
      ['Learn how to invert binary trees', 'Buy milk'],
    );
  }});

  test('posts a new task resource to the canonical endpoint', () async {{
    late String requestBody;
    final client = MockClient((request) async {{
      requestBody = request.body;
      return http.Response(
        '{{"id":2,"text":"Buy milk"}}',
        201,
        headers: {{'content-type': 'application/json'}},
      );
    }});

    final api = HttpTaskApi(
      baseUrl: 'http://localhost:{FOR_ALL_API_PORT}',
      client: client,
    );
    final createdTask = await api.createTask('Buy milk');

    expect(requestBody, '{{"text":"Buy milk"}}');
    expect(createdTask, isA<Task>());
    expect(createdTask.id, 2);
    expect(createdTask.text, 'Buy milk');
  }});

  test('deletes a task through the canonical endpoint', () async {{
    late Uri requestedUri;
    final client = MockClient((request) async {{
      requestedUri = request.url;
      return http.Response('', 204);
    }});

    final api = HttpTaskApi(
      baseUrl: 'http://localhost:{FOR_ALL_API_PORT}',
      client: client,
    );
    await api.deleteTask(7);

    expect(requestedUri.toString(), 'http://localhost:{FOR_ALL_API_PORT}/api/tasks/7');
  }});
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The HTTP Task API Test"
```

### 2. Green: Call The Canonical Task API Endpoints

Create the HTTP adapter production file:

```bash
touch workspace/lib/adapter/http_task_api.dart
```

Put this exact content in `workspace/lib/adapter/http_task_api.dart`:

```dart
import 'dart:convert';

import 'package:http/http.dart' as http;

import '../contracts/task_api.dart';
import '../contracts/task.dart';
import '../contracts/task_list_response.dart';

class HttpTaskApi implements TaskApi {{
  final String baseUrl;
  final http.Client client;

  HttpTaskApi({{required this.baseUrl, http.Client? client}})
    : client = client ?? http.Client();

  @override
  Future<TaskListResponse> getTasks() async {{
    final response = await client.get(Uri.parse('$baseUrl/api/tasks'));
    if (response.statusCode != 200) {{
      throw Exception('failed to load tasks');
    }}
    return TaskListResponse.fromJson(jsonDecode(response.body) as Map<String, dynamic>);
  }}

  @override
  Future<Task> createTask(String text) async {{
    final response = await client.post(
      Uri.parse('$baseUrl/api/tasks'),
      headers: {{'Content-Type': 'application/json'}},
      body: jsonEncode({{'text': text}}),
    );
    if (response.statusCode != 201) {{
      throw Exception('failed to create task');
    }}
    return Task.fromJson(jsonDecode(response.body) as Map<String, dynamic>);
  }}

  @override
  Future<void> deleteTask(int taskId) async {{
    final response = await client.delete(Uri.parse('$baseUrl/api/tasks/$taskId'));
    if (response.statusCode != 204) {{
      throw Exception('failed to delete task');
    }}
  }}
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Call The Canonical Task API Endpoints"
```

### 3. Red: Add The Todo List Page Widget Test

Create the widget test file:

```bash
touch workspace/test/adapter/todo_list_page_test.dart
```

Put this exact content in `workspace/test/adapter/todo_list_page_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:{package_name}/adapter/todo_list_page.dart';
import 'package:{package_name}/contracts/task.dart';
import 'package:{package_name}/contracts/task_api.dart';
import 'package:{package_name}/contracts/task_list_response.dart';

class MockTaskApi extends Mock implements TaskApi {{}}

void main() {{
  testWidgets('loads, adds, and removes tasks', (tester) async {{
    final api = MockTaskApi();
    var loadCount = 0;
    when(() => api.getTasks()).thenAnswer((_) async {{
      loadCount += 1;
      switch (loadCount) {{
        case 1:
          return const TaskListResponse(
            tasks: [Task(id: 1, text: 'Learn how to invert binary trees')],
          );
        case 2:
          return const TaskListResponse(
            tasks: [
              Task(id: 1, text: 'Learn how to invert binary trees'),
              Task(id: 2, text: 'Buy milk'),
            ],
          );
        default:
          return const TaskListResponse(
            tasks: [Task(id: 1, text: 'Learn how to invert binary trees')],
          );
      }}
    }});
    when(() => api.createTask('Buy milk')).thenAnswer(
      (_) async => const Task(id: 2, text: 'Buy milk'),
    );
    when(() => api.deleteTask(2)).thenAnswer((_) async {{}});

    await tester.pumpWidget(
      MaterialApp(
        home: TodoListPage(api: api),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Learn how to invert binary trees'), findsOneWidget);

    await tester.enterText(find.byType(TextField), 'Buy milk');
    await tester.tap(find.text('Add task'));
    await tester.pumpAndSettle();

    expect(find.text('Buy milk'), findsOneWidget);

    await tester.tap(find.byKey(const Key('remove-2')));
    await tester.pumpAndSettle();

    expect(find.text('Buy milk'), findsNothing);
  }});
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Add The Todo List Page Widget Test"
```

### 4. Green: Build The Todo List Page

Create the page production file:

```bash
touch workspace/lib/adapter/todo_list_page.dart
```

Put this exact content in `workspace/lib/adapter/todo_list_page.dart`:

```dart
import 'package:flutter/material.dart';

import '../code/task_list_controller.dart';
import '../contracts/task.dart';
import '../contracts/task_api.dart';

class TodoListPage extends StatefulWidget {{
  final TaskApi api;

  const TodoListPage({{super.key, required this.api}});

  @override
  State<TodoListPage> createState() => _TodoListPageState();
}}

class _TodoListPageState extends State<TodoListPage> {{
  final TextEditingController _taskController = TextEditingController();
  TaskListViewModel _viewModel = const TaskListViewModel(tasks: []);

  @override
  void initState() {{
    super.initState();
    _refreshTasks();
  }}

  Future<void> _refreshTasks() async {{
    final nextViewModel = await loadTasks(widget.api);
    if (!mounted) {{
      return;
    }}
    setState(() {{
      _viewModel = nextViewModel;
    }});
  }}

  Future<void> _addTask() async {{
    final nextViewModel = await addTask(_taskController.text, widget.api);
    _taskController.clear();
    if (!mounted) {{
      return;
    }}
    setState(() {{
      _viewModel = nextViewModel;
    }});
  }}

  Future<void> _removeTask(Task task) async {{
    final nextViewModel = await removeTask(task.id, widget.api);
    if (!mounted) {{
      return;
    }}
    setState(() {{
      _viewModel = nextViewModel;
    }});
  }}

  @override
  Widget build(BuildContext context) {{
    return Scaffold(
      appBar: AppBar(title: const Text('Todo List')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextField(
              controller: _taskController,
              decoration: const InputDecoration(labelText: 'Task'),
            ),
            const SizedBox(height: 12),
            ElevatedButton(
              onPressed: _addTask,
              child: const Text('Add task'),
            ),
            if (_viewModel.errorMessage != null) ...[
              const SizedBox(height: 12),
              Text(_viewModel.errorMessage!),
            ],
            const SizedBox(height: 12),
            Expanded(
              child: ListView(
                children: _viewModel.tasks
                    .map(
                      (task) => ListTile(
                        title: Text(task.text),
                        trailing: IconButton(
                          key: Key('remove-${{task.id}}'),
                          onPressed: () => _removeTask(task),
                          icon: const Icon(Icons.delete_outline),
                        ),
                      ),
                    )
                    .toList(),
              ),
            ),
          ],
        ),
      ),
    );
  }}
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Build The Todo List Page"
```

### 5. Green: Wire The Real Application

Replace `workspace/lib/main.dart` with:

```dart
import 'package:flutter/material.dart';

import 'adapter/http_task_api.dart';
import 'adapter/todo_list_page.dart';

const apiBaseUrl = String.fromEnvironment(
  'API_BASE_URL',
  defaultValue: 'http://localhost:{FOR_ALL_API_PORT}',
);

void main() {{
  runApp(const TodoListApp());
}}

class TodoListApp extends StatelessWidget {{
  const TodoListApp({{super.key}});

  @override
  Widget build(BuildContext context) {{
    return MaterialApp(
      title: 'Todo List',
      home: TodoListPage(
        api: HttpTaskApi(baseUrl: apiBaseUrl),
      ),
    );
  }}
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "5. Green: Wire The Real Application"
```"#,
    );
    let body = body
        .replace(r#"{"id":1,"text":"Learn how to invert binary trees"}"#, r#"{"id":"11111111-1111-1111-1111-111111111111","text":"Learn how to invert binary trees"}"#)
        .replace(r#"{"id":2,"text":"Buy milk"}"#, r#"{"id":"22222222-2222-2222-2222-222222222222","text":"Buy milk"}"#)
        .replace("expect(createdTask.id, 2);", "expect(createdTask.id, '22222222-2222-2222-2222-222222222222');")
        .replace("await api.deleteTask(7);", "await api.deleteTask('77777777-7777-7777-7777-777777777777');")
        .replace(
            &format!(
                "expect(requestedUri.toString(), 'http://localhost:{FOR_ALL_API_PORT}/api/tasks/7');"
            ),
            &format!(
                "expect(requestedUri.toString(), 'http://localhost:{FOR_ALL_API_PORT}/api/tasks/77777777-7777-7777-7777-777777777777');"
            ),
        )
        .replace(
            "Future<void> deleteTask(int taskId) async {",
            "Future<void> deleteTask(String taskId) async {",
        )
        .replace("Task(id: 1,", "Task(id: '11111111-1111-1111-1111-111111111111',")
        .replace("Task(id: 2,", "Task(id: '22222222-2222-2222-2222-222222222222',")
        .replace("api.deleteTask(2)", "api.deleteTask('22222222-2222-2222-2222-222222222222')")
        .replace("Key('remove-2')", "Key('remove-22222222-2222-2222-2222-222222222222')");

    tutorial_file_markdown(
        "Adapter",
        &rewrite_touch_creation_stage_only(&body),
    )
}

fn render_flutter_saying_hello_contracts_content(_spec: &OutputRepoSpec) -> String {
    if is_flutter_local_saying_hello_output_repo(_spec) {
        return tutorial_file_markdown(
            "Contracts",
            &rewrite_stage_commit_checkpoints(&rewrite_touch_creation_stage_only(
                r#"Create the shared contract file:

```bash
touch workspace/lib/contracts/greeting_service.dart
```

Put this exact content in `workspace/lib/contracts/greeting_service.dart`:

```dart
abstract class GreetingService {
  String greet(String name);
}
```

Do not add tests here. Keep this layer limited to interfaces and small shared types.

Then run:

```bash
git add --all
git commit --message "Define greeting service contract"
```"#,
            )),
        );
    }

    tutorial_file_markdown(
        "Contracts",
        &rewrite_stage_commit_checkpoints(&rewrite_touch_creation_stage_only(
            r#"Create the shared contract files:

```bash
touch workspace/lib/contracts/greeting_response.dart
touch workspace/lib/contracts/greeting_api.dart
```

Put this exact content in `workspace/lib/contracts/greeting_response.dart`:

```dart
class GreetingResponse {
  final String message;

  const GreetingResponse({required this.message});

  factory GreetingResponse.fromJson(Map<String, dynamic> json) {
    return GreetingResponse(message: json['message'] as String);
  }
}
```

Put this exact content in `workspace/lib/contracts/greeting_api.dart`:

```dart
import 'greeting_response.dart';

abstract class GreetingApi {
  Future<GreetingResponse> getGreeting(String name);
}
```

Do not add tests here. Keep this layer limited to interfaces and small shared types.

Then run:

```bash
git add --all
git commit --message "Define greeting contracts"
```"#,
        )),
    )
}

fn render_flutter_saying_hello_code_content(spec: &OutputRepoSpec) -> String {
    if is_flutter_local_saying_hello_output_repo(spec) {
        return tutorial_file_markdown(
            "Code",
            &rewrite_touch_creation_stage_only(
                r#"### 1. Red: Add The First Failing Code Test

Create the first code test file:

```bash
touch workspace/test/code/default_greeting_service_test.dart
```

Put this exact content in `workspace/test/code/default_greeting_service_test.dart`:

```dart
import 'package:saying_hello/code/default_greeting_service.dart';
import 'package:test/test.dart';

void main() {
  test('returns the personalized greeting for a non-empty name', () {
    final service = DefaultGreetingService();

    final result = service.greet('Ada');

    expect(result, 'Hello, Ada!');
  });
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The First Failing Code Test"
```

### 2. Green: Return The Personalized Greeting

Create the first production file:

```bash
touch workspace/lib/code/default_greeting_service.dart
```

Put this exact content in `workspace/lib/code/default_greeting_service.dart`:

```dart
import '../contracts/greeting_service.dart';

class DefaultGreetingService implements GreetingService {
  @override
  String greet(String name) {
    return 'Hello, $name!';
  }
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Return The Personalized Greeting"
```

### 3. Red: Trim The Name Before Greeting

Replace `workspace/test/code/default_greeting_service_test.dart` with:

```dart
import 'package:saying_hello/code/default_greeting_service.dart';
import 'package:test/test.dart';

void main() {
  test('returns the personalized greeting for a non-empty name', () {
    final service = DefaultGreetingService();

    final result = service.greet('Ada');

    expect(result, 'Hello, Ada!');
  });

  test('trims whitespace before greeting', () {
    final service = DefaultGreetingService();

    final result = service.greet('  Ada  ');

    expect(result, 'Hello, Ada!');
  });
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Trim The Name Before Greeting"
```

### 4. Green: Trim The Name Before Greeting

Replace `workspace/lib/code/default_greeting_service.dart` with:

```dart
import '../contracts/greeting_service.dart';

class DefaultGreetingService implements GreetingService {
  @override
  String greet(String name) {
    final trimmedName = name.trim();
    return 'Hello, $trimmedName!';
  }
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Trim The Name Before Greeting"
```

### 5. Red: Return The Generic Greeting For Empty Input

Replace `workspace/test/code/default_greeting_service_test.dart` with:

```dart
import 'package:saying_hello/code/default_greeting_service.dart';
import 'package:test/test.dart';

void main() {
  test('returns the personalized greeting for a non-empty name', () {
    final service = DefaultGreetingService();

    final result = service.greet('Ada');

    expect(result, 'Hello, Ada!');
  });

  test('trims whitespace before greeting', () {
    final service = DefaultGreetingService();

    final result = service.greet('  Ada  ');

    expect(result, 'Hello, Ada!');
  });

  test('returns the generic greeting for empty input', () {
    final service = DefaultGreetingService();

    final result = service.greet('');

    expect(result, 'Hello!');
  });

  test('returns the generic greeting for whitespace-only input', () {
    final service = DefaultGreetingService();

    final result = service.greet('   ');

    expect(result, 'Hello!');
  });
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "5. Red: Return The Generic Greeting For Empty Input"
```

### 6. Green: Return The Generic Greeting For Empty Input

Replace `workspace/lib/code/default_greeting_service.dart` with:

```dart
import '../contracts/greeting_service.dart';

class DefaultGreetingService implements GreetingService {
  @override
  String greet(String name) {
    final trimmedName = name.trim();
    if (trimmedName.isEmpty) {
      return 'Hello!';
    }

    return 'Hello, $trimmedName!';
  }
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "6. Green: Return The Generic Greeting For Empty Input"
```"#,
            ),
        );
    }

    tutorial_file_markdown(
        "Code",
        &rewrite_touch_creation_stage_only(
            r#"### 1. Red: Add The First Failing Code Test

Create the first code test file:

```bash
touch workspace/test/code/load_greeting_test.dart
```

Put this exact content in `workspace/test/code/load_greeting_test.dart`:

```dart
import 'package:mocktail/mocktail.dart';
import 'package:saying_hello/code/load_greeting.dart';
import 'package:saying_hello/contracts/greeting_api.dart';
import 'package:saying_hello/contracts/greeting_response.dart';
import 'package:test/test.dart';

class MockGreetingApi extends Mock implements GreetingApi {}

void main() {
  test('returns the personalized greeting for a non-empty name', () async {
    final api = MockGreetingApi();
    when(
      () => api.getGreeting('Ada'),
    ).thenAnswer((_) async => const GreetingResponse(message: 'Hello, Ada!'));

    final result = await loadGreeting('Ada', api);

    expect(result.submittedName, 'Ada');
    expect(result.message, 'Hello, Ada!');
  });
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The First Failing Code Test"
```

### 2. Green: Return The Personalized Greeting

Create the first production file:

```bash
touch workspace/lib/code/load_greeting.dart
```

Put this exact content in `workspace/lib/code/load_greeting.dart`:

```dart
import '../contracts/greeting_api.dart';

class GreetingViewModel {
  final String submittedName;
  final String message;

  const GreetingViewModel({
    required this.submittedName,
    required this.message,
  });
}

Future<GreetingViewModel> loadGreeting(String name, GreetingApi api) async {
  final response = await api.getGreeting(name);

  return GreetingViewModel(
    submittedName: name,
    message: response.message,
  );
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Return The Personalized Greeting"
```

### 3. Red: Trim The Name Before Calling The API

Replace `workspace/test/code/load_greeting_test.dart` with:

```dart
import 'package:mocktail/mocktail.dart';
import 'package:saying_hello/code/load_greeting.dart';
import 'package:saying_hello/contracts/greeting_api.dart';
import 'package:saying_hello/contracts/greeting_response.dart';
import 'package:test/test.dart';

class MockGreetingApi extends Mock implements GreetingApi {}

void main() {
  test('returns the personalized greeting for a non-empty name', () async {
    final api = MockGreetingApi();
    when(
      () => api.getGreeting('Ada'),
    ).thenAnswer((_) async => const GreetingResponse(message: 'Hello, Ada!'));

    final result = await loadGreeting('Ada', api);

    expect(result.submittedName, 'Ada');
    expect(result.message, 'Hello, Ada!');
  });

  test('trims the name before calling the API', () async {
    final api = MockGreetingApi();
    when(
      () => api.getGreeting('Ada'),
    ).thenAnswer((_) async => const GreetingResponse(message: 'Hello, Ada!'));

    final result = await loadGreeting('  Ada  ', api);

    expect(result.submittedName, 'Ada');
    expect(result.message, 'Hello, Ada!');
  });
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Trim The Name Before Calling The API"
```

### 4. Green: Trim The Name Before Calling The API

Replace `workspace/lib/code/load_greeting.dart` with:

```dart
import '../contracts/greeting_api.dart';

class GreetingViewModel {
  final String submittedName;
  final String message;

  const GreetingViewModel({
    required this.submittedName,
    required this.message,
  });
}

Future<GreetingViewModel> loadGreeting(String name, GreetingApi api) async {
  final submittedName = name.trim();
  final response = await api.getGreeting(submittedName);

  return GreetingViewModel(
    submittedName: submittedName,
    message: response.message,
  );
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Trim The Name Before Calling The API"
```

### 5. Red: Return A Friendly Message When The API Is Unavailable

Replace `workspace/test/code/load_greeting_test.dart` with:

```dart
import 'package:mocktail/mocktail.dart';
import 'package:saying_hello/code/load_greeting.dart';
import 'package:saying_hello/contracts/greeting_api.dart';
import 'package:saying_hello/contracts/greeting_response.dart';
import 'package:test/test.dart';

class MockGreetingApi extends Mock implements GreetingApi {}

void main() {
  test('returns the personalized greeting for a non-empty name', () async {
    final api = MockGreetingApi();
    when(
      () => api.getGreeting('Ada'),
    ).thenAnswer((_) async => const GreetingResponse(message: 'Hello, Ada!'));

    final result = await loadGreeting('Ada', api);

    expect(result.submittedName, 'Ada');
    expect(result.message, 'Hello, Ada!');
  });

  test('trims the name before calling the API', () async {
    final api = MockGreetingApi();
    when(
      () => api.getGreeting('Ada'),
    ).thenAnswer((_) async => const GreetingResponse(message: 'Hello, Ada!'));

    final result = await loadGreeting('  Ada  ', api);

    expect(result.submittedName, 'Ada');
    expect(result.message, 'Hello, Ada!');
  });

  test('returns a friendly message when the API is unavailable', () async {
    final api = MockGreetingApi();
    when(() => api.getGreeting('Ada')).thenThrow(Exception('network error'));

    final result = await loadGreeting('Ada', api);

    expect(result.submittedName, 'Ada');
    expect(result.message, 'Sorry, the greeting API is unavailable right now.');
  });
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "5. Red: Return A Friendly Message When The API Is Unavailable"
```

### 6. Green: Return A Friendly Message When The API Is Unavailable

Replace `workspace/lib/code/load_greeting.dart` with:

```dart
import '../contracts/greeting_api.dart';

class GreetingViewModel {
  final String submittedName;
  final String message;

  const GreetingViewModel({
    required this.submittedName,
    required this.message,
  });
}

Future<GreetingViewModel> loadGreeting(String name, GreetingApi api) async {
  final submittedName = name.trim();

  try {
    final response = await api.getGreeting(submittedName);

    return GreetingViewModel(
      submittedName: submittedName,
      message: response.message,
    );
  } catch (_) {
    return GreetingViewModel(
      submittedName: submittedName,
      message: 'Sorry, the greeting API is unavailable right now.',
    );
  }
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "6. Green: Return A Friendly Message When The API Is Unavailable"
```"#,
        ),
    )
}

fn render_flutter_saying_hello_adapter_content(spec: &OutputRepoSpec) -> String {
    if is_flutter_local_saying_hello_output_repo(spec) {
        return tutorial_file_markdown(
            "Adapter",
            &rewrite_touch_creation_stage_only(
                r#"### 1. Red: Add The Greeting Page Widget Test

Create the widget test file:

```bash
touch workspace/test/adapter/greeting_page_test.dart
```

Put this exact content in `workspace/test/adapter/greeting_page_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:saying_hello/adapter/greeting_page.dart';
import 'package:saying_hello/contracts/greeting_service.dart';

class MockGreetingService extends Mock implements GreetingService {}

void main() {
  testWidgets('renders the greeting returned by the service', (tester) async {
    final service = MockGreetingService();
    when(() => service.greet('Ada')).thenReturn('Hello, Ada!');

    await tester.pumpWidget(
      MaterialApp(home: GreetingPage(service: service)),
    );

    await tester.enterText(find.byKey(const ValueKey('name-input')), 'Ada');
    await tester.tap(find.byKey(const ValueKey('submit-button')));
    await tester.pumpAndSettle();

    expect(find.text('Hello, Ada!'), findsOneWidget);
  });
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The Greeting Page Widget Test"
```

### 2. Green: Build The Greeting Page

Create the greeting page file:

```bash
touch workspace/lib/adapter/greeting_page.dart
```

Put this exact content in `workspace/lib/adapter/greeting_page.dart`:

```dart
import 'package:flutter/material.dart';

import '../contracts/greeting_service.dart';

class GreetingPage extends StatefulWidget {
  final GreetingService service;

  const GreetingPage({super.key, required this.service});

  @override
  State<GreetingPage> createState() => _GreetingPageState();
}

class _GreetingPageState extends State<GreetingPage> {
  final _controller = TextEditingController();
  String _message = '';

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _submit() {
    final message = widget.service.greet(_controller.text);
    setState(() {
      _message = message;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Saying Hello')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextField(
              key: const ValueKey('name-input'),
              controller: _controller,
              decoration: const InputDecoration(labelText: 'Name'),
            ),
            const SizedBox(height: 12),
            ElevatedButton(
              key: const ValueKey('submit-button'),
              onPressed: _submit,
              child: const Text('Say hello'),
            ),
            const SizedBox(height: 12),
            Text(_message, key: const ValueKey('greeting-output')),
          ],
        ),
      ),
    );
  }
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Build The Greeting Page"
```

### 3. Red: Add The Integration Test

Create the integration test file:

```bash
touch workspace/integration_test/app_test.dart
```

Put this exact content in `workspace/integration_test/app_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:saying_hello/adapter/greeting_page.dart';
import 'package:saying_hello/code/default_greeting_service.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('submits the form and shows the greeting', (tester) async {
    await tester.pumpWidget(
      MaterialApp(home: GreetingPage(service: DefaultGreetingService())),
    );
    await tester.enterText(find.byKey(const ValueKey('name-input')), 'Ada');
    await tester.tap(find.byKey(const ValueKey('submit-button')));
    await tester.pumpAndSettle();

    expect(find.text('Hello, Ada!'), findsOneWidget);
  });
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Add The Integration Test"
```

### 4. Green: Wire The Real Application

Replace `workspace/lib/main.dart` with:

```dart
import 'package:flutter/material.dart';

import 'adapter/greeting_page.dart';
import 'code/default_greeting_service.dart';

void main() {
  runApp(const SayingHelloApp());
}

class SayingHelloApp extends StatelessWidget {
  const SayingHelloApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Saying Hello',
      home: GreetingPage(service: DefaultGreetingService()),
    );
  }
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Wire The Real Application"
```"#,
            ),
        );
    }

    let mut body = format!(
                r#"### 1. Red: Add The HTTP Adapter Test

Create the first adapter test file:

```bash
touch workspace/test/adapter/http_greeting_api_test.dart
```

Put this exact content in `workspace/test/adapter/http_greeting_api_test.dart`:

```dart
import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:saying_hello/adapter/http_greeting_api.dart';
import 'package:test/test.dart';

void main() {{
  test('requests the canonical greeting endpoint', () async {{
    final client = MockClient((request) async {{
      expect(
        request.url.toString(),
        'http://localhost:{FOR_ALL_API_PORT}/api/greeting?name=Ada',
      );

      return http.Response(jsonEncode({{'message': 'Hello, Ada!'}}), 200);
    }});

    final api = HttpGreetingApi(
      baseUrl: 'http://localhost:{FOR_ALL_API_PORT}',
      client: client,
    );
    final result = await api.getGreeting('Ada');

    expect(result.message, 'Hello, Ada!');
  }});
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The HTTP Adapter Test"
```

### 2. Green: Request The Canonical Greeting Endpoint

Create the first adapter production file:

```bash
touch workspace/lib/adapter/http_greeting_api.dart
```

Put this exact content in `workspace/lib/adapter/http_greeting_api.dart`:

```dart
import 'dart:convert';

import 'package:http/http.dart' as http;

import '../contracts/greeting_api.dart';
import '../contracts/greeting_response.dart';

class HttpGreetingApi implements GreetingApi {{
  final String baseUrl;
  final http.Client client;

  HttpGreetingApi({{
    required this.baseUrl,
    http.Client? client,
  }}) : client = client ?? http.Client();

  @override
  Future<GreetingResponse> getGreeting(String name) async {{
    final trimmedName = name.trim();
    final uri =
        trimmedName.isEmpty
            ? Uri.parse('$baseUrl/api/greeting')
            : Uri.parse('$baseUrl/api/greeting?name=${{Uri.encodeQueryComponent(trimmedName)}}');

    final response = await client.get(uri);
    final json = jsonDecode(response.body) as Map<String, dynamic>;
    return GreetingResponse.fromJson(json);
  }}
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Request The Canonical Greeting Endpoint"
```

### 3. Red: Add The Greeting Page Widget Test

Create the widget test file:

```bash
touch workspace/test/adapter/greeting_page_test.dart
```

Put this exact content in `workspace/test/adapter/greeting_page_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:saying_hello/adapter/greeting_page.dart';
import 'package:saying_hello/contracts/greeting_api.dart';
import 'package:saying_hello/contracts/greeting_response.dart';

class MockGreetingApi extends Mock implements GreetingApi {{}}

void main() {{
  testWidgets('renders the greeting returned by the API', (tester) async {{
    final api = MockGreetingApi();
    when(
      () => api.getGreeting('Ada'),
    ).thenAnswer((_) async => const GreetingResponse(message: 'Hello, Ada!'));

    await tester.pumpWidget(
      MaterialApp(home: GreetingPage(api: api)),
    );

    await tester.enterText(find.byKey(const ValueKey('name-input')), 'Ada');
    await tester.tap(find.byKey(const ValueKey('submit-button')));
    await tester.pumpAndSettle();

    expect(find.text('Hello, Ada!'), findsOneWidget);
  }});
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Add The Greeting Page Widget Test"
```

### 4. Green: Build The Greeting Page

Create the greeting page file:

```bash
touch workspace/lib/adapter/greeting_page.dart
```

Put this exact content in `workspace/lib/adapter/greeting_page.dart`:

```dart
import 'package:flutter/material.dart';

import '../code/load_greeting.dart';
import '../contracts/greeting_api.dart';

class GreetingPage extends StatefulWidget {{
  final GreetingApi api;

  const GreetingPage({{super.key, required this.api}});

  @override
  State<GreetingPage> createState() => _GreetingPageState();
}}

class _GreetingPageState extends State<GreetingPage> {{
  final _controller = TextEditingController();
  String _message = '';

  @override
  void dispose() {{
    _controller.dispose();
    super.dispose();
  }}

  Future<void> _submit() async {{
    final result = await loadGreeting(_controller.text, widget.api);
    setState(() {{
      _message = result.message;
    }});
  }}

  @override
  Widget build(BuildContext context) {{
    return Scaffold(
      appBar: AppBar(title: const Text('Saying Hello')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextField(
              key: const ValueKey('name-input'),
              controller: _controller,
              decoration: const InputDecoration(labelText: 'Name'),
            ),
            const SizedBox(height: 12),
            ElevatedButton(
              key: const ValueKey('submit-button'),
              onPressed: _submit,
              child: const Text('Say hello'),
            ),
            const SizedBox(height: 12),
            Text(
              _message,
              key: const ValueKey('greeting-output'),
            ),
          ],
        ),
      ),
    );
  }}
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Build The Greeting Page"
```

### 5. Red: Add The Integration Test

Create the integration test file:

```bash
touch workspace/integration_test/app_test.dart
```

Put this exact content in `workspace/integration_test/app_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:saying_hello/adapter/greeting_page.dart';
import 'package:saying_hello/contracts/greeting_api.dart';
import 'package:saying_hello/contracts/greeting_response.dart';

class MockGreetingApi extends Mock implements GreetingApi {{}}

void main() {{
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('submits the form and shows the greeting', (tester) async {{
    final api = MockGreetingApi();
    when(
      () => api.getGreeting('Ada'),
    ).thenAnswer((_) async => const GreetingResponse(message: 'Hello, Ada!'));

    await tester.pumpWidget(MaterialApp(home: GreetingPage(api: api)));
    await tester.enterText(find.byKey(const ValueKey('name-input')), 'Ada');
    await tester.tap(find.byKey(const ValueKey('submit-button')));
    await tester.pumpAndSettle();

    expect(find.text('Hello, Ada!'), findsOneWidget);
  }});
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "5. Red: Add The Integration Test"
```

### 6. Green: Wire The Real Application

Replace `workspace/lib/main.dart` with:

```dart
import 'package:flutter/material.dart';

import 'adapter/greeting_page.dart';
import 'adapter/http_greeting_api.dart';

void main() {{
  runApp(const SayingHelloApp());
}}

class SayingHelloApp extends StatelessWidget {{
  const SayingHelloApp({{super.key}});

  @override
  Widget build(BuildContext context) {{
    return MaterialApp(
      title: 'Saying Hello',
      home: GreetingPage(
        api: HttpGreetingApi(baseUrl: 'http://localhost:{FOR_ALL_API_PORT}'),
      ),
    );
  }}
}}
```

Run:

```bash
just check-tests
git add --all
git commit --message "6. Green: Wire The Real Application"
```"#
            );

    if is_flutter_http_saying_hello_output_repo(spec) {
        body = body.replace(
            &format!(
                "import 'package:flutter/material.dart';\n\nimport 'adapter/greeting_page.dart';\nimport 'adapter/http_greeting_api.dart';\n\nvoid main() {{\n  runApp(const SayingHelloApp());\n}}\n\nclass SayingHelloApp extends StatelessWidget {{\n  const SayingHelloApp({{super.key}});\n\n  @override\n  Widget build(BuildContext context) {{\n    return MaterialApp(\n      title: 'Saying Hello',\n      home: GreetingPage(\n        api: HttpGreetingApi(baseUrl: 'http://localhost:{FOR_ALL_API_PORT}'),\n      ),\n    );\n  }}\n}}"
            ),
            &format!(
                "import 'package:flutter/material.dart';\n\nimport 'adapter/greeting_page.dart';\nimport 'adapter/http_greeting_api.dart';\n\nconst apiBaseUrl = String.fromEnvironment(\n  'API_BASE_URL',\n  defaultValue: 'http://localhost:{FOR_ALL_API_PORT}',\n);\n\nvoid main() {{\n  runApp(const SayingHelloApp());\n}}\n\nclass SayingHelloApp extends StatelessWidget {{\n  const SayingHelloApp({{super.key}});\n\n  @override\n  Widget build(BuildContext context) {{\n    return MaterialApp(\n      title: 'Saying Hello',\n      home: GreetingPage(\n        api: HttpGreetingApi(baseUrl: apiBaseUrl),\n      ),\n    );\n  }}\n}}"
            ),
        );
    }

    tutorial_file_markdown("Adapter", &rewrite_touch_creation_stage_only(&body))
}

fn render_astro_saying_hello_contracts_content(_spec: &OutputRepoSpec) -> String {
    tutorial_file_markdown(
        "Contracts",
        &rewrite_touch_creation_checkpoints(
            "Create the shared contract files:\n\n```bash\ntouch workspace/src/contracts/greeting-response.ts\ntouch workspace/src/contracts/greeting-api.ts\n```\n\nPut this exact content in `workspace/src/contracts/greeting-response.ts`:\n\n```ts\nexport interface GreetingResponse {\n  message: string;\n}\n```\n\nPut this exact content in `workspace/src/contracts/greeting-api.ts`:\n\n```ts\nimport type { GreetingResponse } from './greeting-response';\n\nexport interface GreetingApi {\n  getGreeting(name: string): Promise<GreetingResponse>;\n}\n```\n\nDo not add tests here. Keep this layer limited to interfaces and small shared types.\n\nThen run:\n\n```bash\njust format\ngit add --all\ngit commit --message \"Define greeting contracts\"\n```",
        ),
    )
}

fn render_astro_saying_hello_code_content(_spec: &OutputRepoSpec) -> String {
    tutorial_file_markdown(
        "Code",
        &rewrite_touch_creation_checkpoints(
            "### 1. Red: Add The First Failing Code Test\n\nCreate the first code test file:\n\n```bash\ntouch workspace/src/code/load-greeting.test.ts\n```\n\nPut this exact content in `workspace/src/code/load-greeting.test.ts`:\n\n```ts\nimport { describe, expect, it, vi } from 'vitest';\n\nimport type { GreetingApi } from '../contracts/greeting-api';\nimport { loadGreeting } from './load-greeting';\n\ndescribe('loadGreeting', () => {\n  it('returns the personalized greeting for a non-empty name', async () => {\n    const getGreeting = vi.fn().mockResolvedValue({ message: 'Hello, Ada!' });\n    const api: GreetingApi = { getGreeting };\n\n    const result = await loadGreeting('Ada', api);\n\n    expect(getGreeting).toHaveBeenCalledWith('Ada');\n    expect(result).toEqual({\n      submittedName: 'Ada',\n      message: 'Hello, Ada!',\n    });\n  });\n});\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"1. Red: Add The First Failing Code Test\"\n```\n\n### 2. Green: Return The Personalized Greeting\n\nCreate the first production file:\n\n```bash\ntouch workspace/src/code/load-greeting.ts\n```\n\nPut this exact content in `workspace/src/code/load-greeting.ts`:\n\n```ts\nimport type { GreetingApi } from '../contracts/greeting-api';\n\nexport async function loadGreeting(name: string, api: GreetingApi) {\n  const response = await api.getGreeting(name);\n\n  return {\n    submittedName: name,\n    message: response.message,\n  };\n}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"2. Green: Return The Personalized Greeting\"\n```\n\n### 3. Red: Trim The Name Before Calling The API\n\nReplace `workspace/src/code/load-greeting.test.ts` with:\n\n```ts\nimport { describe, expect, it, vi } from 'vitest';\n\nimport type { GreetingApi } from '../contracts/greeting-api';\nimport { loadGreeting } from './load-greeting';\n\ndescribe('loadGreeting', () => {\n  it('returns the personalized greeting for a non-empty name', async () => {\n    const getGreeting = vi.fn().mockResolvedValue({ message: 'Hello, Ada!' });\n    const api: GreetingApi = { getGreeting };\n\n    const result = await loadGreeting('Ada', api);\n\n    expect(getGreeting).toHaveBeenCalledWith('Ada');\n    expect(result).toEqual({\n      submittedName: 'Ada',\n      message: 'Hello, Ada!',\n    });\n  });\n\n  it('trims the name before calling the API', async () => {\n    const getGreeting = vi.fn().mockResolvedValue({ message: 'Hello, Ada!' });\n    const api: GreetingApi = { getGreeting };\n\n    const result = await loadGreeting('  Ada  ', api);\n\n    expect(getGreeting).toHaveBeenCalledWith('Ada');\n    expect(result).toEqual({\n      submittedName: 'Ada',\n      message: 'Hello, Ada!',\n    });\n  });\n});\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"3. Red: Trim The Name Before Calling The API\"\n```\n\n### 4. Green: Trim The Name Before Calling The API\n\nReplace `workspace/src/code/load-greeting.ts` with:\n\n```ts\nimport type { GreetingApi } from '../contracts/greeting-api';\n\nexport async function loadGreeting(name: string, api: GreetingApi) {\n  const submittedName = name.trim();\n  const response = await api.getGreeting(submittedName);\n\n  return {\n    submittedName,\n    message: response.message,\n  };\n}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"4. Green: Trim The Name Before Calling The API\"\n```\n\n### 5. Red: Return A Friendly Message When The API Is Unavailable\n\nReplace `workspace/src/code/load-greeting.test.ts` with:\n\n```ts\nimport { describe, expect, it, vi } from 'vitest';\n\nimport type { GreetingApi } from '../contracts/greeting-api';\nimport { loadGreeting } from './load-greeting';\n\ndescribe('loadGreeting', () => {\n  it('returns the personalized greeting for a non-empty name', async () => {\n    const getGreeting = vi.fn().mockResolvedValue({ message: 'Hello, Ada!' });\n    const api: GreetingApi = { getGreeting };\n\n    const result = await loadGreeting('Ada', api);\n\n    expect(getGreeting).toHaveBeenCalledWith('Ada');\n    expect(result).toEqual({\n      submittedName: 'Ada',\n      message: 'Hello, Ada!',\n    });\n  });\n\n  it('trims the name before calling the API', async () => {\n    const getGreeting = vi.fn().mockResolvedValue({ message: 'Hello, Ada!' });\n    const api: GreetingApi = { getGreeting };\n\n    const result = await loadGreeting('  Ada  ', api);\n\n    expect(getGreeting).toHaveBeenCalledWith('Ada');\n    expect(result).toEqual({\n      submittedName: 'Ada',\n      message: 'Hello, Ada!',\n    });\n  });\n\n  it('returns a friendly message when the API is unavailable', async () => {\n    const getGreeting = vi.fn().mockRejectedValue(new Error('network error'));\n    const api: GreetingApi = { getGreeting };\n\n    const result = await loadGreeting('Ada', api);\n\n    expect(getGreeting).toHaveBeenCalledWith('Ada');\n    expect(result).toEqual({\n      submittedName: 'Ada',\n      message: 'Sorry, the greeting API is unavailable right now.',\n    });\n  });\n});\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"5. Red: Return A Friendly Message When The API Is Unavailable\"\n```\n\n### 6. Green: Return A Friendly Message When The API Is Unavailable\n\nReplace `workspace/src/code/load-greeting.ts` with:\n\n```ts\nimport type { GreetingApi } from '../contracts/greeting-api';\n\nexport async function loadGreeting(name: string, api: GreetingApi) {\n  const submittedName = name.trim();\n\n  try {\n    const response = await api.getGreeting(submittedName);\n\n    return {\n      submittedName,\n      message: response.message,\n    };\n  } catch {\n    return {\n      submittedName,\n      message: 'Sorry, the greeting API is unavailable right now.',\n    };\n  }\n}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"6. Green: Return A Friendly Message When The API Is Unavailable\"\n```",
        ),
    )
}

/*
fn render_astro_saying_hello_adapter_content(_spec: &OutputRepoSpec) -> String {
    tutorial_file_markdown(
        "Adapter",
        &format!(
            "### 1. Red: Add The First Failing HTTP Adapter Test\n\nCreate the first adapter test file:\n\n```bash\ntouch workspace/src/adapter/http-greeting-api.test.ts\n```\n\nPut this exact content in `workspace/src/adapter/http-greeting-api.test.ts`:\n\n```ts\nimport {{ afterEach, describe, expect, it, vi }} from 'vitest';\n\nimport {{ HttpGreetingApi }} from './http-greeting-api';\n\ndescribe('HttpGreetingApi', () => {{\n  afterEach(() => {{\n    vi.unstubAllGlobals();\n  }});\n\n  it('requests the canonical greeting endpoint', async () => {{\n    const fetchMock = vi.fn().mockResolvedValue({{\n      json: vi.fn().mockResolvedValue({{ message: 'Hello, Ada!' }}),\n    }});\n    vi.stubGlobal('fetch', fetchMock);\n\n    const api = new HttpGreetingApi('http://localhost:{FOR_ALL_API_PORT}');\n    const result = await api.getGreeting('Ada');\n\n    expect(fetchMock).toHaveBeenCalledWith('http://localhost:{FOR_ALL_API_PORT}/api/greeting?name=Ada');\n    expect(result).toEqual({{ message: 'Hello, Ada!' }});\n  }});\n}});\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"1. Red: Add The First Failing HTTP Adapter Test\"\n```\n\n### 2. Green: Request The Canonical Greeting Endpoint\n\nCreate the first adapter production file:\n\n```bash\ntouch workspace/src/adapter/http-greeting-api.ts\n```\n\nPut this exact content in `workspace/src/adapter/http-greeting-api.ts`:\n\n```ts\nimport type {{ GreetingApi }} from '../contracts/greeting-api';\nimport type {{ GreetingResponse }} from '../contracts/greeting-response';\n\nexport class HttpGreetingApi implements GreetingApi {{\n  constructor(private readonly baseUrl: string) {{}}\n\n  async getGreeting(name: string): Promise<GreetingResponse> {{\n    const response = await fetch(\n      `${{this.baseUrl}}/api/greeting?name=${{encodeURIComponent(name)}}`,\n    );\n\n    return (await response.json()) as GreetingResponse;\n  }}\n}}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"2. Green: Request The Canonical Greeting Endpoint\"\n```\n\n### 3. Red: Omit The Query String For Empty Input\n\nReplace `workspace/src/adapter/http-greeting-api.test.ts` with:\n\n```ts\nimport {{ afterEach, describe, expect, it, vi }} from 'vitest';\n\nimport {{ HttpGreetingApi }} from './http-greeting-api';\n\ndescribe('HttpGreetingApi', () => {{\n  afterEach(() => {{\n    vi.unstubAllGlobals();\n  }});\n\n  it('requests the canonical greeting endpoint', async () => {{\n    const fetchMock = vi.fn().mockResolvedValue({{\n      json: vi.fn().mockResolvedValue({{ message: 'Hello, Ada!' }}),\n    }});\n    vi.stubGlobal('fetch', fetchMock);\n\n    const api = new HttpGreetingApi('http://localhost:{FOR_ALL_API_PORT}');\n    const result = await api.getGreeting('Ada');\n\n    expect(fetchMock).toHaveBeenCalledWith('http://localhost:{FOR_ALL_API_PORT}/api/greeting?name=Ada');\n    expect(result).toEqual({{ message: 'Hello, Ada!' }});\n  }});\n\n  it('omits the query string for empty input', async () => {{\n    const fetchMock = vi.fn().mockResolvedValue({{\n      json: vi.fn().mockResolvedValue({{ message: 'Hello!' }}),\n    }});\n    vi.stubGlobal('fetch', fetchMock);\n\n    const api = new HttpGreetingApi('http://localhost:{FOR_ALL_API_PORT}');\n    const result = await api.getGreeting('');\n\n    expect(fetchMock).toHaveBeenCalledWith('http://localhost:{FOR_ALL_API_PORT}/api/greeting');\n    expect(result).toEqual({{ message: 'Hello!' }});\n  }});\n}});\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"3. Red: Omit The Query String For Empty Input\"\n```\n\n### 4. Green: Omit The Query String For Empty Input\n\nReplace `workspace/src/adapter/http-greeting-api.ts` with:\n\n```ts\nimport type {{ GreetingApi }} from '../contracts/greeting-api';\nimport type {{ GreetingResponse }} from '../contracts/greeting-response';\n\nexport class HttpGreetingApi implements GreetingApi {{\n  constructor(private readonly baseUrl: string) {{}}\n\n  async getGreeting(name: string): Promise<GreetingResponse> {{\n    const url =\n      name === ''\n        ? `${{this.baseUrl}}/api/greeting`\n        : `${{this.baseUrl}}/api/greeting?name=${{encodeURIComponent(name)}}`;\n\n    const response = await fetch(url);\n\n    return (await response.json()) as GreetingResponse;\n  }}\n}}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"4. Green: Omit The Query String For Empty Input\"\n```\n\n### 5. Red: Add The Browser Binding Test\n\nCreate the browser binding test file:\n\n```bash\ntouch workspace/src/adapter/bind-greeting-form.test.ts\n```\n\nPut this exact content in `workspace/src/adapter/bind-greeting-form.test.ts`:\n\n```ts\nimport {{ describe, expect, it, vi }} from 'vitest';\n\nimport type {{ GreetingApi }} from '../contracts/greeting-api';\nimport {{ bindGreetingForm }} from './bind-greeting-form';\n\ndescribe('bindGreetingForm', () => {{\n  it('renders the greeting returned by the API', async () => {{\n    document.body.innerHTML = `\n      <form data-greeting-form>\n        <input data-greeting-name />\n        <button type=\"submit\">Say hello</button>\n      </form>\n      <p data-greeting-output></p>\n    `;\n\n    const form = document.querySelector('[data-greeting-form]') as HTMLFormElement;\n    const nameInput = document.querySelector('[data-greeting-name]') as HTMLInputElement;\n    const output = document.querySelector('[data-greeting-output]') as HTMLParagraphElement;\n    const getGreeting = vi.fn().mockResolvedValue({{ message: 'Hello, Ada!' }});\n    const api: GreetingApi = {{ getGreeting }};\n\n    bindGreetingForm({{ form, nameInput, output, api }});\n\n    nameInput.value = 'Ada';\n    form.dispatchEvent(new Event('submit', {{ bubbles: true, cancelable: true }}));\n    await new Promise((resolve) => setTimeout(resolve, 0));\n\n    expect(getGreeting).toHaveBeenCalledWith('Ada');\n    expect(output.textContent).toBe('Hello, Ada!');\n  }});\n}});\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"5. Red: Add The Browser Binding Test\"\n```\n\n### 6. Green: Bind The Browser Form\n\nCreate the browser binding file:\n\n```bash\ntouch workspace/src/adapter/bind-greeting-form.ts\n```\n\nPut this exact content in `workspace/src/adapter/bind-greeting-form.ts`:\n\n```ts\nimport type {{ GreetingApi }} from '../contracts/greeting-api';\nimport {{ loadGreeting }} from '../code/load-greeting';\n\ntype BindGreetingFormOptions = {{\n  form: HTMLFormElement;\n  nameInput: HTMLInputElement;\n  output: HTMLElement;\n  api: GreetingApi;\n}};\n\nexport function bindGreetingForm({{\n  form,\n  nameInput,\n  output,\n  api,\n}}: BindGreetingFormOptions) {{\n  form.addEventListener('submit', async (event) => {{\n    event.preventDefault();\n\n    const result = await loadGreeting(nameInput.value, api);\n    output.textContent = result.message;\n  }});\n}}\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"6. Green: Bind The Browser Form\"\n```\n\n### 7. Red: Add The Astro Page Test\n\nCreate the Astro page test file:\n\n```bash\ntouch workspace/src/adapter/index-page.test.ts\n```\n\nPut this exact content in `workspace/src/adapter/index-page.test.ts`:\n\n```ts\nimport {{ experimental_AstroContainer as AstroContainer }} from 'astro/container';\nimport {{ describe, expect, it }} from 'vitest';\n\nimport IndexPage from '../pages/index.astro';\n\ndescribe('index page', () => {{\n  it('renders the greeting form and API endpoint', async () => {{\n    const container = await AstroContainer.create();\n    const result = await container.renderToString(IndexPage);\n\n    expect(result).toContain('data-greeting-form');\n    expect(result).toContain('data-greeting-name');\n    expect(result).toContain('data-greeting-output');\n    expect(result).toContain('http://localhost:{FOR_ALL_API_PORT}');\n  }});\n}});\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"7. Red: Add The Astro Page Test\"\n```\n\n### 8. Green: Wire The Astro Page\n\nCreate the Astro page:\n\n```bash\ntouch workspace/src/pages/index.astro\n```\n\nPut this exact content in `workspace/src/pages/index.astro`:\n\n```astro\n---\nconst apiBaseUrl = 'http://localhost:{FOR_ALL_API_PORT}';\n---\n\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width\" />\n    <title>Saying Hello</title>\n  </head>\n  <body>\n    <main>\n      <h1>Saying Hello</h1>\n      <form data-greeting-form>\n        <label for=\"name-input\">Name</label>\n        <input data-greeting-name id=\"name-input\" name=\"name\" type=\"text\" />\n        <button type=\"submit\">Say hello</button>\n      </form>\n      <p data-greeting-output aria-live=\"polite\"></p>\n    </main>\n\n    <script>\n      import {{ HttpGreetingApi }} from '../adapter/http-greeting-api';\n      import {{ bindGreetingForm }} from '../adapter/bind-greeting-form';\n\n      const form = document.querySelector('[data-greeting-form]');\n      const nameInput = document.querySelector('[data-greeting-name]');\n      const output = document.querySelector('[data-greeting-output]');\n\n      if (\n        form instanceof HTMLFormElement &&\n        nameInput instanceof HTMLInputElement &&\n        output instanceof HTMLElement\n      ) {{\n        bindGreetingForm({{\n          form,\n          nameInput,\n          output,\n          api: new HttpGreetingApi('{format!("http://localhost:{FOR_ALL_API_PORT}")}'),\n        }});\n      }}\n    </script>\n  </body>\n</html>\n```\n\nRun:\n\n```bash\njust check-tests\ngit add --all\ngit commit --message \"8. Green: Wire The Astro Page\"\n```"
        ),
    )
}
*/

fn render_astro_saying_hello_adapter_content(_spec: &OutputRepoSpec) -> String {
    let body = r#"### 1. Red: Add The First Failing HTTP Adapter Test

Create the first adapter test file:

```bash
touch workspace/src/adapter/http-greeting-api.test.ts
```

Put this exact content in `workspace/src/adapter/http-greeting-api.test.ts`:

```ts
import { afterEach, describe, expect, it, vi } from 'vitest';

import { HttpGreetingApi } from './http-greeting-api';

describe('HttpGreetingApi', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('requests the canonical greeting endpoint', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      json: vi.fn().mockResolvedValue({ message: 'Hello, Ada!' }),
    });
    vi.stubGlobal('fetch', fetchMock);

    const api = new HttpGreetingApi('http://localhost:__API_PORT__');
    const result = await api.getGreeting('Ada');

    expect(fetchMock).toHaveBeenCalledWith('http://localhost:__API_PORT__/api/greeting?name=Ada');
    expect(result).toEqual({ message: 'Hello, Ada!' });
  });
});
```

Run:

```bash
just check-tests
git add --all
git commit --message "1. Red: Add The First Failing HTTP Adapter Test"
```

### 2. Green: Request The Canonical Greeting Endpoint

Create the first adapter production file:

```bash
touch workspace/src/adapter/http-greeting-api.ts
```

Put this exact content in `workspace/src/adapter/http-greeting-api.ts`:

```ts
import type { GreetingApi } from '../contracts/greeting-api';
import type { GreetingResponse } from '../contracts/greeting-response';

export class HttpGreetingApi implements GreetingApi {
  constructor(private readonly baseUrl: string) {}

  async getGreeting(name: string): Promise<GreetingResponse> {
    const response = await fetch(
      `${this.baseUrl}/api/greeting?name=${encodeURIComponent(name)}`,
    );

    return (await response.json()) as GreetingResponse;
  }
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "2. Green: Request The Canonical Greeting Endpoint"
```

### 3. Red: Omit The Query String For Empty Input

Replace `workspace/src/adapter/http-greeting-api.test.ts` with:

```ts
import { afterEach, describe, expect, it, vi } from 'vitest';

import { HttpGreetingApi } from './http-greeting-api';

describe('HttpGreetingApi', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('requests the canonical greeting endpoint', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      json: vi.fn().mockResolvedValue({ message: 'Hello, Ada!' }),
    });
    vi.stubGlobal('fetch', fetchMock);

    const api = new HttpGreetingApi('http://localhost:__API_PORT__');
    const result = await api.getGreeting('Ada');

    expect(fetchMock).toHaveBeenCalledWith('http://localhost:__API_PORT__/api/greeting?name=Ada');
    expect(result).toEqual({ message: 'Hello, Ada!' });
  });

  it('omits the query string for empty input', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      json: vi.fn().mockResolvedValue({ message: 'Hello!' }),
    });
    vi.stubGlobal('fetch', fetchMock);

    const api = new HttpGreetingApi('http://localhost:__API_PORT__');
    const result = await api.getGreeting('');

    expect(fetchMock).toHaveBeenCalledWith('http://localhost:__API_PORT__/api/greeting');
    expect(result).toEqual({ message: 'Hello!' });
  });
});
```

Run:

```bash
just check-tests
git add --all
git commit --message "3. Red: Omit The Query String For Empty Input"
```

### 4. Green: Omit The Query String For Empty Input

Replace `workspace/src/adapter/http-greeting-api.ts` with:

```ts
import type { GreetingApi } from '../contracts/greeting-api';
import type { GreetingResponse } from '../contracts/greeting-response';

export class HttpGreetingApi implements GreetingApi {
  constructor(private readonly baseUrl: string) {}

  async getGreeting(name: string): Promise<GreetingResponse> {
    const url =
      name === ''
        ? `${this.baseUrl}/api/greeting`
        : `${this.baseUrl}/api/greeting?name=${encodeURIComponent(name)}`;

    const response = await fetch(url);

    return (await response.json()) as GreetingResponse;
  }
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "4. Green: Omit The Query String For Empty Input"
```

### 5. Red: Add The Browser Binding Test

Create the browser binding test file:

```bash
touch workspace/src/adapter/bind-greeting-form.test.ts
```

Put this exact content in `workspace/src/adapter/bind-greeting-form.test.ts`:

```ts
// @vitest-environment jsdom

import { describe, expect, it, vi } from 'vitest';

import type { GreetingApi } from '../contracts/greeting-api';
import { bindGreetingForm } from './bind-greeting-form';

describe('bindGreetingForm', () => {
  it('renders the greeting returned by the API', async () => {
    document.body.innerHTML = `
      <form data-greeting-form>
        <input data-greeting-name />
        <button type="submit">Say hello</button>
      </form>
      <p data-greeting-output></p>
    `;

    const form = document.querySelector('[data-greeting-form]') as HTMLFormElement;
    const nameInput = document.querySelector('[data-greeting-name]') as HTMLInputElement;
    const output = document.querySelector('[data-greeting-output]') as HTMLParagraphElement;
    const getGreeting = vi.fn().mockResolvedValue({ message: 'Hello, Ada!' });
    const api: GreetingApi = { getGreeting };

    bindGreetingForm({ form, nameInput, output, api });

    nameInput.value = 'Ada';
    form.dispatchEvent(new Event('submit', { bubbles: true, cancelable: true }));
    await new Promise((resolve) => setTimeout(resolve, 0));

    expect(getGreeting).toHaveBeenCalledWith('Ada');
    expect(output.textContent).toBe('Hello, Ada!');
  });
});
```

Run:

```bash
just check-tests
git add --all
git commit --message "5. Red: Add The Browser Binding Test"
```

### 6. Green: Bind The Browser Form

Create the browser binding file:

```bash
touch workspace/src/adapter/bind-greeting-form.ts
```

Put this exact content in `workspace/src/adapter/bind-greeting-form.ts`:

```ts
import type { GreetingApi } from '../contracts/greeting-api';
import { loadGreeting } from '../code/load-greeting';

type BindGreetingFormOptions = {
  form: HTMLFormElement;
  nameInput: HTMLInputElement;
  output: HTMLElement;
  api: GreetingApi;
};

export function bindGreetingForm({
  form,
  nameInput,
  output,
  api,
}: BindGreetingFormOptions) {
  form.addEventListener('submit', async (event) => {
    event.preventDefault();

    const result = await loadGreeting(nameInput.value, api);
    output.textContent = result.message;
  });
}
```

Run:

```bash
just check-tests
git add --all
git commit --message "6. Green: Bind The Browser Form"
```

### 7. Red: Add The Astro Page Test

Create the Astro page test file:

```bash
touch workspace/src/adapter/index-page.test.ts
```

Put this exact content in `workspace/src/adapter/index-page.test.ts`:

```ts
import { experimental_AstroContainer as AstroContainer } from 'astro/container';
import { describe, expect, it } from 'vitest';

import IndexPage from '../pages/index.astro';

describe('index page', () => {
  it('renders the greeting form and API endpoint', async () => {
    const container = await AstroContainer.create();
    const result = await container.renderToString(IndexPage);

    expect(result).toContain('data-greeting-form');
    expect(result).toContain('data-greeting-name');
    expect(result).toContain('data-greeting-output');
    expect(result).toContain('data-api-base-url="http://localhost:__API_PORT__"');
  });
});
```

Run:

```bash
just check-tests
git add --all
git commit --message "7. Red: Add The Astro Page Test"
```

### 8. Green: Wire The Astro Page

Create the Astro page:

```bash
touch workspace/src/pages/index.astro
```

Put this exact content in `workspace/src/pages/index.astro`:

```astro
---
const apiBaseUrl = 'http://localhost:__API_PORT__';
---

<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width" />
    <title>Saying Hello</title>
  </head>
  <body>
    <main data-api-base-url={apiBaseUrl}>
      <h1>Saying Hello</h1>
      <form data-greeting-form>
        <label for="name-input">Name</label>
        <input data-greeting-name id="name-input" name="name" type="text" />
        <button type="submit">Say hello</button>
      </form>
      <p data-greeting-output aria-live="polite"></p>
    </main>

    <script>
      import { HttpGreetingApi } from '../adapter/http-greeting-api';
      import { bindGreetingForm } from '../adapter/bind-greeting-form';

      const page = document.querySelector('[data-api-base-url]');
      const form = document.querySelector('[data-greeting-form]');
      const nameInput = document.querySelector('[data-greeting-name]');
      const output = document.querySelector('[data-greeting-output]');
      const apiBaseUrl = page?.getAttribute('data-api-base-url');

      if (
        form instanceof HTMLFormElement &&
        nameInput instanceof HTMLInputElement &&
        output instanceof HTMLElement &&
        apiBaseUrl
      ) {
        bindGreetingForm({
          form,
          nameInput,
          output,
          api: new HttpGreetingApi(apiBaseUrl),
        });
      }
    </script>
  </body>
</html>
```

Run:

```bash
just check-tests
git add --all
git commit --message "8. Green: Wire The Astro Page"
```"#;

    tutorial_file_markdown(
        "Adapter",
        &rewrite_touch_creation_checkpoints(&body.replace("__API_PORT__", FOR_ALL_API_PORT)),
    )
}

fn render_matching_core_tutorial_link(project_slug: &str, output: &CompiledOutput) -> String {
    let testing = testing_selection(output).unwrap_or("xunit");
    let core_tutorial_path = format!(
        "tutorials/{project_slug}/{}/{}/core/{testing}/README.md",
        output.selections.ecosystem, output.selections.language
    );
    let core_tutorial_url = format!(
        "https://github.com/{GITHUB_OWNER}/for-all_tutorials/blob/main/{core_tutorial_path}"
    );

    format!(
        "Build the real `IGreetingService` implementation next by following the matching core tutorial: \
[{}]({}).",
        format_selection_value(testing),
        core_tutorial_url
    )
}

fn repo_name(project_slug: &str, output: &CompiledOutput) -> String {
    if output.kind == OutputKind::Contracts {
        format!(
            "{}_{}_{}_{}_contracts",
            OUTPUT_REPO_PREFIX,
            project_slug,
            repo_name_selection_value(&output.selections.ecosystem),
            repo_name_selection_value(&output.selections.language),
        )
    } else if output.kind == OutputKind::Core {
        format!(
            "{}_{}_{}_{}_core_{}",
            OUTPUT_REPO_PREFIX,
            project_slug,
            repo_name_selection_value(&output.selections.ecosystem),
            repo_name_selection_value(&output.selections.language),
            repo_name_selection_value(testing_selection(output).unwrap_or("xunit"))
        )
    } else {
        format!(
            "{}_{}_{}_{}_{}_{}_{}_{}_{}",
            OUTPUT_REPO_PREFIX,
            project_slug,
            repo_name_selection_value(&output.selections.ecosystem),
            repo_name_selection_value(&output.selections.language),
            repo_name_selection_value(output.selections.storage.as_deref().unwrap_or("no-storage")),
            repo_name_selection_value(
                output
                    .selections
                    .surface
                    .as_deref()
                    .unwrap_or("unknown-surface"),
            ),
            repo_name_selection_value(
                output
                    .selections
                    .target
                    .as_deref()
                    .unwrap_or("unknown-target"),
            ),
            repo_name_selection_value(
                output
                    .selections
                    .framework
                    .as_deref()
                    .unwrap_or("unknown-framework"),
            ),
            repo_name_selection_value(testing_selection(output).unwrap_or("xunit")),
        )
    }
}

fn contracts_repo_name(project_slug: &str, output: &CompiledOutput) -> String {
    format!(
        "{}_{}_{}_{}_contracts",
        OUTPUT_REPO_PREFIX,
        project_slug,
        repo_name_selection_value(&output.selections.ecosystem),
        repo_name_selection_value(&output.selections.language),
    )
}

fn repo_name_selection_value(value: &str) -> &str {
    match value {
        "no-storage" => "ns",
        "local-files-csv" => "csv",
        "local-files-json" => "json",
        "local-files-yaml" => "yaml",
        "local-files-toml" => "toml",
        "local-files-xml" => "xml",
        "database-firebase" => "firebase",
        "database-sqlite" => "sqlite",
        "database-postgres" => "postgres",
        "database-mysql" => "mysql",
        "command-line" => "cli",
        "graphical" => "gui",
        "web-service" => "svc",
        "client" => "client",
        "web" => "web",
        "all" => "all",
        "full-stack" => "fullstack",
        "no-framework" => "nf",
        "blazor-server" => "blazor",
        other => other,
    }
}

fn repo_description(project_title: &str, output: &CompiledOutput) -> String {
    if output.kind == OutputKind::Contracts {
        format!(
            "Generated contracts library for the {project_title} tutorial in {}/{}.",
            format_selection_value(&output.selections.ecosystem),
            format_selection_value(&output.selections.language),
        )
    } else if output.kind == OutputKind::Core {
        format!(
            "Manual spec-driven, test-driven core library for the {project_title} tutorial in {}/{} with {}.",
            format_selection_value(&output.selections.ecosystem),
            format_selection_value(&output.selections.language),
            format_selection_value(testing_selection(output).unwrap_or("xunit"))
        )
    } else {
        format!(
            "Manual {}/{} {} adapter for the {project_title} tutorial in {}/{} with {}, consuming a shared contracts library.",
            format_selection_value(output.selections.surface.as_deref().unwrap_or("surface")),
            format_selection_value(output.selections.target.as_deref().unwrap_or("target")),
            format_selection_value(output.selections.framework.as_deref().unwrap_or("framework")),
            format_selection_value(&output.selections.ecosystem),
            format_selection_value(&output.selections.language),
            format_selection_value(testing_selection(output).unwrap_or("xunit"))
        )
    }
}

fn pascal_case_slug(slug: &str) -> String {
    slug.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    let mut word = first.to_uppercase().to_string();
                    word.push_str(chars.as_str());
                    word
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

fn format_selection_value(value: &str) -> String {
    match value {
        "dotnet" => ".NET".to_string(),
        "csharp" => "C#".to_string(),
        "xunit" => "xUnit".to_string(),
        "nunit" => "NUnit".to_string(),
        "mstest" => "MSTest".to_string(),
        "tunit" => "TUnit".to_string(),
        other => other
            .split('-')
            .map(|part| {
                if part.chars().all(|ch| ch.is_ascii_uppercase()) {
                    part.to_string()
                } else {
                    let mut chars = part.chars();
                    match chars.next() {
                        Some(first) => {
                            let mut word = first.to_uppercase().to_string();
                            word.push_str(chars.as_str());
                            word
                        }
                        None => String::new(),
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

#[derive(Debug)]
enum AppError {
    Message(String),
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    Askama(askama::Error),
}

impl AppError {
    fn message(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(message) => write!(f, "{message}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::Yaml(error) => write!(f, "{error}"),
            Self::Askama(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_yaml::Error> for AppError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

impl From<askama::Error> for AppError {
    fn from(value: askama::Error) -> Self {
        Self::Askama(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_output_repo_spec() -> OutputRepoSpec {
        OutputRepoSpec {
            repo_name: "fa_tut_saying-hello".to_string(),
            repo_description:
                "Tutorial workspace for the Saying Hello project with default Go/Echo API choices."
                    .to_string(),
            project_slug: "saying-hello".to_string(),
            selections: OutputRepoSelections {
                ecosystem: "go".to_string(),
                language: "go".to_string(),
                testing: "testify".to_string(),
                mocking: "testify-mock".to_string(),
                storage: "no-storage".to_string(),
                surface: "web".to_string(),
                target: "api".to_string(),
                framework: "echo".to_string(),
                protocol: Some("http-json".to_string()),
            },
        }
    }

    fn sample_todo_list_go_output_repo_spec() -> OutputRepoSpec {
        OutputRepoSpec {
            repo_name: "fa_tut_todo-list".to_string(),
            repo_description:
                "Tutorial workspace for the Todo List project with go / go / testify / testify-mock / local-files-json / web / API / echo / http-json choices."
                    .to_string(),
            project_slug: "todo-list".to_string(),
            selections: OutputRepoSelections {
                ecosystem: "go".to_string(),
                language: "go".to_string(),
                testing: "testify".to_string(),
                mocking: "testify-mock".to_string(),
                storage: "local-files-json".to_string(),
                surface: "web".to_string(),
                target: "api".to_string(),
                framework: "echo".to_string(),
                protocol: Some("http-json".to_string()),
            },
        }
    }

    fn sample_todo_list_go_http_json_sqlite_output_repo_spec() -> OutputRepoSpec {
        OutputRepoSpec {
            repo_name: "fa_tut_todo-list".to_string(),
            repo_description:
                "Tutorial workspace for the Todo List project with go / go / testify / testify-mock / SQLite / web / API / echo / http-json choices."
                    .to_string(),
            project_slug: "todo-list".to_string(),
            selections: OutputRepoSelections {
                ecosystem: "go".to_string(),
                language: "go".to_string(),
                testing: "testify".to_string(),
                mocking: "testify-mock".to_string(),
                storage: "database-sqlite".to_string(),
                surface: "web".to_string(),
                target: "api".to_string(),
                framework: "echo".to_string(),
                protocol: Some("http-json".to_string()),
            },
        }
    }

    fn sample_todo_list_go_rest_json_postgres_output_repo_spec() -> OutputRepoSpec {
        OutputRepoSpec {
            repo_name: "fa_tut_todo-list".to_string(),
            repo_description:
                "Tutorial workspace for the Todo List project with go / go / testify / testify-mock / Postgres / web / API / echo / rest-json choices."
                    .to_string(),
            project_slug: "todo-list".to_string(),
            selections: OutputRepoSelections {
                ecosystem: "go".to_string(),
                language: "go".to_string(),
                testing: "testify".to_string(),
                mocking: "testify-mock".to_string(),
                storage: "database-postgres".to_string(),
                surface: "web".to_string(),
                target: "api".to_string(),
                framework: "echo".to_string(),
                protocol: Some("rest-json".to_string()),
            },
        }
    }

    fn sample_astro_output_repo_spec() -> OutputRepoSpec {
        OutputRepoSpec {
            repo_name: "fa_tut_saying-hello".to_string(),
            repo_description:
                "Tutorial workspace for the Saying Hello project with JavaScript / TypeScript / Vitest / Vitest built-in / no-storage / web / front-end / Astro / http-json choices."
                    .to_string(),
            project_slug: "saying-hello".to_string(),
            selections: OutputRepoSelections {
                ecosystem: "javascript".to_string(),
                language: "typescript".to_string(),
                testing: "vitest".to_string(),
                mocking: "vitest-built-in".to_string(),
                storage: "no-storage".to_string(),
                surface: "web".to_string(),
                target: "front-end".to_string(),
                framework: "astro".to_string(),
                protocol: Some("http-json".to_string()),
            },
        }
    }

    fn sample_flutter_output_repo_spec() -> OutputRepoSpec {
        OutputRepoSpec {
            repo_name: "fa_tut_saying-hello".to_string(),
            repo_description:
                "Tutorial workspace for the Saying Hello project with Dart / Dart / test / mocktail / no-storage / client / all / Flutter / http-json choices."
                    .to_string(),
            project_slug: "saying-hello".to_string(),
            selections: OutputRepoSelections {
                ecosystem: "dart".to_string(),
                language: "dart".to_string(),
                testing: "test".to_string(),
                mocking: "mocktail".to_string(),
                storage: "no-storage".to_string(),
                surface: "client".to_string(),
                target: "all".to_string(),
                framework: "flutter".to_string(),
                protocol: Some("http-json".to_string()),
            },
        }
    }

    fn sample_todo_list_flutter_output_repo_spec() -> OutputRepoSpec {
        OutputRepoSpec {
            repo_name: "fa_tut_todo-list".to_string(),
            repo_description:
                "Tutorial workspace for the Todo List project with Dart / Dart / test / mocktail / no-storage / client / all / Flutter / rest-json choices."
                    .to_string(),
            project_slug: "todo-list".to_string(),
            selections: OutputRepoSelections {
                ecosystem: "dart".to_string(),
                language: "dart".to_string(),
                testing: "test".to_string(),
                mocking: "mocktail".to_string(),
                storage: "no-storage".to_string(),
                surface: "client".to_string(),
                target: "all".to_string(),
                framework: "flutter".to_string(),
                protocol: Some("rest-json".to_string()),
            },
        }
    }

    fn sample_flutter_local_output_repo_spec() -> OutputRepoSpec {
        OutputRepoSpec {
            repo_name: "fa_tut_saying-hello".to_string(),
            repo_description:
                "Tutorial workspace for the Saying Hello project with Dart / Dart / test / mocktail / no-storage / client / all / Flutter choices."
                    .to_string(),
            project_slug: "saying-hello".to_string(),
            selections: OutputRepoSelections {
                ecosystem: "dart".to_string(),
                language: "dart".to_string(),
                testing: "test".to_string(),
                mocking: "mocktail".to_string(),
                storage: "no-storage".to_string(),
                surface: "client".to_string(),
                target: "all".to_string(),
                framework: "flutter".to_string(),
                protocol: None,
            },
        }
    }

    fn sample_dotnet_output_repo_spec() -> OutputRepoSpec {
        OutputRepoSpec {
            repo_name: "fa_tut_saying-hello".to_string(),
            repo_description:
                "Tutorial workspace for the Saying Hello project with .NET / C# / xUnit / NSubstitute / no-storage / command-line / all / no-framework choices."
                    .to_string(),
            project_slug: "saying-hello".to_string(),
            selections: OutputRepoSelections {
                ecosystem: "dotnet".to_string(),
                language: "csharp".to_string(),
                testing: "xunit".to_string(),
                mocking: "nsubstitute".to_string(),
                storage: "no-storage".to_string(),
                surface: "command-line".to_string(),
                target: "all".to_string(),
                framework: "no-framework".to_string(),
                protocol: None,
            },
        }
    }

    fn app_root_for_tests() -> PathBuf {
        Path::new(ROOT)
            .parent()
            .and_then(Path::parent)
            .expect("application root")
            .to_path_buf()
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_saying_hello_back_to_dotnet() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("dotnet".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("saying-hello", &overrides)
            .expect("dotnet saying-hello should be supported");

        assert_eq!(selections.ecosystem, "dotnet");
        assert_eq!(selections.testing, "xunit");
        assert_eq!(selections.surface, "command-line");
        assert_eq!(selections.protocol, None);
    }

    #[test]
    fn output_repo_default_saying_hello_selections_are_dotnet() {
        let selections =
            output_repo_selections_for_project("saying-hello", &BootstrapSelectionOverrides::default())
                .expect("default saying-hello selections should be supported");

        assert_eq!(selections.ecosystem, "dotnet");
        assert_eq!(selections.language, "csharp");
        assert_eq!(selections.testing, "xunit");
        assert_eq!(selections.surface, "command-line");
        assert_eq!(selections.target, "all");
        assert_eq!(selections.protocol, None);
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_saying_hello_to_go() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("go".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("saying-hello", &overrides)
            .expect("go saying-hello should be supported");

        assert_eq!(selections.ecosystem, "go");
        assert_eq!(selections.language, "go");
        assert_eq!(selections.testing, "testify");
        assert_eq!(selections.surface, "web");
        assert_eq!(selections.target, "api");
        assert_eq!(selections.protocol, Some("http-json".to_string()));
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_todo_list_to_go() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("go".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("todo-list", &overrides)
            .expect("go todo-list should be supported");

        assert_eq!(selections.ecosystem, "go");
        assert_eq!(selections.language, "go");
        assert_eq!(selections.testing, "testify");
        assert_eq!(selections.mocking, "testify-mock");
        assert_eq!(selections.storage, "local-files-json");
        assert_eq!(selections.surface, "web");
        assert_eq!(selections.target, "api");
        assert_eq!(selections.framework, "echo");
        assert_eq!(selections.protocol, Some("http-json".to_string()));
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_todo_list_to_go_http_json_sqlite() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("go".to_string()),
            storage: Some("database-sqlite".to_string()),
            protocol: Some("http-json".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("todo-list", &overrides)
            .expect("go todo-list http sqlite should be supported");

        assert_eq!(selections.ecosystem, "go");
        assert_eq!(selections.storage, "database-sqlite");
        assert_eq!(selections.framework, "echo");
        assert_eq!(selections.protocol, Some("http-json".to_string()));
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_todo_list_to_go_rest_json_postgres() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("go".to_string()),
            storage: Some("database-postgres".to_string()),
            protocol: Some("rest-json".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("todo-list", &overrides)
            .expect("go todo-list rest postgres should be supported");

        assert_eq!(selections.ecosystem, "go");
        assert_eq!(selections.storage, "database-postgres");
        assert_eq!(selections.framework, "echo");
        assert_eq!(selections.protocol, Some("rest-json".to_string()));
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_saying_hello_to_astro() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("javascript".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("saying-hello", &overrides)
            .expect("astro saying-hello should be supported");

        assert_eq!(selections.ecosystem, "javascript");
        assert_eq!(selections.language, "typescript");
        assert_eq!(selections.testing, "vitest");
        assert_eq!(selections.mocking, "vitest-built-in");
        assert_eq!(selections.target, "front-end");
        assert_eq!(selections.framework, "astro");
        assert_eq!(selections.protocol, Some("http-json".to_string()));
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_saying_hello_to_flutter() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("dart".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("saying-hello", &overrides)
            .expect("flutter saying-hello should be supported");

        assert_eq!(selections.ecosystem, "dart");
        assert_eq!(selections.language, "dart");
        assert_eq!(selections.testing, "test");
        assert_eq!(selections.mocking, "mocktail");
        assert_eq!(selections.surface, "client");
        assert_eq!(selections.target, "all");
        assert_eq!(selections.framework, "flutter");
        assert_eq!(selections.protocol, Some("http-json".to_string()));
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_todo_list_to_flutter_rest_json() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("dart".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("todo-list", &overrides)
            .expect("flutter todo-list should be supported");

        assert_eq!(selections.ecosystem, "dart");
        assert_eq!(selections.language, "dart");
        assert_eq!(selections.testing, "test");
        assert_eq!(selections.mocking, "mocktail");
        assert_eq!(selections.storage, "no-storage");
        assert_eq!(selections.surface, "client");
        assert_eq!(selections.target, "all");
        assert_eq!(selections.framework, "flutter");
        assert_eq!(selections.protocol, Some("rest-json".to_string()));
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_saying_hello_to_flutter_local() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("dart".to_string()),
            surface: Some("client".to_string()),
            target: Some("all".to_string()),
            protocol: Some("none".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("saying-hello", &overrides)
            .expect("flutter local saying-hello should be supported");

        assert_eq!(selections.ecosystem, "dart");
        assert_eq!(selections.surface, "client");
        assert_eq!(selections.target, "all");
        assert_eq!(selections.framework, "flutter");
        assert_eq!(selections.protocol, None);
    }

    #[test]
    fn output_repo_selection_overrides_allow_switching_saying_hello_to_flutter_http_json() {
        let overrides = BootstrapSelectionOverrides {
            ecosystem: Some("dart".to_string()),
            surface: Some("client".to_string()),
            target: Some("all".to_string()),
            protocol: Some("http-json".to_string()),
            ..BootstrapSelectionOverrides::default()
        };

        let selections = output_repo_selections_for_project("saying-hello", &overrides)
            .expect("flutter http saying-hello should be supported");

        assert_eq!(selections.ecosystem, "dart");
        assert_eq!(selections.surface, "client");
        assert_eq!(selections.target, "all");
        assert_eq!(selections.framework, "flutter");
        assert_eq!(selections.protocol, Some("http-json".to_string()));
    }

    #[test]
    fn output_repo_root_justfile_uses_go_commands_and_run_recipe() {
        let spec = sample_output_repo_spec();

        let justfile = render_output_repo_root_justfile_content(&spec);

        assert!(justfile.contains("default:\n\t@just --list"));
        assert!(justfile.contains("\nrestore:\n"));
        assert!(justfile.contains("go mod download"));
        assert!(justfile.contains("gofmt -w"));
        assert!(justfile.contains("go test ./..."));
        assert!(justfile.contains("\nrun:\n"));
        assert!(justfile.contains("go run ./cmd/server"));
    }

    #[test]
    fn output_repo_ci_workflow_targets_workspace_go_module_with_yaml_indentation() {
        let spec = sample_output_repo_spec();

        let workflow = render_output_repo_ci_workflow_content(&spec);

        assert!(workflow.contains("\non:\n  push:\n"));
        assert!(workflow.contains("\njobs:\n  test:\n"));
        assert!(workflow.contains("uses: actions/setup-go@v5"));
        assert!(workflow.contains("hashFiles('workspace/go.mod')"));
        assert!(workflow.contains("working-directory: workspace"));
        assert!(workflow.contains("go test ./..."));
    }

    #[test]
    fn output_repo_tutorial_readme_lists_go_api_http_json_choices() {
        let spec = sample_output_repo_spec();

        let readme = render_output_repo_tutorial_readme_content(&spec);

        assert!(readme.contains("- Ecosystem: `go`"));
        assert!(readme.contains("- Target: `API`"));
        assert!(readme.contains("- Protocol: `http-json`"));
        assert!(readme.contains("- API Port: `25664`"));
        assert!(readme.contains("- App Port: `25616`"));
    }

    #[test]
    fn output_repo_setup_content_uses_go_module_and_workspace_layout() {
        let spec = sample_output_repo_spec();

        let setup = render_output_repo_setup_content(&spec);

        assert!(setup.contains("rm -rf workspace"));
        assert!(setup.contains("rm -rf workspace\n\nmkdir -p workspace"));
        assert!(setup.contains("go mod init github.com/intrepion/fa_tut_saying-hello/workspace"));
        assert!(setup.contains("go get github.com/labstack/echo/v4"));
        assert!(setup.contains("go get github.com/labstack/echo/v4/middleware"));
        assert!(setup.contains("go get github.com/stretchr/testify/assert github.com/stretchr/testify/mock"));
        assert!(setup.contains("Go.gitignore > workspace/.gitignore"));
        assert!(setup.contains("printf '\\n# Repo-local tools\\nbin/\\n\\n# Local runtime data\\ndata/\\n' >> workspace/.gitignore"));
        assert!(setup.contains("run each setup command and checkpoint it before moving to the next one"));
        assert!(!setup.contains("git commit --message \"mkdir -p workspace\""));
        assert!(!setup.contains("mkdir -p workspace\n\njust format"));
        assert!(setup.contains("git commit --message \"curl -L -s https://raw.githubusercontent.com/github/gitignore/refs/heads/main/Go.gitignore > workspace/.gitignore\""));
        assert!(setup.contains("git commit --message \"printf '"));
        assert!(setup.contains("git commit --message \"(cd workspace && go mod init github.com/intrepion/fa_tut_saying-hello/workspace)\""));
        assert!(setup.contains("just format\ngit add --all\ngit commit --message \"(cd workspace && go get github.com/labstack/echo/v4)\""));
        assert!(setup.contains("just format\ngit add --all\ngit commit --message \"(cd workspace && go get github.com/labstack/echo/v4/middleware)\""));
        assert!(setup.contains("a `workspace/.gitignore` for standard Go build output, local tooling files, and local runtime data"));
    }

    #[test]
    fn go_saying_hello_contracts_and_adapter_tutorials_are_concrete() {
        let spec = sample_output_repo_spec();

        let contracts = render_go_saying_hello_contracts_content(&spec);
        let code = render_go_saying_hello_code_content(&spec);
        let adapter = render_go_saying_hello_adapter_content(&spec);

        assert!(contracts.contains("type GreetingService interface"));
        assert!(contracts.contains("type GreetingResponse struct"));
        assert!(contracts.contains("mkdir -p workspace/internal/contracts\ntouch workspace/internal/contracts/greeting.go"));
        assert!(contracts.contains("git commit --message 'touch workspace/internal/contracts/greeting.go'"));
        assert!(contracts.contains("just format\njust check-all\ngit add --all\ngit commit --message \"Define greeting contracts\""));
        assert!(!code.contains("touch workspace/internal/code/greeting_service.go\ntouch workspace/internal/code/greeting_service_test.go"));
        assert!(code.contains("### 1. Red: Add The First Failing Code Test"));
        assert!(code.contains("mkdir -p workspace/internal/code\ntouch workspace/internal/code/greeting_service_test.go"));
        assert!(code.contains("touch workspace/internal/code/greeting_service_test.go"));
        assert!(code.contains("git commit --message 'touch workspace/internal/code/greeting_service_test.go'"));
        assert!(code.contains("### 2. Green: Return The Personalized Greeting"));
        assert!(code.contains("mkdir -p workspace/internal/code\ntouch workspace/internal/code/greeting_service.go"));
        assert!(code.contains("touch workspace/internal/code/greeting_service.go"));
        assert!(code.contains("git commit --message 'touch workspace/internal/code/greeting_service.go'"));
        assert!(adapter.contains("github.com/intrepion/fa_tut_saying-hello/workspace/internal/contracts"));
        assert!(!adapter.contains("touch workspace/internal/adapter/http/greeting_handler.go\ntouch workspace/internal/adapter/http/greeting_handler_test.go"));
        assert!(adapter.contains("Create the first adapter test file:"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/http\ntouch workspace/internal/adapter/http/greeting_handler_test.go"));
        assert!(adapter.contains("git commit --message 'touch workspace/internal/adapter/http/greeting_handler_test.go'"));
        assert!(!adapter.contains("touch workspace/cmd/server/main.go\n```\n\n### 1. Red"));
        assert!(adapter.contains("Create the server entry point:"));
        assert!(adapter.contains("mkdir -p workspace/cmd/server\ntouch workspace/cmd/server/main.go"));
        assert!(adapter.contains("git commit --message 'touch workspace/cmd/server/main.go'"));
        assert!(adapter.contains("\tservice.AssertExpectations(t)"));
        assert!(adapter.contains("middleware.CORSWithConfig"));
        assert!(adapter.contains("AllowOrigins: []string{\"http://localhost:25616\"}"));
        assert!(adapter.contains("e.GET(\"/api/greeting\", handler.GetGreeting)"));
        assert!(adapter.contains("service.On(\"Greet\", \"Ada\").Return(\"Hello, Ada!\")"));
        assert!(adapter.contains("just format"));
        assert!(adapter.contains("just check-all"));
    }

    #[test]
    fn output_repo_finish_content_uses_api_smoke_test_examples_for_saying_hello() {
        let spec = sample_output_repo_spec();

        let finish = render_output_repo_finish_content(&spec);

        assert!(finish.contains("just run"));
        assert!(finish.contains("http://localhost:25664/api/greeting?name=Ada"));
        assert!(finish.contains("http://localhost:25616"));
        assert!(finish.contains("{\"message\":\"Hello, Ada!\"}"));
    }

    #[test]
    fn todo_list_go_output_repo_tutorial_readme_lists_local_files_json_api_choices() {
        let spec = sample_todo_list_go_output_repo_spec();

        let readme = render_output_repo_tutorial_readme_content(&spec);

        assert!(readme.contains("- Project: `todo-list`"));
        assert!(readme.contains("- Testing: `testify`"));
        assert!(readme.contains("- Mocking: `testify-mock`"));
        assert!(readme.contains("- Storage: `local-files-json`"));
        assert!(readme.contains("- Surface: `web`"));
        assert!(readme.contains("- Target: `API`"));
        assert!(readme.contains("- Framework: `echo`"));
        assert!(readme.contains("- Protocol: `http-json`"));
        assert!(readme.contains("- API Port: `25664`"));
        assert!(readme.contains("- App Port: `25616`"));
    }

    #[test]
    fn todo_list_go_output_repo_setup_and_finish_content_use_local_json_api_examples() {
        let spec = sample_todo_list_go_output_repo_spec();

        let setup = render_output_repo_setup_content(&spec);
        let finish = render_output_repo_finish_content(&spec);

        assert!(setup.contains("rm -rf workspace"));
        assert!(setup.contains("go mod init github.com/intrepion/fa_tut_todo-list/workspace"));
        assert!(setup.contains("Go.gitignore > workspace/.gitignore"));
        assert!(setup.contains("printf '\\n# Repo-local tools\\nbin/\\n\\n# Local runtime data\\ndata/\\n' >> workspace/.gitignore"));
        assert!(setup.contains("mkdir -p workspace/data"));
        assert!(!setup.contains("mkdir -p workspace/data\njust format"));
        assert!(setup.contains("workspace/data/tasks.json"));
        assert!(finish.contains("http://localhost:25664/api/tasks"));
        assert!(finish.contains("workspace/data/tasks.json"));
        assert!(finish.contains("{\"tasks\":[],\"lines\":[]}"));
        assert!(finish.contains("{\"tasks\":[\"Buy milk\"],\"lines\":[\"Buy milk\"]}"));
    }

    #[test]
    fn todo_list_go_contracts_code_and_adapter_tutorials_are_concrete() {
        let spec = sample_todo_list_go_output_repo_spec();

        let contracts = render_go_todo_list_contracts_content(&spec);
        let code = render_go_todo_list_code_content(&spec);
        let adapter = render_go_todo_list_adapter_content(&spec);

        assert!(contracts.contains("type TaskStore interface"));
        assert!(contracts.contains("type TaskListService interface"));
        assert!(contracts.contains("type AddTaskRequest struct"));
        assert!(contracts.contains("mkdir -p workspace/internal/contracts\ntouch workspace/internal/contracts/task_list_service.go"));
        assert!(contracts.contains("git commit --message 'touch workspace/internal/contracts/task_list_service.go'"));
        assert!(code.contains("mkdir -p workspace/internal/code\ntouch workspace/internal/code/task_list_service_test.go"));
        assert!(code.contains("touch workspace/internal/code/task_list_service_test.go"));
        assert!(code.contains("mkdir -p workspace/internal/code\ntouch workspace/internal/code/task_list_service.go"));
        assert!(code.contains("touch workspace/internal/code/task_list_service.go"));
        assert!(code.contains("TestTaskListServiceAddTaskPersistsUpdatedTasks"));
        assert!(code.contains("NewTaskListService(store)"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/http\ntouch workspace/internal/adapter/http/task_handler_test.go"));
        assert!(adapter.contains("touch workspace/internal/adapter/http/task_handler_test.go"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/storage\ntouch workspace/internal/adapter/storage/json_task_store_test.go"));
        assert!(adapter.contains("touch workspace/internal/adapter/storage/json_task_store_test.go"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/storage\ntouch workspace/internal/adapter/storage/json_task_store.go"));
        assert!(adapter.contains("touch workspace/internal/adapter/storage/json_task_store.go"));
        assert!(adapter.contains("mkdir -p workspace/cmd/server\ntouch workspace/cmd/server/main.go"));
        assert!(adapter.contains("touch workspace/cmd/server/main.go"));
        assert!(adapter.contains("AllowOrigins: []string{\"http://localhost:25616\"}"));
        assert!(adapter.contains("e.GET(\"/api/tasks\", handler.GetTasks)"));
        assert!(adapter.contains("e.POST(\"/api/tasks\", handler.AddTask)"));
        assert!(adapter.contains("e.DELETE(\"/api/tasks\", handler.RemoveTask)"));
        assert!(adapter.contains("storageadapter.NewJSONTaskStore(\"data/tasks.json\")"));
    }

    #[test]
    fn todo_list_go_http_json_sqlite_tutorial_readme_lists_sqlite_http_choices() {
        let spec = sample_todo_list_go_http_json_sqlite_output_repo_spec();

        let readme = render_output_repo_tutorial_readme_content(&spec);

        assert!(readme.contains("- Project: `todo-list`"));
        assert!(readme.contains("- Storage: `SQLite`"));
        assert!(readme.contains("- Surface: `web`"));
        assert!(readme.contains("- Target: `API`"));
        assert!(readme.contains("- Framework: `echo`"));
        assert!(readme.contains("- Protocol: `http-json`"));
    }

    #[test]
    fn todo_list_go_rest_json_postgres_tutorial_readme_lists_postgres_rest_choices() {
        let spec = sample_todo_list_go_rest_json_postgres_output_repo_spec();

        let readme = render_output_repo_tutorial_readme_content(&spec);

        assert!(readme.contains("- Project: `todo-list`"));
        assert!(readme.contains("- Storage: `Postgres`"));
        assert!(readme.contains("- Surface: `web`"));
        assert!(readme.contains("- Target: `API`"));
        assert!(readme.contains("- Framework: `echo`"));
        assert!(readme.contains("- Protocol: `rest-json`"));
    }

    #[test]
    fn todo_list_go_rest_json_sqlite_setup_and_finish_content_use_sqlite_api_examples() {
        let spec = sample_todo_list_go_http_json_sqlite_output_repo_spec();

        let setup = render_output_repo_setup_content(&spec);
        let finish = render_output_repo_finish_content(&spec);

        assert!(setup.contains("rm -rf workspace"));
        assert!(setup.contains("go mod init github.com/intrepion/fa_tut_todo-list/workspace"));
        assert!(setup.contains("go get modernc.org/sqlite"));
        assert!(setup.contains("printf '\\n# Repo-local tools\\nbin/\\n\\n# Local runtime data\\ndata/\\n' >> workspace/.gitignore"));
        assert!(setup.contains("workspace/data/tasks.db"));
        assert!(!setup.contains("mkdir -p workspace/data\njust format"));
        assert!(finish.contains("http://localhost:25664/api/tasks"));
        assert!(finish.contains("workspace/data/tasks.db"));
        assert!(finish.contains("{\"tasks\":[]}"));
        assert!(finish.contains("{\"id\":1,\"text\":\"Buy milk\"}"));
    }

    #[test]
    fn todo_list_go_rest_json_postgres_root_justfile_setup_and_finish_use_postgres_examples() {
        let spec = sample_todo_list_go_rest_json_postgres_output_repo_spec();

        let justfile = render_output_repo_root_justfile_content(&spec);
        let setup = render_output_repo_setup_content(&spec);
        let finish = render_output_repo_finish_content(&spec);

        assert!(justfile.contains("database_url := \"postgres://postgres@localhost:5432/todo_list?sslmode=disable\""));
        assert!(justfile.contains("generate:\n\t(cd {{workspace}} && ./bin/sqlc generate)"));
        assert!(justfile.contains("just generate\n\t(cd {{workspace}} && go test ./...)"));
        assert!(justfile.contains("TODO_LIST_DATABASE_URL={{database_url}} go run ./cmd/server"));
        assert!(justfile.contains("just generate\n\t(cd {{workspace}} && TODO_LIST_DATABASE_URL={{database_url}} go run ./cmd/server)"));
        assert!(setup.contains("(cd workspace && GOBIN=$(pwd)/bin go install github.com/sqlc-dev/sqlc/cmd/sqlc@v1.30.0)"));
        assert!(setup.contains("go get github.com/jackc/pgx/v5/stdlib"));
        assert!(setup.contains("go get github.com/DATA-DOG/go-sqlmock"));
        assert!(setup.contains("printf '\\n# Repo-local tools\\nbin/\\n\\n# Local runtime data\\ndata/\\n' >> workspace/.gitignore"));
        assert!(setup.contains("touch workspace/sqlc.yaml"));
        assert!(setup.contains("mkdir -p workspace/db/query"));
        assert!(setup.contains("touch workspace/db/schema.sql"));
        assert!(setup.contains("touch workspace/db/query/tasks.sql"));
        assert!(setup.contains("version: \"2\""));
        assert!(setup.contains("CREATE TABLE IF NOT EXISTS tasks"));
        assert!(setup.contains("-- name: ListTasks :many"));
        assert!(setup.contains("git commit --message \"Add sqlc configuration and queries\""));
        assert!(setup.contains("sqlc.yaml"));
        assert!(setup.contains("db/query/tasks.sql"));
        assert!(setup.contains("workspace/bin/sqlc"));
        assert!(setup.contains("createdb --host localhost --username postgres todo_list"));
        assert!(setup.contains("TODO_LIST_DATABASE_URL"));
        assert!(setup.contains("postgres://postgres@localhost:5432/todo_list?sslmode=disable"));
        assert!(finish.contains("local Postgres database named `todo_list`"));
        assert!(finish.contains("createdb --host localhost --username postgres todo_list"));
        assert!(finish.contains("The generated `just run` and `just check-tests` commands call `sqlc generate`"));
        assert!(finish.contains("just --set database_url \"postgres://<user>:<password>@localhost:5432/<database>?sslmode=disable\" run"));
        assert!(finish.contains("http://localhost:25664/api/tasks"));
        assert!(finish.contains("{\"tasks\":[]}"));
        assert!(finish.contains("{\"id\":\"11111111-1111-1111-1111-111111111111\",\"text\":\"Buy milk\"}"));
    }

    #[test]
    fn todo_list_go_http_json_sqlite_contracts_code_and_adapter_tutorials_are_concrete() {
        let spec = sample_todo_list_go_http_json_sqlite_output_repo_spec();

        let contracts = render_go_todo_list_contracts_content(&spec);
        let code = render_go_todo_list_code_content(&spec);
        let adapter = render_go_todo_list_adapter_content(&spec);

        assert!(contracts.contains("type Task struct"));
        assert!(contracts.contains("ErrTaskTextBlank"));
        assert!(contracts.contains("type CreateTaskRequest struct"));
        assert!(contracts.contains("mkdir -p workspace/internal/contracts\ntouch workspace/internal/contracts/task_api.go"));
        assert!(code.contains("mkdir -p workspace/internal/code\ntouch workspace/internal/code/task_service_test.go"));
        assert!(code.contains("touch workspace/internal/code/task_service.go"));
        assert!(code.contains("TestTaskServiceCreateTaskRejectsBlankText"));
        assert!(code.contains("GetTask(taskID int64)"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/http\ntouch workspace/internal/adapter/http/task_handler_test.go"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/storage\ntouch workspace/internal/adapter/storage/sqlite_task_store_test.go"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/storage\ntouch workspace/internal/adapter/storage/sqlite_task_store.go"));
        assert!(adapter.contains("NewSQLiteTaskStore(\"data/tasks.db\")"));
        assert!(adapter.contains("e.GET(\"/api/tasks/:id\", handler.GetTask)"));
        assert!(adapter.contains("e.DELETE(\"/api/tasks/:id\", handler.DeleteTask)"));
        assert!(adapter.contains("http.StatusCreated"));
        assert!(adapter.contains("http.StatusNoContent"));
    }

    #[test]
    fn todo_list_go_rest_json_postgres_contracts_code_and_adapter_tutorials_are_concrete() {
        let spec = sample_todo_list_go_rest_json_postgres_output_repo_spec();

        let contracts = render_go_todo_list_contracts_content(&spec);
        let code = render_go_todo_list_code_content(&spec);
        let adapter = render_go_todo_list_adapter_content(&spec);

        assert!(contracts.contains("type Task struct"));
        assert!(contracts.contains("ErrTaskTextBlank"));
        assert!(contracts.contains("type CreateTaskRequest struct"));
        assert!(contracts.contains("mkdir -p workspace/internal/contracts\ntouch workspace/internal/contracts/task_api.go"));
        assert!(code.contains("mkdir -p workspace/internal/code\ntouch workspace/internal/code/task_service_test.go"));
        assert!(code.contains("touch workspace/internal/code/task_service.go"));
        assert!(code.contains("TestTaskServiceCreateTaskRejectsBlankText"));
        assert!(code.contains("GetTask(taskID string)"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/http\ntouch workspace/internal/adapter/http/task_handler_test.go"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/storage\ntouch workspace/internal/adapter/storage/postgres_task_store_test.go"));
        assert!(adapter.contains("mkdir -p workspace/internal/adapter/storage\ntouch workspace/internal/adapter/storage/postgres_task_store.go"));
        assert!(adapter.contains("Use the `sqlc` configuration and SQL files that you created in `tutorial/setup.md`."));
        assert!(adapter.contains("sqlc generate"));
        assert!(adapter.contains("storedb.New(db)"));
        assert!(adapter.contains("NewPostgresTaskStore(databaseURL)"));
        assert!(adapter.contains("github.com/jackc/pgx/v5/stdlib"));
        assert!(adapter.contains("github.com/DATA-DOG/go-sqlmock"));
        assert!(adapter.contains("TODO_LIST_DATABASE_URL"));
        assert!(adapter.contains("e.GET(\"/api/tasks/:id\", handler.GetTask)"));
        assert!(adapter.contains("e.DELETE(\"/api/tasks/:id\", handler.DeleteTask)"));
        assert!(adapter.contains("http.StatusCreated"));
        assert!(adapter.contains("http.StatusNoContent"));
    }

    #[test]
    fn todo_list_go_rest_json_postgres_uses_custom_rest_spec() {
        let app_root = app_root_for_tests();
        let spec = sample_todo_list_go_rest_json_postgres_output_repo_spec();
        let files = build_output_repo_tutorial_files(&app_root, &spec);

        let spec_markdown = String::from_utf8(
            files
                .iter()
                .find(|file| file.relative_path == "tutorial/spec.md")
                .expect("spec tutorial")
                .contents
                .clone(),
        )
        .expect("spec utf8");

        assert!(spec_markdown.contains("REST-style JSON routes"));
        assert!(spec_markdown.contains("database-generated UUID task ids"));
        assert!(spec_markdown.contains("`sqlc`-generated Go query code"));
        assert!(spec_markdown.contains("Postgres"));
        assert!(spec_markdown.contains("GET /api/tasks/:id"));
    }

    #[test]
    fn todo_list_go_http_json_sqlite_uses_custom_http_spec() {
        let app_root = app_root_for_tests();
        let spec = sample_todo_list_go_http_json_sqlite_output_repo_spec();
        let files = build_output_repo_tutorial_files(&app_root, &spec);

        let spec_markdown = String::from_utf8(
            files
                .iter()
                .find(|file| file.relative_path == "tutorial/spec.md")
                .expect("spec tutorial")
                .contents
                .clone(),
        )
        .expect("spec utf8");

        assert!(spec_markdown.contains("HTTP JSON routes"));
        assert!(spec_markdown.contains("database-generated numeric task ids"));
        assert!(spec_markdown.contains("GET /api/tasks/:id"));
    }

    #[test]
    fn dotnet_output_repo_tutorials_include_checkpoint_commands() {
        let app_root = app_root_for_tests();
        let spec = sample_dotnet_output_repo_spec();
        let files = build_output_repo_tutorial_files(&app_root, &spec);
        let justfile = render_output_repo_root_justfile_content(&spec);

        assert!(justfile.contains("default:\n\t@just --list"));
        let setup = String::from_utf8(
            files
                .iter()
                .find(|file| file.relative_path == "tutorial/setup.md")
                .expect("setup tutorial")
                .contents
                .clone(),
        )
        .expect("setup utf8");
        let contracts = String::from_utf8(
            files
                .iter()
                .find(|file| file.relative_path == "tutorial/contracts.md")
                .expect("contracts tutorial")
                .contents
                .clone(),
        )
        .expect("contracts utf8");
        let code = String::from_utf8(
            files
                .iter()
                .find(|file| file.relative_path == "tutorial/code.md")
                .expect("code tutorial")
                .contents
                .clone(),
        )
        .expect("code utf8");
        let adapter = String::from_utf8(
            files
                .iter()
                .find(|file| file.relative_path == "tutorial/adapter.md")
                .expect("adapter tutorial")
                .contents
                .clone(),
        )
        .expect("adapter utf8");

        assert!(justfile.contains("\nrestore:\n"));
        assert!(justfile.contains("dotnet restore {{workspace}}/{{solution}}"));
        assert!(setup.contains("rm -rf workspace"));
        assert!(!setup.contains("git commit --message \"mkdir -p workspace\""));
        assert!(!setup.contains("mkdir -p workspace\n\njust format"));
        assert!(setup.contains("dotnet new sln --format sln --name SayingHello --output workspace\njust format\ngit add --all"));
        assert!(contracts.contains("dotnet new classlib --language C# --output workspace/src/SayingHello.Contracts --name SayingHello.Contracts\njust format\njust check-all\ngit add --all\ngit commit --message 'dotnet new classlib --language C# --output workspace/src/SayingHello.Contracts --name SayingHello.Contracts'"));
        assert!(contracts.contains("dotnet sln workspace/SayingHello.sln add workspace/src/SayingHello.Contracts/SayingHello.Contracts.csproj\njust format\njust check-all\ngit add --all\ngit commit --message 'dotnet sln workspace/SayingHello.sln add workspace/src/SayingHello.Contracts/SayingHello.Contracts.csproj'"));
        assert!(contracts.contains("mkdir -p workspace/src/SayingHello.Contracts\ntouch workspace/src/SayingHello.Contracts/IGreetingService.cs"));
        assert!(contracts.contains("git commit --message 'touch workspace/src/SayingHello.Contracts/IGreetingService.cs'"));
        assert!(contracts.contains("just format\njust check-all\ngit add --all\ngit commit --message \"Define Greeting Service Contract\""));
        assert!(code.contains("dotnet new classlib --language C# --output workspace/src/SayingHello --name SayingHello\njust format\njust check-all\ngit add --all\ngit commit --message 'dotnet new classlib --language C# --output workspace/src/SayingHello --name SayingHello'"));
        assert!(code.contains("mkdir -p workspace/src/SayingHello\ntouch workspace/src/SayingHello/GreetingService.cs"));
        assert!(code.contains("git commit --message 'touch workspace/src/SayingHello/GreetingService.cs'"));
        assert!(code.contains("just format\ngit add --all\ngit commit --message 'touch workspace/src/SayingHello/GreetingService.cs'"));
        assert!(!code.contains("just format\njust check-all\ngit add --all\ngit commit --message 'touch workspace/src/SayingHello/GreetingService.cs'"));
        assert!(code.contains("just format\njust check-all\ngit add --all\ngit commit --message \"1. Red: Add The First Failing Test\""));
        assert!(adapter.contains("dotnet new console --language C# --output workspace/src/SayingHello.CommandLine --name SayingHello.CommandLine\njust format\njust check-all\ngit add --all\ngit commit --message 'dotnet new console --language C# --output workspace/src/SayingHello.CommandLine --name SayingHello.CommandLine'"));
        assert!(adapter.contains("mkdir -p workspace/src/SayingHello.CommandLine\ntouch workspace/src/SayingHello.CommandLine/CommandLineGreeting.cs"));
        assert!(adapter.contains("git commit --message 'touch workspace/src/SayingHello.CommandLine/CommandLineGreeting.cs'"));
        assert!(adapter.contains("just format\ngit add --all\ngit commit --message 'touch workspace/src/SayingHello.CommandLine/CommandLineGreeting.cs'"));
        assert!(!adapter.contains("just format\njust check-all\ngit add --all\ngit commit --message 'touch workspace/src/SayingHello.CommandLine/CommandLineGreeting.cs'"));
        assert!(adapter.contains("just format\njust check-all\ngit add --all\ngit commit --message \"1. Red: Add The First Failing Adapter Test\""));
    }

    #[test]
    fn astro_output_repo_root_justfile_uses_npm_commands_and_run_recipe() {
        let spec = sample_astro_output_repo_spec();

        let justfile = render_output_repo_root_justfile_content(&spec);

        assert!(justfile.contains("default:\n\t@just --list"));
        assert!(justfile.contains("\nrestore:\n"));
        assert!(justfile.contains("npm --prefix {{workspace}} ci"));
        assert!(justfile.contains("npm --prefix {{workspace}} run format"));
        assert!(justfile.contains("npm --prefix {{workspace}} run test"));
        assert!(justfile.contains("\nrun:\n"));
        assert!(justfile.contains("npm --prefix {{workspace}} run dev"));
    }

    #[test]
    fn flutter_output_repo_root_justfile_uses_flutter_commands_and_run_recipe() {
        let spec = sample_flutter_output_repo_spec();

        let justfile = render_output_repo_root_justfile_content(&spec);

        assert!(justfile.contains("default:\n\t@just --list"));
        assert!(justfile.contains("\nrestore:\n"));
        assert!(justfile.contains("(cd {{workspace}} && flutter pub get)"));
        assert!(justfile.contains("(cd {{workspace}} && dart format lib test integration_test)"));
        assert!(justfile.contains("(cd {{workspace}} && flutter test)"));
        assert!(justfile.contains("\nrun:\n"));
        assert!(justfile.contains("\nrun-web:\n"));
        assert!(justfile.contains("(cd {{workspace}} && flutter run -d chrome --web-port 25616 --dart-define=API_BASE_URL={{api_base_url}})"));
        assert!(justfile.contains("\nrun-ios device=\"\":\n"));
        assert!(justfile.contains("Use `just devices` and rerun with device=\"<ios-device-id-or-name>\"."));
        assert!(!justfile.contains("(cd {{workspace}} && flutter run -d ios --dart-define=API_BASE_URL={{api_base_url}})"));
        assert!(justfile.contains("\nrun-android device=\"\":\n"));
        assert!(justfile.contains("Use `just devices` and rerun with device=\"<android-device-id-or-name>\"."));
        assert!(justfile.contains("\nrun-macos:\n"));
        assert!(justfile.contains("(cd {{workspace}} && flutter run -d macos --dart-define=API_BASE_URL={{api_base_url}})"));
    }

    #[test]
    fn flutter_local_output_repo_root_justfile_runs_across_platforms_without_api_base_url() {
        let spec = sample_flutter_local_output_repo_spec();

        let justfile = render_output_repo_root_justfile_content(&spec);

        assert!(justfile.contains("default:\n\t@just --list"));
        assert!(justfile.contains("(cd {{workspace}} && flutter pub get)"));
        assert!(justfile.contains("(cd {{workspace}} && flutter test)"));
        assert!(justfile.contains("\ndevices:\n"));
        assert!(justfile.contains("\nemulators:\n"));
        assert!(justfile.contains("(cd {{workspace}} && flutter emulators)"));
        assert!(justfile.contains("\nrun-web:\n"));
        assert!(justfile.contains("(cd {{workspace}} && flutter run -d chrome --web-port 25616)"));
        assert!(justfile.contains("\nrun-ios device=\"\":\n"));
        assert!(justfile.contains("normalized_device='{{device}}'"));
        assert!(justfile.contains("normalized_device=\"${normalized_device#device=}\""));
        assert!(justfile.contains("if [ -n \"$normalized_device\" ]; then"));
        assert!(justfile.contains("(cd {{workspace}} && flutter run -d \"$normalized_device\")"));
        assert!(justfile.contains("Use `just devices` and rerun with device=\"<ios-device-id-or-name>\"."));
        assert!(!justfile.contains("(cd {{workspace}} && flutter run -d ios)"));
        assert!(justfile.contains("\nrun-android device=\"\":\n"));
        assert!(justfile.contains("\nrun-macos:\n"));
        assert!(justfile.contains("\nrun-windows:\n"));
        assert!(justfile.contains("\nrun-linux:\n"));
        assert!(!justfile.contains("api_base_url :="));
    }

    #[test]
    fn astro_output_repo_ci_workflow_targets_workspace_package_lock() {
        let spec = sample_astro_output_repo_spec();

        let workflow = render_output_repo_ci_workflow_content(&spec);

        assert!(workflow.contains("uses: actions/setup-node@v6"));
        assert!(workflow.contains("hashFiles('workspace/package-lock.json')"));
        assert!(workflow.contains("working-directory: workspace"));
        assert!(workflow.contains("run: npm ci"));
        assert!(workflow.contains("run: npm run check-formatting"));
        assert!(workflow.contains("run: npm test"));
    }

    #[test]
    fn flutter_output_repo_ci_workflow_targets_workspace_pubspec() {
        let spec = sample_flutter_output_repo_spec();

        let workflow = render_output_repo_ci_workflow_content(&spec);

        assert!(workflow.contains("uses: subosito/flutter-action@v2"));
        assert!(workflow.contains("hashFiles('workspace/pubspec.yaml')"));
        assert!(workflow.contains("run: flutter pub get"));
        assert!(workflow.contains("run: dart format --output=none --set-exit-if-changed lib test integration_test"));
        assert!(workflow.contains("run: flutter test"));
    }

    #[test]
    fn astro_output_repo_tutorial_readme_lists_front_end_http_json_choices() {
        let spec = sample_astro_output_repo_spec();

        let readme = render_output_repo_tutorial_readme_content(&spec);

        assert!(readme.contains("- Ecosystem: `JavaScript`"));
        assert!(readme.contains("- Language: `TypeScript`"));
        assert!(readme.contains("- Testing: `Vitest`"));
        assert!(readme.contains("- Mocking: `Vitest built-in`"));
        assert!(readme.contains("- Target: `front-end`"));
        assert!(readme.contains("- Framework: `Astro`"));
        assert!(readme.contains("- Protocol: `http-json`"));
        assert!(readme.contains("- API Port: `25664`"));
        assert!(readme.contains("- App Port: `25616`"));
    }

    #[test]
    fn flutter_output_repo_tutorial_readme_lists_client_http_json_choices() {
        let spec = sample_flutter_output_repo_spec();

        let readme = render_output_repo_tutorial_readme_content(&spec);

        assert!(readme.contains("- Ecosystem: `Dart`"));
        assert!(readme.contains("- Language: `Dart`"));
        assert!(readme.contains("- Unit testing: `test`"));
        assert!(readme.contains("- Widget testing: `flutter_test`"));
        assert!(readme.contains("- Integration testing: `integration_test`"));
        assert!(readme.contains("- Mocking: `mocktail`"));
        assert!(readme.contains("- Surface: `client`"));
        assert!(readme.contains("- Target: `all`"));
        assert!(readme.contains("- Framework: `Flutter`"));
        assert!(readme.contains("- Platforms: `web`, `ios`, `android`, `macos`, `windows`, `linux`"));
        assert!(readme.contains("- Protocol: `http-json`"));
        assert!(readme.contains("- API Port: `25664`"));
        assert!(readme.contains("- App Port: `25616`"));
    }

    #[test]
    fn todo_list_flutter_output_repo_tutorial_readme_lists_client_rest_json_choices() {
        let spec = sample_todo_list_flutter_output_repo_spec();

        let readme = render_output_repo_tutorial_readme_content(&spec);

        assert!(readme.contains("- Project: `todo-list`"));
        assert!(readme.contains("- Ecosystem: `Dart`"));
        assert!(readme.contains("- Language: `Dart`"));
        assert!(readme.contains("- Unit testing: `test`"));
        assert!(readme.contains("- Widget testing: `flutter_test`"));
        assert!(readme.contains("- Integration testing: `integration_test`"));
        assert!(readme.contains("- Mocking: `mocktail`"));
        assert!(readme.contains("- Surface: `client`"));
        assert!(readme.contains("- Target: `all`"));
        assert!(readme.contains("- Framework: `Flutter`"));
        assert!(readme.contains("- Protocol: `rest-json`"));
        assert!(readme.contains("- API Port: `25664`"));
        assert!(readme.contains("- App Port: `25616`"));
    }

    #[test]
    fn flutter_local_output_repo_tutorial_readme_lists_client_choices_without_protocol() {
        let spec = sample_flutter_local_output_repo_spec();

        let readme = render_output_repo_tutorial_readme_content(&spec);

        assert!(readme.contains("- Surface: `client`"));
        assert!(readme.contains("- Target: `all`"));
        assert!(readme.contains("- Platforms: `web`, `ios`, `android`, `macos`, `windows`, `linux`"));
        assert!(!readme.contains("- Protocol:"));
        assert!(!readme.contains("- API Port:"));
        assert!(readme.contains("- App Port: `25616`"));
    }

    #[test]
    fn astro_output_repo_setup_content_uses_astro_workspace_layout() {
        let spec = sample_astro_output_repo_spec();

        let setup = render_output_repo_setup_content(&spec);
        let gitignore_download = setup
            .find("curl -L -s https://raw.githubusercontent.com/github/gitignore/refs/heads/main/Node.gitignore > workspace/.gitignore")
            .expect("expected Node.gitignore download command");
        let astro_install = setup
            .find("npm install astro")
            .expect("expected astro install command");
        let format_script = setup
            .find("(cd workspace && npm pkg set scripts.format=\"prettier --write .\")")
            .expect("expected format script command");

        assert!(gitignore_download < astro_install);
        assert!(setup.contains("printf '\\n# Astro\\n.astro/\\ndist/\\n\\n# Vitest\\ncoverage/\\n' >> workspace/.gitignore"));
        assert!(setup.contains("npm install --save-dev typescript vitest jsdom prettier @types/node"));
        assert!(!setup.contains("mkdir -p workspace\njust format\ngit add --all"));
        assert!(setup.contains("git commit --message \"(cd workspace && npm init --yes)\""));
        assert!(setup.contains("(cd workspace && npm pkg set scripts.format=\"prettier --write .\")\njust format\ngit add --all"));
        assert!(format_script < setup.find("mkdir -p workspace\ntouch workspace/astro.config.mjs\njust format\ngit add --all").expect("expected formatting before touched config staging"));
        assert!(setup.contains("git commit --message \"touch workspace/astro.config.mjs\""));
        assert!(setup.contains("git commit --message \"Add Astro workspace configuration files\""));
        assert!(setup.contains("npm pkg set scripts.dev=\"astro dev --host 0.0.0.0 --port 25616\""));
        assert!(setup.contains("workspace/vitest.config.ts"));
        assert!(setup.contains("environment: 'node'"));
        assert!(setup.contains("opts into `jsdom` explicitly"));
        assert!(setup.contains("index-page.test.ts"));
        assert!(!setup.contains("touch workspace/.gitignore"));
        assert!(!setup.contains("Put this exact content in `workspace/.gitignore`:"));
    }

    #[test]
    fn flutter_output_repo_setup_content_uses_cross_platform_flutter_workspace_layout() {
        let spec = sample_flutter_output_repo_spec();

        let setup = render_output_repo_setup_content(&spec);

        assert!(setup.contains("flutter create --platforms=web,android,ios,macos,windows,linux --org com.intrepion --project-name saying_hello workspace"));
        assert!(setup.contains("rm workspace/test/widget_test.dart"));
        assert!(setup.contains("flutter pub add http"));
        assert!(setup.contains("flutter pub add --dev mocktail"));
        assert!(setup.contains("flutter pub add --dev integration_test --sdk flutter"));
        assert!(setup.contains("app_test.dart"));
        assert!(setup.contains("just run-web"));
        assert!(setup.contains("just run-ios device=\"<ios-device-id-or-name>\""));
        assert!(setup.contains("just run-android device=\"<android-device-id-or-name>\""));
        assert!(setup.contains("just run-macos"));
        assert!(setup.contains("just --set api_base_url http://localhost:25664 run-macos"));
        assert!(setup.contains("workspace/macos/Runner/DebugProfile.entitlements"));
        assert!(setup.contains("workspace/macos/Runner/Release.entitlements"));
        assert!(setup.contains("com.apple.security.network.client"));
        assert!(setup.contains("git commit --message \"Enable macOS network client entitlement\""));
        assert!(setup.contains("just run-windows"));
        assert!(setup.contains("just run-linux"));
        assert!(setup.contains("load_greeting.dart"));
        assert!(setup.contains("http_greeting_api.dart"));
        assert!(setup.contains("sudo gem install cocoapods"));
        assert!(setup.contains("open -a Simulator"));
        assert!(setup.contains("Flutter does not accept bare `ios` as a generic simulator target"));
        assert!(setup.contains("flutter emulators --launch <emulator-id>"));
        assert!(setup.contains("just --set api_base_url http://10.0.2.2:25664 run-android device=\"<android-device-id-or-name>\""));
        assert!(setup.contains("workspace/ios/Podfile.lock"));
        assert!(setup.contains("workspace/macos/Runner.xcodeproj/project.pbxproj"));
        assert!(setup.contains("workspace/macos/Runner.xcworkspace/contents.xcworkspacedata"));
        assert!(setup.contains("workspace/macos/Podfile.lock"));
        assert!(setup.contains("git add --all"));
        assert!(setup.contains("git commit --message \"Add iOS CocoaPods workspace files\""));
        assert!(setup.contains("git commit --message \"Add macOS CocoaPods workspace files\""));
        assert!(setup.contains("workspace/android/local.properties"));
        assert!(setup.contains("workspace/macos/Pods/"));
    }

    #[test]
    fn todo_list_flutter_output_repo_setup_content_uses_cross_platform_flutter_workspace_layout() {
        let spec = sample_todo_list_flutter_output_repo_spec();

        let setup = render_output_repo_setup_content(&spec);

        assert!(setup.contains("flutter create --platforms=web,android,ios,macos,windows,linux --org com.intrepion --project-name todo_list workspace"));
        assert!(setup.contains("flutter pub add http"));
        assert!(setup.contains("task_api.dart"));
        assert!(setup.contains("task_list_response.dart"));
        assert!(setup.contains("task_list_controller.dart"));
        assert!(setup.contains("http_task_api.dart"));
        assert!(setup.contains("todo_list_page.dart"));
        assert!(setup.contains("just run-ios device=\"<ios-device-id-or-name>\""));
        assert!(setup.contains("just --set api_base_url http://10.0.2.2:25664 run-android device=\"<android-device-id-or-name>\""));
        assert!(setup.contains("just --set api_base_url http://localhost:25664 run-macos"));
        assert!(setup.contains("com.apple.security.network.client"));
        assert!(setup.contains("git commit --message \"Enable macOS network client entitlement\""));
        assert!(setup.contains("git commit --message \"Add iOS CocoaPods workspace files\""));
        assert!(setup.contains("git commit --message \"Add macOS CocoaPods workspace files\""));
    }

    #[test]
    fn flutter_local_output_repo_setup_content_uses_cross_platform_flutter_workspace_layout() {
        let spec = sample_flutter_local_output_repo_spec();

        let setup = render_output_repo_setup_content(&spec);

        assert!(setup.contains("flutter create --platforms=web,android,ios,macos,windows,linux --org com.intrepion --project-name saying_hello workspace"));
        assert!(setup.contains("rm workspace/test/widget_test.dart"));
        assert!(!setup.contains("flutter pub add http"));
        assert!(setup.contains("flutter pub add --dev mocktail"));
        assert!(setup.contains("default_greeting_service.dart"));
        assert!(setup.contains("greeting_service.dart"));
        assert!(setup.contains("just run"));
        assert!(setup.contains("just run-web"));
        assert!(setup.contains("just run-ios device=\"<ios-device-id-or-name>\""));
        assert!(setup.contains("just run-android device=\"<android-device-id-or-name>\""));
        assert!(setup.contains("just run-macos"));
        assert!(setup.contains("just run-windows"));
        assert!(setup.contains("just run-linux"));
        assert!(setup.contains("sudo gem install cocoapods"));
        assert!(setup.contains("open -a Simulator"));
        assert!(setup.contains("just devices"));
        assert!(setup.contains("just emulators"));
        assert!(setup.contains("Flutter does not accept bare `ios` as a generic simulator target"));
        assert!(setup.contains("flutter emulators --launch <emulator-id>"));
        assert!(setup.contains("workspace/ios/Runner.xcodeproj/project.pbxproj"));
        assert!(setup.contains("workspace/ios/Runner.xcworkspace/contents.xcworkspacedata"));
        assert!(setup.contains("workspace/ios/Podfile.lock"));
        assert!(setup.contains("workspace/macos/Runner.xcodeproj/project.pbxproj"));
        assert!(setup.contains("workspace/macos/Runner.xcworkspace/contents.xcworkspacedata"));
        assert!(setup.contains("workspace/macos/Podfile.lock"));
        assert!(setup.contains("git add --all"));
        assert!(setup.contains("git commit --message \"Add iOS CocoaPods workspace files\""));
        assert!(setup.contains("git commit --message \"Add macOS CocoaPods workspace files\""));
        assert!(setup.contains("workspace/android/local.properties"));
        assert!(setup.contains("workspace/.gradle/"));
        assert!(setup.contains("workspace/macos/Pods/"));
    }

    #[test]
    fn astro_saying_hello_contracts_code_and_adapter_tutorials_are_concrete() {
        let spec = sample_astro_output_repo_spec();

        let contracts = render_astro_saying_hello_contracts_content(&spec);
        let code = render_astro_saying_hello_code_content(&spec);
        let adapter = render_astro_saying_hello_adapter_content(&spec);

        assert!(contracts.contains("export interface GreetingApi"));
        assert!(contracts.contains("export interface GreetingResponse"));
        assert!(contracts.contains("mkdir -p workspace/src/contracts\ntouch workspace/src/contracts/greeting-response.ts"));
        assert!(contracts.contains("git commit --message 'touch workspace/src/contracts/greeting-response.ts'"));
        assert!(contracts.contains("git commit --message 'touch workspace/src/contracts/greeting-api.ts'"));
        assert!(contracts.contains("just format\ngit add --all\ngit commit --message \"Define greeting contracts\""));
        assert!(code.contains("mkdir -p workspace/src/code\ntouch workspace/src/code/load-greeting.test.ts"));
        assert!(code.contains("touch workspace/src/code/load-greeting.test.ts"));
        assert!(code.contains("git commit --message 'touch workspace/src/code/load-greeting.test.ts'"));
        assert!(code.contains("mkdir -p workspace/src/code\ntouch workspace/src/code/load-greeting.ts"));
        assert!(code.contains("git commit --message 'touch workspace/src/code/load-greeting.ts'"));
        assert!(code.contains("### 2. Green: Return The Personalized Greeting"));
        assert!(code.contains("Sorry, the greeting API is unavailable right now."));
        assert!(adapter.contains("mkdir -p workspace/src/adapter\ntouch workspace/src/adapter/http-greeting-api.test.ts"));
        assert!(adapter.contains("touch workspace/src/adapter/http-greeting-api.test.ts"));
        assert!(adapter.contains("git commit --message 'touch workspace/src/adapter/http-greeting-api.test.ts'"));
        assert!(adapter.contains("git commit --message 'touch workspace/src/adapter/bind-greeting-form.test.ts'"));
        assert!(adapter.contains("mkdir -p workspace/src/pages\ntouch workspace/src/pages/index.astro"));
        assert!(adapter.contains("git commit --message 'touch workspace/src/pages/index.astro'"));
        assert!(adapter.contains("Create the browser binding test file:"));
        assert!(adapter.contains("// @vitest-environment jsdom"));
        assert!(adapter.contains("experimental_AstroContainer as AstroContainer"));
        assert!(adapter.contains("const apiBaseUrl = 'http://localhost:25664';"));
        assert!(adapter.contains("data-api-base-url={apiBaseUrl}"));
        assert!(adapter.contains("const apiBaseUrl = page?.getAttribute('data-api-base-url');"));
        assert!(adapter.contains("api: new HttpGreetingApi(apiBaseUrl)"));
        assert!(adapter.contains("data-api-base-url=\"http://localhost:25664\""));
        assert!(adapter.contains("just format"));
        assert!(adapter.contains("just check-all"));
    }

    #[test]
    fn flutter_local_contracts_code_and_adapter_tutorials_are_concrete() {
        let spec = sample_flutter_local_output_repo_spec();

        let contracts = render_flutter_saying_hello_contracts_content(&spec);
        let code = render_flutter_saying_hello_code_content(&spec);
        let adapter = render_flutter_saying_hello_adapter_content(&spec);
        let finish = render_output_repo_finish_content(&spec);

        assert!(contracts.contains("abstract class GreetingService"));
        assert!(contracts.contains("mkdir -p workspace/lib/contracts\ntouch workspace/lib/contracts/greeting_service.dart"));
        assert!(contracts.contains("touch workspace/lib/contracts/greeting_service.dart"));
        assert!(code.contains("mkdir -p workspace/test/code\ntouch workspace/test/code/default_greeting_service_test.dart"));
        assert!(code.contains("touch workspace/test/code/default_greeting_service_test.dart"));
        assert!(code.contains("mkdir -p workspace/lib/code\ntouch workspace/lib/code/default_greeting_service.dart"));
        assert!(code.contains("touch workspace/lib/code/default_greeting_service.dart"));
        assert!(code.contains("returns the generic greeting for empty input"));
        assert!(adapter.contains("MockGreetingService"));
        assert!(adapter.contains("GreetingPage(service: service)"));
        assert!(adapter.contains("DefaultGreetingService()"));
        assert!(!adapter.contains("HttpGreetingApi"));
        assert!(finish.contains("just run-web"));
        assert!(finish.contains("just run-ios device=\"<ios-device-id-or-name>\""));
        assert!(finish.contains("just run-android device=\"<android-device-id-or-name>\""));
        assert!(finish.contains("workspace/ios/Runner.xcodeproj/project.pbxproj"));
        assert!(finish.contains("workspace/ios/Podfile.lock"));
        assert!(finish.contains("workspace/macos/Runner.xcodeproj/project.pbxproj"));
        assert!(finish.contains("workspace/macos/Podfile.lock"));
        assert!(finish.contains("git add --all"));
        assert!(finish.contains("git commit --message \"Add iOS CocoaPods workspace files\""));
        assert!(finish.contains("git commit --message \"Add macOS CocoaPods workspace files\""));
        assert!(finish.contains("workspace/android/local.properties"));
        assert!(finish.contains("workspace/macos/Pods/"));
        assert!(!finish.contains("API is running"));
    }

    #[test]
    fn flutter_http_adapter_and_finish_use_api_base_url() {
        let spec = sample_flutter_output_repo_spec();

        let adapter = render_flutter_saying_hello_adapter_content(&spec);
        let finish = render_output_repo_finish_content(&spec);

        assert!(adapter.contains("HttpGreetingApi(baseUrl: apiBaseUrl)"));
        assert!(adapter.contains("const apiBaseUrl = String.fromEnvironment("));
        assert!(adapter.contains("mkdir -p workspace/test/adapter\ntouch workspace/test/adapter/http_greeting_api_test.dart"));
        assert!(adapter.contains("mkdir -p workspace/lib/adapter\ntouch workspace/lib/adapter/http_greeting_api.dart"));
        assert!(finish.contains("just run-web"));
        assert!(finish.contains("just run-ios device=\"<ios-device-id-or-name>\""));
        assert!(finish.contains("just --set api_base_url http://10.0.2.2:25664 run-android device=\"<android-device-id-or-name>\""));
        assert!(finish.contains("just --set api_base_url http://localhost:25664 run-macos"));
        assert!(finish.contains("workspace/ios/Runner.xcodeproj/project.pbxproj"));
        assert!(finish.contains("workspace/ios/Podfile.lock"));
        assert!(finish.contains("workspace/macos/Runner.xcodeproj/project.pbxproj"));
        assert!(finish.contains("workspace/macos/Podfile.lock"));
        assert!(finish.contains("git add --all"));
        assert!(finish.contains("git commit --message \"Add iOS CocoaPods workspace files\""));
        assert!(finish.contains("git commit --message \"Add macOS CocoaPods workspace files\""));
        assert!(finish.contains("workspace/android/local.properties"));
        assert!(finish.contains("workspace/macos/Pods/"));
        assert!(finish.contains("matching Saying Hello API is running"));
    }

    #[test]
    fn todo_list_flutter_contracts_code_adapter_and_finish_are_concrete() {
        let spec = sample_todo_list_flutter_output_repo_spec();

        let contracts = render_flutter_todo_list_rest_json_contracts_content(&spec);
        let code = render_flutter_todo_list_rest_json_code_content(&spec);
        let adapter = render_flutter_todo_list_rest_json_adapter_content(&spec);
        let finish = render_output_repo_finish_content(&spec);

        assert!(contracts.contains("class Task {"));
        assert!(contracts.contains("abstract class TaskApi"));
        assert!(contracts.contains("class TaskListResponse"));
        assert!(contracts.contains("mkdir -p workspace/lib/contracts\ntouch workspace/lib/contracts/task.dart"));
        assert!(contracts.contains("touch workspace/lib/contracts/task_list_response.dart"));
        assert!(contracts.contains("touch workspace/lib/contracts/task_api.dart"));
        assert!(code.contains("mkdir -p workspace/test/code\ntouch workspace/test/code/task_list_controller_test.dart"));
        assert!(code.contains("touch workspace/test/code/task_list_controller_test.dart"));
        assert!(code.contains("mkdir -p workspace/lib/code\ntouch workspace/lib/code/task_list_controller.dart"));
        assert!(code.contains("touch workspace/lib/code/task_list_controller.dart"));
        assert!(code.contains("Task must not be blank."));
        assert!(code.contains("Sorry, the task API is unavailable right now."));
        assert!(code.contains("api.createTask('Buy milk')"));
        assert!(code.contains("api.deleteTask('22222222-2222-2222-2222-222222222222')"));
        assert!(adapter.contains("mkdir -p workspace/test/adapter\ntouch workspace/test/adapter/http_task_api_test.dart"));
        assert!(adapter.contains("touch workspace/test/adapter/http_task_api_test.dart"));
        assert!(adapter.contains("touch workspace/test/adapter/todo_list_page_test.dart"));
        assert!(adapter.contains("mkdir -p workspace/lib/adapter\ntouch workspace/lib/adapter/http_task_api.dart"));
        assert!(adapter.contains("touch workspace/lib/adapter/http_task_api.dart"));
        assert!(adapter.contains("touch workspace/lib/adapter/todo_list_page.dart"));
        assert!(adapter.contains("expect(requestedUri.toString(), 'http://localhost:25664/api/tasks/77777777-7777-7777-7777-777777777777');"));
        assert!(adapter.contains("body: jsonEncode({'text': text})"));
        assert!(adapter.contains("Uri.parse('$baseUrl/api/tasks/$taskId')"));
        assert!(adapter.contains("Future<void> deleteTask(String taskId) async"));
        assert!(adapter.contains("Key('remove-${task.id}')"));
        assert!(adapter.contains("HttpTaskApi(baseUrl: apiBaseUrl)"));
        assert!(adapter.contains("TodoListPage(api: api)"));
        assert!(finish.contains("matching Todo List API is running"));
        assert!(finish.contains("just --set api_base_url http://10.0.2.2:25664 run-android device=\"<android-device-id-or-name>\""));
        assert!(finish.contains("just --set api_base_url http://localhost:25664 run-macos"));
        assert!(finish.contains("git commit --message \"Add macOS CocoaPods workspace files\""));
    }

    #[test]
    fn astro_output_repo_finish_content_uses_front_end_and_api_ports() {
        let spec = sample_astro_output_repo_spec();

        let finish = render_output_repo_finish_content(&spec);

        assert!(finish.contains("http://localhost:25664"));
        assert!(finish.contains("http://localhost:25616"));
        assert!(finish.contains("Hello, Ada!"));
        assert!(finish.contains("Sorry, the greeting API is unavailable right now."));
    }

    #[test]
    fn starter_gitignore_contains_source_markers_for_each_global_section() {
        let gitignore = starter_gitignore_content("dotnet");

        assert!(gitignore.contains(
            "#### START https://github.com/github/gitignore/blob/main/Global/Linux.gitignore"
        ));
        assert!(gitignore.contains(
            "#### END https://github.com/github/gitignore/blob/main/Global/macOS.gitignore"
        ));
        assert!(gitignore.contains(
            "#### START https://github.com/github/gitignore/blob/main/Global/Windows.gitignore"
        ));
    }
}
