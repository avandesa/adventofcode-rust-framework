use crate::{regex, PuzzleSolver};
use once_cell::sync::Lazy;
use regex::RegexSet;

pub struct Day07 {
    fs: FsItem,
}

static LINE_REGEXES_NOCAPTURE: Lazy<RegexSet> = Lazy::new(|| {
    RegexSet::new([
        r#"^\$ cd (?:/|\.\.|\w+)$"#,
        r#"^\$ ls$"#,
        r#"^dir \w+$"#,
        r#"^\d+ [\w.]+$"#,
    ])
    .unwrap()
});

impl PuzzleSolver for Day07 {
    fn with_input(input: String) -> Self {
        let parsed_lines = input
            .lines()
            .map(TerminalLine::parse_from_line)
            .collect::<Vec<_>>();

        let fs_root = build_root(&parsed_lines);

        Self { fs: fs_root }
    }

    fn part1(&self) -> String {
        self.fs.sum_sizes_under_1k().to_string()
    }

    fn part2(&self) -> String {
        let space_to_free = self.fs.size() - (70000000 - 30000000);
        self.fs
            .deleting_frees_enough(space_to_free)
            .unwrap()
            .to_string()
    }
}

#[derive(Debug)]
enum FsItem {
    Directory { items: Vec<FsItem>, total_size: u32 },
    File { size: u32 },
}

impl FsItem {
    fn size(&self) -> u32 {
        match self {
            Self::Directory { total_size, .. } => *total_size,
            Self::File { size, .. } => *size,
        }
    }

    fn sum_sizes_under_1k(&self) -> u32 {
        match self {
            Self::Directory {
                items, total_size, ..
            } => {
                let sub_item_sum = items.iter().map(|i| i.sum_sizes_under_1k()).sum();
                if *total_size < 100000 {
                    sub_item_sum + total_size
                } else {
                    sub_item_sum
                }
            }
            Self::File { .. } => 0,
        }
    }

    fn deleting_frees_enough(&self, space_to_free: u32) -> Option<u32> {
        match self {
            Self::Directory {
                items, total_size, ..
            } => items
                .iter()
                .filter_map(|i| i.deleting_frees_enough(space_to_free))
                .min()
                .or_else(|| (*total_size >= space_to_free).then_some(*total_size)),
            Self::File { .. } => None,
        }
    }
}

fn build_root(lines: &[TerminalLine]) -> FsItem {
    let (root, r) = build_dir(lines);
    assert!(r.is_empty());
    println!("Done building FS from input");
    root
}

fn build_dir(lines: &[TerminalLine]) -> (FsItem, &[TerminalLine]) {
    assert!(
        matches!(lines[0], TerminalLine::CmdCd(CdArg::Path(_))),
        "expected `cd <dirname>`, was {:?}",
        lines[0]
    );
    assert!(
        matches!(lines[1], TerminalLine::CmdLs),
        "expected `ls`, was {:?}",
        lines[1]
    );

    let (files, item_count) = build_files(&lines[2..]);

    let (dirs, remaining) = build_dirs(&lines[2 + item_count..]);

    let items = {
        let mut items = files;
        let mut dirs = dirs;
        items.append(&mut dirs);
        items
    };

    let total_size = items.iter().map(|i| i.size()).sum();

    let remaining = if remaining.is_empty() {
        remaining
    } else {
        // Consume the `UpOne` if present
        &remaining[1..]
    };

    (FsItem::Directory { items, total_size }, remaining)
}

/// Parse a list of items within a directory after an `ls`, returning a list of found files and the
/// total number of files and directories.
fn build_files(lines: &[TerminalLine]) -> (Vec<FsItem>, usize) {
    let item_count = lines.iter().take_while(|l| l.is_fs_item()).count();
    let files = lines
        .iter()
        .take_while(|l| l.is_fs_item())
        .filter_map(file_from_line)
        .collect::<Vec<_>>();

    (files, item_count)
}

/// Recursively parse directory listings, consuming lines from `cd <dirname>` to the corresponding `cd ..`.
/// Returns all parsed directories and the remaining lines.
fn build_dirs(lines: &[TerminalLine]) -> (Vec<FsItem>, &[TerminalLine]) {
    if lines.is_empty() {
        return (vec![], lines);
    }

    let mut dirs = Vec::new();
    let mut remaining = lines;

    while !remaining.is_empty() && !matches!(&remaining[0], TerminalLine::CmdCd(CdArg::UpOne)) {
        let (dir, r) = build_dir(remaining);
        dirs.push(dir);
        remaining = r;
    }

    (dirs, remaining)
}

fn file_from_line(line: &TerminalLine) -> Option<FsItem> {
    match line {
        TerminalLine::File { size, .. } => Some(FsItem::File { size: *size }),
        _ => None,
    }
}

#[derive(Debug)]
enum TerminalLine {
    CmdCd(CdArg),
    CmdLs,
    Directory,
    File { size: u32 },
}

#[derive(Debug)]
enum CdArg {
    Root,
    UpOne,
    Path(String),
}

impl TerminalLine {
    fn is_fs_item(&self) -> bool {
        matches!(
            self,
            TerminalLine::Directory { .. } | TerminalLine::File { .. }
        )
    }

    fn parse_from_line(line: &str) -> Self {
        let matches = LINE_REGEXES_NOCAPTURE.matches(line);
        let first_match_idx = matches
            .iter()
            .next()
            .unwrap_or_else(|| panic!("at least one match in line '{line}'"));
        match first_match_idx {
            0 => {
                let cd_regex = regex!(r#"^\$ cd (?:(?P<up>\.\.)|(?P<dir>\w+|/))$"#);
                let caps = cd_regex.captures(line).unwrap();
                let arg = if let Some(dir) = caps.name("dir") {
                    CdArg::Path(dir.as_str().to_string())
                } else if caps.name("root").is_some() {
                    CdArg::Root
                } else if caps.name("up").is_some() {
                    CdArg::UpOne
                } else {
                    unreachable!("Invalid `cd` command: {}", line);
                };
                TerminalLine::CmdCd(arg)
            }
            1 => TerminalLine::CmdLs,
            2 => TerminalLine::Directory,
            3 => {
                let file_regex = regex!(r#"^(?P<size>\d+) [\w.]+$"#);
                let caps = file_regex.captures(line).unwrap();
                let size = caps
                    .name("size")
                    .expect("file size not captured")
                    .as_str()
                    .parse()
                    .unwrap();
                TerminalLine::File { size }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(7, "95437", "24933642");
