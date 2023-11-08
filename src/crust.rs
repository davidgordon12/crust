use reqwest::Error;
pub fn execute_request(uri: String) -> Result<(), Box<Error>> {
    let res = reqwest::blocking::get(uri)?.text()?;
    println!("{:#?}", res);
    Ok(())
}