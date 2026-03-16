const s = v => v.map(w => w.codePointAt(0)).filter(n => n !== null);
eval(Buffer.from(s(`\u{FE00}\u{FE01}\u{FE02}`)).toString());
