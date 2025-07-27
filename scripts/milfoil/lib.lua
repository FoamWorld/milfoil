local module = {};

require("app_aliases");
local app = require("app");
app.i18n.load_file("milfoil.ftl");
app.queue.push_marked(app.i18n.t("milfoil-test-markdown"));

return module;
