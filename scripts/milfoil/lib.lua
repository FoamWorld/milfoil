local module = {};

require("app_aliases");
require("hierarchy");
local app = require("app");
app.i18n.load_file("milfoil.ftl");

local obj = require("objects");

require("milfoil/registers");
obj.types["milfoil-seat"] = require("milfoil/types/seat");

local actives = require("milfoil/actives");

Tree:add_child("#1", require("milfoil/objects/staff_lounge"));
Tree:add_child("#2", require("milfoil/objects/sofa"), "#1");
actives.operator.position = "#2";

local function update()
	local node = Tree.nodes[actives.operator.position];
	obj.apply(node, "setup_actions");
end

update();

return module;
