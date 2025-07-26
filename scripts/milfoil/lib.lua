local module = {};

local app = require("app");
app.queue.push_message("测试！**粗体**与*斜体*与 `code`", "markdown", 0);

return module;
