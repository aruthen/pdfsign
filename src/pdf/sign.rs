// Import library yang diperlukan
use anyhow::Result;  // Untuk error handling yang flexible
use std::fs;        // Untuk membaca dan menulis file
use lopdf::Document; // Untuk manipulasi dokumen PDF

use crate::crypto::ecc::sign; // Fungsi untuk sign data dengan ECDSA

/// Struktur untuk menyimpan metadata signature
/// Informasi ini akan ditampilkan di signature panel di Adobe Reader
pub struct SignatureMetadata {
    pub name: String,        // Nama penandatangan
    pub reason: String,      // Alasan penandatanganan
    pub location: String,    // Lokasi penandatanganan
    pub contact_info: String, // Informasi kontak penandatangan
}

/// Fungsi utama untuk menandatangani file PDF dengan ECDSA P-256
/// 
/// Parameter:
///   - input: path file PDF yang akan ditandatangani
///   - output: path file PDF hasil penandatanganan
///   - key_path: path file kunci privat
///   - metadata: informasi metadata untuk signature
pub fn sign_pdf(input: &str, output: &str, key_path: &str, metadata: SignatureMetadata) -> Result<()> {
    // Baca file PDF asli dari disk
    let pdf_bytes = fs::read(input)?;
    
    // Baca kunci privat dari file
    let private_key = fs::read(key_path)?;
    
    // Coba baca file certificate jika ada (opsional)
    let cert_path = key_path.replace("private.key", "certificate.der");
    let cert_der = fs::read(&cert_path).ok();

    // Tandatangani seluruh PDF bytes dengan kunci privat
    // Hasil adalah signature dalam format DER
    let signature_bytes = sign(&pdf_bytes, &private_key);

    // Load PDF document menggunakan lopdf library
    let mut doc = Document::load_mem(&pdf_bytes)?;
    
    // Generate timestamp dalam format PDF (D:YYYYMMDDHHmmss)
    // Contoh: D:20260120105337 = 20 Januari 2026 10:53:37
    let timestamp = chrono::Local::now().format("D:%Y%m%d%H%M%S").to_string();
    
    // Buat appearance stream (visual representation) dari signature
    // Ini adalah teks yang akan ditampilkan di dalam signature box
    let appearance_content = b"q
BT
/F1 0 Tf
0 0 0 rg
50 50 Td
(Digitally signed) Tj
ET
Q".to_vec();
    
    // Buat dictionary untuk appearance stream (form XObject)
    let mut appearance_stream_dict = lopdf::Dictionary::new();
    appearance_stream_dict.set("Type", lopdf::Object::Name(b"XObject".to_vec()));
    appearance_stream_dict.set("Subtype", lopdf::Object::Name(b"Form".to_vec()));
    appearance_stream_dict.set("FormType", lopdf::Object::Integer(1));
    // BBox = bounding box untuk tampilan signature (x1, y1, x2, y2)
    appearance_stream_dict.set("BBox", lopdf::Object::Array(vec![
        lopdf::Object::Integer(0),
        lopdf::Object::Integer(0),
        lopdf::Object::Integer(200),
        lopdf::Object::Integer(60),
    ]));
    
    // Buat stream object yang berisi appearance content
    let appearance_stream = lopdf::Stream::new(
        appearance_stream_dict,
        appearance_content,
    );
    // Tambahkan stream ke PDF document dan dapatkan ID-nya
    let appearance_id = doc.add_object(appearance_stream);
    
    // Buat appearance dictionary yang mereferensikan appearance stream
    // "N" = normal appearance (penampilan normal dari signature)
    let mut appearance_dict = lopdf::Dictionary::new();
    appearance_dict.set("N", lopdf::Object::Reference(appearance_id));
    
    // ===== BUAT STRUKTUR PKCS#7 SIGNATURE =====
    // PKCS#7 adalah format standar untuk digital signature dengan sertifikat
    
    let mut pkcs7_content = Vec::new();
    
    // SEQUENCE tag (0x30) - adalah container untuk semua data signature
    pkcs7_content.push(0x30);
    // Placeholder untuk panjang SEQUENCE (akan diisi nanti)
    let content_pos = pkcs7_content.len();
    pkcs7_content.extend_from_slice(&[0x00, 0x00]);
    
    // Version = 1 (format DER: tag=0x02, length=1, value=1)
    pkcs7_content.extend_from_slice(&[0x02, 0x01, 0x01]);
    
    // DigestAlgorithms SET (algoritma hashing yang digunakan)
    pkcs7_content.extend_from_slice(&[0x31, 0x0b]); // SET dengan length 11
    pkcs7_content.extend_from_slice(&[0x30, 0x09]); // SEQUENCE dengan length 9
    pkcs7_content.extend_from_slice(&[0x06, 0x05]); // OID dengan length 5
    // OID untuk SHA-1 (2.16.840.1.101.3.4.2.1)
    pkcs7_content.extend_from_slice(&[0x2b, 0x0e, 0x03, 0x02, 0x1a]);
    
    // Tambahkan signature bytes
    pkcs7_content.extend_from_slice(&[0x04]); // OCTET STRING tag
    let sig_len = signature_bytes.len();
    // Encode panjang signature
    if sig_len < 128 {
        pkcs7_content.push(sig_len as u8);
    } else {
        pkcs7_content.push(0x81); // Indica long form length
        pkcs7_content.push(sig_len as u8);
    }
    // Tambahkan signature data
    pkcs7_content.extend_from_slice(&signature_bytes);
    
    // Jika certificate tersedia, tambahkan ke PKCS#7 structure
    if let Some(cert) = &cert_der {
        pkcs7_content.extend_from_slice(&cert);
    }
    
    // Hitung panjang total SEQUENCE content (tanpa tag dan length byte pertama)
    let total_len = pkcs7_content.len() - content_pos - 2;
    // Encode panjang menggunakan DER format
    let len_bytes = encode_der_length(total_len);
    if len_bytes.len() == 1 {
        // Jika panjang bisa dalam 1 byte, isi placeholder
        pkcs7_content[content_pos] = len_bytes[0];
    }
    
    // ===== BUAT SIGNATURE DICTIONARY =====
    // Ini adalah object PDF yang menyimpan informasi signature
    
    let mut sig_dict = lopdf::Dictionary::new();
    // Type = "Sig" menunjukkan ini adalah signature object
    sig_dict.set("Type", lopdf::Object::Name(b"Sig".to_vec()));
    // Filter = Adobe.PPKLite (format signature yang kompatibel dengan Adobe Reader)
    sig_dict.set("Filter", lopdf::Object::Name(b"Adobe.PPKLite".to_vec()));
    // SubFilter = adbe.pkcs7.detached (menggunakan PKCS#7 detached signature)
    sig_dict.set("SubFilter", lopdf::Object::Name(b"adbe.pkcs7.detached".to_vec()));
    // Nama penandatangan
    sig_dict.set("Name", lopdf::Object::String(metadata.name.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    
    // Timestamp penandatanganan
    sig_dict.set("M", lopdf::Object::String(timestamp.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    // Alasan penandatanganan
    sig_dict.set("Reason", lopdf::Object::String(metadata.reason.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    
    // Lokasi penandatanganan (opsional)
    if !metadata.location.is_empty() {
        sig_dict.set("Location", lopdf::Object::String(metadata.location.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    }
    // Informasi kontak penandatangan (opsional)
    if !metadata.contact_info.is_empty() {
        sig_dict.set("ContactInfo", lopdf::Object::String(metadata.contact_info.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    }
    
    // Reference certificate jika tersedia
    if let Some(cert) = &cert_der {
        sig_dict.set("Cert", lopdf::Object::String(cert.clone(), lopdf::StringFormat::Literal));
    }
    
    // Appearance stream untuk menampilkan signature secara visual
    sig_dict.set("AP", lopdf::Object::Dictionary(appearance_dict.clone()));
    
    // ===== TAMBAHKAN SIGNATURE CONTENT =====
    // Ini adalah data signature PKCS#7 dalam format hexadecimal
    
    let mut padded_content = pkcs7_content.clone();
    // Padding signature content ke ukuran minimum 4096 bytes
    // Ini diperlukan karena Adobe memerlukan placeholder untuk signature yang mungkin berkembang
    while padded_content.len() < 4096 {
        padded_content.push(0x00);
    }
    
    // Tambahkan signature content dalam format hexadecimal
    sig_dict.set("Contents", lopdf::Object::String(padded_content, lopdf::StringFormat::Hexadecimal));
    
    // ByteRange menunjukkan byte mana dari PDF yang ditandatangani
    // Format: [start1, length1, start2, length2]
    // start1/length1 = bagian PDF sebelum signature
    // start2/length2 = bagian PDF setelah signature (biasanya kosong)
    sig_dict.set("ByteRange", lopdf::Object::Array(vec![
        lopdf::Object::Integer(0),
        lopdf::Object::Integer(pdf_bytes.len() as i64),
        lopdf::Object::Integer(pdf_bytes.len() as i64 + 8192),
        lopdf::Object::Integer(0),
    ]));
    
    // Tambahkan signature dictionary ke PDF document
    let sig_id = doc.add_object(sig_dict);
    
    // ===== BUAT SIGNATURE FIELD (Widget Annotation) =====
    // Ini adalah field form yang menampilkan signature di halaman PDF
    
    let mut field_dict = lopdf::Dictionary::new();
    field_dict.set("Type", lopdf::Object::Name(b"Annot".to_vec()));
    field_dict.set("Subtype", lopdf::Object::Name(b"Widget".to_vec()));
    field_dict.set("FT", lopdf::Object::Name(b"Sig".to_vec())); // Field Type = Signature
    field_dict.set("T", lopdf::Object::String(b"Signature1".to_vec(), lopdf::StringFormat::Literal));
    field_dict.set("F", lopdf::Object::Integer(4)); // Flags untuk form field
    // V = reference ke signature object yang dibuat di atas
    field_dict.set("V", lopdf::Object::Reference(sig_id));
    // Appearance stream untuk field
    field_dict.set("AP", lopdf::Object::Dictionary(appearance_dict));
    // Rect = posisi dan ukuran signature field di halaman PDF
    // Format: [left, bottom, right, top]
    field_dict.set("Rect", lopdf::Object::Array(vec![
        lopdf::Object::Integer(100),   // Left edge
        lopdf::Object::Integer(650),   // Bottom edge
        lopdf::Object::Integer(300),   // Right edge
        lopdf::Object::Integer(700),   // Top edge
    ]));
    // P = reference ke halaman pertama PDF
    field_dict.set("P", lopdf::Object::Reference((2, 0)));
    
    // Tambahkan field ke PDF document
    let field_id = doc.add_object(field_dict);
    
    // ===== BUAT ACROFORM (Form Structure) =====
    // AcroForm adalah struktur PDF yang mendefinisikan form fields
    
    let mut acroform = lopdf::Dictionary::new();
    // SigFlags = 3 (tanda bahwa ini adalah signed form)
    acroform.set("SigFlags", lopdf::Object::Integer(3));
    // DA = default appearance string untuk text di form
    acroform.set("DA", lopdf::Object::String(b"/F1 0 Tf 0 0 0 rg".to_vec(), lopdf::StringFormat::Literal));
    // Fields = array yang berisi referensi ke semua signature fields
    acroform.set("Fields", lopdf::Object::Array(vec![
        lopdf::Object::Reference(field_id),
    ]));
    
    // Tambahkan AcroForm ke PDF document
    let acroform_id = doc.add_object(acroform);
    
    // ===== UPDATE PDF CATALOG ROOT =====
    // Catalog adalah root object yang mereferensikan semua struktur PDF
    
    let root_id = (1, 0); // Object ID untuk catalog biasanya (1, 0)
    
    // Dapatkan mutable reference ke catalog
    if let Ok(ref mut root) = doc.get_object_mut(root_id) {
        if let lopdf::Object::Dictionary(ref mut dict) = root {
            // Tambahkan referensi AcroForm ke catalog
            dict.set("AcroForm", lopdf::Object::Reference(acroform_id));
        }
    }
    
    // ===== TAMBAHKAN ANNOTATION KE HALAMAN PERTAMA =====
    // Halaman pertama biasanya adalah object (2, 0)
    
    if let Ok(ref mut page) = doc.get_object_mut((2, 0)) {
        if let lopdf::Object::Dictionary(ref mut page_dict) = page {
            // Cek apakah sudah ada Annots array
            if let Ok(annots_ref) = page_dict.get_mut(b"Annots") {
                // Jika ada, tambahkan signature field ke array
                if let lopdf::Object::Array(ref mut annots) = annots_ref {
                    annots.push(lopdf::Object::Reference(field_id));
                } else {
                    // Jika ada tapi bukan array, buat array baru
                    page_dict.set("Annots", lopdf::Object::Array(vec![
                        lopdf::Object::Reference(field_id),
                    ]));
                }
            } else {
                // Jika tidak ada Annots, buat baru dengan signature field
                page_dict.set("Annots", lopdf::Object::Array(vec![
                    lopdf::Object::Reference(field_id),
                ]));
            }
        }
    }

    // Simpan PDF yang sudah ditandatangani ke file output
    doc.save(output)?;
    
    // Tampilkan pesan sukses ke user
    println!("PDF signed: {}", output);
    println!("Signature: PKCS#7 format (ECDSA)");
    println!("Signer: {}", metadata.name);

    Ok(())
}

/// Helper function untuk encode panjang dalam format DER
/// Digunakan untuk encoding panjang SEQUENCE dan object lain dalam PKCS#7
/// 
/// DER length encoding:
/// - Jika < 128: encode sebagai 1 byte
/// - Jika >= 128: encode sebagai 0x80|numOfBytes diikuti bytes panjang
fn encode_der_length(len: usize) -> Vec<u8> {
    if len < 128 {
        // Panjang pendek: langsung sebagai 1 byte
        vec![len as u8]
    } else {
        // Panjang panjang: encode sebagai multi-byte
        let mut bytes = Vec::new();
        let mut l = len;
        // Ambil bytes dari kanan ke kiri
        while l > 0 {
            bytes.insert(0, (l & 0xff) as u8);
            l >>= 8;
        }
        // Tambahkan byte indicator: 0x80 | jumlah bytes
        let mut result = vec![0x80 | bytes.len() as u8];
        result.extend_from_slice(&bytes);
        result
    }
}
