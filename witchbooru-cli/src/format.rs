use witchbooru::{Prediction, Tag};

use itertools::{EitherOrBoth, Itertools, Position};
use std::fmt;

pub struct Display<'a>(pub &'a Prediction<'a>);

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const SCORE_WIDTH: usize = 5;
        const SEPARATOR: &str = "─";

        let (left, right) = (self.0.general(), self.0.character());
        let (left_category, right_category) = ("General tag", "Character");
        let (left_name_width, right_name_width) = (
            left.iter()
                .fold(left_category.len(), |m, tag| m.max(tag.name.len())),
            right
                .iter()
                .fold(right_category.len(), |m, tag| m.max(tag.name.len())),
        );

        writeln!(
            f,
            " {:left_name_width$} {score:>score_width$}   {:right_name_width$} {score:>score_width$} ",
            left_category,
            right_category,
            score = "Score",
            left_name_width = left_name_width,
            right_name_width = right_name_width,
            score_width = SCORE_WIDTH,
        )?;
        writeln!(
            f,
            "{left_sep} {right_sep}",
            left_sep = SEPARATOR.repeat(left_name_width + SCORE_WIDTH + 3),
            right_sep = SEPARATOR.repeat(right_name_width + SCORE_WIDTH + 3)
        )?;

        let (left, right) = (
            left.iter()
                .map(|tag| format_tag(tag, left_name_width, SCORE_WIDTH)),
            right
                .iter()
                .map(|tag| format_tag(tag, right_name_width, SCORE_WIDTH)),
        );
        for row in left
            .zip_longest(right)
            .map(EitherOrBoth::or_default)
            .with_position()
        {
            let last = matches!(row, Position::Last(_) | Position::Only(_));

            let (left, right) = row.into_inner();
            write!(
                f,
                " {:width$}   {} ",
                left,
                right,
                width = left_name_width + SCORE_WIDTH + 1
            )?;

            if !last {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn format_tag(tag: &Tag, name_width: usize, score_width: usize) -> String {
    format!(
        "{} {:>score_width$.3}",
        Hyperlink {
            text: tag.name,
            url: &format!("https://danbooru.donmai.us/wiki_pages/{}", tag.name),
            width: name_width,
        },
        tag.score,
        score_width = score_width
    )
}

struct Hyperlink<'a, 'b> {
    text: &'a str,
    url: &'b str,
    width: usize,
}

impl fmt::Display for Hyperlink<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda

        // we need to implement "width" by ourselves because
        // "{:width$}" format doesn't take escape sequences into account

        write!(
            f,
            "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\{}",
            self.url,
            self.text,
            " ".repeat(self.width.saturating_sub(self.text.len()))
        )
    }
}
