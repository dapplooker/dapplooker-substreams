DSN ?= clickhouse://default:password@<host>:<port>/ethereum
ENDPOINT ?= mainnet.eth.streamingfast.io:443


.PHONY: build
build:
	cargo +stable build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml db_out -s 12292922 -t +10

.PHONY: stream_db
stream_db: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: create_db
create_db:
	substreams-sink-sql setup "$(DSN)" manifest/sink_eth.yaml

.PHONY: sink_db_out
sink_db_out: build
	TRACE=true substreams-sink-sql run "$(DSN)" manifest/sink_eth.yaml 12296288:17942367 --undo-buffer-size 10
