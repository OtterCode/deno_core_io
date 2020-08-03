// Credit to MDN for the encode/decode functions.

function to_uint8array(str) {
  var utf8 = [];
  for (var i=0; i < str.length; i++) {
    var charcode = str.charCodeAt(i);
    if (charcode < 0x80) utf8.push(charcode);
    else if (charcode < 0x800) {
      utf8.push(0xc0 | (charcode >> 6),
      0x80 | (charcode & 0x3f));
    }
    else if (charcode < 0xd800 || charcode >= 0xe000) {
      utf8.push(0xe0 | (charcode >> 12),
      0x80 | ((charcode>>6) & 0x3f),
      0x80 | (charcode & 0x3f));
    }
    // surrogate pair
    else {
      i++;
      // UTF-16 encodes 0x10000-0x10FFFF by
      // subtracting 0x10000 and splitting the
      // 20 bits of 0x0-0xFFFFF into two halves
      charcode = 0x10000 + (((charcode & 0x3ff)<<10)
      | (str.charCodeAt(i) & 0x3ff))
      utf8.push(0xf0 | (charcode >>18),
      0x80 | ((charcode>>12) & 0x3f),
      0x80 | ((charcode>>6) & 0x3f),
      0x80 | (charcode & 0x3f));
    }
  }
  return new Uint8Array(utf8);
}


function utf8ArrayToString(aBytes) {
    var sView = "";

    for (var nPart, nLen = aBytes.length, nIdx = 0; nIdx < nLen; nIdx++) {
        nPart = aBytes[nIdx];

        sView += String.fromCharCode(
            nPart > 251 && nPart < 254 && nIdx + 5 < nLen ? /* six bytes */
                /* (nPart - 252 << 30) may be not so safe in ECMAScript! So...: */
                (nPart - 252) * 1073741824 + (aBytes[++nIdx] - 128 << 24) + (aBytes[++nIdx] - 128 << 18) + (aBytes[++nIdx] - 128 << 12) + (aBytes[++nIdx] - 128 << 6) + aBytes[++nIdx] - 128
            : nPart > 247 && nPart < 252 && nIdx + 4 < nLen ? /* five bytes */
                (nPart - 248 << 24) + (aBytes[++nIdx] - 128 << 18) + (aBytes[++nIdx] - 128 << 12) + (aBytes[++nIdx] - 128 << 6) + aBytes[++nIdx] - 128
            : nPart > 239 && nPart < 248 && nIdx + 3 < nLen ? /* four bytes */
                (nPart - 240 << 18) + (aBytes[++nIdx] - 128 << 12) + (aBytes[++nIdx] - 128 << 6) + aBytes[++nIdx] - 128
            : nPart > 223 && nPart < 240 && nIdx + 2 < nLen ? /* three bytes */
                (nPart - 224 << 12) + (aBytes[++nIdx] - 128 << 6) + aBytes[++nIdx] - 128
            : nPart > 191 && nPart < 224 && nIdx + 1 < nLen ? /* two bytes */
                (nPart - 192 << 6) + aBytes[++nIdx] - 128
            : /* nPart < 127 ? */ /* one byte */
                nPart
        );
    }

    return sView;
}


console.log = (...strs) => {
  Deno.core.print(strs.join("")+"\n")
}
