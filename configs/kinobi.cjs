const path = require("path");
const {
  Kinobi,
  RenderJavaScriptVisitor,
  UpdateProgramsVisitor,
} = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = new Kinobi([path.join(idlDir, "mpl_trifle.json")]);

// Update programs.
kinobi.update(
  new UpdateProgramsVisitor({
    trifle: { name: "mplTrifle" },
  })
);

// Render JavaScript.
const jsDir = path.join(clientDir, "js", "src", "generated");
kinobi.accept(
  new RenderJavaScriptVisitor(jsDir, {
    prettier: require(path.join(clientDir, "js", ".prettierrc.json")),
    dependencyMap: {
      mplTokenMetadata: "@metaplex-foundation/mpl-token-metadata",
    },
  })
);
