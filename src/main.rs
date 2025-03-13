use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufWriter, Cursor, Read, Write},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use clap::Parser;
use color_eyre::{Result, eyre::Context};
use lazy_regex::regex_replace_all;
use sha2::{Digest as _, Sha256};

#[derive(Debug, Parser)]
#[clap(about, version)]
struct Options {
    /// The target entrypoint to bundle.
    /// Recursively follows files starting from this file.
    #[clap(default_value = "index.js")]
    target: PathBuf,

    /// The output file to write the bundle to.
    #[clap(default_value = "index.bundle.js")]
    output: PathBuf,
}

/// The working directory for the bundling operation.
#[derive(Debug, Clone)]
struct Cwd(PathBuf);

impl Cwd {
    /// Join the path.
    fn join(&self, path: impl AsRef<Path>) -> PathBuf {
        self.0.join(path)
    }

    /// Create a new working directory from the given path.
    fn from_root(path: impl AsRef<Path>) -> Self {
        static CWD: LazyLock<Cwd> = LazyLock::new(|| Cwd(std::env::current_dir().expect("cwd")));
        Self::derived(&CWD, path)
    }

    /// Create a derived working directory.
    fn derived(cwd: &Self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            cwd.0.join(path)
        };

        if path.is_dir() {
            Self(path)
        } else {
            let parent = path.parent().expect("no parent");
            Self(parent.to_path_buf())
        }
    }
}
/// A file that another file imports.
#[derive(Debug, Clone)]
struct Import {
    /// The path to the imported file.
    path: PathBuf,

    /// The digest of the imported file.
    digest: Digest,
}

impl Import {
    /// Resolve the raw import if possible.
    /// Returns `None` if the file doesn't exist; this is typical for node builtins.
    fn resolve(cwd: &Cwd, raw: &str) -> Result<Import> {
        let path = cwd.join(raw);
        let path = if !path.ends_with(".js") {
            path.with_extension("js")
        } else {
            path
        }
        .canonicalize()
        .context("canonicalize")?;

        let digest = Digest::from_content_of(&path).context("hash")?;
        eprintln!("resolve import {raw} -> {path:?} as {}", digest.to_hex());
        Ok(Import { path, digest })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Digest(Vec<u8>);

impl Digest {
    /// Render the digest to hex.
    fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }

    /// Create a new digest from the given buffer.
    fn from_buffer(buffer: impl AsRef<[u8]>) -> Self {
        let reader = Cursor::new(buffer.as_ref());
        Self::from_reader(reader).unwrap()
    }

    /// Create a new digest from the given content reader.
    fn from_reader(mut reader: impl Read) -> Result<Self> {
        let mut hasher = Sha256::new();
        std::io::copy(&mut reader, &mut hasher).context("hash content")?;
        Ok(Self(hasher.finalize().to_vec()))
    }

    /// Create a new digest of the content of the file at the given path.
    fn from_content_of(path: &Path) -> Result<Self> {
        let file = File::open(path).context("open file")?;
        Self::from_reader(file)
    }
}

#[derive(Debug, Clone)]
struct JsFile {
    content: String,
    digest: Digest,
    imports: Vec<Import>,
}

impl JsFile {
    /// Read and parse an instance from disk.
    fn read(cwd: &Cwd, path: &Path) -> Result<Self> {
        eprintln!("reading {path:?}");
        let content = std::fs::read_to_string(path).context("read file content")?;
        let digest = Digest::from_buffer(&content);
        let (content, imports) = Self::parse_imports(cwd, &content)?;

        Ok(Self {
            content,
            digest,
            imports,
        })
    }

    /// Parse the imports (the targets of `require()` calls) from the file content.
    /// Replaces the actual import string in the JS with the hash of the file being imported.
    fn parse_imports(cwd: &Cwd, content: &str) -> Result<(String, Vec<Import>)> {
        let mut imports = Vec::new();
        let mut err = None;

        let mut derive_replacement = |raw: &str| -> Result<String> {
            let import = Import::resolve(cwd, raw).context("resolve import")?;
            let hex = import.digest.to_hex();
            imports.push(import);
            Ok(format!("require('{hex}')"))
        };

        let replaced = regex_replace_all!(r#"require\('([^']+)'\)"#, content, |_, raw| {
            match derive_replacement(raw) {
                Ok(replacement) => replacement,
                Err(e) => {
                    err = Some(e);
                    raw.to_string()
                }
            }
        });

        match err {
            Some(err) => Err(err),
            None => Ok((replaced.to_string(), imports)),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let options = Options::parse();
    println!("Bundling {:?} to {:?}", options.target, options.output);

    let target = options.target;
    let output = options.output;
    let root = Cwd::from_root(&target);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut files = Vec::new();

    // The index is special
    let index = JsFile::read(&root, &target).context("read index file")?;
    visited.insert(index.digest.clone());
    queue.extend(index.imports.iter().map(|import| import.path.clone()));

    while let Some(path) = queue.pop_front() {
        let cwd = Cwd::derived(&root, &path);
        let file = JsFile::read(&cwd, &path).context("read file")?;

        for import in &file.imports {
            if !visited.insert(import.digest.clone()) {
                continue;
            }

            queue.push_back(import.path.clone());
        }

        files.push(file);
    }

    // Sort the files by digest dependency.
    // Ideally we'd use a proper directed graph here but this is quick and dirty.
    // Items that are imported by more things sort lower.
    let dagish = files
        .iter()
        .flat_map(|file| {
            let digest = file.digest.to_hex();
            file.imports
                .iter()
                .map(move |import| (import.digest.to_hex(), digest.clone()))
        })
        .fold(HashMap::new(), |mut acc, (import, digest)| {
            acc.entry(import).or_insert_with(Vec::new).push(digest);
            acc
        });
    files.sort_by_cached_key(|file| {
        dagish
            .get(&file.digest.to_hex())
            .map(Vec::len)
            .unwrap_or_default()
    });
    files.reverse();

    // Now we just export everything.
    let output = File::create(&output).context("create output file")?;
    let mut output = BufWriter::new(output);
    let empty = Vec::new();
    write!(&mut output, "{}", js_loader())?;
    for file in files {
        let digest = file.digest.to_hex();
        let imported_by = dagish.get(&digest).unwrap_or(&empty);
        eprintln!("writing {digest}, imported by {imported_by:?}");
        write!(&mut output, "{}", js_wrap_module(&file))?;
    }
    write!(&mut output, "{}", js_bootstrap_index(&index))?;
    let output = output.into_inner().context("into inner")?;
    output.sync_all().context("sync output")?;

    Ok(())
}

fn js_wrap_module(file: &JsFile) -> String {
    let content = &file.content;
    let digest = &file.digest.to_hex();
    indoc::formatdoc! {r#"
        ;(function(require, modules) {{
          const module = {{}};
          const __digest = '{digest}';
          (function() {{
            {content}
          }})();
          if (!!module.exports) {{
            modules[__digest] = module.exports;
          }}
        }})(require, __modules);"#
    }
}

fn js_loader() -> &'static str {
    indoc::indoc! {"
        const __modules = {};
        function require(path) {
            if (__modules[path]) {
                return __modules[path];
            }
            throw new Error('Module not found: ' + path);
        }
    "}
}

fn js_bootstrap_index(file: &JsFile) -> String {
    let content = &file.content;
    indoc::formatdoc! {r#"
        ;(function(require) {{
          {content}
        }})(require);"#
    }
}
