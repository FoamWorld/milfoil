local module = {};

require("app_aliases");
require("hierarchy");
local app = require("app");
app.i18n.load_file("milfoil.ftl");

local obj = require("objects");
for _, value in ipairs({ "bar", "item_supervision_screen", "physical", "seat" }) do
	obj.declare("milfoil-" .. value, require("milfoil/types/" .. value));
end

require("milfoil/registers");

local actives = require("milfoil/actives");

local tree = require("hierarchy");

tree:add_child("#1", require("milfoil/objects/staff_lounge"), nil);
tree:reach("root", "#1", tree.NODE_LOADED);
actives.operator:relocate("#1");

app.queue.push_plain(app.i18n.t("milfoil-plot-introduction-1"));

function module.routine()
	app.actions:clear();
	local pos = actives.operator.position;
	obj.apply(tree.nodes[actives.operator.position], "setup_actions");
	for id, _ in pairs(tree:get_children_id(pos)) do
		local node = tree.nodes[id];
		obj.apply(node, "setup_actions");
	end
	app.queue.flush();
end

function app.update()
	xpcall(module.routine, function(err)
		print(err);
	end);
end

app.update();

return module;
