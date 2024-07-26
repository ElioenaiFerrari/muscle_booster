include .env

.PHONY: dev
.PHONY: prd

dev:
	cargo watch -x run

prd:
	docker build -t elioenaiferrari/muscle_booster .
	docker run -p 8000:8000 -e OPENAI_API_KEY=${OPENAI_API_KEY}  elioenaiferrari/muscle_booster