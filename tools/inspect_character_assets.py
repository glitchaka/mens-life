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
