local module = {};

require("app_aliases");
local app = require("app");
app.queue.push_marked("测试！**粗体**与*斜体*与 `code`");

return module;
