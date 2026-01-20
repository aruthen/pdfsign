#!/usr/bin/env python3
import PyPDF2
import sys

def verify_pdf_signature(pdf_path):
    """Verify if a PDF has digital signature"""
    try:
        with open(pdf_path, 'rb') as f:
            pdf_reader = PyPDF2.PdfReader(f)
            
            # Check for AcroForm (form fields)
            if "/AcroForm" in pdf_reader.trailer["/Root"]:
                acroform = pdf_reader.trailer["/Root"]["/AcroForm"]
                print(f"✓ PDF has AcroForm signature structure")
                
                # Get signature fields
                if "/Fields" in acroform:
                    fields = acroform["/Fields"]
                    print(f"✓ Found {len(fields)} signature field(s)")
                    
                    for field_ref in fields:
                        field = field_ref.get_object()
                        if field["/FT"] == "/Sig":
                            print(f"\n📝 Signature Details:")
                            print(f"   - Field Name: {field.get('/T', 'Unknown')}")
                            
                            if "/V" in field:
                                sig_dict = field["/V"].get_object()
                                print(f"   - Filter: {sig_dict.get('/Filter', 'Unknown')}")
                                print(f"   - SubFilter: {sig_dict.get('/SubFilter', 'Unknown')}")
                                print(f"   - Reason: {sig_dict.get('/Reason', 'N/A')}")
                                print(f"   - Location: {sig_dict.get('/Location', 'N/A')}")
                                print(f"   - Date: {sig_dict.get('/M', 'N/A')}")
                                print(f"   - Signer: {sig_dict.get('/Name', 'Unknown')}")
                return True
            else:
                print("✗ No signature found in PDF")
                return False
                
    except Exception as e:
        print(f"✗ Error reading PDF: {e}")
        return False

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python verify_signature.py <pdf_file>")
        sys.exit(1)
    
    verify_pdf_signature(sys.argv[1])
