all: install format lint test

install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

format:	
	black src --line-length 100 --verbose

lint:
	ruff check src/ --fix --verbose

test:
	python -m pytest -vv src/
	rm -rf *.png *.pdf

run:
	python src/main.py

rust_format:
	cargo fmt --manifest-path ./analysis/Cargo.toml

rust_lint:
	cargo clippy --manifest-path ./analysis/Cargo.toml -- -D warnings


rust_run:
	cargo run --manifest-path ./analysis/Cargo.toml

clean:
	rm -rf $(VENV)