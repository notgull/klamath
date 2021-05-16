# Apache 2.0 License
# 
# This Makefile assembles all of the resources necessary to build klamath.wad and then calls DeuTeX to 
# create the WAD.

BASEDIR=$(PWD)
CARGO=cargo
CP=cp
DEHACKED=dist/klamath.deh
DEUTEX=deutex
DEUTEX_BASIC_ARGS=-v0 -rate accept
DEUTEX_ARGS=$(DEUTEX_BASIC_ARGS) -doom2 bootstrap/
KLAMATH=dist/klamath.wad
UTIL_DIR=util
UTIL=$(UTIL_DIR)/target/release/klamath-util
RM=$(UTIL) rm

BOOTSTRAP=bootstrap/bootstrap.wad
COLORMAP=lumps/colormap.lmp
DEHLUMP=lumps/dehacked.lmp
DMXGUS=lumps/dmxgus.lmp
GENMIDI=lumps/genmidi.lmp
PLAYPAL=lumps/playpal.lmp

all: $(KLAMATH) $(DEHACKED)

$(KLAMATH): $(BOOTSTRAP) $(COLORMAP) $(DEHLUMP) $(DMXGUS) $(GENMIDI) $(PLAYPAL) wadinfo.txt
	$(RM) dist
	@mkdir -p dist
	$(DEUTEX) $(DEUTEX_ARGS) -iwad -build wadinfo.txt $@

$(DEHACKED): dehacked/dehacked.deh
	@mkdir -p dist
	$(CP) $< $@

$(PLAYPAL): $(UTIL)
	@mkdir -p lumps
	$(UTIL) playpal > $@

$(BOOTSTRAP): $(UTIL) $(PLAYPAL)
	@mkdir -p bootstrap
	$(UTIL) bootstrap < $(PLAYPAL) > $(BOOTSTRAP)

$(COLORMAP): $(UTIL) $(PLAYPAL)
	@mkdir -p lumps
	$(UTIL) colormap < $(PLAYPAL) > $(COLORMAP)

$(DEHLUMP): dehacked/dehacked.deh
	$(CP) $< $@

$(DMXGUS): $(UTIL) dmxgus/dmxgus.yml
	@mkdir -p lumps
	$(UTIL) dmxgus $(BASEDIR)/dmxgus/dmxgus.yml > $(DMXGUS)

$(GENMIDI): $(UTIL) $(wildcard genmidi/*)
	@mkdir -p lumps
	$(UTIL) genmidi $(BASEDIR)/genmidi > $(GENMIDI)

$(UTIL): $(wildcard $(UTIL_DIR)/src/*)
	cd $(UTIL_DIR); $(CARGO) build --release
