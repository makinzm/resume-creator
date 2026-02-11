"""
Automated demo recorder for resume-creator.
Drives the TUI with pexpect and writes asciinema v2 .cast files directly,
then converts to GIF using agg.

Scenario 1: Create a resume, save as JSON, export PDF and JSON
Scenario 2: Import the saved JSON, edit it, export PDF and JSON
"""
import os
import sys
import time
import json
import threading
import subprocess
import pexpect

BINARY        = os.path.abspath("../target/release/resume-creator")
CAST_1        = os.path.abspath("scenario1.cast")
CAST_2        = os.path.abspath("scenario2.cast")
GIF_1         = os.path.abspath("scenario1.gif")
GIF_2         = os.path.abspath("scenario2.gif")
COLS, ROWS    = 100, 35

JSON_SAVE     = os.path.expanduser("~/resume_demo.json")
PDF_1         = os.path.expanduser("~/resume_demo_s1.pdf")
JSON_1        = os.path.expanduser("~/resume_demo_s1.json")
PDF_2         = os.path.expanduser("~/resume_demo_s2.pdf")
JSON_2        = os.path.expanduser("~/resume_demo_s2.json")


class Recorder:
    """Drives a TUI process via pexpect and records output as asciinema v2."""

    def __init__(self, cols=COLS, rows=ROWS):
        self.cols = cols
        self.rows = rows
        self.frames = []
        self._start = None
        self._child = None
        self._lock = threading.Lock()
        self._stop = threading.Event()

    def _ts(self):
        return round(time.monotonic() - self._start, 6)

    def _reader(self):
        while not self._stop.is_set():
            try:
                data = self._child.read_nonblocking(size=8192, timeout=0.05)
                if data:
                    with self._lock:
                        self.frames.append([self._ts(), "o", data])
            except pexpect.TIMEOUT:
                continue
            except (pexpect.EOF, OSError):
                break

    def start(self):
        env = os.environ.copy()
        env["TERM"] = "xterm-256color"
        env["COLUMNS"] = str(self.cols)
        env["LINES"] = str(self.rows)
        self._child = pexpect.spawn(
            BINARY, env=env,
            dimensions=(self.rows, self.cols),
            encoding=None,   # bytes mode for accurate capture
            codec_errors="replace",
        )
        self._child.delaybeforesend = 0
        self._start = time.monotonic()
        t = threading.Thread(target=self._reader, daemon=True)
        t.start()
        self._thread = t

    def send(self, data: bytes):
        time.sleep(0.05)
        self._child.send(data)

    def type(self, text: str, delay=0.07):
        for ch in text:
            self._child.send(ch.encode())
            time.sleep(delay)

    def key(self, k: bytes, after=0.6):
        self._child.send(k)
        time.sleep(after)

    def wait(self, s=0.8):
        time.sleep(s)

    def stop(self):
        self._stop.set()
        try:
            self._child.close(force=True)
        except Exception:
            pass
        self._thread.join(timeout=2)

    def save(self, path, title=""):
        header = {
            "version": 2,
            "width": self.cols,
            "height": self.rows,
            "timestamp": int(time.time()),
            "title": title,
            "env": {"TERM": "xterm-256color"},
        }
        with open(path, "w", encoding="utf-8") as f:
            f.write(json.dumps(header) + "\n")
            with self._lock:
                for ts, kind, data in self.frames:
                    if isinstance(data, bytes):
                        data = data.decode("utf-8", errors="replace")
                    f.write(json.dumps([ts, kind, data]) + "\n")
        print(f"  cast saved: {path}  ({len(self.frames)} frames)")


def cast_to_gif(cast, gif):
    r = subprocess.run(
        ["agg", "--font-size", "14", cast, gif],
        capture_output=True, text=True
    )
    if r.returncode == 0:
        print(f"  gif  saved: {gif}")
    else:
        print(f"  agg failed: {r.stderr.strip()}")


# ── Scenario helpers ──────────────────────────────────────────────────────────

def fill_path(rec, path):
    """Clear the default path and type a custom one."""
    rec.key(b"\x15", after=0.2)   # Ctrl+U clear line
    rec.type(path, delay=0.04)
    rec.key(b"\r", after=1.0)


def export(rec, format_idx, out_path):
    """Open export menu, choose format by index (0=MD,1=JSON,2=PDF), set path."""
    rec.key(b"e", after=0.9)               # open export screen
    for _ in range(format_idx):
        rec.key(b"j", after=0.3)           # navigate to desired format
    rec.key(b"\r", after=0.6)              # confirm format
    fill_path(rec, out_path)               # set output path


# ── Scenario 1 ────────────────────────────────────────────────────────────────

def scenario1():
    """Create a fresh resume, save JSON, export PDF and JSON."""
    rec = Recorder()
    rec.start()
    rec.wait(2.0)                          # TUI init

    # ── Personal Info ──
    rec.key(b"i", after=0.5)
    rec.type("Yamada Taro",  delay=0.07); rec.key(b"\t", after=0.4)
    rec.type("taro@example.com", delay=0.07); rec.key(b"\t", after=0.4)
    rec.type("+81-90-1234-5678", delay=0.07); rec.key(b"\t", after=0.4)
    rec.type("Tokyo, Japan", delay=0.07); rec.key(b"\t", after=0.4)
    rec.type("https://github.com/yamada-taro", delay=0.07)
    rec.key(b"\x1b", after=0.8)           # Esc → exit edit mode

    # ── Summary ──
    rec.key(b"\t", after=0.6)             # Tab → Summary section
    rec.key(b"i",  after=0.4)
    rec.type("Full-stack engineer with 5 years of Rust experience.", delay=0.06)
    rec.key(b"\x1b", after=0.8)

    # ── Experience ──
    rec.key(b"\t", after=0.6)
    rec.key(b"a",  after=0.6)             # add entry
    rec.type("Acme Corp",        delay=0.07); rec.key(b"\t", after=0.3)
    rec.type("Senior Engineer",  delay=0.07); rec.key(b"\t", after=0.3)
    rec.type("2022-01",          delay=0.07); rec.key(b"\t", after=0.3)
    rec.type("2025-12",          delay=0.07); rec.key(b"\t", after=0.3)
    rec.type("Led Rust microservices platform; reduced latency 40%.", delay=0.06)
    rec.key(b"\x1b", after=0.8)

    # ── Skills ──
    rec.key(b"\t", after=0.4)             # → Education
    rec.key(b"\t", after=0.4)             # → Skills
    rec.key(b"a",  after=0.5)
    rec.type("Rust", delay=0.07); rec.key(b"\t", after=0.3)
    rec.type("Expert", delay=0.07)
    rec.key(b"\x1b", after=0.8)

    # ── Save JSON ──
    rec.key(b"s", after=0.7)
    fill_path(rec, JSON_SAVE)

    # ── Export PDF ──
    export(rec, format_idx=2, out_path=PDF_1)

    # ── Export JSON ──
    export(rec, format_idx=1, out_path=JSON_1)

    # ── Quit ──
    rec.key(b"q", after=0.5)
    rec.key(b"y", after=1.0)
    rec.stop()
    return rec


# ── Scenario 2 ────────────────────────────────────────────────────────────────

def scenario2():
    """Import saved JSON, edit it, export updated PDF and JSON."""
    rec = Recorder()
    rec.start()
    rec.wait(2.0)

    # ── Import ──
    rec.key(b"j", after=0.3)              # navigate to Import on home menu
    rec.key(b"j", after=0.3)
    rec.key(b"\r", after=0.7)
    rec.type(JSON_SAVE, delay=0.04)
    rec.key(b"\r", after=1.2)             # load file

    # ── Edit summary ──
    rec.key(b"\t", after=0.6)
    rec.key(b"i",  after=0.4)
    rec.key(b"\x15", after=0.2)           # clear existing text
    rec.type("Senior Rust engineer. Open-source contributor. 5 YOE.", delay=0.06)
    rec.key(b"\x1b", after=0.8)

    # ── Add skill ──
    rec.key(b"\t", after=0.4)             # → Experience
    rec.key(b"\t", after=0.4)             # → Education
    rec.key(b"\t", after=0.4)             # → Skills
    rec.key(b"a",  after=0.5)
    rec.type("Japanese", delay=0.07); rec.key(b"\t", after=0.3)
    rec.type("Native",   delay=0.07)
    rec.key(b"\x1b", after=0.8)

    # ── Export updated PDF ──
    export(rec, format_idx=2, out_path=PDF_2)

    # ── Export updated JSON ──
    export(rec, format_idx=1, out_path=JSON_2)

    # ── Quit ──
    rec.key(b"q", after=0.5)
    rec.key(b"y", after=1.0)
    rec.stop()
    return rec


# ── Main ──────────────────────────────────────────────────────────────────────

if __name__ == "__main__":
    print("=== Resume Creator Demo Recorder ===")
    if not os.path.exists(BINARY):
        print(f"ERROR: binary not found: {BINARY}")
        sys.exit(1)

    for f in [JSON_SAVE, PDF_1, JSON_1, PDF_2, JSON_2]:
        if os.path.exists(f):
            os.remove(f)

    print("\n[1/4] Scenario 1 — create, save, export…")
    r1 = scenario1()
    r1.save(CAST_1, title="Resume Creator: Create & Export")

    print("[2/4] Converting to GIF…")
    cast_to_gif(CAST_1, GIF_1)

    print("\n[3/4] Scenario 2 — import, edit, export…")
    r2 = scenario2()
    r2.save(CAST_2, title="Resume Creator: Import, Edit & Export")

    print("[4/4] Converting to GIF…")
    cast_to_gif(CAST_2, GIF_2)

    print("\nAll done!")
    print(f"  demo/scenario1.gif  →  embedded in README.md")
    print(f"  demo/scenario2.gif  →  embedded in README.md")
    print(f"\nReplay:")
    print(f"  asciinema play {CAST_1}")
    print(f"  asciinema play {CAST_2}")
