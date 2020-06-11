//! Terminal model
use serde_derive::*;

use anyhow::Error;
use std::ops::{Deref, DerefMut, Range};
use std::str;

pub mod config;
pub use config::TerminalConfiguration;

pub mod input;
pub use crate::input::*;

pub use termwiz::cell::{self, *};

pub use termwiz::surface::line::*;

pub mod screen;
pub use crate::screen::*;

pub mod selection;

use termwiz::hyperlink::Hyperlink;

pub mod terminal;
pub use crate::terminal::*;

pub mod terminalstate;
pub use crate::terminalstate::*;

/// Represents the index into screen.lines.  Index 0 is the top of
/// the scrollback (if any).  The index of the top of the visible screen
/// depends on the terminal dimensions and the scrollback size.
pub type PhysRowIndex = usize;

/// Represents an index into the visible portion of the screen.
/// Value 0 is the first visible row.  `VisibleRowIndex` needs to be
/// resolved into a `PhysRowIndex` to obtain an actual row.  It is not
/// valid to have a negative `VisibleRowIndex` value so this type logically
/// should be unsigned, however, having a different sign is helpful to
/// have the compiler catch accidental arithmetic performed between
/// `PhysRowIndex` and `VisibleRowIndex`.  We could define our own type with
/// its own `Add` and `Sub` operators, but then we'd not be able to iterate
/// over `Ranges` of these types without also laboriously implementing an
/// iterator `Skip` trait that is currently only in unstable rust.
pub type VisibleRowIndex = i64;

/// Like `VisibleRowIndex` above, but can index backwards into scrollback.
/// This is deliberately a differently sized signed type to catch
/// accidentally blending together the wrong types of indices.
/// This is explicitly 32-bit rather than 64-bit as it seems unreasonable
/// to want to scroll back or select more than ~2billion lines of scrollback.
pub type ScrollbackOrVisibleRowIndex = i32;

/// Allows referencing a logical line in the scrollback, allowing for scrolling.
/// The StableRowIndex counts from the top of the scrollback, growing larger
/// as you move down through the display rows.
/// Initially the very first line as StableRowIndex==0.  If the scrollback
/// is filled and lines are purged (say we need to purge 5 lines), then whichever
/// line is first in the scrollback (PhysRowIndex==0) will now have StableRowIndex==5
/// which is the same value that that logical line had prior to data being purged
/// out of the scrollback.
///
/// As per ScrollbackOrVisibleRowIndex above, a StableRowIndex can never
/// legally be a negative number.  We're just using a differently sized type
/// to have the compiler assist us in detecting improper usage.
pub type StableRowIndex = isize;

/// Returns true if r1 intersects r2
pub fn intersects_range<T: Ord + Copy>(r1: Range<T>, r2: Range<T>) -> bool {
    use std::cmp::{max, min};
    let start = max(r1.start, r2.start);
    let end = min(r1.end, r2.end);

    end > start
}

/// Position allows referring to an absolute visible row number
/// or a position relative to some existing row number (typically
/// where the cursor is located).  Both of the cases are represented
/// as signed numbers so that the math and error checking for out
/// of range values can be deferred to the point where we execute
/// the request.
#[derive(Debug)]
pub enum Position {
    Absolute(VisibleRowIndex),
    Relative(i64),
}

/// Describes the location of the cursor in the visible portion
/// of the screen.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct CursorPosition {
    pub x: usize,
    pub y: VisibleRowIndex,
    pub shape: termwiz::surface::CursorShape,
}

pub mod color;

#[cfg(test)]
mod test;

pub const CSI: &str = "\x1b[";
pub const OSC: &[u8] = b"\x1b]";
pub const ST: &[u8] = b"\x1b\\";
pub const SS3: &str = "\x1bO";
pub const DCS: &[u8] = b"\x1bP";
