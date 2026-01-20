use anyhow::Result;
use std::fs;
use lopdf::Document;

use crate::crypto::ecc::sign;

pub fn sign_pdf(input: &str, output: &str, key_path: &str) -> Result<()> {
    let pdf_bytes = fs::read(input)?;
    let private_key = fs::read(key_path)?;

    let signature = sign(&pdf_bytes, &private_key);

    let mut doc = Document::load_mem(&pdf_bytes)?;
    doc.trailer.set(
        b"SignedBy".to_vec(),
        lopdf::Object::Name(b"pdfsign-cli".to_vec()),
    );
    doc.trailer.set(
        b"Signature".to_vec(),
        lopdf::Object::String(signature, lopdf::StringFormat::Hexadecimal),
    );

    doc.save(output)?;
    println!("PDF signed: {}", output);

    Ok(())
}
