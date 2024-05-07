DSN ?= clickhouse://default:@localhost:9000/erc20
ENDPOINT ?= mainnet.eth.streamingfast.io:443


.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run map_block -e ${ENDPOINT} -s 22207900 -t +1000 -o jsonl

.PHONY: stream_db_out
stream_db_out: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -s 821418 -o json

.PHONY: create_clickhouse_db
create_clickhouse_db: 
	substreams-sink-sql setup "$(DSN)" sink/substreams-clickhouse.dev.yaml	

.PHONY: sink_clickhouse_db_out
sink_clickhouse_db_out: build
	substreams-sink-sql run -e ${ENDPOINT} --on-module-hash-mistmatch=warn --flush-interval 1 "$(DSN)" sink/substreams-clickhouse.dev.yaml 821418: --undo-buffer-size 10
