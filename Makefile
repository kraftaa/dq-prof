.PHONY: demo

demo:
	@echo "dq-prof demo (wow dataset)" && \
	./target/debug/dq-prof examples/wow_demo.csv --fail-on warning --color never || true
