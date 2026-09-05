CONTAINER  ?= container
IMAGE      := docker.io/library/rust:slim-bookworm
CARGO_VOL  := rust-cargo-home
TARGET_VOL := rust-builder-target
UIDGID     := $(shell id -u):$(shell id -g)

DOCKER_RUN = $(CONTAINER) run --rm \
	-c 4 -m 4G \
	-u $(UIDGID) \
	-v $(CURDIR)/builder:/work/builder \
	-v $(CURDIR)/mks:/work/mks:ro \
	-v $(CURDIR)/docs:/work/docs \
	-v $(CARGO_VOL):/usr/local/cargo/registry \
	-v $(TARGET_VOL):/rust-target \
	-e CARGO_TARGET_DIR=/rust-target \
	-e HOME=/tmp \
	-w /work/builder $(IMAGE)

.PHONY: all build test volumes-init cache-clean

all: build

volumes-init:
	@$(CONTAINER) system start --enable-kernel-install >/dev/null 2>&1 || true
	@created=0; \
	for vol in $(CARGO_VOL) $(TARGET_VOL); do \
		if ! $(CONTAINER) volume inspect $$vol >/dev/null 2>&1; then \
			echo "creating volume $$vol"; \
			$(CONTAINER) volume create $$vol >/dev/null; \
			created=1; \
		fi; \
	done; \
	if [ "$$created" = "1" ]; then \
		echo "preparing volumes (chown to $(UIDGID))"; \
		$(CONTAINER) run --rm \
			-v $(CARGO_VOL):/cargo-home \
			-v $(TARGET_VOL):/rust-target \
			$(IMAGE) chown -R $(UIDGID) /cargo-home /rust-target; \
	fi

build: volumes-init
	$(DOCKER_RUN) cargo run -- --in ../mks/ --out ../docs

test: volumes-init
	$(DOCKER_RUN) cargo test

cache-clean:
	@for vol in $(CARGO_VOL) $(TARGET_VOL); do \
		$(CONTAINER) volume delete $$vol 2>/dev/null || true; \
	done
