const fs = require("fs");
const path = require("path");
const yaml = require("js-yaml");

try {
  const YAML_FILE_NAME = "pls_config.yml";
  const JSON_FILE_NAME = "pls_config.json";

  // Read from YAML file
  const ymlPath = path.join(__dirname, YAML_FILE_NAME);
  const doc = yaml.load(fs.readFileSync(ymlPath));

  // Wrangle data
  doc["$id"] = doc["$id"].replace(/\.yml$/, ".json");

  // Prepare destination directory
  const schemasDir = path.join(
    __dirname,
    "..",
    "docs",
    ".vuepress",
    "public",
    "schemas"
  );
  fs.mkdirSync(schemasDir, { recursive: true });

  // Write to JSON file
  const jsonPath = path.join(schemasDir, JSON_FILE_NAME);
  fs.writeFileSync(jsonPath, JSON.stringify(doc, null, 2));

  // Write to YAML file
  fs.copyFileSync(ymlPath, path.join(schemasDir, YAML_FILE_NAME));
} catch (e) {
  console.error(e);
}
