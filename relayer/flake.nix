{
  description = "VM Demo";
  inputs = {
    nixpkgs-unstable.url = "nixpkgs/nixos-unstable";
    nixpkgs.url = "github:NixOS/nixpkgs/60ddbcfc9e5f02f97564fa01a5646b62d82e0756";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, nixpkgs-unstable, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs {
            inherit system;
            config.allowUnfree = true;
          };
          pkgs-unstable = import nixpkgs-unstable {
            inherit system;
            config.allowUnfree = true;
          };
      in
        rec {
          # Another non-portable single binary zeit/pkg delivered under the hood :)
          packages.hasura-cli-ext = pkgs.stdenv.mkDerivation rec {
            pname = "hasura-cli-ext";
            version = "1.3.3";
            src = pkgs.fetchurl {
              url = "https://github.com/hasura/graphql-engine/releases/download/v${version}/cli-ext-hasura-linux.tar.gz";
              sha256 = "0a49w1lb3j33i8yz232inr7lm8j6vxg6y52izljnn71w2l25mqlw";
            };
            sourceRoot = ".";
            buildPhase = ":";
            installPhase = ''
                  mkdir -p $out/bin
                  cp cli-ext-hasura-linux $out/bin/
                  chmod +x $out/bin/cli-ext-hasura-linux
                '';
            preFixup =
              let
                libPath = pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc ];
              in ''
                  orig_size=$(stat --printf=%s $out/bin/cli-ext-hasura-linux)

                  patchelf --set-interpreter "$(cat $NIX_CC/nix-support/dynamic-linker)" $out/bin/cli-ext-hasura-linux
                  patchelf --set-rpath ${libPath} $out/bin/cli-ext-hasura-linux
                  new_size=$(stat --printf=%s $out/bin/cli-ext-hasura-linux)

                  ###### zeit-pkg fixing starts here.
                  # we're replacing plaintext js code that looks like
                  # PAYLOAD_POSITION = '1234                  ' | 0
                  # [...]
                  # PRELUDE_POSITION = '1234                  ' | 0
                  # ^-----20-chars-----^^------22-chars------^
                  # ^-- grep points here
                  #
                  # var_* are as described above
                  # shift_by seems to be safe so long as all patchelf adjustments occur
                  # before any locations pointed to by hardcoded offsets

                  var_skip=20
                  var_select=22
                  shift_by=$(expr $new_size - $orig_size)

                  function fix_offset {
                    # $1 = name of variable to adjust
                    location=$(grep -obUam1 "$1" $out/bin/cli-ext-hasura-linux | cut -d: -f1)
                    location=$(expr $location + $var_skip)

                    value=$(dd if=$out/bin/cli-ext-hasura-linux iflag=count_bytes,skip_bytes skip=$location \
                               bs=1 count=$var_select status=none)
                    value=$(expr $shift_by + $value)

                    echo -n $value | dd of=$out/bin/cli-ext-hasura-linux bs=1 seek=$location conv=notrunc
                  }

                  fix_offset PAYLOAD_POSITION
                  fix_offset PRELUDE_POSITION
            '';
            dontStrip = true;
          };
          # Patch existing CLI to directly provided cli-ext as parameter
          packages.hasura-cli-wrapped = pkgs.symlinkJoin {
              name = "hasura";
              paths = [ pkgs-unstable.hasura-cli ];
              buildInputs = [ pkgs.makeWrapper ];
              postBuild = ''
                wrapProgram $out/bin/hasura \
                  --add-flags "--cli-ext-path ${packages.hasura-cli-ext}/bin/cli-ext-hasura-linux"
              '';
          };
          devShell = pkgs.mkShell {
            buildInputs = [
              pkgs.docker-compose
              pkgs.pkg-config
              pkgs.openssl
              pkgs.nodejs-16_x
              pkgs.jq
              packages.hasura-cli-wrapped
            ];
            DATABASE_URL = "postgres://postgres:postgrespassword@0.0.0.0:5432/postgres";
          };
        }
    );
}
