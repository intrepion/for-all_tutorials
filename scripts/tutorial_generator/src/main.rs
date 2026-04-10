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
        } => {
            bootstrap_output_repos(
                &app_root,
                repos_root,
                owner,
                sync_branch_name.as_deref(),
                project.as_deref(),
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
    },
    CleanupOutputRepos {
        repos_root: PathBuf,
        owner: String,
        apply: bool,
    },
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
    ecosystem: String,
    project_slug: String,
}

fn collect_output_repo_specs(app_root: &Path) -> Result<Vec<OutputRepoSpec>, AppError> {
    collect_output_repo_specs_for_project(app_root, None)
}

fn collect_output_repo_specs_for_project(
    app_root: &Path,
    project_filter: Option<&str>,
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

        specs.push(OutputRepoSpec {
            repo_name: output_repo_name(&manifest.project),
            repo_description: output_repo_description(&project_title),
            ecosystem: "dotnet".to_string(),
            project_slug: manifest.project.clone(),
        });
    }

    specs.sort_by(|left, right| left.repo_name.cmp(&right.repo_name));
    Ok(specs)
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
) -> Result<(), AppError> {
    let specs = collect_output_repo_specs_for_project(app_root, project_filter)?;
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
            contents: starter_gitignore_content(&spec.ecosystem).into_bytes(),
        },
    ];

    if spec.ecosystem == "dotnet" {
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
    let solution_name = workspace_solution_name(&spec.project_slug);
    format!(
        "set shell := [\"bash\", \"-eu\", \"-c\"]\n\n\
         workspace := \"workspace\"\n\
         solution := \"{solution_name}.sln\"\n\n\
         format:\n\
         \tcd {{{{workspace}}}} && dotnet format {{{{solution}}}}\n\n\
         check-formatting:\n\
         \tcd {{{{workspace}}}} && dotnet format {{{{solution}}}} --verify-no-changes\n\n\
         check-tests:\n\
         \tcd {{{{workspace}}}} && dotnet test {{{{solution}}}}\n\n\
         check-all:\n\
         \tjust check-formatting\n\
         \tjust check-tests\n"
    )
}

fn render_output_repo_ci_workflow_content(spec: &OutputRepoSpec) -> String {
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
        tutorial_file_markdown(
            "Contracts",
            &format!(
                "{}\n\n{}",
                render_output_repo_contracts_scaffold(spec),
                rewrite_for_single_repo_tutorial(&partial.body)
            ),
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
                &format!(
                    "{}\n\n{}",
                    render_output_repo_code_scaffold(spec),
                    rewrite_for_single_repo_tutorial(&code_partial.body)
                ),
            )
            .into_bytes(),
        },
        ManagedRepoFile {
            relative_path: "tutorial/adapter.md".to_string(),
            contents: tutorial_file_markdown(
                "Adapter",
                &format!(
                    "{}\n\n{}",
                    render_output_repo_adapter_scaffold(spec),
                    rewrite_for_single_repo_tutorial(&adapter_partial.body)
                ),
            )
            .into_bytes(),
        },
    ]
}

fn render_output_repo_tutorial_readme_content(spec: &OutputRepoSpec) -> String {
    format!(
        "# Tutorial\n\nChoices for this repo:\n\n- Project: `{}`\n- Workspace folder: `workspace/`\n- Ecosystem: `.NET`\n- Language: `C#`\n- Testing: `xUnit`\n- Mocking: `NSubstitute`\n- Storage: `no-storage`\n- Surface: `command-line`\n- Target: `all`\n- Framework: `no-framework`\n\nWork through these files in order:\n\n1. [Spec](spec.md)\n2. [Setup](setup.md)\n3. [Contracts](contracts.md)\n4. [Code](code.md)\n5. [Adapter](adapter.md)\n",
        spec.project_slug
    )
}

fn render_output_repo_setup_content(spec: &OutputRepoSpec) -> String {
    let solution_name = workspace_solution_name(&spec.project_slug);
    let contracts_project_name = contracts_project_name(&spec.project_slug);
    let code_project_name = code_project_name(&spec.project_slug);
    let code_test_project_name = code_test_project_name(&spec.project_slug);
    let adapter_project_name = adapter_project_name(&spec.project_slug);
    let adapter_test_project_name = adapter_test_project_name(&spec.project_slug);

    tutorial_file_markdown(
        "Setup",
        &format!(
            "Keep the repository root for shared files like `README.md`, `LICENSE`, `.gitignore`, `.github/`, and `tutorial/`.\n\nPut all .NET code inside a single `workspace/` folder.\n\nFrom the repository root, run:\n\n```bash\nmkdir -p workspace\ndotnet new sln --format sln --name {solution_name} --output workspace\ndotnet new gitignore --output workspace\n```\n\nAfter those commands finish, stay in the repository root for the rest of this tutorial sequence unless a step explicitly says otherwise.\n\nThis gives you:\n\n- a root-level `.gitignore` for operating-system noise and editor leftovers\n- a `workspace/.gitignore` for standard `.NET` build output and local tooling files\n\nWhen the full workspace is finished, it should contain these projects:\n\n- `workspace/src/{contracts_project_name}`\n- `workspace/src/{code_project_name}`\n- `workspace/tests/{code_test_project_name}`\n- `workspace/src/{adapter_project_name}`\n- `workspace/tests/{adapter_test_project_name}`\n\nThe next files assume this layout:\n\n```text\nworkspace/\n  .gitignore\n  {solution_name}.sln\n  src/\n    {contracts_project_name}/\n    {code_project_name}/\n    {adapter_project_name}/\n  tests/\n    {code_test_project_name}/\n    {adapter_test_project_name}/\n```"
        ),
    )
}

fn tutorial_file_markdown(title: &str, body: &str) -> String {
    format!("# {title}\n\n{}\n", normalize_text(body))
}

fn generic_contracts_tutorial(spec: &OutputRepoSpec, spec_body: &str) -> String {
    let contract_section = extract_section(spec_body, "Core Logic Contract").unwrap_or_else(|| {
        "Define the shared interfaces, request and response types, enums, and small value objects that both the code layer and adapter layer need.".to_string()
    });

    tutorial_file_markdown(
        "Contracts",
        &format!(
            "{}\n\nUse this file to define the shared contracts that the code layer implements and the adapter layer depends on.\n\nDo not add tests here. Keep this layer limited to interfaces, request and response types, enums, and small shared value objects.\n\n## Core Logic Contract\n\n{}",
            render_output_repo_contracts_scaffold(spec),
            normalize_text(&contract_section)
        ),
    )
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

    format!(
        "Create the contracts library inside `workspace/` first.\n\nFrom the repository root, run:\n\n```bash\ndotnet new classlib --language C# --output {project_path} --name {project_name}\ndotnet sln workspace/{solution_name}.sln add {project_path}/{project_name}.csproj\n```\n\nAfter those commands finish, stay in the repository root and continue with the rest of this file."
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

    format!(
        "Create the code library and its test library inside `workspace/`.\n\nBoth the code library and the code test library should reference the contracts library.\n\nFrom the repository root, run:\n\n```bash\ndotnet new classlib --language C# --output {library_project_path} --name {library_project_name}\ndotnet new xunit --language C# --output {test_project_path} --name {test_project_name}\ndotnet sln workspace/{solution_name}.sln add {library_project_path}/{library_project_name}.csproj\ndotnet sln workspace/{solution_name}.sln add {test_project_path}/{test_project_name}.csproj\ndotnet add {library_project_path}/{library_project_name}.csproj reference {contracts_project_path}\ndotnet add {test_project_path}/{test_project_name}.csproj reference {contracts_project_path}\ndotnet add {test_project_path}/{test_project_name}.csproj reference {library_project_path}/{library_project_name}.csproj\n```\n\nAfter those commands finish, stay in the repository root and continue with the rest of this file."
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

    format!(
        "Create the adapter library and its test library inside `workspace/`.\n\nBoth the adapter library and the adapter test library should reference the contracts library. The adapter library should also reference the code library.\n\nFrom the repository root, run:\n\n```bash\ndotnet new console --language C# --output {adapter_project_path} --name {adapter_project_name}\ndotnet new xunit --language C# --output {adapter_test_project_path} --name {adapter_test_project_name}\ndotnet sln workspace/{solution_name}.sln add {adapter_project_path}/{adapter_project_name}.csproj\ndotnet sln workspace/{solution_name}.sln add {adapter_test_project_path}/{adapter_test_project_name}.csproj\ndotnet add {adapter_project_path}/{adapter_project_name}.csproj reference {contracts_project_path}\ndotnet add {adapter_test_project_path}/{adapter_test_project_name}.csproj reference {contracts_project_path}\ndotnet add {adapter_project_path}/{adapter_project_name}.csproj reference {code_project_path}\ndotnet add {adapter_test_project_path}/{adapter_test_project_name}.csproj reference {adapter_project_path}/{adapter_project_name}.csproj\n```\n\nAfter those commands finish, stay in the repository root and continue with the rest of this file."
    )
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
         git commit -m \"Create the Solution and Projects\"\n\
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
         git commit -m \"Create the Contracts Solution and Project\"\n\
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
         git commit -m \"Create the Adapter Solution and Projects\"\n\
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
             git commit -m \"Add Root Justfile\"\n\
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
             git commit -m \"Add Root Justfile\"\n\
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
             git commit -m \"Add Root Justfile\"\n\
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
        "format:\n\
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
            "format:\n\
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
        "format:\n\
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

fn output_repo_description(project_title: &str) -> String {
    format!(
        "Tutorial workspace for the {project_title} project with default .NET/C# command-line choices."
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
