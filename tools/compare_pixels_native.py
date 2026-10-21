#!/usr/bin/env python3
import argparse
import hashlib
import json
import math
import struct
import sys
from pathlib import Path

# D3D9 pixel centers are integer coordinates, including the synthetic sampler.
W, H = 1920, 1080
FRAMES = (0, 3, 5)

class Incomplete(Exception):
    pass

def require(ok, message):
    if not ok:
        raise Incomplete(message)

def read_limited(path, limit):
    with Path(path).open("rb") as stream:
        data = stream.read(limit + 1)
    require(len(data) <= limit, "input exceeds size limit")
    return data

class BMP:
    def __init__(self, data):
        require(len(data) >= 54, "truncated BMP header")
        sig, size, r1, r2, off = struct.unpack_from("<2sIHHI", data)
        dib, w, h, planes, bits, compression, image_size = struct.unpack_from("<IiiHHII", data, 14)
        require(sig == b"BM" and size == len(data) and not (r1 or r2), "invalid BMP file header")
        require(dib == 40 and off >= 54, "unsupported BMP header")
        require(w == W and abs(h) == H, "BMP must be native 1920x1080")
        require(planes == 1 and bits in (24, 32) and compression == 0, "only uncompressed 24/32-bit BMP is supported")
        require(struct.unpack_from("<I", data, 46)[0] == 0, "BMP palette is unsupported")
        self.step = bits // 8
        self.stride = ((W * bits + 31) // 32) * 4
        required = self.stride * H
        end = off + required
        extent_ok = len(data) == end or (
            bits == 32 and off == 54 and image_size == 0
            and len(data) == end + 2 and data[end:] == b"\0\0"
        )
        require(image_size in (0, required) and extent_ok, "invalid BMP pixel extent")
        self.data, self.off, self.top_down = data, off, h < 0

    def pixel(self, x, y):
        row = y if self.top_down else H - 1 - y
        pos = self.off + row * self.stride + x * self.step
        b, g, r = self.data[pos:pos + 3]
        return r, g, b

def chroma(rgb):
    lo, hi = min(rgb), max(rgb)
    return None if hi == lo else tuple((v - lo) / (hi - lo) for v in rgb)

def center(image, marker):
    ex, ey = marker["expected_xy"]
    hue = chroma(marker["rgb"])
    require(hue is not None, "achromatic marker")
    left, top = math.floor(ex) - 16, math.floor(ey) - 16
    mask = {}
    for y in range(top, top + 33):
        for x in range(left, left + 33):
            rgb = image.pixel(x, y)
            weight = max(rgb) - min(rgb)
            color = chroma(rgb)
            if weight >= 8 and max(abs(a - b) for a, b in zip(color, hue)) <= 0.12:
                mask[x, y] = weight
    require(40 <= len(mask) <= 180, "missing or contaminated cross")
    unseen, components = set(mask), []
    while unseen:
        stack, component = [unseen.pop()], []
        while stack:
            x, y = stack.pop()
            component.append((x, y))
            for dy in (-1, 0, 1):
                for dx in (-1, 0, 1):
                    neighbor = (x + dx, y + dy)
                    if neighbor in unseen:
                        unseen.remove(neighbor)
                        stack.append(neighbor)
        components.append(component)
    require(len(components) == 1, "ambiguous/disconnected color candidates")
    xs, ys = [p[0] for p in mask], [p[1] for p in mask]
    require(14 <= max(xs) - min(xs) + 1 <= 18 and 14 <= max(ys) - min(ys) + 1 <= 18, "cross dimensions are unsupported")
    require(min(xs) > left and max(xs) < left + 32 and min(ys) > top and max(ys) < top + 32, "cross touches ROI boundary")
    total, peak = sum(mask.values()), max(mask.values())
    require(65 <= total / peak <= 110, "cross area is unsupported")
    cx = sum(x * v for (x, y), v in mask.items()) / total
    cy = sum(y * v for (x, y), v in mask.items()) / total
    outside = sum(v for (x, y), v in mask.items() if min(abs(x - cx), abs(y - cy)) > 2.5)
    require(outside <= total * 0.03, "cross shape is unsupported")
    for axis, sign in ((0, -1), (0, 1), (1, -1), (1, 1)):
        arm = 0
        for (x, y), weight in mask.items():
            d = (x - cx, y - cy)
            if sign * d[axis] >= 4 and abs(d[1 - axis]) <= 2.5:
                arm += weight
        require(arm >= 6 * peak, "cross arm is missing or occluded")
    return [cx, cy]

def compare(calibration, buffers):
    rows = []
    for frame in FRAMES:
        markers = [m for m in calibration["markers"] if m["frame"] == frame]
        try:
            image = BMP(buffers[frame])
        except Incomplete as error:
            rows.extend(dict(name=m["name"], frame=frame, status="incomplete", reason=str(error)) for m in markers)
            continue
        for marker in markers:
            row = dict(name=marker["name"], frame=frame, expected_xy=marker["expected_xy"])
            try:
                measured = center(image, marker)
                error = math.dist(measured, marker["expected_xy"])
                row.update(measured_xy=measured, error_px=error, status="passed" if error <= 0.5 else "failed")
            except Incomplete as error:
                row.update(status="incomplete", reason=str(error))
            rows.append(row)
    statuses = [r["status"] for r in rows]
    status = "failed" if "failed" in statuses else "incomplete" if len(rows) != 12 or "incomplete" in statuses else "passed"
    return dict(status=status, comparison_complete=len(rows) == 12 and "incomplete" not in statuses, dimensions=[W, H], resampling_applied=False, rows=rows)

def synthetic(calibration, frame, shift=0.0, bits=24, top_down=False):
    step = bits // 8
    pixels = bytearray(bytes((32, 32, 32, 255)[:step]) * (W * H))
    for marker in calibration["markers"]:
        if marker["frame"] != frame:
            continue
        cx, cy = marker["expected_xy"]
        cx += shift
        color = [round(255 * v) for v in marker["rgb"]]
        for y in range(math.floor(cy) - 9, math.floor(cy) + 10):
            for x in range(math.floor(cx) - 9, math.floor(cx) + 10):
                covered = 0
                for sy in range(8):
                    for sx in range(8):
                        dx = abs(x + (sx + 0.5) / 8 - 0.5 - cx)
                        dy = abs(y + (sy + 0.5) / 8 - 0.5 - cy)
                        covered += (dx <= 8 and dy <= 1.5) or (dx <= 1.5 and dy <= 8)
                alpha = covered / 64
                rgb = [round(32 + alpha * (v - 32)) for v in color]
                row = y if top_down else H - 1 - y
                pos = (row * W + x) * step
                pixels[pos:pos + 3] = bytes(reversed(rgb))
    header = struct.pack("<2sIHHI", b"BM", 54 + len(pixels), 0, 0, 54)
    header += struct.pack("<IiiHHIIiiII", 40, W, -H if top_down else H, 1, bits, 0, len(pixels), 0, 0, 0, 0)
    return header + pixels

def self_test(calibration):
    good = {f: synthetic(calibration, f, bits=32 if f == 3 else 24, top_down=f != 0) for f in FRAMES}
    require(compare(calibration, good)["status"] == "passed", "correct-cross self-test")
    shifted = dict(good)
    shifted[0] = synthetic(calibration, 0, shift=0.75)
    require(compare(calibration, shifted)["status"] == "failed", "shift rejection self-test")
    for malformed in (b"BM", good[0][:53], good[0][:-1]):
        try:
            BMP(malformed)
        except Incomplete:
            continue
        raise Incomplete("malformed-BMP self-test")
    return dict(status="synthetic_self_test_passed", actual_mmd_measurement="not_run")

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("calibration")
    parser.add_argument("--frame0")
    parser.add_argument("--frame3")
    parser.add_argument("--frame5")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    try:
        raw = read_limited(args.calibration, 1_000_000)
        calibration = json.loads(raw)
        require((calibration["width"], calibration["height"]) == (W, H) and calibration["frames"] == list(FRAMES), "unsupported calibration")
        require(calibration["cross_half_arm_pixels"] == 8 and calibration["cross_half_thickness_pixels"] == 1.5, "unsupported cross geometry")
        markers = calibration["markers"]
        require(len(markers) == 12 and len({m["name"] for m in markers}) == 12, "calibration must contain 12 unique markers")
        for frame in FRAMES:
            group = [m for m in markers if m["frame"] == frame]
            require(len(group) == 4 and {m["point_index"] for m in group} == set(range(4)), "each frame requires four distinct points")
        for marker in markers:
            xy, rgb = marker["expected_xy"], marker["rgb"]
            require(len(xy) == 2 and all(math.isfinite(v) for v in xy) and 18 <= xy[0] < W - 18 and 18 <= xy[1] < H - 18, "invalid expected center")
            require(len(rgb) == 3 and all(math.isfinite(v) and 0 <= v <= 1 for v in rgb) and chroma(rgb) is not None, "invalid marker color")
        if args.self_test:
            result = self_test(calibration)
        else:
            paths = {0: args.frame0, 3: args.frame3, 5: args.frame5}
            require(all(paths.values()), "explicit --frame0/--frame3/--frame5 are required")
            buffers = {f: read_limited(p, 16_000_000) for f, p in paths.items()}
            result = compare(calibration, buffers)
            result["bitmap_sha256"] = {f: hashlib.sha256(b).hexdigest() for f, b in buffers.items()}
            result["native_render_provenance"] = "requires caller evidence"
        result["calibration_sha256"] = hashlib.sha256(raw).hexdigest()
    except (Incomplete, OSError, ValueError, KeyError, TypeError) as error:
        result = dict(status="incomplete", reason=str(error))
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return {"passed": 0, "synthetic_self_test_passed": 0, "failed": 1}.get(result["status"], 2)

if __name__ == "__main__":
    sys.exit(main())
