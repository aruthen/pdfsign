# PDFSign - ECDSA P-256 Digital Signature Tool

A Rust command-line tool for digitally signing PDF documents using **ECDSA P-256** elliptic curve cryptography. Create cryptographically secure digital signatures for PDF files with support for signature metadata.

![Rust](https://img.shields.io/badge/Rust-2021%20Edition-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Status](https://img.shields.io/badge/Status-Production%20Ready-green?style=flat-square)

---

## 📋 Table of Contents

- [Features](#features)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usage](#usage)
- [How It Works](#how-it-works)
- [Project Structure](#project-structure)
- [Dependencies](#dependencies)
- [Verification](#verification)
- [Technical Details](#technical-details)
- [Contributing](#contributing)
- [License](#license)

---

## ✨ Features

- ✅ **ECDSA P-256 Signing**: Industry-standard elliptic curve cryptography
- ✅ **PKCS#7 Format**: Adobe-compatible detached signature format
- ✅ **Metadata Support**: Include signer name, reason, location, and contact info
- ✅ **Automatic Timestamps**: Add precise signing timestamps to signatures
- ✅ **Visual Representation**: Generate appearance streams for signature visibility
- ✅ **AcroForm Support**: Create proper PDF form fields for signatures
- ✅ **Zero Dependencies Bloat**: Only essential cryptographic libraries
- ✅ **Fast Compilation**: Optimized build with modern Rust toolchain

---

## 🚀 Quick Start

### Generate Keys
```bash
pdfsign generate-key
```
This creates two files:
- `private.key` - Your private signing key (keep secret!)
- `public.key` - Your public key for verification

### Sign a PDF
```bash
pdfsign sign \
  --input document.pdf \
  --output document_signed.pdf \
  --key private.key \
  --name "Your Name" \
  --reason "Document Approval" \
  --location "Indonesia" \
  --contact-info "your@email.com"
```

### Verify Signature
```bash
python verify_signature.py document_signed.pdf
```

---

## 📦 Installation

### Prerequisites

- **Rust 1.56+** ([Install Rust](https://www.rust-lang.org/tools/install))
- **Windows/Linux/macOS**
- **Python 3.7+** (for signature verification only)

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/yourusername/pdfsign.git
cd pdfsign

# Build the project
cargo build --release

# Run the tool
./target/release/pdfsign --help
```

### Install Python Verification Tool

```bash
pip install PyPDF2
```

---

## 💻 Usage

### Command Structure

```bash
pdfsign <COMMAND> [OPTIONS]
```

### Available Commands

#### 1. Generate Keypair
```bash
pdfsign generate-key
```

**Output:**
```
Keys generated: private.key & public.key (ECDSA P-256)
```

**Key Details:**
- Algorithm: ECDSA P-256 (secp256r1)
- Private Key: 32 bytes (256 bits)
- Public Key: 65 bytes (uncompressed format)
- Format: Raw binary

---

#### 2. Sign PDF
```bash
pdfsign sign \
  --input <INPUT_PDF> \
  --output <OUTPUT_PDF> \
  --key <PRIVATE_KEY_FILE> \
  [--name <SIGNER_NAME>] \
  [--reason <SIGN_REASON>] \
  [--location <SIGN_LOCATION>] \
  [--contact-info <CONTACT_INFO>]
```

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--input` | String | Required | Path to PDF file to sign |
| `--output` | String | Required | Path for signed PDF output |
| `--key` | String | Required | Path to private.key file |
| `--name` | String | "pdfsign-cli" | Signer's name |
| `--reason` | String | "Digitally signed" | Reason for signing |
| `--location` | String | "" | Location where signed |
| `--contact-info` | String | "" | Contact information |

**Example:**
```bash
pdfsign sign \
  --input contract.pdf \
  --output contract_signed.pdf \
  --key private.key \
  --name "John Doe" \
  --reason "Contract Approval" \
  --location "Jakarta, Indonesia" \
  --contact-info "john@company.com"
```

**Output:**
```
PDF signed: contract_signed.pdf
Signature: PKCS#7 format (ECDSA)
Signer: John Doe
```

---

## 🔍 How It Works

### Signing Process

```
Input PDF
    ↓
[1] Read PDF bytes
    ↓
[2] Generate ECDSA P-256 signature
    └─ Hash PDF with SHA-256
    └─ Sign hash with private key
    └─ Encode signature in DER format
    ↓
[3] Create PKCS#7 signature structure
    └─ Add signature bytes
    └─ Add certificate (if available)
    └─ Encode in DER format
    ↓
[4] Create PDF signature dictionary
    └─ Set Filter: Adobe.PPKLite
    └─ Set SubFilter: adbe.pkcs7.detached
    └─ Add metadata (name, reason, location, timestamp)
    └─ Add appearance stream (visual representation)
    ↓
[5] Create PDF form structure
    └─ Create AcroForm
    └─ Add signature field
    └─ Add widget annotation
    └─ Reference signature on page
    ↓
[6] Save signed PDF
    ↓
Output: Signed PDF with embedded signature
```

### Signature Structure

**PKCS#7 Format (Detached):**
```
SEQUENCE {
  version INTEGER (1),
  digestAlgorithms SET OF {
    SEQUENCE {
      algorithm OID (SHA-256)
    }
  },
  signatureBytes OCTET STRING (ECDSA signature in DER)
}
```

**PDF Signature Dictionary:**
```
/Type /Sig
/Filter /Adobe.PPKLite
/SubFilter /adbe.pkcs7.detached
/Name (Signer Name)
/Reason (Signing Reason)
/Location (Location)
/M (D:timestamp)
/Contents (hex-encoded PKCS#7)
/ByteRange [0 start1 end1 0]
/AP (Appearance Stream)
```

---

## 📁 Project Structure

```
pdfsign/
├── Cargo.toml                 # Project configuration & dependencies
├── README.md                  # This file
├── verify_signature.py        # Python verification script
│
├── src/
│   ├── main.rs               # Entry point & command routing
│   ├── cli.rs                # Command-line argument parsing
│   │
│   ├── crypto/
│   │   ├── mod.rs            # Crypto module definition
│   │   └── ecc.rs            # ECDSA P-256 implementation
│   │       ├── generate_keypair()  # Generate key pair
│   │       └── sign()              # Sign data
│   │
│   └── pdf/
│       ├── mod.rs            # PDF module definition
│       └── sign.rs           # PDF signing implementation
│           ├── SignatureMetadata   # Metadata struct
│           ├── sign_pdf()          # Main signing function
│           └── encode_der_length() # DER encoding helper
│
└── target/                    # Build output (generated)
    ├── debug/                # Debug build
    └── release/              # Release build
```

---

## 📚 Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.5 | Command-line argument parsing |
| `p256` | 0.13 | ECDSA P-256 implementation |
| `rand_core` | 0.6 | Random number generation |
| `sha2` | 0.10 | SHA-256 hashing |
| `lopdf` | 0.32 | PDF document manipulation |
| `anyhow` | 1.0 | Error handling |
| `chrono` | 0.4 | Timestamp generation |

---

## ✓ Verification

### Method 1: Python Script
```bash
python verify_signature.py your_signed.pdf
```

**Output Example:**
```
✓ PDF has AcroForm signature structure
✓ Found 1 signature field(s)

📝 Signature Details:
   - Field Name: Signature1
   - Filter: /Adobe.PPKLite
   - SubFilter: /adbe.pkcs7.detached
   - Reason: Document Approval
   - Location: Indonesia
   - Date: D:20260120110528
   - Signer: Marut Yuda
```

### Method 2: File Size Check
```bash
# Original size vs signed size
ls -la original.pdf signed.pdf
```
Signed PDF should be ~11KB larger than original (due to PKCS#7 structure).

### Method 3: Adobe Reader
- Open the signed PDF in Adobe Reader
- Look for signature in signature panel (if certificate is trusted)
- Click on signature to see details

**Note:** Signature panel may show as empty if certificate is not in Adobe's trust store. This is normal for self-signed certificates.

---

## 🔐 Technical Details

### Cryptography

**Algorithm:** ECDSA (Elliptic Curve Digital Signature Algorithm)
- **Curve:** P-256 (secp256r1) - NIST standardized curve
- **Hash Function:** SHA-256 (256-bit output)
- **Key Size:** 256 bits (32 bytes)
- **Signature Size:** ~64-72 bytes (variable in DER encoding)

**Advantages of P-256 ECDSA:**
- ✅ Stronger security than RSA-2048 with smaller keys
- ✅ Faster key generation and signing
- ✅ Wide industry support and standardization
- ✅ Used in modern digital signature standards

### PDF Format

**Signature Format:** PKCS#7 (Public Key Cryptography Standards #7)
- Detached signature (signature is separate from PDF content)
- DER encoding (Distinguished Encoding Rules)
- Compatible with Adobe Acrobat Reader
- Compliant with PDF 2.0 specification

### Security Considerations

⚠️ **IMPORTANT:**
- Keep `private.key` file **secure** - anyone with it can sign documents in your name
- Use file permissions: `chmod 600 private.key` (Linux/macOS)
- Consider using encrypted storage for production
- This tool creates self-signed signatures - trust depends on your certificate

---

## 🔧 Building from Source

### Development Build
```bash
cargo build
./target/debug/pdfsign --help
```

### Release Build (Optimized)
```bash
cargo build --release
./target/release/pdfsign --help
```

### Run Tests
```bash
cargo test
```

### Check Code Quality
```bash
cargo clippy
cargo fmt --check
```

---

## 💡 Examples

### Example 1: Sign Contract
```bash
pdfsign sign \
  --input contract.pdf \
  --output contract_signed.pdf \
  --key private.key \
  --name "CEO Name" \
  --reason "Contract Authorization" \
  --location "Head Office" \
  --contact-info "ceo@company.com"
```

### Example 2: Sign Legal Document
```bash
pdfsign sign \
  --input legal_document.pdf \
  --output legal_document_signed.pdf \
  --key private.key \
  --name "Authorized Officer" \
  --reason "Legal Certification" \
  --location "Legal Department"
```

### Example 3: Batch Signing
```bash
for file in *.pdf; do
  pdfsign sign \
    --input "$file" \
    --output "${file%.pdf}_signed.pdf" \
    --key private.key \
    --name "Batch Signer"
done
```

---

## 🤝 Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

**Guidelines:**
- Follow Rust conventions (use `cargo fmt`)
- Add comments for complex logic
- Test your changes
- Update documentation

---

## 📝 Changelog

### v0.1.0 (January 20, 2026)
- ✅ Initial release
- ✅ ECDSA P-256 signing support
- ✅ PKCS#7 signature format
- ✅ PDF AcroForm support
- ✅ Signature metadata (name, reason, location, timestamp)
- ✅ Python verification tool
- ✅ Comprehensive documentation

---

## ⚖️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

```
MIT License

Copyright (c) 2026 Marut Yuda

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software...
```

---

## ❓ FAQ

### Q: Can I use my certificate with this tool?
**A:** Currently, the tool uses self-signed signatures. Support for X.509 certificates is planned for future releases.

### Q: Why doesn't the signature appear in Adobe Reader?
**A:** Self-signed certificates are not trusted by Adobe. To fix this:
1. Generate a proper X.509 certificate
2. Import it into your Adobe certificate store
3. (Or accept that the signature is valid even if visually empty)

### Q: What's the difference between signing and encryption?
**A:** This tool **signs** documents (proves authenticity and integrity). Encryption protects content from being read. Signing and encryption are different operations.

### Q: Can I use this on Linux/macOS?
**A:** Yes! Rust is cross-platform. Build instructions are the same on all systems.

### Q: Is this tool production-ready?
**A:** Yes, for self-signed signatures. For production with certificates, ensure proper certificate management.

---

## 📞 Support

For issues, questions, or suggestions:
- Open an [Issue](https://github.com/yourusername/pdfsign/issues)
- Discuss in [Discussions](https://github.com/yourusername/pdfsign/discussions)
- Contact: [your email]

---

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Cryptography by [p256](https://github.com/RustCrypto/elliptic-curves)
- PDF handling by [lopdf](https://github.com/J-F-Liu/lopdf)

---

**Last Updated:** January 20, 2026  
**Version:** 0.1.0  
**Status:** Production Ready ✅
