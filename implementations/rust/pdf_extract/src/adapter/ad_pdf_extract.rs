use pdf_extract::extract_text_from_mem;

pub fn extract(input_path: String) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(&input_path)?;
    let out = extract_text_from_mem(&bytes)?;
    println!("{}", out);
    Ok(out)
}
