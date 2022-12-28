let music = Application("Music")
let app = Application.currentApplication()
app.includeStandardAdditions = true


let metadata_path = Path("/Users/bilalshussain/Code/local/vgmdb/scripts/metadata.json")
let metadata = JSON.parse(app.read(metadata_path))
metadata