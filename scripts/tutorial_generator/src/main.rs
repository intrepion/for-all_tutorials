use askama::Template;
use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const GITHUB_OWNER: &str = "intrepion";

fn main() -> Result<(), AppError> {
    let app_root = Path::new(ROOT)
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| AppError::message("unable to resolve application root"))?
        .to_path_buf();

    let args = Args::parse(&app_root)?;
    let shared_projects = Partial::load(&app_root.join("partials/projects/README.md"))?;
    let manifest_paths = collect_manifest_paths(&app_root)?;

    for manifest_path in manifest_paths {
        generate_from_manifest(
            &app_root,
            &args.output_root,
            &shared_projects,
            &manifest_path,
        )?;
    }

    Ok(())
}

#[derive(Debug)]
struct Args {
    output_root: PathBuf,
}

impl Args {
    fn parse(app_root: &Path) -> Result<Self, AppError> {
        let mut output_root = app_root.join("tutorials");
        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--output-root" => {
                    let path = args
                        .next()
                        .ok_or_else(|| AppError::message("missing value for --output-root"))?;
                    output_root = absolute_path(app_root, Path::new(&path));
                }
                other => return Err(AppError::message(format!("unknown argument: {other}"))),
            }
        }

        Ok(Self { output_root })
    }
}

fn absolute_path(base: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
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

    for output in &manifest.compiled_outputs {
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

#[derive(Debug, Deserialize)]
struct CompiledOutput {
    #[allow(dead_code)]
    id: String,
    kind: OutputKind,
    tutorial_path: String,
    selections: Selections,
    sources: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum OutputKind {
    Core,
    Adapter,
}

#[derive(Debug, Deserialize)]
struct Selections {
    ecosystem: String,
    language: String,
    testing: String,
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
        KeyValue {
            key: "Testing".to_string(),
            value: format_selection_value(&output.selections.testing),
        },
    ];

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
    let role_instructions = find_partial(
        partials,
        if output.kind == OutputKind::Core {
            PartialKind::CoreInstructions
        } else {
            PartialKind::AdapterInstructions
        },
    )?;
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
    push_section(
        &mut sections,
        &role_instructions.title,
        Some(role_instructions.body.clone()),
    );
    if output.kind == OutputKind::Core {
        if let Some(ci_partial) = ci_partial {
            push_section(
                &mut sections,
                &ci_partial.title,
                Some(ci_partial.body.clone()),
            );
        }
    }
    push_section(
        &mut sections,
        "Shared Finish Checklist",
        extract_section(&shared_projects.body, "Shared Finish Checklist"),
    );

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

    if output.kind == OutputKind::Adapter {
        body.push_str(&format!(
            "\n- keep this adapter repo next to the matching core repo working copy so this local reference shape works:\n  `{}`",
            format!("../{}", core_repo_name(project_slug, output))
        ));
    }

    Some(body)
}

fn render_root_readme(repo_name: &str, repo_description: &str) -> String {
    let workflow_url = format!(
        "https://github.com/{GITHUB_OWNER}/{repo_name}/actions/workflows/ci.yml"
    );
    let badge_url = format!("{workflow_url}/badge.svg");

    format!(
        "Create the file:\n\n\
         ```bash\n\
         touch README.md\n\
         ```\n\n\
         Then put this exact content in `README.md`:\n\n\
         ```md\n\
         # {repo_name}\n\
         {repo_description}\n\n\
         [![CI]({badge_url})]({workflow_url})\n\
         ```"
    )
}

fn recommended_dotnet_core_scaffold(project_slug: &str, output: &CompiledOutput) -> Option<String> {
    if output.kind != OutputKind::Core || output.selections.ecosystem != "dotnet" {
        return None;
    }

    let (test_template, template_install_command) =
        dotnet_test_template_short_name(&output.selections.testing);
    let solution_name = pascal_case_slug(project_slug);
    let library_project_name = solution_name.clone();
    let test_project_name = format!("{solution_name}.Tests");
    let solution_file = format!("{solution_name}.sln");
    let library_project_path = format!("src/{library_project_name}");
    let test_project_path = format!("tests/{test_project_name}");
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
         Use these names and paths, then run:\n\n\
         ```bash\n\
         dotnet new sln --format sln --name {solution_name}\n\
         dotnet new gitignore\n\
         {template_install_line}\
         dotnet new classlib --name {library_project_name} --output {library_project_path}\n\
         dotnet new {test_template} --name {test_project_name} --output {test_project_path}\n\
         dotnet sln {solution_file} add {library_project_path}/{library_project_name}.csproj\n\
         dotnet sln {solution_file} add {test_project_path}/{test_project_name}.csproj\n\
         dotnet add {test_project_path}/{test_project_name}.csproj reference {library_project_path}/{library_project_name}.csproj\n\
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
        dotnet_test_template_short_name(&output.selections.testing);
    let core_repo_name = core_repo_name(project_slug, output);
    let core_library_project_name = pascal_case_slug(project_slug);
    let adapter_name = format!("{}.CommandLine", pascal_case_slug(project_slug));
    let adapter_test_project_name = format!("{adapter_name}.Tests");
    let solution_name = adapter_name.clone();
    let solution_file = format!("{solution_name}.sln");
    let adapter_project_path = format!("src/{adapter_name}");
    let adapter_test_project_path = format!("tests/{adapter_test_project_name}");
    let core_project_reference_path = format!(
        "../{core_repo_name}/src/{core_library_project_name}/{core_library_project_name}.csproj"
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
         - Local core repo assumption: sibling checkout at `../{core_repo_name}`\n\
         - Local core project reference path: `{core_project_reference_path}`\n\n\
         Use these names and paths, then run:\n\n\
         ```bash\n\
         dotnet new sln --format sln --name {solution_name}\n\
         dotnet new gitignore\n\
         {template_install_line}\
         dotnet new console --name {adapter_name} --output {adapter_project_path}\n\
         dotnet new {test_template} --name {adapter_test_project_name} --output {adapter_test_project_path}\n\
         dotnet sln {solution_file} add {adapter_project_path}/{adapter_name}.csproj\n\
         dotnet sln {solution_file} add {adapter_test_project_path}/{adapter_test_project_name}.csproj\n\
         dotnet add {adapter_project_path}/{adapter_name}.csproj reference {core_project_reference_path}\n\
         dotnet add {adapter_test_project_path}/{adapter_test_project_name}.csproj reference {adapter_project_path}/{adapter_name}.csproj\n\
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

    let test_project_path = dotnet_test_project_csproj_path(project_slug, output)?;
    let intro = justfile_partial
        .map(|partial| normalize_text(&partial.body))
        .filter(|body| !body.is_empty())
        .unwrap_or_else(|| {
            "Add a repo-root `justfile` so local developer commands and CI checks use the same entry points."
                .to_string()
        });

    let body = if output.selections.testing == "tunit" {
        format!(
            "{intro}\n\n\
             Create the file:\n\n\
             ```bash\n\
             touch justfile\n\
             ```\n\n\
             Then put this exact content in `justfile`:\n\n\
             ```just\n\
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
             \tjust check-branch-cover\n\
             ```\n\n\
             For `TUnit`, both coverage checks collect one Cobertura report with Microsoft.Testing.Platform, then validate either the line-rate or the branch-rate from that same report."
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
             \tjust check-branch-cover\n\
             ```"
        )
    };

    Some(body)
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

fn repo_name(project_slug: &str, output: &CompiledOutput) -> String {
    if output.kind == OutputKind::Core {
        format!(
            "for-all_tutorial_manual_{}_{}_{}_core_{}",
            project_slug,
            output.selections.ecosystem,
            output.selections.language,
            output.selections.testing
        )
    } else {
        format!(
            "for-all_tutorial_manual_{}_{}_{}_{}_{}_{}_{}_{}",
            project_slug,
            output.selections.ecosystem,
            output.selections.language,
            output.selections.storage.as_deref().unwrap_or("no-storage"),
            output
                .selections
                .surface
                .as_deref()
                .unwrap_or("unknown-surface"),
            output
                .selections
                .target
                .as_deref()
                .unwrap_or("unknown-target"),
            output
                .selections
                .framework
                .as_deref()
                .unwrap_or("unknown-framework"),
            output.selections.testing,
        )
    }
}

fn core_repo_name(project_slug: &str, output: &CompiledOutput) -> String {
    format!(
        "for-all_tutorial_manual_{}_{}_{}_core_{}",
        project_slug,
        output.selections.ecosystem,
        output.selections.language,
        output.selections.testing
    )
}

fn repo_description(project_title: &str, output: &CompiledOutput) -> String {
    if output.kind == OutputKind::Core {
        format!(
            "Manual spec-driven, test-driven core library for the {project_title} tutorial in {}/{} with {}.",
            format_selection_value(&output.selections.ecosystem),
            format_selection_value(&output.selections.language),
            format_selection_value(&output.selections.testing)
        )
    } else {
        format!(
            "Manual {}/{} {} adapter for the {project_title} tutorial in {}/{} with {}, consuming a separately tested core library.",
            format_selection_value(output.selections.surface.as_deref().unwrap_or("surface")),
            format_selection_value(output.selections.target.as_deref().unwrap_or("target")),
            format_selection_value(output.selections.framework.as_deref().unwrap_or("framework")),
            format_selection_value(&output.selections.ecosystem),
            format_selection_value(&output.selections.language),
            format_selection_value(&output.selections.testing)
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
