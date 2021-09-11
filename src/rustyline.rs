use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::{CompletionType, Config, EditMode, Editor, Context};
use rustyline_derive::{Helper, Highlighter, Validator};

pub use rustyline::error::ReadlineError;

#[derive(Helper, Validator, Highlighter)]
#[allow(dead_code)]
pub struct RLHelper {
    completer: FilenameCompleter,
    hinter: HistoryHinter,
}
impl Completer for RLHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for RLHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

pub fn my_editor() -> Editor<RLHelper> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build();
    let mut editor = Editor::with_config(config);
    let helper = RLHelper {
        completer: FilenameCompleter::new(),
        hinter: HistoryHinter {},
    };
    editor.set_helper(Some(helper));
    editor
}
