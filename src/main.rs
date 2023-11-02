use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::IResult;
use std::error::Error;

fn parser(s: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("abc"))(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    // In actual code, the input comes as a string through:
    // let input = std::fs::read_to_string(INPUT_FILE_PATH)?
    let input = "abcabc123".to_string();

    // This following fails, without mapping the error:

    // let (remaining, parsed) = parser(&input)?;

    // Whereas it works fine if I ensure that the content of the error is owned
    // (which is internally cloning the &str content, I assume):
    let (remaining, parsed) = parser(&input).map_err(|e| e.map_input(|s| s.to_owned()))?;

    assert_eq!(remaining, "123");
    assert_eq!(parsed, vec!["abc", "abc"]);
    Ok(())
}
