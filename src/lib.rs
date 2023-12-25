//! # the Meaning of Life
//! Just answer you the meaning of life.


/// tell you the answer
/// 
/// # Examples
/// ```
/// use the_meaning_of_life::answer;
/// let your_answer = answer();
/// ```
/// 
pub fn answer() -> String {
    let answer = String::from(r#"
Your existence is the meaning of life.
Wishing everyone happiness every day."#);

    println!("{:?}", answer);

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        let my_answer = "
Your existence is the meaning of life.
Wishing everyone happiness every day.";

        assert_eq!(&(answer()), my_answer);
    }

}
