import sys
import unicodedata

for codepoint in range(100000):
    unicode = chr(codepoint)
    try:
        bits = unicode.encode('utf-8')
    except UnicodeEncodeError:
        continue
    name = unicodedata.name(unicode, '')
    if codepoint < 32:
        unicode = repr(unicode)[1:-1]  # control characters and other non-printable things

    print(f'{codepoint:04d} {unicode:8s} {int.from_bytes(bits, byteorder=sys.byteorder):032b} "{name}"')
