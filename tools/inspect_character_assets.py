import json, struct
from pathlib import Path

ASSETS = [
    "anime_girl_mia_ter_excited_preview.glb",
    "kurumi_schoolgirl_-_bourin.glb",
    "mai_maid_-bourin.glb",
    "rigged_anime_girl_outfits__facial_expressions.glb",
    "3d_anime_girl..glb",
]

for name in ASSETS:
    path = Path("assets") / name
    data = path.read_bytes()
    if data[:4] != b"glTF":
        print(f"{name}: not GLB")
        continue
    off = 12
    doc = None
    while off + 8 <= len(data):
        length, kind = struct.unpack_from("<II", data, off)
        off += 8
        chunk = data[off:off+length]
        off += length
        if kind == 0x4E4F534A:
            doc = json.loads(chunk.rstrip(b" \t\r\n\x00"))
            break
    if doc is None:
        print(f"{name}: no JSON")
        continue
    nodes = doc.get("nodes", [])
    animations = doc.get("animations", [])
    print(f"\n=== {name} ===")
    print(f"animations={len(animations)} skins={len(doc.get('skins', []))} nodes={len(nodes)}")
    for i, anim in enumerate(animations):
        targets = []
        for ch in anim.get("channels", []):
            target = ch.get("target", {})
            ni = target.get("node")
            nn = nodes[ni].get("name", f"node#{ni}") if isinstance(ni, int) and ni < len(nodes) else "?"
            targets.append(f"{nn}:{target.get('path','?')}")
        print(f"[{i}] {anim.get('name','<unnamed>')} channels={len(anim.get('channels', []))} sample={', '.join(targets[:8])}")

    # Inspect embedded texture sizes without decoding or re-encoding the original assets.
    bin_offset = 20 + struct.unpack_from("<I", data, 12)[0]
    bin_length = struct.unpack_from("<I", data, bin_offset)[0] if bin_offset + 8 <= len(data) else 0
    bin_start = bin_offset + 8
    if struct.unpack_from("<I", data, bin_offset + 4)[0] == 0x004E4942:
        binary = memoryview(data)[bin_start:bin_start + bin_length]
        views = doc.get("bufferViews", [])
        images = doc.get("images", [])
        print(f"textures={len(images)}")
        for i, image in enumerate(images):
            index = image.get("bufferView")
            if not isinstance(index, int) or index >= len(views):
                print(f"  texture[{i}]: external ({image.get('uri','?')})")
                continue
            view = views[index]
            start = view.get("byteOffset", 0)
            length = view.get("byteLength", 0)
            raw = bytes(binary[start:start+length])
            width = height = 0
            if raw.startswith(b"\\x89PNG\\r\\n\\x1a\\n") and len(raw) >= 24:
                width, height = struct.unpack_from(">II", raw, 16)
            elif raw.startswith(b"\\xff\\xd8"):
                pos = 2
                while pos + 4 < len(raw):
                    if raw[pos] != 0xFF:
                        pos += 1
                        continue
                    marker = raw[pos + 1]
                    if marker in (0xC0, 0xC1, 0xC2, 0xC3, 0xC5, 0xC6, 0xC7, 0xC9, 0xCA, 0xCB, 0xCD, 0xCE, 0xCF):
                        height, width = struct.unpack_from(">HH", raw, pos + 5)
                        break
                    if marker in (0xD8, 0xD9, 0x01) or 0xD0 <= marker <= 0xD7:
                        pos += 2
                        continue
                    seglen = struct.unpack_from(">H", raw, pos + 2)[0]
                    if seglen < 2:
                        break
                    pos += 2 + seglen
            print(f"  texture[{i}] {image.get('name','')} {width}x{height} embedded={length} mime={image.get('mimeType','?')}")
