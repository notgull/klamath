# Apache 2.0 License

BASEDIR=$(PWD)
CARGO=cargo
DEUTEX=deutex
DEUTEX_BASIC_ARGS=-v0 -rate accept
DEUTEX_ARGS=$(DEUTEX_BASIC_ARGS) -doom2 bootstrap/
KLAMATH=dist/klamath.wad
UTIL_DIR=util
UTIL=$(UTIL_DIR)/target/release/klamath-util
RM=$(UTIL) rm

BOOTSTRAP=bootstrap/bootstrap.wad
COLORMAP=lumps/colormap.lmp
GENMIDI=lumps/genmidi.lmp
PLAYPAL=lumps/playpal.lmp

all: $(KLAMATH)

$(KLAMATH): $(BOOTSTRAP) $(COLORMAP) $(GENMIDI) $(PLAYPAL) wadinfo.txt
	$(RM) dist
	@mkdir -p dist
	$(DEUTEX) $(DEUTEX_ARGS) -iwad -build wadinfo.txt $@

$(PLAYPAL): $(UTIL)
	@mkdir -p lumps
	$(UTIL) playpal > $@

$(BOOTSTRAP): $(UTIL) $(PLAYPAL)
	@mkdir -p bootstrap
	$(UTIL) bootstrap < $(PLAYPAL) > $(BOOTSTRAP)

$(COLORMAP): $(UTIL) $(PLAYPAL)
	@mkdir -p lumps
	$(UTIL) colormap < $(PLAYPAL) > $(COLORMAP)

$(GENMIDI): $(UTIL) $(wildcard genmidi/*)
	@mkdir -p lumps
	$(UTIL) genmidi $(BASEDIR)/genmidi > $(GENMIDI)

$(UTIL): $(wildcard $(UTIL_DIR)/src/*)
	cd $(UTIL_DIR); $(CARGO) build --release
