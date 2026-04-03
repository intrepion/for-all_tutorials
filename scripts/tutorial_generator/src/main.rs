use askama::Template;
use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

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
        generate_from_manifest(&app_root, &args.output_root, &shared_projects, &manifest_path)?;
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
                    let path = args.next().ok_or_else(|| {
                        AppError::message("missing value for --output-root")
                    })?;
                    output_root = absolute_path(app_root, Path::new(&path));
                }
                other => {
                    return Err(AppError::message(format!(
                        "unknown argument: {other}"
                    )))
                }
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
    LanguagePartial,
    ToolchainItem,
    TestingPartial,
    AdapterPartial,
    FrameworkPartial,
    TestingIndex,
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
        let (frontmatter_text, markdown) = split_frontmatter(&raw)
            .ok_or_else(|| AppError::message(format!("missing frontmatter in {}", path.display())))?;
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

    let project_root = find_partial(partials, PartialKind::ProjectRoot)?;
    let spec = find_partial(partials, PartialKind::ProjectSpec)?;
    let instructions_index = find_partial(partials, PartialKind::InstructionsIndex)?;
    let role_instructions = find_partial(
        partials,
        if output.kind == OutputKind::Core {
            PartialKind::CoreInstructions
        } else {
            PartialKind::AdapterInstructions
        },
    )?;
    let ecosystem_root = find_optional_partial(partials, PartialKind::EcosystemRoot);
    let storage_root = find_optional_partial(partials, PartialKind::StorageRoot);

    let mut sections = Vec::new();
    push_section(
        &mut sections,
        "Shared Workflow",
        extract_section(&shared_projects.body, "Shared Workflow"),
    );
    push_section(
        &mut sections,
        "Shared Output Model",
        extract_section(&shared_projects.body, "Shared Output Model"),
    );
    push_section(
        &mut sections,
        "Shared Repository Creation",
        extract_section(&shared_projects.body, "Shared Repository Creation"),
    );
    push_section(
        &mut sections,
        "Shared Coverage Policy",
        extract_section(&shared_projects.body, "Shared Coverage Policy"),
    );
    push_section(
        &mut sections,
        "Project Purpose",
        extract_section(&project_root.body, "Purpose"),
    );
    push_section(
        &mut sections,
        "Project Spec",
        Some(shift_headings(&spec.body, 1)),
    );
    push_section(
        &mut sections,
        "Project-Specific Flow",
        extract_section(&instructions_index.body, "Project-Specific Flow"),
    );
    push_section(
        &mut sections,
        "Setup Overview",
        ecosystem_root.map(|partial| intro_excerpt(&partial.body)),
    );

    for partial in partials.iter().filter(|partial| partial.meta.partial_kind == PartialKind::LanguagePartial)
    {
        push_subsection(&mut sections, &partial.title, &partial.body, 0);
    }
    for partial in partials.iter().filter(|partial| partial.meta.partial_kind == PartialKind::ToolchainItem)
    {
        push_subsection(&mut sections, &partial.title, &partial.body, 0);
    }
    for partial in partials.iter().filter(|partial| partial.meta.partial_kind == PartialKind::TestingPartial)
    {
        push_subsection(&mut sections, &partial.title, &partial.body, 2);
    }
    if output.kind == OutputKind::Adapter {
        push_section(
            &mut sections,
            "Storage",
            storage_root.map(|partial| partial.body.clone()),
        );
    }
    for partial in partials.iter().filter(|partial| partial.meta.partial_kind == PartialKind::AdapterPartial)
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

fn push_subsection(sections: &mut Vec<RenderedSection>, heading: &str, body: &str, heading_shift: usize) {
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

fn intro_excerpt(markdown_body: &str) -> String {
    let lines: Vec<&str> = markdown_body.lines().collect();
    let cutoff = lines
        .iter()
        .position(|line| line.starts_with("## "))
        .unwrap_or(lines.len());
    lines[..cutoff].join("\n").trim().to_string()
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
        let label = captures.get(1).map(|value| value.as_str()).unwrap_or_default();
        let target = captures.get(2).map(|value| value.as_str()).unwrap_or_default();
        if target.starts_with("http://") || target.starts_with("https://") || target.starts_with('#') {
            captures.get(0).map(|value| value.as_str()).unwrap_or_default().to_string()
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
            "for-all_tutorial_manual_{}_{}_{}_{}_core",
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
            output.selections.testing,
            output.selections.storage.as_deref().unwrap_or("no-storage"),
            output.selections.surface.as_deref().unwrap_or("unknown-surface"),
            output.selections.target.as_deref().unwrap_or("unknown-target"),
            output.selections.framework.as_deref().unwrap_or("unknown-framework"),
        )
    }
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

fn format_selection_value(value: &str) -> String {
    match value {
        "dotnet" => ".NET".to_string(),
        "csharp" => "C#".to_string(),
        "xunit" => "xUnit".to_string(),
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
