use crate::enums::choice::Choice::{*, self};
/*
* First return is for user, the last for computer
*/
pub fn winner(user_choice: Choice, computer_choice: Choice) -> Option<bool> {
    match (user_choice, computer_choice) {
        (ROCK, SCISSOR) => Some(true),
        (SCISSOR, ROCK) => Some(false),
        (SCISSOR, PAPER) =>  Some(true),
        (PAPER, SCISSOR) => Some(false),
        (PAPER, ROCK) =>  Some(true),
        (ROCK, PAPER) => Some(false),
        (SCISSOR, SCISSOR) => None,
        (ROCK, ROCK) => None,
        (PAPER, PAPER) => None,
    }
}

#[cfg(test)]
#[test]
fn user_rock_sharpens_sissors() {
    assert_eq!(winner(ROCK, SCISSOR), Some(true));
}

#[test]
fn computer_rock_sharpens_sissors() {
    assert_eq!(winner(SCISSOR, ROCK), Some(false));
}

#[test]
fn user_scissors_cut_paper() {
    assert_eq!(winner(SCISSOR, PAPER), Some(true));
   
}

#[test]
fn computer_scissors_cut_paper() {
    assert_eq!(winner(PAPER, SCISSOR),Some(false));
}

#[test]
fn user_paper_wraps_rock() {
    assert_eq!(winner(PAPER, ROCK),Some(true));
}

#[test]
fn computer_paper_wraps_rock() {
    assert_eq!(winner(ROCK, PAPER), Some(false));
}

#[test]
fn same_choice_is_draw() {
    assert!(winner(ROCK, ROCK).is_none());
    assert!(winner(PAPER, PAPER).is_none());
    assert!(winner(SCISSOR, SCISSOR).is_none());
}
