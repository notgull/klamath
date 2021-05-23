# Apache 2.0 License
# 
# This Makefile assembles all of the resources necessary to build klamath.wad and then calls DeuTeX to 
# create the WAD.

# PHONY: all

BASEDIR=$(PWD)
CARGO=cargo
CP=cp
DEHACKED=dist/klamath.deh
DEUTEX=deutex
DEUTEX_BASIC_ARGS=-v0 -rate accept
DEUTEX_ARGS=$(DEUTEX_BASIC_ARGS) -doom2 bootstrap/
KLAMATH=dist/klamath.wad
#MANUAL=dist/manual.pdf
PDFTEX=pdftex
UPLTEMPL=dist/klamath.txt
UTIL_DIR=util
UTIL=$(UTIL_DIR)/target/release/klamath-util
RM=$(UTIL) rm

BOOTSTRAP=bootstrap/doom2.wad
COLORMAP=lumps/colormap.lmp
DEHLUMP=lumps/dehacked.lmp
DMXGUS=lumps/dmxgus.lmp
GENMIDI=lumps/genmidi.lmp
PLAYPAL=lumps/playpal.lmp
WADINFO=wadinfo.txt

FLATS=$(wildcard flats/*)
LEVELS=levels/map01.wad
PATCHES=$(wildcard patches/*)
TEXTURES=textures/texture1.txt

RESOURCES=$(FLATS) $(LEVELS) $(PATCHES) $(TEXTURES)

all: $(KLAMATH) $(DEHACKED) $(UPLTEMPL)

$(KLAMATH): models_out $(BOOTSTRAP) $(COLORMAP) $(DEHLUMP) $(DMXGUS) $(GENMIDI) $(PLAYPAL) $(RESOURCES) $(WADINFO)
	$(RM) $(KLAMATH)
	@mkdir -p dist
	$(DEUTEX) $(DEUTEX_ARGS) -iwad -build $(WADINFO) $@

$(DEHACKED): dehacked/dehacked.deh
	@mkdir -p dist
	$(CP) $< $@

$(UPLTEMPL): klamath.txt
	@mkdir -p dist
	$(CP) $< $@

$(PLAYPAL): $(UTIL) playpal/playpal.lmp
	@mkdir -p lumps
	$(UTIL) playpal playpal/playpal.lmp > $@

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

#$(MANUAL): manual/manual.tex
#	@mkdir -p dist
#	$(PDFTEX) -synctex=1 -interaction=nonstopmode $< --output-directory=dist

include models/Makefile
