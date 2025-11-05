#!/usr/bin/env python3
"""
seed.py

Read `TestReviews.csv` and POST review records to the backend `/reviews` endpoint.

Behavior & mapping:
- Input CSV expected to have columns: `review` and `class` (as in the attached TestReviews.csv).
- If `class` == '0' (or 0) -> assign `review_rating` randomly 1-2.
- Otherwise -> assign `review_rating` randomly 3-5.
- `product_id` is random integer between 1 and 10 (inclusive) unless overridden.
- `review_title` is generated from the first 60 chars of the review (trimmed) by default.

Usage examples:
  python seed.py --file TestReviews.csv --url http://localhost:8000/reviews --dry-run
  python seed.py --file TestReviews.csv --limit 50 --delay 0.2

Requires: requests (pip install requests)
"""

import argparse
import csv
import random
import time
import sys
from typing import Optional

try:
	import requests
except Exception as e:
	print("Missing dependency 'requests'. Install with: pip install requests")
	raise


def generate_title(body: str, max_len: int = 60) -> str:
	if not body:
		return ""
	s = " ".join(body.strip().split())
	if len(s) <= max_len:
		return s
	# prefer splitting at sentence/word boundary
	cut = s[:max_len]
	last_space = cut.rfind(" ")
	if last_space > 10:
		return cut[:last_space] + "..."
	return cut + "..."


def parse_args():
	p = argparse.ArgumentParser(description="Seed reviews from CSV to backend /reviews endpoint.")
	p.add_argument("--file", "-f", default="TestReviews.csv", help="CSV file path (default: TestReviews.csv)")
	p.add_argument("--url", "-u", default="http://localhost:8000/reviews", help="Target POST URL")
	p.add_argument("--limit", "-n", type=int, default=0, help="Limit number of rows to send (0 = all)")
	p.add_argument("--delay", "-d", type=float, default=0.0, help="Delay between requests in seconds")
	p.add_argument("--dry-run", action="store_true", help="Print payloads instead of sending")
	p.add_argument("--product-min", type=int, default=1, help="Min product_id (inclusive)")
	p.add_argument("--product-max", type=int, default=10, help="Max product_id (inclusive)")
	p.add_argument("--timeout", type=float, default=10.0, help="Request timeout seconds")
	return p.parse_args()


def make_payload(review_text: str, cls: Optional[str], product_min: int, product_max: int) -> dict:
	body = review_text.strip()
	title = generate_title(body)
	# map class to rating
	try:
		cls_int = int(str(cls).strip()) if cls is not None and str(cls).strip() != "" else 1
	except Exception:
		cls_int = 1

	if cls_int == 0:
		rating = random.randint(1, 2)
	else:
		rating = random.randint(3, 5)

	product_id = random.randint(product_min, product_max)

	payload = {
		"review_title": title,
		"review_body": body,
		"product_id": str(product_id),
		"review_rating": rating,
	}
	return payload


def send_payload(url: str, payload: dict, timeout: float) -> requests.Response:
	headers = {
		"Accept": "*/*",
		"Content-Type": "application/json",
		"Origin": "http://localhost:3000",
		"Referer": "http://localhost:3000/",
		"User-Agent": "seed-script/1.0",
	}
	resp = requests.post(url, json=payload, headers=headers, timeout=timeout)
	return resp


def main():
	args = parse_args()

	sent = 0
	failed = 0

	try:
		f = open(args.file, newline="", encoding="utf-8", errors="replace")
	except Exception as e:
		print(f"Failed to open CSV file '{args.file}': {e}")
		sys.exit(2)

	reader = csv.DictReader(f)
	if "review" not in reader.fieldnames or "class" not in reader.fieldnames:
		print("CSV must contain 'review' and 'class' columns. Found: %s" % (reader.fieldnames,))
		sys.exit(2)

	for i, row in enumerate(reader):
		if args.limit and sent >= args.limit:
			break

		review_text = row.get("review", "")
		cls = row.get("class", None)

		if not review_text or review_text.strip() == "":
			# skip empty review rows
			continue

		payload = make_payload(review_text, cls, args.product_min, args.product_max)

		if args.dry_run:
			print(f"DRY RUN payload #{i+1}:", payload)
			sent += 1
		else:
			try:
				resp = send_payload(args.url, payload, timeout=args.timeout)
				if 200 <= resp.status_code < 300:
					print(f"OK #{i+1} -> {resp.status_code}")
					sent += 1
				else:
					print(f"FAILED #{i+1} -> {resp.status_code}: {resp.text}")
					failed += 1
			except Exception as e:
				print(f"EXCEPTION #{i+1}: {e}")
				failed += 1

		if args.delay and args.delay > 0:
			time.sleep(args.delay)

	f.close()

	print("\nSummary: sent=%d, failed=%d" % (sent, failed))


if __name__ == "__main__":
	main()
