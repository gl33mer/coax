// Glassworm Attack Sample 001
// This file contains a simulated Glassworm attack pattern for testing

// Hidden payload using variation selectors
const hiddenPayload = "console.log('malicious')";

// Decoder function characteristic of Glassworm
const codes = hiddenPayload.split('').map(c => c.codePointAt(0));
const filtered = codes.filter(c => c !== null);

// Reconstruct and execute
const decoded = String.fromCharCode(...filtered);
eval(decoded);

// Alternative pattern: Buffer decoding
const hexPayload = "636f6e736f6c652e6c6f6728276861636b65642729";
const decoded2 = Buffer.from(hexPayload, 'hex').toString();
eval(decoded2);
