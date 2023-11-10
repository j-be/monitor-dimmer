build: src/*.rs
	cargo build -r

install: build
	sudo mkdir -p /opt/monitor-dimmer
	sudo install -m111 target/release/monitor-dimmer /opt/monitor-dimmer/
	sudo cp monitor-dimmer.service /etc/systemd/system/
	sudo systemctl daemon-reload
	sudo systemctl enable monitor-dimmer

clean:
	rm -rf target/
