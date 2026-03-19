.PHONY: demo

demo:
	@echo "dq-prof demo (alert-heavy dataset)" && \
	./target/debug/dq-prof examples/demo_alerts.csv --fail-on warning --color never || true
