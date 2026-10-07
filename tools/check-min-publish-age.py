#!/usr/bin/env -S uv run --no-project --with tomli python
import argparse
import http.client
import json
import sys
import time
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

argparser = argparse.ArgumentParser()
argparser.add_argument("--age-days", type=int)
argparser.add_argument("lockfile", nargs="?", default="Cargo.lock")
args = argparser.parse_args()


def age_days():
    if args.age_days is not None:
        return args.age_days
    try:
        with open(".cargo/config.toml", "rb") as file:
            config = tomllib.load(file)
    except FileNotFoundError:
        sys.exit("no --age-days and no .cargo/config.toml")
    value = config.get("registry", {}).get("global-min-publish-age")
    days = str(value or "").removesuffix("days").strip()
    if not days.isdigit():
        sys.exit("no --age-days and no global-min-publish-age in .cargo/config.toml")
    return int(days)


def index_path(name):
    if len(name) <= 2:
        return f"{len(name)}/{name}"
    if len(name) == 3:
        return f"3/{name[0]}/{name}"
    return f"{name[:2]}/{name[2:4]}/{name}"


def fetch_index(name, vers):
    url = f"https://index.crates.io/{index_path(name.lower())}"
    last_error = None
    for attempt in range(3):
        try:
            with urllib.request.urlopen(url, timeout=30) as response:
                return response.read().splitlines()
        except urllib.error.HTTPError as error:
            if error.code == 404:
                return None
            last_error = error
        except (OSError, http.client.HTTPException) as error:
            last_error = error
        if attempt < 2:
            time.sleep(2**attempt)
    sys.exit(f"{name} {vers} fetch failed: {last_error}")


def check(entry, cutoff):
    name, vers = entry
    lines = fetch_index(name, vers)
    if lines is None:
        return f"{name} {vers} not found in index"
    for line in lines:
        try:
            version = json.loads(line)
        except ValueError as error:
            return f"{name} {vers} unexpected index response: {error}"
        if version["vers"] == vers:
            pubtime = version.get("pubtime")
            if pubtime is None:
                return f"{name} {vers} missing publish time"
            published = datetime.fromisoformat(pubtime.replace("Z", "+00:00")).timestamp()
            if published > cutoff:
                return f"{name} {vers} published {pubtime}"
            return None
    return f"{name} {vers} not found in index"


age = age_days()
cutoff = time.time() - age * 86400
lock = tomllib.load(open(args.lockfile, "rb"))
entries = [
    (package["name"], package["version"])
    for package in lock.get("package", [])
    if package.get("source") == "registry+https://github.com/rust-lang/crates.io-index"
]
if not entries:
    sys.exit(f"no packages found in {args.lockfile}")
with ThreadPoolExecutor(max_workers=16) as pool:
    results = list(pool.map(lambda entry: check(entry, cutoff), entries))
bad = [result for result in results if result]
print("\n".join(bad) or f"all {len(entries)} packages older than {age} days")
sys.exit(1 if bad else 0)
